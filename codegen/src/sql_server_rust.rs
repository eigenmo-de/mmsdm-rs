// use anyhow::anyhow;
// use serde::{Deserialize, Serialize};
use crate::{mms, pdr};
use std::{collections, fs, str};

pub fn run() -> anyhow::Result<()> {
    let rdr = fs::File::open("mmsdm.json")?;
    let mapping = fs::read_to_string("table_mapping.csv").unwrap();
    let mut map = collections::HashMap::new();
    // use std::iter::Iterator;
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
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();
    let mut fmt_str = String::new();
    fmt_str.push_str(
        r#"
use crate::mmsdm::*;
use futures::{AsyncRead, AsyncWrite};
use std::convert::TryInto;

impl crate::AemoFile {
/// This function is meant to be used in conjunction with the iterator over
/// the data contained within the AemoFile struct
pub async fn load_data<S>(&self, client: &mut tiberius::Client<S>) -> crate::Result<()>
where
S: AsyncRead + AsyncWrite + Unpin + Send,
{
for file_key in self.data.keys() {
    match (
        file_key.data_set_name.as_str(),
        file_key.table_name.as_str(),
        file_key.version,
    ) {
    "#,
    );
    for (data_set, tables) in local_info.into_iter() {
        // let fmtr = codegen::Formatter::new(&mut fmt_str);
        for (table_key, _) in tables.into_iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key,
            };
            if let Some(pdr_report) = map.get(&mms_report) {
                use heck::SnakeCase;
                let block = format!(
                    r#"
            ("{data_set_name}","{table_name}",{version}_i32) =>  {{
                #[cfg(feature = "{module}")]
                {{
                    let d: Vec<{module}::{local_name}> = self.get_table()?;
                    let total = d.len();
                    let mut current = 0_usize;
                    for chunk in d.chunks(100_000_usize) {{
                        current += chunk.len();
                        let json = serde_json::to_string(chunk)?;
                        client.execute(
                            "exec Insert{local_name} @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                        let num = rust_decimal::Decimal::new(current.try_into().unwrap(), 0_u32);
                        let denom = rust_decimal::Decimal::new(total.try_into().unwrap(), 0_u32);
                        dbg!(num / denom);
                    }}
                }}
                #[cfg(not(feature = "{module}"))]
                {{
                    log::error!("Unhandled file key {{:?}}", file_key);
                    continue;
                }}
                
            }}"#,
                    data_set_name = pdr_report.name,
                    table_name = pdr_report.sub_type,
                    version = pdr_report.version,
                    local_name = pdr_report.struct_name(),
                    module = data_set.to_snake_case(),
                );
                fmt_str.push_str(&block);
            };
        }
    }
    fmt_str.push_str(
        r#"
        _ => {
            log::error!("Unhandled file key {:?}", file_key);
            continue;
        }
    }
}
Ok(())
}
}"#,
    );
    // use heck::SnakeCase;
    fs::write(format!("src/sql_server.rs"), fmt_str)?;

    Ok(())
}
