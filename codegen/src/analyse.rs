use anyhow::format_err;
use log::info;

use crate::{json::DataModel, mms, rust::TableMapping};

pub fn run(local_info: DataModel) -> anyhow::Result<()> {
    let map = TableMapping::read()?;

    for (data_set, package) in local_info.packages.iter() {
        // println!("Processing data set {data_set}");

        if data_set == "HISTORICAL" {
            // skip
            continue;
        }

        let mut fmt_str = String::new();
        fmt_str.push_str("#![no_std]\n#![allow(unused_imports)]\nextern crate alloc;\nuse alloc::string::ToString;\nuse chrono::Datelike as _;\n#[cfg(feature = \"arrow\")]\nextern crate std;");
        let _fmtr = codegen::Formatter::new(&mut fmt_str);

        for (table_key, _table_header) in package.tables.iter() {
            info!("Processing table {table_key}");

            let table = local_info
                .tables
                .get(table_key)
                .ok_or_else(|| format_err!("missing table {table_key}"))?;

            let mms_report = mms::Report {
                sub_type: table_key.clone(),
            };

            if mms_report.should_skip() {
                continue;
            }

            // dbg!(&data_set, &table_key, &table);

            match map.get(&mms_report) {
                Some(_pdr_report) => {
                    for column in table.columns() {
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
