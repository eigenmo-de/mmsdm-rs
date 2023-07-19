// use anyhow::anyhow;
// use serde::{Deserialize, Serialize};
use crate::{
    mms,
    rust::{TableMapping, VERSION},
};
use std::{collections, fs};
use heck::ToSnakeCase;

pub fn run() -> anyhow::Result<()> {
    let rdr = fs::File::open(format!("mmsdm_v{VERSION}.json"))?;
    let map = TableMapping::read()?;

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
                sub_type: table_key,
            };
            if let Some(pdr_report) = map.get(&mms_report) {

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

    let parsed = syn::parse_file(&fmt_str)?;
    let formatted = prettyplease::unparse(&parsed);
    fs::write("src/sql_server.rs", formatted)?;

    Ok(())
}
