use std::{collections, fs, str};

use crate::{mms, pdr};

impl mms::DataType {
    fn as_pyarrow_schema_type(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } => "pyarrow.large_utf8()".to_string(),
            mms::DataType::Char => "pyarrow.large_utf8()".to_string(),
            mms::DataType::Date => "pyarrow.timestamp('s')".to_string(),
            mms::DataType::DateTime => "pyarrow.timestamp('s')".to_string(),
            mms::DataType::Decimal { .. } => {
                // we simply ignore the AEMO specified precision and scale
                // this is because the CSV files often contain values outside these precisions
                // the decimal128 has 34 significant digits anyway so we may as well use the largest
                // possible precision. The 18 for scale is the largest amount that has been empirically
                // observed, however this balance may need to be updated in future.
                "pyarrow.decimal128(34,18)".to_string()
            }
            mms::DataType::Integer { .. } => "pyarrow.int64()".to_string(),
        }
    }
}

impl mms::TableColumn {
    fn as_pyarrow_field(&self) -> String {
        format!(
            "pyarrow.field(\"{name}\", {ty}, {nullable})",
            name = self.field_name(),
            ty = self.data_type.as_pyarrow_schema_type(),
            nullable = if self.mandatory { "False" } else { "True" },
        )
    }
}

impl mms::TableColumns {
    fn as_pyarrow_schema(&self) -> String {
        format!(
            r#"pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        {fields}
    ])"#,
            fields = self
                .columns
                .iter()
                .map(|col| col.as_pyarrow_field())
                .collect::<Vec<_>>()
                .join(",\n        "),
        )
    }
}

pub fn run() -> anyhow::Result<()> {
    let rdr = fs::File::open("mmsdm.json")?;
    let mapping = fs::read_to_string("table_mapping.csv").unwrap();
    let mut map = collections::HashMap::new();
    for row in mapping.split('\n').skip(1) {
        if row.contains(',') {
            let pieces = row.split(',').collect::<Vec<&str>>();
            let mms = mms::Report {
                name: pieces[0].to_string(),
                sub_type: pieces[1].to_string(),
            };
            let pdr = pdr::Report {
                name: pieces[2].to_string(),
                sub_type: match pieces[3] {
                    "" => None,
                    otherwise => Some(otherwise.to_string()),
                },
                version: pieces[4].parse().unwrap(),
                transaction_type: pieces[5].to_string(),
                row_filter: pieces[6].to_string(),
            };
            map.insert(mms, pdr);
        }
    }
    // abv
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();

    let mut fmt_str = String::from(
        r#"
from typing import List, Optional
import datetime

def get_row_partition_name(row: List[str]) -> str:
    if len(row) < 4:
        raise Exception("Row of length {} is too short, should be at least 4".format(len(row)))
    if row[0] != 'D':
        raise Exception("Row should be a data row but was instead `{}`".format(row[0]))
    mapping = {
"#,
    );
    for (data_set, tables) in local_info.iter() {
        for (table_key, table) in tables.iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key.clone(),
            };

            if mms_report.should_skip() {
                continue;
            }

            if let Some(pdr_report) = map.get(&mms_report) {
                if let Some((idx, col)) = table
                    .columns()
                    .columns
                    .iter()
                    .enumerate()
                    .find(|(_, c)| c.name == "SETTLEMENTDATE")
                {
                    dbg!(idx, col);
                    fmt_str.push_str(&format!(
                        "        (\"{}\",\"{}\"): {},\n",
                        pdr_report.name,
                        pdr_report.sub_type.as_ref().unwrap_or(&String::new()),
                        idx + 4
                    ));
                }
                dbg!(pdr_report);
                // dbg!(table);
                // dbg!(mms_report);
            }
        }
    }

    fmt_str.push_str(
        r#"    } 

    if mapping.get((row[1], row[2])) is not None:
        sd = row[mapping[(row[1], row[2])]]
        parsed = datetime.datetime.strptime(sd, "%Y/%m/%d %H:%M:%S").date()
        return "{}-{}-v{}-{}-{}".format(row[1], row[2], row[3], parsed.year, parsed.month)
    else:
        return "{}-{}-v{}".format(row[1], row[2], row[3])
    "#,
    );
    fs::write("python/mmsdm/partition.py", fmt_str)?;

    let mut fmt_str = String::from(
        r#"from typing import Optional
import pyarrow
import pyarrow.csv as pc
"#,
    );
    let mut extract_str = String::from(
        "def get_csv_reader(*,file_path: str, data_set: str, sub_type: Optional[str] = None):
    mapping = {
",
    );
    for (data_set, tables) in local_info.iter() {
        for (table_key, table) in tables.iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key.clone(),
            };

            if mms_report.should_skip() {
                continue;
            }

            if let Some(pdr_report) = map.get(&mms_report) {
                let data_set = pdr_report.name.to_lowercase();
                let sub_type = pdr_report
                    .sub_type
                    .clone()
                    .unwrap_or_else(|| "null".to_string())
                    .to_lowercase();
                let schema_fn_name = format!("{}_{}_v{}", data_set, sub_type, pdr_report.version);
                // the include_columns and include_missing_columns ensures that when we have less than expected columns
                // we simply get null columns instead of an error. This allows earlier version data sets to be
                // processed as the most current version. It is important that old data can be reprocessed
                // so that the schema matches
                fmt_str.push_str(&format!(r#"
def {fn_name}(file_path):
    schema = {schema}
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(include_columns=schema.names, include_missing_columns=True, column_types={{ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }}, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)
"#, 
                    fn_name = schema_fn_name,
                    schema = table.columns().as_pyarrow_schema(),
                ));
                extract_str.push_str(&format!("        (\"{data_set}\", \"{sub_type}\"): {schema_fn_name},\n"));
            }
        }
    }

    extract_str.push_str(
        r#"    }
    if sub_type is not None:
        return mapping[(data_set.lower(), sub_type.lower())](file_path)
    else: 
        return mapping[(data_set.lower(), "null")](file_path)
    "#,
    );

    fmt_str.push('\n');
    fmt_str.push_str(&extract_str);

    fs::write("python/mmsdm/data_model.py", fmt_str)?;

    Ok(())
}
