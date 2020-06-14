use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections, fs, iter, str, string};

pub fn run() {
    let rdr = fs::File::open("mmsdm.json")?;
    let mapping = fs::read_to_string("table_mapping.csv").unwrap();
    let mut map = collections::HashMap::new();
    use std::iter::Iterator;
    for row in mapping.split("\n").skip(1) {
        if row.contains(',') {
            let pieces = row.split(",").collect::<Vec<&str>>();
            let mms = MMSReport {
                name: pieces[0].to_string(),
                sub_type: pieces[1].to_string(),
            };
            let pdr = PDRReport {
                name: pieces[2].to_string(),
                sub_type: pieces[3].to_string(),
                version: pieces[4].parse().unwrap(),
                transaction_type: pieces[5].to_string(),
                row_filter: pieces[6].to_string(),
            };
            map.insert(mms, pdr);
        }
    }
    // abv
    let mut table_str = r#"
create table FileLog (
    id bigint identity(1,1) not null primary key,
    file_name varchar(255) not null,
    data_set varchar(255) not null,
    sub_type varchar(255) not null,
    version tinyint not null
)
go
            "#
    .to_string();
    let mut proc_str = String::new();
    let local_info: MMSPackages = serde_json::from_reader(rdr).unwrap();
    for (data_set, tables) in local_info.into_iter() {
        for (table_key, table) in tables.into_iter() {
            let mms_report = MMSReport {
                name: data_set.clone(),
                sub_type: table_key,
            };
            if let Some(pdr_report) = map.get(&mms_report) {
                let create_table = format!(
                    r#"
create table {table_name} (
file_log_id bigint not null references FileLog(id),
{columns}
{primary_key}
)
go
create clustered columnstore index cci_{table_name} on {table_name};
go
                        "#,
                    table_name = pdr_report.struct_name(),
                    columns = table.columns.get_sql(),
                    //primary_key = table.primary_key_columns.get_sql(),
                    primary_key = ""
                );

                table_str.push_str(&create_table);

                use heck::CamelCase;
                let insert_proc = format!(
                    r#"
create or alter procedure Insert{target_table}
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = '{data_set}'
    and sub_type = '{sub_type}'
    and version = '{version}'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = '{data_set}'
    and sub_type = '{sub_type}'
    and version = '{version}'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, '{data_set}', '{sub_type}', {version});

insert into {target_table}(
file_log_id,
{insert_columns}
)
select 
(select h.id from @header h),
{select_columns}
from openjson(@data) with (
{column_schema}
) d
end
go"#,
                    target_table = pdr_report.struct_name(),
                    data_set = pdr_report.name.to_camel_case(),
                    sub_type = pdr_report.sub_type.to_camel_case(),
                    version = pdr_report.version,
                    insert_columns = table.columns.get_columns_sql(None),
                    select_columns = table.columns.get_columns_sql(Some("d")),
                    column_schema = table.columns.get_column_schema(),
                );
                proc_str.push_str(&insert_proc);
            }
        }
    }
    fs::write("sql/mmsdm_tables.sql", table_str)?;
    fs::write("sql/mmsdm_procs.sql", proc_str)?;
    Ok(())
}
