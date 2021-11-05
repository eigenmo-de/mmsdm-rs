use anyhow::anyhow;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase, TitleCase};
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

    let mut fmt_str = String::from(r#"
from typing import List, Optional
import datetime

def get_row_partition_name(row: List[str]) -> str:
    if len(row) < 4:
        raise Exception("Row of length {} is too short, should be at least 4".format(len(row)))
    if row[0] != 'D':
        raise Exception("Row should be a data row but was instead `{}`".format(row[0]))
    mapping = {
"#);
    for (data_set, tables) in local_info.into_iter() {
        for (table_key, table) in tables.into_iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key,
            };

            if mms_report.should_skip() {
                continue;
            }

            if let Some(pdr_report) = map.get(&mms_report) {

                if let Some((idx, col)) = table.columns.columns.iter().enumerate().find(|(_, c)| c.name == "SETTLEMENTDATE") {
                    dbg!(idx, col);
                    fmt_str.push_str(&format!("        (\"{}\",\"{}\"): {},\n", pdr_report.name, pdr_report.sub_type.as_ref().unwrap_or(&String::new()), idx + 4));
                }
                dbg!(pdr_report);
                // dbg!(table);
                // dbg!(mms_report);
            }

        }
    }

    fmt_str.push_str(r#"    } 

    if mapping.get((row[1], row[2])) is not None:
        sd = row[mapping[(row[1], row[2])]]
        parsed = datetime.datetime.strptime(sd, "%Y/%m/%d %H:%M:%S").date()
        return "{}-{}-v{}-{}-{}".format(row[1], row[2], row[3], parsed.year, parsed.month)
    else:
        return "{}-{}-v{}".format(row[1], row[2], row[3])
    "#);
    fs::write(
        "python/mmsdm/partition.py",
        fmt_str,
    )?;

    Ok(())
}
