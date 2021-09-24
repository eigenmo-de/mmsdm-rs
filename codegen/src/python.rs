

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections, fs, iter, str, string};
use heck::{CamelCase, ShoutySnakeCase, TitleCase, SnakeCase};

use crate::{mms, pdr};

impl mms::DataType {
    fn as_python_type(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } => "str",
            mms::DataType::Char => "str",
            mms::DataType::Date => "datetime.date",
            mms::DataType::DateTime => "datetime.datetime",
            mms::DataType::Decimal { .. } => "decimal.Decimal",
            mms::DataType::Integer { .. } => "int",
        }
        .into()
    }
}

impl mms::TableColumn {
    fn to_python_type(&self) -> String {
        if self.mandatory {
            format!("{}", self.data_type.as_python_type())
        } else {
            format!("typing.Optional[{}]", self.data_type.as_python_type())
        }
    }
}

impl pdr::Report {
    pub fn get_python_class_name(&self) -> String {
        if let Some(sub_type) = &self.sub_type {
            format!(
                "{}{}{}",
                self.name.to_camel_case(),
                sub_type.to_camel_case(),
                self.version
            )
        } else {
            format!("{}{}", self.name.to_camel_case(), self.version)
        }
    }
    pub fn get_python_file_key_literal(&self) -> String {
        format!(
r#"key.TableKey(
            collection="{}",
            name="{}",
            version={}
        )"#,
            data_set = self.name.to_shouty_snake_case(),
            table = if let Some(sub_type) = &self.sub_type {
                sub_type
            } else {
                ""
            }
            .to_shouty_snake_case(),
            version = self.version,
        )
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


// import mmsdm.data_model.settlement_data as settlement_data
// from typing import Callable, Any, List
// import mmsdm
// import mmsdm.key as key


// # this needs to be updated as move modules are added above
// def mapping(key: key.TableKey) -> Callable[[List[str]], Any]:
//     to_fn = {
//         # start settlements
//         settlement_data.DayTrack.key(): settlement_data.DayTrack.from_row,
//         settlement_data.Cpdata.key(): settlement_data.Cpdata.from_row,
//         settlement_data.FcasRecovery.key(): settlement_data.FcasRecovery.from_row,
//         settlement_data.Marketfees.key(): settlement_data.Marketfees.from_row,
//         settlement_data.NmasRecovery.key(): settlement_data.NmasRecovery.from_row,
//         settlement_data.Reallocations.key(): settlement_data.Reallocations.from_row,
//         # end settlements
//     }
//     return to_fn[key]

    let mut imports = collections::BTreeSet::new();
    let mut dataset_mappings = String::new();

    for (data_set, tables) in local_info.iter() {
        for (table_key, _) in tables.iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key.to_string(),
            };

            if mms_report.should_skip() {
                continue;
            }

            if let Some(pdr_report) = map.get(&mms_report) {
                // println!("DS {}, TK {}", data_set.to_snake_case(), pdr_report.get_python_class_name());
                imports.insert(data_set.to_snake_case());
                dataset_mappings.push_str(
                    &format!(
                        "       {data_set}.{python_class}.key(): {data_set}.{python_class}.from_row,\n",
                        data_set = data_set.to_snake_case(),
                        python_class = pdr_report.get_python_class_name(),
                    )
                );
                
            }
        }
    }

    fs::write(
        "python/mmsdm/data_model/__init__.py",
        format!(r#"import mmsdm
import mmsdm.key as key
from typing import Callable, Any, List
{imports}
        
def mapping(key: key.TableKey) -> Callable[[List[str]], Any]:
    to_fn = {{
{dataset_mappings}
    }}
    return to_fn[key]
"#,
        imports = imports.iter().map(|ds| format!("import mmsdm.data_model.{0} as {0}", ds)).collect::<Vec<_>>().join("\n"),
        dataset_mappings = dataset_mappings,
        ),
    )?;

    for (data_set, tables) in local_info.into_iter() {
        
        let mut fmt_str = String::new();
        fmt_str.push_str(r#"
import typing
import datetime
import mmsdm
import mmsdm.key as key

from dataclasses import dataclass
import decimal
        "#);

        for (table_key, table) in tables.into_iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key,
            };

            if mms_report.should_skip() {
                continue;
            }

            if let Some(pdr_report) = map.get(&mms_report) {
                // watch for keywords in column names?
                // likely, "type"

                let column_definitions = table.columns.columns.iter()
                    .map(|col| format!("{}: {}", col.name.to_snake_case(), col.to_python_type()))
                    .collect::<Vec<_>>()
                    .join("\n    ");

                let column_extractors = table.columns.columns.iter()
                    .enumerate()
                    .map(|(idx, col)| {
                        let row_part = format!("row[{}]", idx + 4);
                        let extractor = match (col.data_type.clone(), col.mandatory) {
                            (mms::DataType::Varchar { .. }, _) => row_part,
                            (mms::DataType::Char, _) => row_part,
                            (mms::DataType::Decimal { .. }, true) =>  format!("decimal.Decimal({})", row_part),
                            (mms::DataType::Integer { .. }, true) => format!("int({})", row_part),
                            (mms::DataType::Date, true) => format!("datetime.datetime.strptime({}, \"%Y/%m/%d %H:%M:%S\").date()", row_part),
                            (mms::DataType::DateTime, true) => format!("datetime.datetime.strptime({}, \"%Y/%m/%d %H:%M:%S\")", row_part),
                            // the col name will be a variable created earlier that is None if the data is missing
                            (mms::DataType::Decimal { .. }, false) => col.name.to_snake_case(),
                            (mms::DataType::Integer { .. }, false) => col.name.to_snake_case(),
                            (mms::DataType::Date, false) => col.name.to_snake_case(),
                            (mms::DataType::DateTime, false) => col.name.to_snake_case(),
                        };
                        format!("{}={}", col.name.to_snake_case(), extractor)
                    })
                    .collect::<Vec<_>>()
                    .join(",\n            ");
                
                let optional_extractors = table.columns.columns.iter()
                    .enumerate()
                    .filter(|(_, col)| !col.mandatory)
                    .filter_map(|(idx, col)| {
                        let row_part = format!("row[{}]", idx + 4);
                        let extractor = match col.data_type {
                            mms::DataType::Decimal { .. } =>  format!("decimal.Decimal({})", row_part),
                            mms::DataType::Integer { .. } => format!("int({})", row_part),
                            mms::DataType::Date => format!("datetime.datetime.strptime({}, \"%Y/%m/%d %H:%M:%S\").date()", row_part),
                            mms::DataType::DateTime => format!("datetime.datetime.strptime({}, \"%Y/%m/%d %H:%M:%S\")", row_part),
                            _ => return None,
                        };

                        Some(format!(r#"
        if {row_part} is None or {row_part} == "":
            {column_name} = None
        else:
            {column_name} = {extractor}
"#,
                        row_part = row_part,
                        column_name = col.name.to_snake_case(),
                        extractor = extractor,
                        ))
                    })
                    .collect::<Vec<_>>()
                    .join("");

                fmt_str.push_str(&format!(r#"
@dataclass(frozen=True)
class {class_name}:
    {column_definitions}

    @staticmethod
    def key() -> key.TableKey:
        return {table_key_literal}

    @staticmethod
    def from_row(row: typing.List[str]) -> "{class_name}":
        {optional_extractors}
        return {class_name}(
            {column_extractors}
        )           
"#, 
                column_definitions = column_definitions,
                column_extractors = column_extractors,
                optional_extractors = optional_extractors,
                table_key_literal = pdr_report.get_python_file_key_literal(),
                class_name = pdr_report.get_python_class_name(),
                ))
            } else {
                println!("Cannot find:");
                dbg!(mms_report);
            }
        }
        fs::write(
            format!("python/mmsdm/data_model/{}.py", data_set.to_snake_case()),
            fmt_str,
        )?;
    }

    Ok(())
}
