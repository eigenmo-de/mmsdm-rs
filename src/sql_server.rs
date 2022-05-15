use crate::data_model;
use futures_util::{AsyncRead, AsyncWrite};
use std::collections;

impl crate::AemoFile {
    async fn log_file<S>(
        &self,
        client: &mut tiberius::Client<S>,
        key: &crate::FileKey,
        total_rows: i64,
    ) -> crate::Result<i64>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        // declare @header table (id bigint not null);
        // output inserted.id into @header
        let first_row = client
            .query(
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
            )
            .await?
            .into_row()
            .await?;
        let row = first_row.ok_or(crate::Error::CreateFileLogError)?;
        row.try_get(0)?.ok_or(crate::Error::CreateFileLogError)
    }

    async fn batched_insert<S, D>(
        &self,
        client: &mut tiberius::Client<S>,
        file_key: &crate::FileKey,
        data: &[D],
        proc: &str,
        chunk_size: Option<usize>,
    ) -> crate::Result<()>
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
            if let Err(e) = client.execute(proc, &[&file_log_id, &json]).await {
                client.execute(
                        "update mmsdm.FileLog set [status] = 'E', rows_inserted = @P1, message = @P2 where file_log_id = @P3",
                        &[&current, &e.to_string(), &file_log_id],
                    ).await?;
                return Err(e.into());
            } else {
                current += i64::try_from(chunk.len())?;
                client
                    .execute(
                        "update mmsdm.FileLog set rows_inserted = @P1 where file_log_id = @P2",
                        &[&current, &file_log_id],
                    )
                    .await?;
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
    pub async fn load_data<S>(
        &self,
        client: &mut tiberius::Client<S>,
        skip_keys: Option<&collections::HashSet<crate::FileKey>>,
        chunk_size: Option<usize>,
    ) -> crate::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        for file_key in self.data.keys() {
            if skip_keys.map(|set| set.contains(file_key)).unwrap_or(false) {
                log::info!(
                    "Skippping file key {} as it is in the list of keys to skip",
                    file_key
                );
                continue;
            }
            match (
                file_key.data_set_name.as_str(),
                file_key.table_name.as_deref(),
                file_key.version,
            ) {
                ("ASOFFER", Some("OFFERAGCDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<data_model::AsofferOfferagcdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferagcdata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERASTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<data_model::AsofferOfferastrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferastrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERLSHEDDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<data_model::AsofferOfferlsheddata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferlsheddata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERRESTARTDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<data_model::AsofferOfferrestartdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferrestartdata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERRPOWERDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<data_model::AsofferOfferrpowerdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferrpowerdata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BIDS", Some("BIDDAYOFFER"), version) if version <= 1_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::BidsBiddayoffer1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidsBiddayoffer1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BID", Some("BIDDAYOFFER_D"), version) if version <= 2_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::BidBiddayofferD2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidBiddayofferD2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BIDS", Some("BIDOFFERFILETRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::BidsBidofferfiletrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidsBidofferfiletrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BIDS", Some("BIDOFFERPERIOD"), version) if version <= 1_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::BidsBidofferperiod1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidsBidofferperiod1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BID", Some("BIDPEROFFER_D"), version) if version <= 2_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::BidBidperofferD2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidBidperofferD2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BIDS", Some("MNSP_BIDOFFERPERIOD"), version) if version <= 1_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::BidsMnspBidofferperiod1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidsMnspBidofferperiod1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BIDS", Some("MNSP_DAYOFFER"), version) if version <= 1_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::BidsMnspDayoffer1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidsMnspDayoffer1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("MTPASA_OFFERDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::OfferMtpasaOfferdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferMtpasaOfferdata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("MTPASA_OFFERFILETRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<data_model::OfferMtpasaOfferfiletrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferMtpasaOfferfiletrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("BILLINGCALENDAR"), version) if version <= 2_i32 => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<data_model::BillingConfigBillingcalendar2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigBillingcalendar2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_BAS_CLASS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<data_model::BillingConfigGstBasClass1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstBasClass1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_RATE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<data_model::BillingConfigGstRate1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstRate1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_TRANSACTION_CLASS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<data_model::BillingConfigGstTransactionClass1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstTransactionClass1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_TRANSACTION_TYPE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<data_model::BillingConfigGstTransactionType1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstTransactionType1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("SECDEPOSIT_INTEREST_RATE"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<data_model::BillingConfigSecdepositInterestRate1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigSecdepositInterestRate1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("SECDEPOSIT_PROVISION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<data_model::BillingConfigSecdepositProvision1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigSecdepositProvision1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("ASPAYMENTS"), version) if version <= 6_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingAspayments6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingAspayments6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("ASRECOVERY"), version) if version <= 7_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingAsrecovery7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingAsrecovery7 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("CPDATA"), version) if version <= 6_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingCpdata6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingCpdata6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("DAYTRK"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingDaytrk5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingDaytrk5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("FEES"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingFees5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingFees5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("FINANCIALADJUSTMENTS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingFinancialadjustments5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingFinancialadjustments5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("GENDATA"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingGendata5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingGendata5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("INTERRESIDUES"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingInterresidues5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingInterresidues5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("INTRARESIDUES"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingIntraresidues5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIntraresidues5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRAUCSURPLUS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingIraucsurplus5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIraucsurplus5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRAUCSURPLUSSUM"), version) if version <= 7_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingIraucsurplussum7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIraucsurplussum7 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRNSPSURPLUS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingIrnspsurplus5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrnspsurplus5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRNSPSURPLUSSUM"), version) if version <= 6_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingIrnspsurplussum6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrnspsurplussum6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRPARTSURPLUS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingIrpartsurplus5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrpartsurplus5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRPARTSURPLUSSUM"), version) if version <= 7_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingIrpartsurplussum7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrpartsurplussum7 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("PRIORADJUSTMENTS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingPrioradjustments5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingPrioradjustments5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REALLOC"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingRealloc5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRealloc5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REALLOC_DETAIL"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingReallocDetail5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingReallocDetail5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REGIONEXPORTS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingRegionexports5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRegionexports5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REGIONFIGURES"), version) if version <= 6_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingRegionfigures6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRegionfigures6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REGIONIMPORTS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingRegionimports5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRegionimports5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("RUNTRK"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingRuntrk5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRuntrk5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("APC_COMPENSATION"), version) if version <= 2_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingApcCompensation2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingApcCompensation2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("APC_RECOVERY"), version) if version <= 2_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingApcRecovery2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingApcRecovery2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("BILLING_CO2E_PUBLICATION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingBillingCo2ePublication1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingBillingCo2ePublication1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("BILLING_CO2E_PUBLICATION_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingBillingCo2ePublicationTrk1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingBillingCo2ePublicationTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("DAILY_ENERGY_SUMMARY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingDailyEnergySummary1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingDailyEnergySummary1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("BILLING_DIRECTION_RECON_OTHER"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingBillingDirectionReconOther1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingBillingDirectionReconOther1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("EFTSHORTFALL_AMOUNT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingEftshortfallAmount1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingEftshortfallAmount1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("EFTSHORTFALL_DETAIL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingEftshortfallDetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingEftshortfallDetail1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("GST_DETAIL"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingGstDetail5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingGstDetail5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("GST_SUMMARY"), version) if version <= 5_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingGstSummary5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingGstSummary5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_PAYMENTS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingNmasTstPayments1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstPayments1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_RECOVERY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingNmasTstRecovery1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstRecovery1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_RECVRY_RBF"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingNmasTstRecvryRbf1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstRecvryRbf1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_RECVRY_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingNmasTstRecvryTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstRecvryTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SECDEPOSIT_APPLICATION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingSecdepositApplication1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSecdepositApplication1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SECDEP_INTEREST_PAY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingSecdepInterestPay1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSecdepInterestPay1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SECDEP_INTEREST_RATE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingSecdepInterestRate1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSecdepInterestRate1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SUBST_DEMAND"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingSubstDemand1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSubstDemand1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SUBST_RUN_VERSION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingSubstRunVersion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSubstRunVersion1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("WDR"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingWdr1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingWdr1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("WDR_DETAIL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingWdrDetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingWdrDetail1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("RESERVETRADERPAYMENT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingReservetraderpayment1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingReservetraderpayment1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("RESERVETRADERRECOVERY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<data_model::BillingReservetraderrecovery1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingReservetraderrecovery1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OPERATIONAL_DEMAND", Some("ACTUAL"), version) if version <= 3_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::OperationalDemandActual3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOperationalDemandActual3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OPERATIONAL_DEMAND", Some("FORECAST"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::OperationalDemandForecast1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOperationalDemandForecast1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_CLUSTER_AVAIL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandIntermittentClusterAvail2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentClusterAvail2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_CLUSTER_AVAIL_DAY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandIntermittentClusterAvailDay1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentClusterAvailDay1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_DS_PRED"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandIntermittentDsPred1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentDsPred1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_DS_RUN"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandIntermittentDsRun1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentDsRun1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORECAST", Some("INTERMITTENT_GEN"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::ForecastIntermittentGen1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForecastIntermittentGen1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORECAST", Some("INTERMITTENT_GEN_DATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::ForecastIntermittentGenData1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForecastIntermittentGenData1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_GEN_LIMIT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandIntermittentGenLimit1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentGenLimit1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_GEN_LIMIT_DAY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandIntermittentGenLimitDay1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentGenLimitDay1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("MTPASA_INTERMITTENT_AVAIL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandMtpasaIntermittentAvail2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandMtpasaIntermittentAvail2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("MTPASA_INTERMITTENT_LIMIT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandMtpasaIntermittentLimit1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandMtpasaIntermittentLimit1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("PERIOD"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandPeriod1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandPeriod1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::DemandTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ROOFTOP", Some("ACTUAL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::RooftopActual2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertRooftopActual2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ROOFTOP", Some("FORECAST"), version) if version <= 1_i32 => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<data_model::RooftopForecast1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertRooftopForecast1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRICELOAD", Some("CONSTRAINTRELAXATION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::PriceloadConstraintrelaxation1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPriceloadConstraintrelaxation1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("BLOCKED_CONSTRAINTS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchBlockedConstraints1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchBlockedConstraints1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("CASE_SOLUTION"), version) if version <= 2_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchCaseSolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchCaseSolution2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("CONSTRAINT"), version) if version <= 5_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchConstraint5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchConstraint5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("INTERCONNECTORRES"), version) if version <= 3_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchInterconnectorres3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchInterconnectorres3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("UNIT_SOLUTION"), version) if version <= 3_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchUnitSolution3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchUnitSolution3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("OFFERTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchOffertrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchOffertrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("PRICE"), version) if version <= 4_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchPrice4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchPrice4 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("REGIONSUM"), version) if version <= 6_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchRegionsum6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchRegionsum6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRICELOAD", Some("CONSTRAINT_FCAS_OCD"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::PriceloadConstraintFcasOcd1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPriceloadConstraintFcasOcd1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("FCAS_REQ"), version) if version <= 2_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchFcasReq2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchFcasReq2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("INTERCONNECTION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchInterconnection1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchInterconnection1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("LOCAL_PRICE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchLocalPrice1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchLocalPrice1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("MNSPBIDTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchMnspbidtrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchMnspbidtrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("MR_SCHEDULE_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchMrScheduleTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchMrScheduleTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRICELOAD", Some("PRICE_REVISION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::PriceloadPriceRevision1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPriceloadPriceRevision1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("UNIT_CONFORMANCE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchUnitConformance1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchUnitConformance1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("UNIT_SCADA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchUnitScada1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchUnitScada1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("INTERMITTENT_FORECAST_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchIntermittentForecastTrk1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchIntermittentForecastTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("NEGATIVE_RESIDUE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<data_model::DispatchNegativeResidue1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchNegativeResidue1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("APEVENT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ApApevent1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApApevent1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("APEVENTREGION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ApApeventregion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApApeventregion1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("IRFMAMOUNT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ForceMajeureIrfmamount1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureIrfmamount1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("IRFMEVENTS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ForceMajeureIrfmevents1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureIrfmevents1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_REGIME_SUM"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ForceMajeureMarketSuspendRegimeSum1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendRegimeSum1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_REGION_SUM"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ForceMajeureMarketSuspendRegionSum1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendRegionSum1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_SCHEDULE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ForceMajeureMarketSuspendSchedule1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendSchedule1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_SCHEDULE_TRK"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ForceMajeureMarketSuspendScheduleTrk1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendScheduleTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("OVERRIDERRP"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ForceMajeureOverriderrp1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureOverriderrp1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("REGIONAPC"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ApRegionapc1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApRegionapc1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("REGIONAPCINTERVALS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<data_model::ApRegionapcintervals1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApRegionapcintervals1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GD_INSTRUCT", Some("GDINSTRUCT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "gd_instruct")]
                    {
                        let d: Vec<data_model::GdInstructGdinstruct1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGdInstructGdinstruct1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "gd_instruct"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GD_INSTRUCT", Some("INSTRUCTIONSUBTYPE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "gd_instruct")]
                    {
                        let d: Vec<data_model::GdInstructInstructionsubtype1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGdInstructInstructionsubtype1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "gd_instruct"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GD_INSTRUCT", Some("INSTRUCTIONTYPE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "gd_instruct")]
                    {
                        let d: Vec<data_model::GdInstructInstructiontype1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGdInstructInstructiontype1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "gd_instruct"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENERIC_CONSTRAINT", Some("EMSMASTER"), version) if version <= 1_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GenericConstraintEmsmaster1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenericConstraintEmsmaster1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENCONDATA", Some("NULL"), version) if version <= 6_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GencondataNull6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGencondataNull6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENCONSET", Some("NULL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GenconsetNull1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenconsetNull1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENERIC_CONSTRAINT", Some("GENCONSETINVOKE"), version) if version <= 2_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GenericConstraintGenconsetinvoke2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenericConstraintGenconsetinvoke2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENCONSETTRK", Some("NULL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GenconsettrkNull2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenconsettrkNull2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GCRHS", Some("NULL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GcrhsNull1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGcrhsNull1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GEQDESC", Some("NULL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GeqdescNull2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGeqdescNull2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GEQRHS", Some("NULL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::GeqrhsNull1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGeqrhsNull1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SPDCPC", Some("NULL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::SpdcpcNull2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSpdcpcNull2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SPDICC", Some("NULL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::SpdiccNull1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSpdiccNull1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SPDRC", Some("NULL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<data_model::SpdrcNull2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSpdrcNull2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionConfigAuction1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuction1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_CALENDAR"), version) if version <= 2_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionConfigAuctionCalendar2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionCalendar2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_IC_ALLOCATIONS"), version)
                    if version <= 2_i32 =>
                {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionConfigAuctionIcAllocations2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionIcAllocations2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_REVENUE_ESTIMATE"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionConfigAuctionRevenueEstimate1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionRevenueEstimate1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_REVENUE_TRACK"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionConfigAuctionRevenueTrack1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionRevenueTrack1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_RP_ESTIMATE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionConfigAuctionRpEstimate1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionRpEstimate1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_TRANCHE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionConfigAuctionTranche1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionTranche1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("RESIDUECONTRACTPAYMENTS"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::SettlementConfigResiduecontractpayments1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigResiduecontractpayments1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_BIDS", Some("FILE_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionBidsFileTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionBidsFileTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_BID_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResidueBidTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueBidTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CONTRACTS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResidueContracts1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueContracts1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CON_DATA"), version) if version <= 2_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResidueConData2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueConData2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CON_ESTIMATES_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResidueConEstimatesTrk1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueConEstimatesTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CON_FUNDS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResidueConFunds1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueConFunds1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_BIDS", Some("FUNDS_BID"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionBidsFundsBid1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionBidsFundsBid1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_BIDS", Some("PRICE_BID"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionBidsPriceBid1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionBidsPriceBid1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_PRICE_FUNDS_BID"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResiduePriceFundsBid1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResiduePriceFundsBid1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_PUBLIC_DATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResiduePublicData1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResiduePublicData1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionResidueTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_CASH_SECURITY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraCashSecurity1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraCashSecurity1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUCPAY_DETAIL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraFinancialAucpayDetail1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucpayDetail1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUCPAY_SUM"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraFinancialAucpaySum1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucpaySum1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUC_MARDETAIL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraFinancialAucMardetail1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucMardetail1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUC_MARGIN"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraFinancialAucMargin1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucMargin1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUC_RECEIPTS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraFinancialAucReceipts1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucReceipts1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_RUNTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraFinancialRuntrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialRuntrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_OFFER_PRODUCT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraOfferProduct1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraOfferProduct1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_OFFER_PROFILE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraOfferProfile1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraOfferProfile1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_CASH_SECURITY"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraPrudentialCashSecurity1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialCashSecurity1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_COMP_POSITION"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraPrudentialCompPosition1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialCompPosition1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_EXPOSURE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraPrudentialExposure1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialExposure1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_RUN"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionSraPrudentialRun1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialRun1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("VALUATIONID"), version) if version <= 1_i32 => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<data_model::IrauctionValuationid1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionValuationid1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("BIDTYPES"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigBidtypes1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigBidtypes1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("BIDTYPESTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigBidtypestrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigBidtypestrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTERCONNECTOR"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigInterconnector1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigInterconnector1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTERCONNECTORALLOC"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigInterconnectoralloc1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigInterconnectoralloc1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTERCONNECTORCONSTRAINT"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigInterconnectorconstraint1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigInterconnectorconstraint1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTRAREGIONALLOC"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigIntraregionalloc1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigIntraregionalloc1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("LOSSFACTORMODEL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigLossfactormodel1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigLossfactormodel1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("LOSSMODEL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigLossmodel1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigLossmodel1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("MARKET_PRICE_THRESHOLDS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigMarketPriceThresholds1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigMarketPriceThresholds1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("REGION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigRegion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigRegion1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("REGIONSTANDINGDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigRegionstandingdata1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigRegionstandingdata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("TRANSMISSIONLOSSFACTOR"), version) if version <= 2_i32 => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<data_model::MarketConfigTransmissionlossfactor2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigTransmissionlossfactor2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_NOTICE", Some("MARKETNOTICEDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_notice")]
                    {
                        let d: Vec<data_model::MarketNoticeMarketnoticedata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketNoticeMarketnoticedata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_notice"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_NOTICE", Some("MARKETNOTICETYPE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_notice")]
                    {
                        let d: Vec<data_model::MarketNoticeMarketnoticetype1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketNoticeMarketnoticetype1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_notice"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_NOTICE", Some("PARTICIPANTNOTICETRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "market_notice")]
                    {
                        let d: Vec<data_model::MarketNoticeParticipantnoticetrk1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketNoticeParticipantnoticetrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_notice"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MCC", Some("CASESOLUTION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "mcc_dispatch")]
                    {
                        let d: Vec<data_model::MccCasesolution1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMccCasesolution1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mcc_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MCC", Some("CONSTRAINTSOLUTION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "mcc_dispatch")]
                    {
                        let d: Vec<data_model::MccConstraintsolution1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMccConstraintsolution1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mcc_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("AGGREGATE_READS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<data_model::MeterdataAggregateReads1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataAggregateReads1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("INDIVIDUAL_READS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<data_model::MeterdataIndividualReads1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataIndividualReads1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("INTERCONNECTOR"), version) if version <= 1_i32 => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<data_model::MeterdataInterconnector1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataInterconnector1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("WDR_READS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<data_model::MeterdataWdrReads1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataWdrReads1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("EQUIPMENTDETAIL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkEquipmentdetail2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkEquipmentdetail2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("OUTAGECONSTRAINTSET"), version) if version <= 1_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkOutageconstraintset1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkOutageconstraintset1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("OUTAGEDETAIL"), version) if version <= 4_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkOutagedetail4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkOutagedetail4 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("OUTAGESTATUSCODE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkOutagestatuscode1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkOutagestatuscode1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("RATING"), version) if version <= 1_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkRating1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkRating1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("REALTIMERATING"), version) if version <= 1_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkRealtimerating1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkRealtimerating1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("STATICRATING"), version) if version <= 1_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkStaticrating1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkStaticrating1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("SUBSTATIONDETAIL"), version) if version <= 2_i32 => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<data_model::NetworkSubstationdetail2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkSubstationdetail2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("BLOCKED_CONSTRAINTS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minBlockedConstraints1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minBlockedConstraints1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("CASESOLUTION"), version) if version <= 2_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minCasesolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minCasesolution2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("CONSTRAINTSOLUTION"), version) if version <= 6_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minConstraintsolution6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minConstraintsolution6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("INTERCONNECTORSOLN"), version) if version <= 4_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minInterconnectorsoln4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minInterconnectorsoln4 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("INTERSENSITIVITIES"), version) if version <= 1_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minIntersensitivities1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minIntersensitivities1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("LOCAL_PRICE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minLocalPrice1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minLocalPrice1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("PRICESENSITIVITIES"), version) if version <= 1_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minPricesensitivities1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minPricesensitivities1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("REGIONSOLUTION"), version) if version <= 7_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minRegionsolution7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minRegionsolution7 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("SCENARIODEMAND"), version) if version <= 1_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minScenariodemand1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minScenariodemand1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("SCENARIODEMANDTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minScenariodemandtrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minScenariodemandtrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("UNITSOLUTION"), version) if version <= 4_i32 => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<data_model::P5minUnitsolution4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minUnitsolution4 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("BIDDUIDDETAILS"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationBidduiddetails1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetails1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("BIDDUIDDETAILSTRK"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationBidduiddetailstrk1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetailstrk1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DISPATCHABLEUNIT"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationDispatchableunit1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDispatchableunit1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DUALLOC"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationDualloc1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationDualloc1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DUDETAIL"), version) if version <= 4_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationDudetail4> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationDudetail4 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DUDETAILSUMMARY"), version)
                    if version <= 5_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationDudetailsummary5> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDudetailsummary5 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("GENMETER"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationGenmeter1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationGenmeter1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("GENUNITS"), version) if version <= 2_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationGenunits2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationGenunits2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("GENUNITS_UNIT"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationGenunitsUnit1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationGenunitsUnit1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("MNSP_INTERCONNECTOR"), version)
                    if version <= 2_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationMnspInterconnector2> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspInterconnector2 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("MNSP_PARTICIPANT"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationMnspParticipant1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspParticipant1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationParticipant1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationParticipant1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTACCOUNT"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationParticipantaccount1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantaccount1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCATEGORY"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationParticipantcategory1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategory1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCATEGORYALLOC"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationParticipantcategoryalloc1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategoryalloc1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCLASS"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationParticipantclass1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantclass1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCREDITDETAIL"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationParticipantcreditdetail1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcreditdetail1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PMS_GROUP"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationPmsGroup1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationPmsGroup1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PMS_GROUPNMI"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationPmsGroupnmi1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationPmsGroupnmi1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PMS_GROUPSERVICE"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationPmsGroupservice1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationPmsGroupservice1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STADUALLOC"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationStadualloc1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationStadualloc1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationStation1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationStation1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATIONOPERATINGSTATUS"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationStationoperatingstatus1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStationoperatingstatus1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATIONOWNER"), version) if version <= 1_i32 => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationStationowner1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationStationowner1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATIONOWNERTRK"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<data_model::ParticipantRegistrationStationownertrk1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStationownertrk1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PDPASA", Some("CASESOLUTION"), version) if version <= 3_i32 => {
                    #[cfg(feature = "pdpasa")]
                    {
                        let d: Vec<data_model::PdpasaCasesolution3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPdpasaCasesolution3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pdpasa"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PDPASA", Some("CONSTRAINTSOLUTION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pdpasa")]
                    {
                        let d: Vec<data_model::PdpasaConstraintsolution1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPdpasaConstraintsolution1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pdpasa"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PDPASA", Some("INTERCONNECTORSOLN"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pdpasa")]
                    {
                        let d: Vec<data_model::PdpasaInterconnectorsoln1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPdpasaInterconnectorsoln1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pdpasa"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PDPASA", Some("REGIONSOLUTION"), version) if version <= 7_i32 => {
                    #[cfg(feature = "pdpasa")]
                    {
                        let d: Vec<data_model::PdpasaRegionsolution7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPdpasaRegionsolution7 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pdpasa"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("BLOCKED_CONSTRAINTS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchBlockedConstraints1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchBlockedConstraints1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("CASE_SOLUTION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchCaseSolution1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchCaseSolution1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("CONSTRAINT_SOLUTION"), version) if version <= 5_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchConstraintSolution5> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchConstraintSolution5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("INTERCONNECTOR_SOLN"), version) if version <= 3_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchInterconnectorSoln3> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchInterconnectorSoln3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("INTERCONNECTR_SENS"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchInterconnectrSens1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchInterconnectrSens1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("UNIT_SOLUTION"), version) if version <= 2_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchUnitSolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchUnitSolution2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("OFFERTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchOffertrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchOffertrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("REGION_PRICES"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchRegionPrices1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchRegionPrices1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("PRICESENSITIVITIES"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchPricesensitivities1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchPricesensitivities1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("REGION_SOLUTION"), version) if version <= 6_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchRegionSolution6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchRegionSolution6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("SCENARIO_DEMAND"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchScenarioDemand1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchScenarioDemand1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("SCENARIO_DEMAND_TRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchScenarioDemandTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchScenarioDemandTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("REGIONFCASREQUIREMENT"), version) if version <= 2_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchRegionfcasrequirement2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchRegionfcasrequirement2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("LOCAL_PRICE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchLocalPrice1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchLocalPrice1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("MNSPBIDTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<data_model::PredispatchMnspbidtrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchMnspbidtrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRUDENTIAL", Some("COMPANY_POSITION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "prudentials")]
                    {
                        let d: Vec<data_model::PrudentialCompanyPosition1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPrudentialCompanyPosition1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "prudentials"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRUDENTIAL", Some("RUNTRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "prudentials")]
                    {
                        let d: Vec<data_model::PrudentialRuntrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPrudentialRuntrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "prudentials"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MTPASA", Some("RESERVELIMIT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<data_model::MtpasaReservelimit1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMtpasaReservelimit1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MTPASA", Some("RESERVELIMIT_REGION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<data_model::MtpasaReservelimitRegion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMtpasaReservelimitRegion1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MTPASA", Some("RESERVELIMIT_SET"), version) if version <= 1_i32 => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<data_model::MtpasaReservelimitSet1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMtpasaReservelimitSet1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("RESERVE_DATA", Some("RESERVE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<data_model::ReserveDataReserve1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertReserveDataReserve1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("ANCILLARY_RECOVERY_SPLIT"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigAncillaryRecoverySplit1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigAncillaryRecoverySplit1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKETFEE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigMarketfee1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketfee1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKETFEEDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigMarketfeedata1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketfeedata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKETFEETRK"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigMarketfeetrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketfeetrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_CAT_EXCL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigMarketFeeCatExcl1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeCatExcl1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_CAT_EXCL_TRK"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigMarketFeeCatExclTrk1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeCatExclTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_EXCLUSION"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigMarketFeeExclusion1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeExclusion1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_EXCLUSION_TRK"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigMarketFeeExclusionTrk1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeExclusionTrk1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("PARTICIPANT_BANDFEE_ALLOC"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigParticipantBandfeeAlloc1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigParticipantBandfeeAlloc1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETCFG", Some("REALLOCATION"), version) if version <= 2_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SetcfgReallocation2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSetcfgReallocation2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETCFG", Some("REALLOCATIONINTERVAL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SetcfgReallocationinterval1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSetcfgReallocationinterval1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("SETCFG_PARTICIPANT_MPF"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigSetcfgParticipantMpf1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpf1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("SETCFG_PARTICIPANT_MPFTRK"), version)
                    if version <= 1_i32 =>
                {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementConfigSetcfgParticipantMpftrk1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpftrk1 @P1, @P2", chunk_size).await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS_CONFIG", Some("WDRRR_CALENDAR"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementsConfigWdrrrCalendar1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsConfigWdrrrCalendar1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS_CONFIG", Some("WDR_REIMBURSE_RATE"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<data_model::SettlementsConfigWdrReimburseRate1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsConfigWdrReimburseRate1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("DAYTRACK"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsDaytrack6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsDaytrack6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("CPDATA"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsCpdata6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsCpdata6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("CPDATAREGION"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsCpdataregion5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsCpdataregion5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("FCASREGIONRECOVERY"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsFcasregionrecovery5> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsFcasregionrecovery5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("GENDATA"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsGendata6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsGendata6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("GENDATAREGION"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsGendataregion5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsGendataregion5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("INTRAREGIONRESIDUES"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsIntraregionresidues5> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIntraregionresidues5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRAUCSURPLUS"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsIraucsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIraucsurplus6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRNSPSURPLUS"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsIrnspsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIrnspsurplus6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRPARTSURPLUS"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsIrpartsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIrpartsurplus6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRSURPLUS"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsIrsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIrsurplus6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LOCALAREAENERGY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsLocalareaenergy1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLocalareaenergy1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LOCALAREATNI"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsLocalareatni1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLocalareatni1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LSHEDPAYMENT"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsLshedpayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLshedpayment5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("MARKETFEES"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsMarketfees6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsMarketfees6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("REALLOCATIONS"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsReallocations5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsReallocations5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RESTARTPAYMENT"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsRestartpayment6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRestartpayment6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RESTARTRECOVERY"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsRestartrecovery6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRestartrecovery6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RPOWERPAYMENT"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsRpowerpayment6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRpowerpayment6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("SMALLGENDATA"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsSmallgendata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsSmallgendata1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("ANCILLARY_SUMMARY"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsAncillarySummary5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsAncillarySummary5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("APC_COMPENSATION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsApcCompensation1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsApcCompensation1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("APC_RECOVERY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsApcRecovery1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsApcRecovery1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("FCAS_PAYMENT"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsFcasPayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsFcasPayment5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("FCAS_RECOVERY"), version) if version <= 6_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsFcasRecovery6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsFcasRecovery6 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("SET_FCAS_REGULATION_TRK"), version) if version <= 2_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsSetFcasRegulationTrk2> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsSetFcasRegulationTrk2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("NMAS_RECOVERY"), version) if version <= 2_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsNmasRecovery2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsNmasRecovery2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("NMAS_RECOVERY_RBF"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsNmasRecoveryRbf1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsNmasRecoveryRbf1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RECOVERY_ENERGY"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsRecoveryEnergy1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRecoveryEnergy1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RUN_PARAMETER"), version) if version <= 5_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsRunParameter5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRunParameter5 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("SUBST_DEMAND"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsSubstDemand1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsSubstDemand1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("SUBST_RUN_VERSION"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsSubstRunVersion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsSubstRunVersion1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("WDR_RECON_DETAIL"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsWdrReconDetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsWdrReconDetail1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("WDR_TRANSACT"), version) if version <= 1_i32 => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<data_model::SettlementsWdrTransact1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsWdrTransact1 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("CASESOLUTION"), version) if version <= 3_i32 => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<data_model::StpasaCasesolution3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaCasesolution3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("CONSTRAINTSOLUTION"), version) if version <= 3_i32 => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<data_model::StpasaConstraintsolution3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaConstraintsolution3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("INTERCONNECTORSOLN"), version) if version <= 3_i32 => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<data_model::StpasaInterconnectorsoln3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaInterconnectorsoln3 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("REGIONSOLUTION"), version) if version <= 7_i32 => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<data_model::StpasaRegionsolution7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaRegionsolution7 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("TRADING", Some("AVERAGEPRICE30"), version) if version <= 1_i32 => {
                    #[cfg(feature = "trading_data")]
                    {
                        let d: Vec<data_model::TradingAverageprice301> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertTradingAverageprice301 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "trading_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("TRADING", Some("INTERCONNECTORRES"), version) if version <= 2_i32 => {
                    #[cfg(feature = "trading_data")]
                    {
                        let d: Vec<data_model::TradingInterconnectorres2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertTradingInterconnectorres2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "trading_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("TRADING", Some("PRICE"), version) if version <= 2_i32 => {
                    #[cfg(feature = "trading_data")]
                    {
                        let d: Vec<data_model::TradingPrice2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertTradingPrice2 @P1, @P2",
                            chunk_size,
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "trading_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                _ => {
                    log::error!("Unexpected file key {:?}", file_key);
                    continue;
                }
            }
        }
        Ok(())
    }
}
