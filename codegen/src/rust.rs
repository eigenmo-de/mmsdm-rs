use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections, fs, iter, str, string};

use crate::{mms, pdr};

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
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();
    for (data_set, tables) in local_info.into_iter() {
        let mut fmt_str = String::new();
        let mut fmtr = codegen::Formatter::new(&mut fmt_str);
        for (table_key, table) in tables.into_iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key,
            };
            if let Some(pdr_report) = map.get(&mms_report) {
                let mut current_struct = codegen::Struct::new(&pdr_report.struct_name());
                current_struct
                    .vis("pub")
                    .doc(&table.get_doc(&pdr_report))
                    .derive("Debug")
                    .derive("Clone")
                    .derive("PartialEq")
                    .derive("Eq")
                    .derive("serde::Deserialize")
                    .derive("serde::Serialize");
                for col in table.columns.columns {
                    if &col.field_name() == "type" {
                        let mut field = codegen::Field::new("pub type_", &col.to_rust_type());
                        field.annotation(vec!["#[serde(rename = \"type\")]"]);
                        current_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPPP") {
                        // parse as DispatchPeriod
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::DispatchPeriod",
                        );
                        field.annotation(vec!["#[serde(with = \"crate::dispatch_period\")]"]);
                        current_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPP") {
                        // parse as TradingPeriod
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::TradingPeriod",
                        );
                        field.annotation(vec!["#[serde(with = \"crate::trading_period\")]"]);
                        current_struct.push_field(field);
                    } else if col.data_type == mms::DataType::Date {
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            &col.to_rust_type(),
                        );
                        if col.mandatory {
                            field.annotation(vec!["#[serde(with = \"crate::mms_datetime\")]"]);
                        } else {
                            field.annotation(vec!["#[serde(with = \"crate::mms_datetime_opt\")]"]);
                        };
                        current_struct.push_field(field);
                    } else {
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            &col.to_rust_type(),
                        );
                        field.doc(vec![&col.get_comment()]);
                        current_struct.push_field(field);
                    };
                }
                current_struct.fmt(&mut fmtr)?;

                let mut current_impl = codegen::Impl::new("crate::AemoFile");
                current_impl.impl_trait(format!("crate::GetTable<{}>", pdr_report.struct_name()));

                let mut get_file_key = codegen::Function::new("get_file_key");
                get_file_key.ret("crate::FileKey");

                get_file_key.line(&pdr_report.file_key_literal());

                current_impl.push_fn(get_file_key);
                current_impl.fmt(&mut fmtr)?;
            } else {
                println!("Cannot find:");
                dbg!(mms_report);
            }
        }
        use heck::SnakeCase;
        fs::write(
            format!("src/mmsdm/{}.rs", data_set.to_snake_case()),
            fmt_str,
        )?;
    }

    Ok(())
}
