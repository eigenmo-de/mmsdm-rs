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
use std::collections;
use crate::data_model;
use futures_util::{AsyncRead, AsyncWrite};

impl crate::AemoFile {
    async fn log_file<S>(&self, client: &mut tiberius::Client<S>, key: &mmsdm_core::FileKey, total_rows: i64) -> crate::Result<i64> 
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        // declare @header table (id bigint not null);
        // output inserted.id into @header
        let first_row = client.query(
            "insert into mmsdm.FileLog(
                data_source,
                file_name,
                from_participant,
                to_participant,
                effective_date,
                effective_time,
                serial_number,
                file_name_2,
                serial_number_2,
                data_set,
                sub_type,
                version,
                total_rows
            )
            output inserted.file_log_id
            values (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11, @P12, @P13);",
            &[
                &self.header.data_source,
                &self.header.file_name,
                &self.header.from_participant,
                &self.header.to_participant,
                &self.header.effective_date,
                &self.header.effective_time,
                &self.header.serial_number,
                &self.header.file_name_2,
                &self.header.serial_number_2,
                &key.data_set_name.as_str(),
                &key.table_name(),
                &key.version,
                &total_rows,
            ],
        ).await?.into_row().await?;
        let row = first_row.ok_or(mmsdm_core::Error::CreateFileLogError)?;
        row.try_get(0)?.ok_or(mmsdm_core::Error::CreateFileLogError.into())

    }


    async fn batched_insert<S, D>(&self, client: &mut tiberius::Client<S>, file_key: &mmsdm_core::FileKey, data: &[D], proc: &str, chunk_size: Option<usize>) -> crate::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
        D: serde::Serialize,
    {
        let total = i64::try_from(data.len())?;
        let chunk_size = chunk_size.unwrap_or(4_500);

        let file_log_id = self.log_file(client, file_key, total).await?;
        
        let mut current = 0_i64;
        
        for chunk in data.chunks(chunk_size) {
            let json = serde_json::to_string(chunk)?;
            if let Err(e) = client
                .execute(
                    proc,
                    &[&file_log_id, &json],
                ).await {
                    client.execute(
                        "update mmsdm.FileLog set [status] = 'E', rows_inserted = @P1, message = @P2 where file_log_id = @P3",
                        &[&current, &e.to_string(), &file_log_id],
                    ).await?;
                    return Err(e.into());
            } else {
                current += i64::try_from(chunk.len())?;
                client.execute(
                    "update mmsdm.FileLog set rows_inserted = @P1 where file_log_id = @P2",
                    &[&current, &file_log_id],
                ).await?;
                log::debug!("Progress: {} out of {} rows saved", current, total);
            }
        }
        client.execute(
            "update mmsdm.FileLog set [status] = 'C', rows_inserted = @P1 where file_log_id = @P2",
            &[&current, &file_log_id],
        ).await?;
        Ok(())
    }


/// This function is meant to be used in conjunction with the iterator over
/// the data contained within the AemoFile struct
pub async fn load_data<S>(&self, client: &mut tiberius::Client<S>, skip_keys: Option<&collections::HashSet<mmsdm_core::FileKey>>, chunk_size: Option<usize>) -> crate::Result<()>
where
S: AsyncRead + AsyncWrite + Unpin + Send,
{
for file_key in self.data.keys() {
    if skip_keys.map(|set| set.contains(file_key)).unwrap_or(false) {
    	log::info!("Skippping file key {} as it is in the list of keys to skip", file_key);                            
        continue;                                                                                                      
    }   
    match (
        file_key.data_set_name.as_str(),
        file_key.table_name.as_deref(),
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
            ("{data_set_name}",{table_name}, version) if version <= {version}_i32 => {{
                #[cfg(feature = "{module}")]
                {{
                    let d: Vec<data_model::{local_name}> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.Insert{db_name} @P1, @P2", chunk_size).await?;
                }}
                #[cfg(not(feature = "{module}"))]
                {{
                    log::warn!("Unhandled file key {{:?}}", file_key);
                    continue;
                }}
                
            }}"#,
                    data_set_name = pdr_report.name,
                    table_name = if let Some(sub_type) = &pdr_report.sub_type {
                        format!("Some(\"{}\")", sub_type)
                    } else {
                        "None".to_string()
                    },
                    version = pdr_report.version,
                    local_name = pdr_report.get_rust_struct_name(),
                    db_name = pdr_report.sql_table_name(),
                    module = data_set.to_snake_case(),
                );
                fmt_str.push_str(&block);
            };
        }
    }
    fmt_str.push_str(
        r#"
        _ => {
            log::error!("Unexpected file key {:?}", file_key);
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
