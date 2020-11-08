use crate::mmsdm::*;
use futures::{AsyncRead, AsyncWrite};

impl crate::AemoFile {
    async fn log_file<S>(
        &self,
        client: &mut tiberius::Client<S>,
        key: &crate::FileKey,
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
                participant_name,
                privacy_level,
                effective_date,
                serial_number,
                data_set,
                sub_type,
                version
            )
            output inserted.file_log_id
            values (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8);",
                &[
                    &self.header.data_source,
                    &self.header.participant_name,
                    &self.header.privacy_level,
                    &self.header.get_effective(),
                    &self.header.serial_number,
                    &key.data_set_name.as_str(),
                    &key.table_name(),
                    &key.version,
                ],
            )
            .await?
            .into_row()
            .await?;
        let row = first_row.ok_or_else(|| crate::Error::CreateFileLogError)?;
        row.try_get(0)?
            .ok_or_else(|| crate::Error::CreateFileLogError)
    }

    async fn batched_insert<S, D>(
        &self,
        client: &mut tiberius::Client<S>,
        file_key: &crate::FileKey,
        data: &[D],
        proc: &str,
    ) -> crate::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
        D: serde::Serialize,
    {
        let file_log_id = self.log_file(client, file_key).await?;

        let total = data.len();
        let mut current = 0_usize;
        for chunk in data.chunks(100_000_usize) {
            current += chunk.len();
            let json = serde_json::to_string(chunk)?;
            if let Err(e) = client.execute(proc, &[&file_log_id, &json]).await {
                client.execute(
                    "update mmsdm.FileLog set [status] = 'E', message = @P1 where file_log_id = @P2",
                    &[&e.to_string(), &file_log_id],
                ).await?;
                return Err(e.into());
            } else {
                log::debug!("Progress: {} out of {} rows saved", current, total);
            }
        }
        client
            .execute(
                "update mmsdm.FileLog set [status] = 'C' where file_log_id = @P1",
                &[&file_log_id],
            )
            .await?;
        Ok(())
    }

    /// This function is meant to be used in conjunction with the iterator over
    /// the data contained within the AemoFile struct
    pub async fn load_data<S>(&self, client: &mut tiberius::Client<S>) -> crate::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        for file_key in self.data.keys() {
            match (
                file_key.data_set_name.as_str(),
                file_key.table_name.as_ref().map(|s| s.as_str()),
                file_key.version,
            ) {
                ("DEMAND", Some("TRK"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::Trk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OPERATIONAL_DEMAND", Some("ACTUAL"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::Actual1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOperationalDemandActual1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORECAST", Some("INTERMITTENT_GEN"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentGen1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForecastIntermittentGen1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_CLUSTER_AVAIL_DAY"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentClusterAvailDay1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentClusterAvailDay1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_DS_RUN"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentDsRun1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentDsRun1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ROOFTOP", Some("FORECAST"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::Forecast1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertRooftopForecast1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("PERIOD"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::Period1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandPeriod1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("MTPASA_INTERMITTENT_AVAIL"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::MtpasaIntermittentAvail1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandMtpasaIntermittentAvail1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORECAST", Some("INTERMITTENT_GEN_DATA"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentGenData1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForecastIntermittentGenData1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_DS_PRED"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentDsPred1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentDsPred1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_GEN_LIMIT_DAY"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentGenLimitDay1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentGenLimitDay1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("MTPASA_INTERMITTENT_LIMIT"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::MtpasaIntermittentLimit1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandMtpasaIntermittentLimit1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OPERATIONAL_DEMAND", Some("FORECAST"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::Forecast1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOperationalDemandForecast1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_CLUSTER_AVAIL"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentClusterAvail1> =
                            self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentClusterAvail1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ROOFTOP", Some("ACTUAL"), 2_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::Actual2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertRooftopActual2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DEMAND", Some("INTERMITTENT_GEN_LIMIT"), 1_i32) => {
                    #[cfg(feature = "demand_forecasts")]
                    {
                        let d: Vec<demand_forecasts::IntermittentGenLimit1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDemandIntermittentGenLimit1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "demand_forecasts"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("TRADING", Some("PRICE"), 2_i32) => {
                    #[cfg(feature = "trading_data")]
                    {
                        let d: Vec<trading_data::Price2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertTradingPrice2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "trading_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("TRADING", Some("REGIONSUM"), 4_i32) => {
                    #[cfg(feature = "trading_data")]
                    {
                        let d: Vec<trading_data::Regionsum4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertTradingRegionsum4 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "trading_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("TRADING", Some("UNIT_SOLUTION"), 2_i32) => {
                    #[cfg(feature = "trading_data")]
                    {
                        let d: Vec<trading_data::UnitSolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertTradingUnitSolution2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "trading_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("TRADING", Some("INTERCONNECTORRES"), 2_i32) => {
                    #[cfg(feature = "trading_data")]
                    {
                        let d: Vec<trading_data::Interconnectorres2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertTradingInterconnectorres2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "trading_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MR", Some("EVENT_SCHEDULE"), 1_i32) => {
                    #[cfg(feature = "mrevent")]
                    {
                        let d: Vec<mrevent::EventSchedule1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMrEventSchedule1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mrevent"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MR", Some("EVENT"), 1_i32) => {
                    #[cfg(feature = "mrevent")]
                    {
                        let d: Vec<mrevent::Event1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMrEvent1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mrevent"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MR", Some("DAYOFFER_STACK"), 1_i32) => {
                    #[cfg(feature = "mrevent")]
                    {
                        let d: Vec<mrevent::DayofferStack1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMrDayofferStack1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mrevent"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MR", Some("PEROFFER_STACK"), 1_i32) => {
                    #[cfg(feature = "mrevent")]
                    {
                        let d: Vec<mrevent::PerofferStack1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMrPerofferStack1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mrevent"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("INTERCONNECTORSOLN"), 2_i32) => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<stpasa_solution::Interconnectorsoln2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaInterconnectorsoln2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("CONSTRAINTSOLUTION"), 2_i32) => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<stpasa_solution::Constraintsolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaConstraintsolution2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("REGIONSOLUTION"), 5_i32) => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<stpasa_solution::Regionsolution5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaRegionsolution5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("STPASA", Some("CASESOLUTION"), 3_i32) => {
                    #[cfg(feature = "stpasa_solution")]
                    {
                        let d: Vec<stpasa_solution::Casesolution3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertStpasaCasesolution3 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "stpasa_solution"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("REGIONAPC"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::Regionapc1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApRegionapc1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_SCHEDULE"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::MarketSuspendSchedule1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendSchedule1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("IRFMAMOUNT"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::Irfmamount1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureIrfmamount1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("APEVENT"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::Apevent1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApApevent1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_REGIME_SUM"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::MarketSuspendRegimeSum1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendRegimeSum1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_REGION_SUM"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::MarketSuspendRegionSum1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendRegionSum1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("APEVENTREGION"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::Apeventregion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApApeventregion1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("MARKET_SUSPEND_SCHEDULE_TRK"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::MarketSuspendScheduleTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureMarketSuspendScheduleTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("AP", Some("REGIONAPCINTERVALS"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::Regionapcintervals1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertApRegionapcintervals1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("IRFMEVENTS"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::Irfmevents1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureIrfmevents1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("FORCE_MAJEURE", Some("OVERRIDERRP"), 1_i32) => {
                    #[cfg(feature = "force_majeure")]
                    {
                        let d: Vec<force_majeure::Overriderrp1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertForceMajeureOverriderrp1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "force_majeure"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTRAREGIONALLOC"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Intraregionalloc1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigIntraregionalloc1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("TRANSMISSIONLOSSFACTOR"), 2_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Transmissionlossfactor2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigTransmissionlossfactor2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("REGION"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Region1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigRegion1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("BIDTYPES"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Bidtypes1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigBidtypes1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("BIDTYPESTRK"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Bidtypestrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigBidtypestrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTERCONNECTOR"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Interconnector1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigInterconnector1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("REGIONSTANDINGDATA"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Regionstandingdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigRegionstandingdata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("LOSSFACTORMODEL"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Lossfactormodel1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigLossfactormodel1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("LOSSMODEL"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Lossmodel1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigLossmodel1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTERCONNECTORCONSTRAINT"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Interconnectorconstraint1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigInterconnectorconstraint1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("INTERCONNECTORALLOC"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::Interconnectoralloc1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigInterconnectoralloc1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_CONFIG", Some("MARKET_PRICE_THRESHOLDS"), 1_i32) => {
                    #[cfg(feature = "market_config")]
                    {
                        let d: Vec<market_config::MarketPriceThresholds1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketConfigMarketPriceThresholds1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("SECDEPOSIT_PROVISION"), 1_i32) => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<billing_config::SecdepositProvision1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigSecdepositProvision1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_TRANSACTION_CLASS"), 1_i32) => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<billing_config::GstTransactionClass1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstTransactionClass1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("SECDEPOSIT_INTEREST_RATE"), 1_i32) => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<billing_config::SecdepositInterestRate1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigSecdepositInterestRate1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_TRANSACTION_TYPE"), 1_i32) => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<billing_config::GstTransactionType1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstTransactionType1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_BAS_CLASS"), 1_i32) => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<billing_config::GstBasClass1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstBasClass1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("BILLINGCALENDAR"), 2_i32) => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<billing_config::Billingcalendar2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigBillingcalendar2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING_CONFIG", Some("GST_RATE"), 1_i32) => {
                    #[cfg(feature = "billing_config")]
                    {
                        let d: Vec<billing_config::GstRate1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingConfigGstRate1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("SCENARIO_DEMAND_TRK"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::ScenarioDemandTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchScenarioDemandTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("OFFERTRK"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::Offertrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchOffertrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("INTERCONNECTR_SENS"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::InterconnectrSens1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchInterconnectrSens1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("INTERCONNECTOR_SOLN"), 3_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::InterconnectorSoln3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchInterconnectorSoln3 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("PRICESENSITIVITIES"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::Pricesensitivities1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchPricesensitivities1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("CONSTRAINT_SOLUTION"), 5_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::ConstraintSolution5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchConstraintSolution5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("REGION_PRICES"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::RegionPrices1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchRegionPrices1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("LOCAL_PRICE"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::LocalPrice1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchLocalPrice1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("REGIONFCASREQUIREMENT"), 2_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::Regionfcasrequirement2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchRegionfcasrequirement2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("MNSPBIDTRK"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::Mnspbidtrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchMnspbidtrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("CASE_SOLUTION"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::CaseSolution1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchCaseSolution1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("UNIT_SOLUTION"), 2_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::UnitSolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchUnitSolution2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("REGION_SOLUTION"), 4_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::RegionSolution4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchRegionSolution4 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("BLOCKED_CONSTRAINTS"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::BlockedConstraints1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchBlockedConstraints1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PREDISPATCH", Some("SCENARIO_DEMAND"), 1_i32) => {
                    #[cfg(feature = "pre_dispatch")]
                    {
                        let d: Vec<pre_dispatch::ScenarioDemand1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPredispatchScenarioDemand1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pre_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_NOTICE", Some("PARTICIPANTNOTICETRK"), 1_i32) => {
                    #[cfg(feature = "market_notice")]
                    {
                        let d: Vec<market_notice::Participantnoticetrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketNoticeParticipantnoticetrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_notice"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_NOTICE", Some("MARKETNOTICEDATA"), 1_i32) => {
                    #[cfg(feature = "market_notice")]
                    {
                        let d: Vec<market_notice::Marketnoticedata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketNoticeMarketnoticedata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_notice"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MARKET_NOTICE", Some("MARKETNOTICETYPE"), 1_i32) => {
                    #[cfg(feature = "market_notice")]
                    {
                        let d: Vec<market_notice::Marketnoticetype1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMarketNoticeMarketnoticetype1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "market_notice"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("VOLTAGE_INSTRUCTION", Some("TRACK"), 2_i32) => {
                    #[cfg(feature = "voltage_instructions")]
                    {
                        let d: Vec<voltage_instructions::Track2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertVoltageInstructionTrack2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "voltage_instructions"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("VOLTAGE_INSTRUCTION", Some("INSTRUCTION"), 2_i32) => {
                    #[cfg(feature = "voltage_instructions")]
                    {
                        let d: Vec<voltage_instructions::Instruction2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertVoltageInstructionInstruction2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "voltage_instructions"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_IC_ALLOCATIONS"), 2_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::AuctionIcAllocations2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionIcAllocations2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_RUN"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraPrudentialRun1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialRun1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUC_RECEIPTS"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraFinancialAucReceipts1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucReceipts1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_CASH_SECURITY"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraCashSecurity1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraCashSecurity1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_BID_TRK"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResidueBidTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueBidTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUCPAY_DETAIL"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraFinancialAucpayDetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucpayDetail1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_BIDS", Some("FILE_TRK"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::FileTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionBidsFileTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUCPAY_SUM"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraFinancialAucpaySum1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucpaySum1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CONTRACTS"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResidueContracts1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueContracts1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_BIDS", Some("FUNDS_BID"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::FundsBid1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionBidsFundsBid1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_RP_ESTIMATE"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::AuctionRpEstimate1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionRpEstimate1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_CASH_SECURITY"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraPrudentialCashSecurity1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialCashSecurity1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUC_MARGIN"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraFinancialAucMargin1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucMargin1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_AUC_MARDETAIL"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraFinancialAucMardetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialAucMardetail1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_OFFER_PROFILE"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraOfferProfile1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraOfferProfile1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_BIDS", Some("PRICE_BID"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::PriceBid1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionBidsPriceBid1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_TRANCHE"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::AuctionTranche1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionTranche1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_REVENUE_TRACK"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::AuctionRevenueTrack1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionRevenueTrack1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_REVENUE_ESTIMATE"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::AuctionRevenueEstimate1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionRevenueEstimate1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::Auction1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuction1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_TRK"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResidueTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("RESIDUECONTRACTPAYMENTS"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::Residuecontractpayments1> = self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigResiduecontractpayments1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("VALUATIONID"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::Valuationid1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionValuationid1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CON_ESTIMATES_TRK"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResidueConEstimatesTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueConEstimatesTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION_CONFIG", Some("AUCTION_CALENDAR"), 2_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::AuctionCalendar2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionConfigAuctionCalendar2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_PRICE_FUNDS_BID"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResiduePriceFundsBid1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResiduePriceFundsBid1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_COMP_POSITION"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraPrudentialCompPosition1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialCompPosition1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_PRUDENTIAL_EXPOSURE"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraPrudentialExposure1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraPrudentialExposure1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_PUBLIC_DATA"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResiduePublicData1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResiduePublicData1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CON_DATA"), 2_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResidueConData2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueConData2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_FINANCIAL_RUNTRK"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraFinancialRuntrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraFinancialRuntrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("RESIDUE_CON_FUNDS"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::ResidueConFunds1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionResidueConFunds1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("IRAUCTION", Some("SRA_OFFER_PRODUCT"), 1_i32) => {
                    #[cfg(feature = "irauction")]
                    {
                        let d: Vec<irauction::SraOfferProduct1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertIrauctionSraOfferProduct1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "irauction"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BID", Some("MNSP_FILETRK"), 1_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::MnspFiletrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidMnspFiletrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BID", Some("BIDDAYOFFER_D"), 2_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::BiddayofferD2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidBiddayofferD2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("MNSP_DAYOFFER"), 2_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::MnspDayoffer2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferMnspDayoffer2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("MTPASA_OFFERFILETRK"), 1_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::MtpasaOfferfiletrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferMtpasaOfferfiletrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("BIDDAYOFFER"), 2_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::Biddayoffer2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferBiddayoffer2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("MNSP_OFFERTRK"), 1_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::MnspOffertrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferMnspOffertrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("MNSP_PEROFFER"), 1_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::MnspPeroffer1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferMnspPeroffer1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BID", Some("BIDPEROFFER_D"), 2_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::BidperofferD2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBidBidperofferD2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("BIDPEROFFER"), 1_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::Bidperoffer1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferBidperoffer1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("BIDOFFERFILETRK"), 1_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::Bidofferfiletrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferBidofferfiletrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("OFFER", Some("MTPASA_OFFERDATA"), 1_i32) => {
                    #[cfg(feature = "bids")]
                    {
                        let d: Vec<bids::MtpasaOfferdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertOfferMtpasaOfferdata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "bids"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("ANCILLARY_RECOVERY_SPLIT"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::AncillaryRecoverySplit1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigAncillaryRecoverySplit1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETCFG", Some("REALLOCATION"), 2_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::Reallocation2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSetcfgReallocation2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKETFEEDATA"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::Marketfeedata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketfeedata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("SETCFG_PARTICIPANT_MPF"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::SetcfgParticipantMpf1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpf1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETCFG", Some("REALLOCATIONINTERVAL"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::Reallocationinterval1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSetcfgReallocationinterval1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_EXCLUSION_TRK"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::MarketFeeExclusionTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeExclusionTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKETFEE"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::Marketfee1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketfee1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_CAT_EXCL_TRK"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::MarketFeeCatExclTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeCatExclTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_CAT_EXCL"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::MarketFeeCatExcl1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeCatExcl1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKET_FEE_EXCLUSION"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::MarketFeeExclusion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketFeeExclusion1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("MARKETFEETRK"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::Marketfeetrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementConfigMarketfeetrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("PARTICIPANT_BANDFEE_ALLOC"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::ParticipantBandfeeAlloc1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigParticipantBandfeeAlloc1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENT_CONFIG", Some("SETCFG_PARTICIPANT_MPFTRK"), 1_i32) => {
                    #[cfg(feature = "settlement_config")]
                    {
                        let d: Vec<settlement_config::SetcfgParticipantMpftrk1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpftrk1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "settlement_config"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERRESTARTDATA"), 1_i32) => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<asoffer::Offerrestartdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferrestartdata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERLSHEDDATA"), 1_i32) => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<asoffer::Offerlsheddata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferlsheddata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERAGCDATA"), 1_i32) => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<asoffer::Offeragcdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferagcdata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERRPOWERDATA"), 1_i32) => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<asoffer::Offerrpowerdata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferrpowerdata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("ASOFFER", Some("OFFERASTRK"), 1_i32) => {
                    #[cfg(feature = "asoffer")]
                    {
                        let d: Vec<asoffer::Offerastrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertAsofferOfferastrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "asoffer"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("INTERCONNECTORSOLN"), 4_i32) => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<p5min::Interconnectorsoln4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minInterconnectorsoln4 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("LOCAL_PRICE"), 1_i32) => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<p5min::LocalPrice1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minLocalPrice1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("CONSTRAINTSOLUTION"), 6_i32) => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<p5min::Constraintsolution6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minConstraintsolution6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("BLOCKED_CONSTRAINTS"), 1_i32) => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<p5min::BlockedConstraints1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minBlockedConstraints1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("REGIONSOLUTION"), 5_i32) => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<p5min::Regionsolution5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minRegionsolution5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("UNITSOLUTION"), 3_i32) => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<p5min::Unitsolution3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minUnitsolution3 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("P5MIN", Some("CASESOLUTION"), 2_i32) => {
                    #[cfg(feature = "p5min")]
                    {
                        let d: Vec<p5min::Casesolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertP5minCasesolution2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "p5min"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENERIC_CONSTRAINT", Some("GENCONSETINVOKE"), 2_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Genconsetinvoke2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenericConstraintGenconsetinvoke2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GEQRHS", Some("NULL"), 1_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGeqrhsNull1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENERIC_CONSTRAINT", Some("EMSMASTER"), 1_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Emsmaster1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenericConstraintEmsmaster1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENCONSET", Some("NULL"), 1_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenconsetNull1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENCONSETTRK", Some("NULL"), 2_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGenconsettrkNull2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SPDRC", Some("NULL"), 2_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSpdrcNull2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SPDICC", Some("NULL"), 1_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSpdiccNull1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GENCONDATA", Some("NULL"), 6_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGencondataNull6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SPDCPC", Some("NULL"), 2_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSpdcpcNull2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GCRHS", Some("NULL"), 1_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGcrhsNull1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GEQDESC", Some("NULL"), 2_i32) => {
                    #[cfg(feature = "generic_constraint")]
                    {
                        let d: Vec<generic_constraint::Null2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGeqdescNull2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "generic_constraint"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PDPASA", Some("REGIONSOLUTION"), 5_i32) => {
                    #[cfg(feature = "pdpasa")]
                    {
                        let d: Vec<pdpasa::Regionsolution5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPdpasaRegionsolution5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pdpasa"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PDPASA", Some("CASESOLUTION"), 3_i32) => {
                    #[cfg(feature = "pdpasa")]
                    {
                        let d: Vec<pdpasa::Casesolution3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPdpasaCasesolution3 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "pdpasa"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MTPASA", Some("RESERVELIMIT_SET"), 1_i32) => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<reserve_data::ReservelimitSet1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMtpasaReservelimitSet1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("RESERVE_DATA", Some("RESERVE"), 1_i32) => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<reserve_data::Reserve1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertReserveDataReserve1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MTPASA", Some("RESERVELIMIT_REGION"), 1_i32) => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<reserve_data::ReservelimitRegion1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMtpasaReservelimitRegion1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MTPASA", Some("RESERVELIMIT"), 1_i32) => {
                    #[cfg(feature = "reserve_data")]
                    {
                        let d: Vec<reserve_data::Reservelimit1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMtpasaReservelimit1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "reserve_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MCC", Some("CASESOLUTION"), 1_i32) => {
                    #[cfg(feature = "mcc_dispatch")]
                    {
                        let d: Vec<mcc_dispatch::Casesolution1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMccCasesolution1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mcc_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("MCC", Some("CONSTRAINTSOLUTION"), 1_i32) => {
                    #[cfg(feature = "mcc_dispatch")]
                    {
                        let d: Vec<mcc_dispatch::Constraintsolution1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMccConstraintsolution1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "mcc_dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRUDENTIAL", Some("COMPANY_POSITION"), 1_i32) => {
                    #[cfg(feature = "prudentials")]
                    {
                        let d: Vec<prudentials::CompanyPosition1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPrudentialCompanyPosition1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "prudentials"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRUDENTIAL", Some("RUNTRK"), 1_i32) => {
                    #[cfg(feature = "prudentials")]
                    {
                        let d: Vec<prudentials::Runtrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPrudentialRuntrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "prudentials"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRNSPSURPLUSSUM"), 6_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Irnspsurplussum6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrnspsurplussum6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("FINANCIALADJUSTMENTS"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Financialadjustments5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingFinancialadjustments5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SECDEP_INTEREST_RATE"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::SecdepInterestRate1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSecdepInterestRate1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("BILLING_DIRECTION_RECON_OTHER"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::BillingDirectionReconOther1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingBillingDirectionReconOther1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REGIONIMPORTS"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Regionimports5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRegionimports5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("GENDATA"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Gendata5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingGendata5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("BILLING_CO2E_PUBLICATION_TRK"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::BillingCo2ePublicationTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingBillingCo2ePublicationTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("RES_TRADER_PAYMENT"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::ResTraderPayment1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingResTraderPayment1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("PRIORADJUSTMENTS"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Prioradjustments5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingPrioradjustments5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("DAYTRK"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Daytrk5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingDaytrk5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_RECVRY_TRK"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::NmasTstRecvryTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstRecvryTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("APC_RECOVERY"), 2_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::ApcRecovery2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingApcRecovery2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("GST_SUMMARY"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::GstSummary5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingGstSummary5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("FEES"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Fees5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingFees5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("WHITEHOLE"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Whitehole5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingWhitehole5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REGIONEXPORTS"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Regionexports5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRegionexports5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRPARTSURPLUS"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Irpartsurplus5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrpartsurplus5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("DIRECTION_RECONCILIATN"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::DirectionReconciliatn1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingDirectionReconciliatn1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SECDEP_INTEREST_PAY"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::SecdepInterestPay1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSecdepInterestPay1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRAUCSURPLUSSUM"), 7_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Iraucsurplussum7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIraucsurplussum7 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("CPDATA"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Cpdata5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingCpdata5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SMELTERREDUCTION"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Smelterreduction5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSmelterreduction5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("INTERRESIDUES"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Interresidues5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingInterresidues5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("MR_SUMMARY"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::MrSummary5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingMrSummary5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("EFTSHORTFALL_DETAIL"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::EftshortfallDetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingEftshortfallDetail1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRFM"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Irfm5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrfm5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("SECDEPOSIT_APPLICATION"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::SecdepositApplication1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingSecdepositApplication1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_RECOVERY"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::NmasTstRecovery1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstRecovery1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("GST_DETAIL"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::GstDetail5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingGstDetail5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("ASRECOVERY"), 7_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Asrecovery7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingAsrecovery7 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REALLOC"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Realloc5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRealloc5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("APC_COMPENSATION"), 2_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::ApcCompensation2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingApcCompensation2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_RECVRY_RBF"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::NmasTstRecvryRbf1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstRecvryRbf1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("INTRARESIDUES"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Intraresidues5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIntraresidues5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("BILLING_CO2E_PUBLICATION"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::BillingCo2ePublication1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingBillingCo2ePublication1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("DAILY_ENERGY_SUMMARY"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::DailyEnergySummary1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingDailyEnergySummary1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("MR_PAYMENT"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::MrPayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingMrPayment5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("ASPAYMENTS"), 6_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Aspayments6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingAspayments6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("RUNTRK"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Runtrk5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRuntrk5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REGIONFIGURES"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Regionfigures5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingRegionfigures5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("RES_TRADER_RECOVERY"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::ResTraderRecovery1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingResTraderRecovery1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("EFTSHORTFALL_AMOUNT"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::EftshortfallAmount1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingEftshortfallAmount1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("MR_SHORTFALL"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::MrShortfall5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingMrShortfall5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("MR_RECOVERY"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::MrRecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingMrRecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("NMAS_TST_PAYMENTS"), 1_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::NmasTstPayments1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingNmasTstPayments1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRAUCSURPLUS"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Iraucsurplus5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIraucsurplus5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRPARTSURPLUSSUM"), 7_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Irpartsurplussum7> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrpartsurplussum7 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("REALLOC_DETAIL"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::ReallocDetail5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingReallocDetail5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("BILLING", Some("IRNSPSURPLUS"), 5_i32) => {
                    #[cfg(feature = "billing_run")]
                    {
                        let d: Vec<billing_run::Irnspsurplus5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertBillingIrnspsurplus5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "billing_run"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCREDITDETAIL"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Participantcreditdetail1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcreditdetail1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATIONOWNER"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Stationowner1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationStationowner1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DUDETAIL"), 3_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Dudetail3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationDudetail3 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("GENMETER"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Genmeter1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationGenmeter1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("MNSP_PARTICIPANT"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::MnspParticipant1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspParticipant1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCATEGORYALLOC"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Participantcategoryalloc1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategoryalloc1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("GENUNITS_UNIT"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::GenunitsUnit1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationGenunitsUnit1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("BIDDUIDDETAILS"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Bidduiddetails1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetails1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("MNSP_INTERCONNECTOR"), 2_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::MnspInterconnector2> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspInterconnector2 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTACCOUNT"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Participantaccount1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantaccount1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STADUALLOC"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Stadualloc1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationStadualloc1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("BIDDUIDDETAILSTRK"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Bidduiddetailstrk1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetailstrk1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATIONOPERATINGSTATUS"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Stationoperatingstatus1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStationoperatingstatus1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATIONOWNERTRK"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Stationownertrk1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStationownertrk1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DISPATCHABLEUNIT"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Dispatchableunit1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDispatchableunit1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCLASS"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Participantclass1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantclass1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DUALLOC"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Dualloc1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationDualloc1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("DUDETAILSUMMARY"), 4_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Dudetailsummary4> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDudetailsummary4 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("STATION"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Station1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationStation1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANTCATEGORY"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Participantcategory1> =
                            self.get_table()?;
                        self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategory1 @P1, @P2").await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("GENUNITS"), 2_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Genunits2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationGenunits2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PARTICIPANT_REGISTRATION", Some("PARTICIPANT"), 1_i32) => {
                    #[cfg(feature = "participant_registration")]
                    {
                        let d: Vec<participant_registration::Participant1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertParticipantRegistrationParticipant1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "participant_registration"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("INDIVIDUAL_READS"), 1_i32) => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<meter_data::IndividualReads1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataIndividualReads1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("AGGREGATE_READS"), 1_i32) => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<meter_data::AggregateReads1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataAggregateReads1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("INTERCONNECTOR"), 1_i32) => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<meter_data::Interconnector1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataInterconnector1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("METERDATA", Some("TRK"), 1_i32) => {
                    #[cfg(feature = "meter_data")]
                    {
                        let d: Vec<meter_data::Trk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertMeterdataTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "meter_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("FCAS_RECOVERY"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::FcasRecovery6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsFcasRecovery6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LSHEDPAYMENT"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Lshedpayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLshedpayment5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("INTERVENTION"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Intervention5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIntervention5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("FCASREGIONRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Fcasregionrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsFcasregionrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("ANCILLARY_SUMMARY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::AncillarySummary5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsAncillarySummary5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("MR_RECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::MrRecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsMrRecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("AGCRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Agcrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsAgcrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("SMALLGENDATA"), 1_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Smallgendata1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsSmallgendata1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("NMAS_RECOVERY"), 2_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::NmasRecovery2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsNmasRecovery2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRSURPLUS"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Irsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIrsurplus6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("VICENERGYFLOW"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Vicenergyflow5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsVicenergyflow5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LUNLOADRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Lunloadrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLunloadrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RESTARTRECOVERY"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Restartrecovery6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRestartrecovery6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRNSPSURPLUS"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Irnspsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIrnspsurplus6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("CPDATA"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Cpdata5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsCpdata5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRFMRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Irfmrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIrfmrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("CPDATAREGION"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Cpdataregion5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsCpdataregion5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("INTERVENTIONRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Interventionrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsInterventionrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("GENDATAREGION"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Gendataregion5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsGendataregion5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("AGCPAYMENT"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Agcpayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsAgcpayment5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("FCASCOMP"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Fcascomp5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsFcascomp5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RUN_PARAMETER"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::RunParameter5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRunParameter5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RESTARTPAYMENT"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Restartpayment6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRestartpayment6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RPOWERPAYMENT"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Rpowerpayment6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRpowerpayment6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("REALLOCATIONS"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Reallocations5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsReallocations5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("RPOWERRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Rpowerrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsRpowerrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("NMAS_RECOVERY_RBF"), 1_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::NmasRecoveryRbf1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsNmasRecoveryRbf1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("VICENERGYFIGURES"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Vicenergyfigures5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsVicenergyfigures5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("DAYTRACK"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Daytrack5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsDaytrack5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LUNLOADPAYMENT"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Lunloadpayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLunloadpayment5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LULOADRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Luloadrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLuloadrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("LSHEDRECOVERY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Lshedrecovery5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsLshedrecovery5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("APC_COMPENSATION"), 1_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::ApcCompensation1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsApcCompensation1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("INTRAREGIONRESIDUES"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Intraregionresidues5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIntraregionresidues5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("GENDATA"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Gendata6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsGendata6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("APC_RECOVERY"), 1_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::ApcRecovery1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsApcRecovery1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRPARTSURPLUS"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Irpartsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIrpartsurplus6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("VICBOUNDARYENERGY"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Vicboundaryenergy5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsVicboundaryenergy5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("MARKETFEES"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Marketfees5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsMarketfees5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("SET_FCAS_REGULATION_TRK"), 1_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::SetFcasRegulationTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsSetFcasRegulationTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("IRAUCSURPLUS"), 6_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::Iraucsurplus6> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsIraucsurplus6 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("FCAS_PAYMENT"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::FcasPayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsFcasPayment5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("SETTLEMENTS", Some("MR_PAYMENT"), 5_i32) => {
                    #[cfg(feature = "settlement_data")]
                    {
                        let d: Vec<settlement_data::MrPayment5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertSettlementsMrPayment5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "settlement_data"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GD_INSTRUCT", Some("INSTRUCTIONSUBTYPE"), 1_i32) => {
                    #[cfg(feature = "gd_instruct")]
                    {
                        let d: Vec<gd_instruct::Instructionsubtype1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGdInstructInstructionsubtype1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "gd_instruct"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GD_INSTRUCT", Some("INSTRUCTIONTYPE"), 1_i32) => {
                    #[cfg(feature = "gd_instruct")]
                    {
                        let d: Vec<gd_instruct::Instructiontype1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGdInstructInstructiontype1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "gd_instruct"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("GD_INSTRUCT", Some("GDINSTRUCT"), 1_i32) => {
                    #[cfg(feature = "gd_instruct")]
                    {
                        let d: Vec<gd_instruct::Gdinstruct1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertGdInstructGdinstruct1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "gd_instruct"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("RATING"), 1_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Rating1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkRating1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("STATICRATING"), 1_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Staticrating1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkStaticrating1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("OUTAGECONSTRAINTSET"), 1_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Outageconstraintset1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkOutageconstraintset1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("OUTAGEDETAIL"), 3_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Outagedetail3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkOutagedetail3 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("EQUIPMENTDETAIL"), 1_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Equipmentdetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkEquipmentdetail1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("SUBSTATIONDETAIL"), 1_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Substationdetail1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkSubstationdetail1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("OUTAGESTATUSCODE"), 1_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Outagestatuscode1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkOutagestatuscode1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("NETWORK", Some("REALTIMERATING"), 1_i32) => {
                    #[cfg(feature = "network")]
                    {
                        let d: Vec<network::Realtimerating1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertNetworkRealtimerating1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "network"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("INTERCONNECTORRES"), 3_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Interconnectorres3> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchInterconnectorres3 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("INTERCONNECTION"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Interconnection1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchInterconnection1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("UNIT_CONFORMANCE"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::UnitConformance1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchUnitConformance1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("LOCAL_PRICE"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::LocalPrice1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchLocalPrice1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("MNSPBIDTRK"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Mnspbidtrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchMnspbidtrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("UNIT_SCADA"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::UnitScada1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchUnitScada1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRICELOAD", Some("CONSTRAINTRELAXATION"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Constraintrelaxation1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPriceloadConstraintrelaxation1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("CONSTRAINT"), 5_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Constraint5> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchConstraint5 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("CASE_SOLUTION"), 2_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::CaseSolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchCaseSolution2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRICELOAD", Some("CONSTRAINT_FCAS_OCD"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::ConstraintFcasOcd1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPriceloadConstraintFcasOcd1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("OFFERTRK"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Offertrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchOffertrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("NEGATIVE_RESIDUE"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::NegativeResidue1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchNegativeResidue1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("INTERMITTENT_FORECAST_TRK"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::IntermittentForecastTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchIntermittentForecastTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("FCAS_REQ"), 2_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::FcasReq2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchFcasReq2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("BLOCKED_CONSTRAINTS"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::BlockedConstraints1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchBlockedConstraints1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("PRICE"), 4_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Price4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchPrice4 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("MR_SCHEDULE_TRK"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::MrScheduleTrk1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchMrScheduleTrk1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("UNIT_SOLUTION"), 2_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::UnitSolution2> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchUnitSolution2 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("DISPATCH", Some("REGIONSUM"), 4_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::Regionsum4> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertDispatchRegionsum4 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
                    {
                        log::warn!("Unhandled file key {:?}", file_key);
                        continue;
                    }
                }
                ("PRICELOAD", Some("PRICE_REVISION"), 1_i32) => {
                    #[cfg(feature = "dispatch")]
                    {
                        let d: Vec<dispatch::PriceRevision1> = self.get_table()?;
                        self.batched_insert(
                            client,
                            file_key,
                            &d,
                            "exec mmsdm_proc.InsertPriceloadPriceRevision1 @P1, @P2",
                        )
                        .await?;
                    }
                    #[cfg(not(feature = "dispatch"))]
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
