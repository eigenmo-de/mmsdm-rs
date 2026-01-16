use anyhow::format_err;
use log::info;

use crate::{
    json::DataModel,
    mms::{self, DataType},
    rust::TableMapping,
};

pub fn run(local_info: DataModel) -> anyhow::Result<()> {
    let map = TableMapping::read()?;

    let mut rows = Vec::new();
    rows.push(Vec::from([
        "data_set".to_string(),
        "table".to_string(),
        "pdr".to_string(),
        "pdr_sub_type".to_string(),
        "length".to_string(),
        "comment".to_string(),
    ]));

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
                    dbg!(&mms_report, &_pdr_report);
                    for column in table.columns() {
                        if let DataType::Varchar { length } = column.data_type {
                            rows.push(
                                [
                                    data_set.to_string(),
                                    table_key.to_string(),
                                    _pdr_report.name.to_string(),
                                    _pdr_report
                                        .sub_type
                                        .as_ref()
                                        .map(|x| x.to_string())
                                        .unwrap_or_default(),
                                    length.to_string(),
                                    column.comment.to_string(),
                                ]
                                .into(),
                            );
                        }

                        // if column.is_dispatch_period() && column.is_trading_period() {
                        //     dbg!(&data_set, &table_key, column);
                        // }
                    }
                }
                None => eprintln!("Cannot find PDR mapping for MMS Report: {mms_report:?}"),
            }
        }
    }

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("./varchar_columns.tsv")?;

    let mut builder = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

    for row in rows {
        builder.write_record(row)?;
    }

    builder.flush()?;

    Ok(())
}
