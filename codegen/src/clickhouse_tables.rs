use std::{collections, fs, str};

use crate::{mms, pdr};

impl mms::PkColumns {
    fn get_clickhouse_sql(&self) -> String {
        use heck::SnakeCase;
        let cols = self
            .cols
            .iter()
            .map(|c| format!("`{}`", c.to_snake_case()))
            .collect::<Vec<_>>();
        cols.join(", ")
    }
}

impl mms::TableColumns {
    fn get_column_schema_clickhouse(&self) -> String {
        self.columns
            .iter()
            .map(|c| c.get_sql_clickhouse())
            .collect::<Vec<_>>()
            .join(",\n")
    }
}

impl mms::TableColumn {
    fn get_sql_clickhouse(&self) -> String {
        format!("`{}` {}", self.field_name(), self.sql_type_clickhouse(),)
    }
    fn sql_type_clickhouse(&self) -> String {
        if self.mandatory {
            format!("{}", self.data_type.as_sql_type_clickhouse())
        } else {
            format!("Nullable({})", self.data_type.as_sql_type_clickhouse())
        }
    }
}

impl mms::DataType {
    fn as_sql_type_clickhouse(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } => "String".into(), // investigate use of LowCardinality here
            mms::DataType::Char => "FixedString(1)".into(),
            mms::DataType::Date => "DateTime('Australia/Brisbane')".into(),
            mms::DataType::Decimal { precision, scale } => {
                format!("Decimal({},{})", precision, scale)
            }
            mms::DataType::Integer { precision } => format!("Decimal({},0)", precision),
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let rdr = fs::File::open("mmsdm.json")?;
    let mapping = fs::read_to_string("table_mapping.csv").unwrap();
    let mut map = collections::HashMap::new();

    for row in mapping.split("\n").skip(1) {
        if row.contains(',') {
            let pieces = row.split(",").collect::<Vec<&str>>();
            let mms = mms::Report {
                name: pieces[0].to_string(),
                sub_type: pieces[1].to_string(),
            };
            let pdr = pdr::Report {
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
    file_id Uuid,
    file_name LowCardinality(String),
    data_set LowCardinality(String),
    sub_type LowCardinality(String),
    version LowCardinality(Int8),
    inserted DateTime64,
)
ENGINE = MergeTree()
ORDER BY (file_name, data_set, sub_type, version, id);"#
        .to_string();
    let mut proc_str = String::new();
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();
    for (data_set, tables) in local_info.into_iter() {
        for (table_key, table) in tables.into_iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key,
            };
            if let Some(pdr_report) = map.get(&mms_report) {
                let create_table = format!(
                    r#"
create table {table_name} (
file_id Uuid,
{columns}
)
ENGINE = MergeTree()
ORDER BY ({order_by_cols});
                        "#,
                    table_name = pdr_report.struct_name(),
                    columns = table.columns.get_column_schema_clickhouse(),
                    order_by_cols = table.primary_key_columns.get_clickhouse_sql(),
                );

                table_str.push_str(&create_table);
            }
        }
    }
    fs::write("sql/clickhouse_tables.sql", table_str)?;
    Ok(())
}
