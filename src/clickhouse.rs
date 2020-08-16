use crate::{mmsdm::*, GetTable};
use futures::{AsyncRead, AsyncWrite};
use clickhouse_rs::{Block, Pool};


fn naive_to_utc(naive: chrono::NaiveDateTime) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::offset::FixedOffset)
}

impl crate::AemoFile {
    /// This function is meant to be used in conjunction with the iterator over
    /// the data contained within the AemoFile struct
    pub async fn load_data_clickhouse<S>(&self, client: &mut clickhouse_rs::ClientHandle) -> crate::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        for file_key in self.data.keys() {
            match (
                file_key.data_set_name.as_str(),
                file_key.table_name.as_str(),
                file_key.version,
            ) {
                ("DISPATCH", "OFFERTRK", 1_i32) => {

                    let d: Vec<dispatch::DispatchOffertrk1> = self.get_table()?;
                    let file_uuids: Vec<i32> = std::iter::repeat(5).take(d.len()).collect();


                    let mut settlementdate_vec = Vec::new();
                    let mut duid_vec = Vec::new();
                    let mut bidtype_vec = Vec::new();
                    let mut bidsettlementdate_vec = Vec::new();
                    let mut bidofferdate_vec = Vec::new();
                    let mut lastchanged_vec = Vec::new();

                    for row in d {
                        settlementdate_vec.push(naive_to_utc(row.settlementdate));
                        duid_vec.push(row.duid);
                        bidtype_vec.push(row.bidtype_vec);
                        bidsettlementdate_vec.push(naive_to_utc(row.bidsettlementdate_vec));
                        bidofferdate_vec.push(row.bidofferdate_vec);
                        lastchanged_vec.push(naive_to_utc(row.lastchanged));
                    }

                    let block = clickhouse_rs::Block::new()
                        .column("file_log_id", file_uuids)
                        .column("settlementdate", settlementdate_vec)
                        .column("duid", duid_vec)
                        .column("bidtype", bidtype_vec)
                        .column("bidsettlementdate", bidsettlementdate_vec)
                        .column("bidofferdate", bidofferdate_vec)
                        .column("lastchanged", lastchanged_vec);
                    client.insert("DispatchOffertrk1", block).await?;
                }
                _ => {
                    log::error!("Unhandled file key {:?}", file_key);
                    continue;
                }
            }
        }
        Ok(())
    }
}
