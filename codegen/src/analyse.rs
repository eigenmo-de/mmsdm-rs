use std::fs;

use crate::{VERSION, mms, rust::TableMapping};

pub fn run() -> anyhow::Result<()> {
    let rdr = fs::File::open(format!("mmsdm_v{VERSION}.json"))?;
    let map = TableMapping::read()?;

    // abv
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();

    for (data_set, tables) in local_info.into_iter() {
        // println!("Processing data set {data_set}");

        if data_set == "HISTORICAL" {
            // skip
            continue;
        }

        let mut fmt_str = String::new();
        fmt_str.push_str("#![no_std]\n#![allow(unused_imports)]\nextern crate alloc;\nuse alloc::string::ToString;\nuse chrono::Datelike as _;\n#[cfg(feature = \"arrow\")]\nextern crate std;");
        let fmtr = codegen::Formatter::new(&mut fmt_str);

        for (table_key, table) in tables.clone().into_iter() {
            // println!("Processing table {table_key}");
            // panic!();
            let mms_report = mms::Report {
                sub_type: table_key.clone(),
            };

            if mms_report.should_skip() {
                continue;
            }

            // dbg!(&data_set, &table_key, &table);

            match map.get(&mms_report) {
                Some(pdr_report) => {
                    for column in table.columns().columns {
                        if column.is_dispatch_period() && column.is_trading_period() {
                            dbg!(&data_set, &table_key, column);
                        }
                    }
                }
                None => eprintln!("Cannot find PDR mapping for MMS Report: {mms_report:?}"),
            }
        }
    }

    Ok(())
}
