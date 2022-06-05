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
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();
    let mut fmt_str = String::new();
    fmt_str.push_str(
        r#"
#[cfg(feature = "sql_server")]
pub async fn save_all<'a, S>(file: impl Into<mmsdm_core::MmsFile<'a>>, skip_keys: Option<&std::collections::HashSet<mmsdm_core::FileKey>>, client: &mut tiberius::Client<S>, chunk_size: Option<usize>) -> mmsdm_core::Result<()>
where S: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin + Send,
{
    let mut mms_file = file.into();
    for file_key in mms_file.file_keys() {
        if skip_keys.map(|set| set.contains(&file_key)).unwrap_or(false) {
            log::info!("Skippping file key {} as it is in the list of keys to skip", file_key);                            
            continue;                                                                                                      
        }   
        match file_key.data_set_name.as_str() {
"#,);
    let mut data_map = collections::BTreeMap::new();

    for (data_set, tables) in local_info.into_iter() {
        // let fmtr = codegen::Formatter::new(&mut fmt_str);
        for (table_key, _) in tables.into_iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key,
            };
            if let Some(pdr_report) = map.get(&mms_report) {
                use heck::SnakeCase;

                data_map
                    .entry(data_set.to_snake_case())
                    .and_modify(|e: &mut collections::BTreeSet<String>| {
                        e.insert(pdr_report.name.to_string());
                    })
                    .or_insert_with(|| collections::BTreeSet::from([pdr_report.name.to_string()]));
            };
        }
    }
    for (feature, matches) in data_map {
        let block = format!(
            r#"            "{options}" => {{
                #[cfg(feature = "{feature}")]
                {{
                    mmsdm_{feature}::save(&mut mms_file, &file_key, client, chunk_size).await?;
                }}
                #[cfg(not(feature = "{feature}"))]
                {{
                    log::warn!("File key {{:?}} is not handled as the feature {feature} is not activated", file_key);
                }}
            }}
"#,
            options = if feature == "irauction" {
                matches
                    .into_iter()
                    .filter(|o| o != "SETTLEMENT_CONFIG")
                    .collect::<Vec<String>>()
                    .join("\" | \"")
            } else {
                matches.into_iter().collect::<Vec<String>>().join("\" | \"")
            },
        );
        fmt_str.push_str(&block);
    }
    fmt_str.push_str(
        r#"
            _ => {
                log::error!("Unexpected file key {:?}", file_key);
            }
        }
    }
Ok(())
}"#,
    );
    // use heck::SnakeCase;
    fs::write("src/sql_server.rs", fmt_str)?;

    Ok(())
}
