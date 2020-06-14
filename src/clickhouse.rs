

use crate::{mmsdm::*, GetTable};
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
                    ("DISPATCH","OFFERTRK",1_i32) =>  {
                        #[cfg(feature = "dispatch")]
                        {
                            let file_id: i64 = client.query(
                                "insert into FileLog(file_name, data_set, sub_type, version)
                                output inserted.id 
                                values (@P1, 'Dispatch', 'Offertrk', 1);",
                                        &[&self.header.get_filename()],
                                ).await?.into_row().await?.unwrap().get(0).unwrap();

                            let d: Vec<dispatch::DispatchOffertrk1> = self.get_table()?;
                            let total = d.len();
                            for (i, data) in d.iter().enumerate() {
                                client.execute(
                                    "insert into DispatchOffertrk1(
                                        file_log_id,
                                        [settlementdate],
                                        [duid],
                                        [bidtype],
                                        [bidsettlementdate],
                                        [bidofferdate],
                                        [lastchanged]
                                    ) values (
                                        @P1,
                                        @P2,
                                        @P3,
                                        @P4,
                                        @P5,
                                        @P6,
                                        @P7
                                    )",
                                    &[&file_id,
                                      &data.settlementdate,
                                      &data.duid,
                                      &data.bidtype,
                                      &data.bidsettlementdate.unwrap(),
                                      &data.bidofferdate.unwrap(),
                                      &data.lastchanged.unwrap(),
                                    ],
                                    ).await?;

                                if i % 10000 == 0 {
                                    let num = rust_decimal::Decimal::new(i.try_into().unwrap(), 0_u32);
                                    let denom = rust_decimal::Decimal::new(total.try_into().unwrap(), 0_u32);
                                    dbg!(num / denom);
                                }
                            }
                        }
                        #[cfg(not(feature = "dispatch"))]
                        {
                            log::error!("Unhandled file key {:?}", file_key);
                            continue;
                        }
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
