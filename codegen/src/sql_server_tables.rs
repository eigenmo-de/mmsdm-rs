use std::{fs, str};
use heck::ToSnakeCase;

use crate::{
    mms,
    rust::{TableMapping, VERSION},
};

impl mms::PkColumns {
    fn get_sql(&self) -> String {
        let cols = self
            .cols
            .iter()
            .map(|c| c.to_snake_case())
            .collect::<Vec<_>>();
        format!("primary key ([{}])", cols.join("],["))
    }

    fn merge_check(&self) -> String {
        let mut sql = String::new();
        for (idx, col) in self.cols.iter().enumerate() {
            let col = col.to_snake_case();
            if idx == 0 {
                sql.push_str(&format!("    tgt.[{0}] = src.[{0}]\n", col));
            } else {
                sql.push_str(&format!("    and tgt.[{0}] = src.[{0}]\n", col));
            }
        }
        sql
    }
}

impl mms::TableColumns {
    fn merge_update(&self) -> String {
        self.columns
            .iter()
            .map(|c| format!("tgt.[{0}] = src.[{0}]", c.field_name()))
            .collect::<Vec<_>>()
            .join(",\n        ")
    }

    fn get_sql(&self) -> String {
        self.columns
            .iter()
            .map(|c| format!("{},", c.get_sql()))
            .collect::<Vec<_>>()
            .join("\n    ")
    }
    fn get_columns_sql(&self, prefix: Option<&'static str>) -> String {
        self.columns
            .iter()
            .map(|c| {
                if let Some(pfx) = prefix {
                    format!("{}.[{}]", pfx, c.field_name())
                } else {
                    format!("[{}]", c.field_name())
                }
            })
            .collect::<Vec<_>>()
            .join(",\n        ")
    }
    fn get_column_schema(&self) -> String {
        self.columns
            .iter()
            .map(|c| format!("[{}] {}", c.field_name(), c.data_type.as_sql_type()))
            .collect::<Vec<_>>()
            .join(",\n        ")
    }
}

impl mms::TableColumn {
    fn get_sql(&self) -> String {
        format!("[{}] {}", self.field_name(), self.sql_type(),)
    }
    fn sql_type(&self) -> String {
        if self.mandatory {
            format!("{} not null", self.data_type.as_sql_type())
        } else {
            format!("{} null", self.data_type.as_sql_type())
        }
    }
}

impl mms::DataType {
    fn as_sql_type(&self) -> String {
        match self {
            mms::DataType::Varchar { length } => format!("varchar({})", length),
            mms::DataType::Char => "char(1)".into(),
            mms::DataType::Date => "datetime2".into(),
            mms::DataType::DateTime => "datetime2".into(),
            mms::DataType::Decimal { precision, scale } => {
                format!("decimal({},{})", precision, scale)
            }
            mms::DataType::Integer { precision } => format!("decimal({},0)", precision),
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let rdr = fs::File::open(format!("mmsdm_v{VERSION}.json"))?;
    let map = TableMapping::read()?;

    // abv
    let mut table_str = r#"
create schema mmsdm
go
create schema mmsdm_proc
go
create table mmsdm.FileLog (
    file_log_id bigint identity(1,1) not null primary key,
    data_source varchar(255) not null,
    file_name varchar(255) not null,
    from_participant varchar(255) not null,
    to_participant varchar(255) not null,
    effective_date date,
    effective_time time,
    serial_number bigint not null,
    file_name_2 varchar(255) null,
    serial_number_2 bigint null,
    data_set varchar(255) not null,
    sub_type varchar(255) not null,
    version tinyint not null,
    [status] char(1) not null default 'P' check ([status] in ('P','E','C')),
    rows_inserted bigint not null default 0,
    total_rows bigint not null,
    message varchar(max) null,
    check ((status != 'E' and message is null) or (status = 'E' and message is not null)),
    unique (to_participant, serial_number, data_set, sub_type, version)
)
go
            "#
    .to_string();
    let mut proc_str = String::new();
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();
    for (_, tables) in local_info.into_iter() {
        for (table_key, table) in tables.into_iter() {
            let mms_report = mms::Report {
                sub_type: table_key,
            };
            if let Some(pdr_report) = map.get(&mms_report) {
                let create_table = format!(
                    r#"
create table mmsdm.{table_name} (
    file_log_id bigint not null references mmsdm.FileLog(file_log_id),
    {columns}
    {primary_key}
)
go

create nonclustered index Mmsdm{table_name}FileLogId on mmsdm.{table_name}(file_log_id)
go
                        "#,
                    table_name = pdr_report.sql_table_name(),
                    columns = table.columns().get_sql(),
                    primary_key = table.primary_key_columns().get_sql(),
                    // primary_key = ""
                );

                table_str.push_str(&create_table);

                if table
                    .columns()
                    .columns
                    .iter()
                    .any(|col| col.name.to_lowercase() == "lastchanged")
                {
                    let merge_proc = format!(
                        r#"
create or alter procedure mmsdm_proc.Insert{target_table}
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.{target_table} as tgt 
using (
    select 
        {select_columns_d}
    from openjson(@data) with (
        {column_schema}
    ) d
) as src 
on (
{merge_check}
)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        {update_columns}
when not matched
    then insert (
        file_log_id,
        {insert_columns}
    ) values (
        @file_log_id,
        {select_columns_src}
    );
    
end
go"#,
                        target_table = pdr_report.sql_table_name(),
                        merge_check = table.primary_key_columns().merge_check(),
                        insert_columns = table.columns().get_columns_sql(None),
                        select_columns_d = table.columns().get_columns_sql(Some("d")),
                        select_columns_src = table.columns().get_columns_sql(Some("src")),
                        update_columns = table.columns().merge_update(),
                        column_schema = table.columns().get_column_schema(),
                    );
                    proc_str.push_str(&merge_proc);
                } else {
                    let insert_proc = format!(
                        r#"
create or alter procedure mmsdm_proc.Insert{target_table}
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.{target_table}(
file_log_id,
{insert_columns}
)
select 
@file_log_id,
{select_columns}
from openjson(@data) with (
{column_schema}
) d
end
go"#,
                        target_table = pdr_report.sql_table_name(),
                        insert_columns = table.columns().get_columns_sql(None),
                        select_columns = table.columns().get_columns_sql(Some("d")),
                        column_schema = table.columns().get_column_schema(),
                    );
                    proc_str.push_str(&insert_proc);
                }
            }
        }
    }
    fs::write("sql/mmsdm_tables.sql", table_str)?;
    fs::write("sql/mmsdm_procs.sql", proc_str)?;
    Ok(())
}
