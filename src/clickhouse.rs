use crate::{mmsdm::*, GetTable};
use futures::{AsyncRead, AsyncWrite};
use clickhouse_rs::{Block, Pool};

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
                    let block = clickhouse_rs::Block::new()
                        .column("file_log_id", file_uuids)
                        .column("settlementdate", d.iter().map(|r| chrono::DateTime::<chrono::Utc>::from_utc(r.settlementdate, chrono::offset::FixedOffset) ).collect::<Vec<_>>())
                        .column("duid", d.iter().map(|r| r.duid).collect::<Vec<_>>())
                        .column("bidtype", d.iter().map(|r| r.bidtype).collect::<Vec<_>>())
                        .column(
                            "bidsettlementdate",
                            d.iter().map(|r| r.bidsettlementdate.map(|d| chrono::DateTime::<chrono::Utc>::from_utc(d, chrono::Utc)).unwrap() ).collect::<Vec<_>>(),
                        )
                        .column("bidofferdate", d.iter().map(|r| r.bidofferdate.map(|d| chrono::DateTime::<chrono::Utc>::from_utc(d, chrono::Utc)).unwrap() ).collect::<Vec<_>>())
                        .column("lastchanged", d.iter().map(|r| r.lastchanged.map(|d| chrono::DateTime::<chrono::Utc>::from_utc(d, chrono::Utc)).unwrap() ).collect::<Vec<_>>());
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
