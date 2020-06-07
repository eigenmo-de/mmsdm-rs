/// Data Set Name: Demand
/// File Name: Intermittent Cluster Avail Day
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentClusterAvailDay1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit 
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    clusterid: String,
}
impl crate::GetTable<DemandIntermittentClusterAvailDay1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "INTERMITTENT_CLUSTER_AVAIL_DAY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Mtpasa Intermittent Avail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandMtpasaIntermittentAvail1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    clusterid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE. Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable.
    elements_unavailable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandMtpasaIntermittentAvail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "MTPASA_INTERMITTENT_AVAIL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Intermittent Ds Pred
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentDsPred1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    origin: String,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values<br>
    forecast_priority: rust_decimal::Decimal,
    /// Forecast MW value for this interval_DateTime
    forecast_mean: Option<rust_decimal::Decimal>,
    /// Forecast 10% POE MW value for this interval_DateTime
    forecast_poe10: Option<rust_decimal::Decimal>,
    /// Forecast 50% POE MW value for this interval_DateTime. Used in Dispatch.
    forecast_poe50: Option<rust_decimal::Decimal>,
    /// Forecast 90% POE MW value for this interval_DateTime
    forecast_poe90: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandIntermittentDsPred1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "INTERMITTENT_DS_PRED".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Forecast
/// File Name: Intermittent Gen Data
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForecastIntermittentGenData1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// &nbsp; 
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// &nbsp; 
    powermean: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    powerpoe50: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    powerpoelow: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    powerpoehigh: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForecastIntermittentGenData1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORECAST".into(),
                        table_name: "INTERMITTENT_GEN_DATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Operational Demand
/// File Name: Actual
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OperationalDemandActual1 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    regionid: String,
    /// Operational demand value
    operational_demand: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OperationalDemandActual1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OPERATIONAL_DEMAND".into(),
                        table_name: "ACTUAL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Period
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandPeriod1 {
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Differentiates this region from all other regions
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Half hourly trading intervals from 04:30.
    periodid: rust_decimal::Decimal,
    /// The version of the RESDEMAND file for this date
    versionno: rust_decimal::Decimal,
    /// Base Demand forecast for period
    resdemand: Option<rust_decimal::Decimal>,
    /// Demand at 90% probability of exceedance
    demand90probability: Option<rust_decimal::Decimal>,
    /// Demand level for a 10% probability of exceedance
    demand10probability: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// MR_Schedule = Unrestricted Demand - POE
    mr_schedule: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandPeriod1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "PERIOD".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Rooftop
/// File Name: Forecast
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct RooftopForecast1 {
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// Region identifier
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// The average forecast value in MW at the interval end
    powermean: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance forecast value in MW at the interval end
    powerpoe50: Option<rust_decimal::Decimal>,
    /// 10% probability of exceedance forecast value in MW at the interval end
    powerpoelow: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance forecast value in MW at the interval end
    powerpoehigh: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<RooftopForecast1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "ROOFTOP".into(),
                        table_name: "FORECAST".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Forecast
/// File Name: Intermittent Gen
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForecastIntermittentGen1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// Identifier of the intermittent generator.
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    start_interval_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    end_interval_datetime: chrono::NaiveDateTime,
    /// Versioning information for resolution back to AEMO's wind generation forecasting system.
    versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForecastIntermittentGen1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORECAST".into(),
                        table_name: "INTERMITTENT_GEN".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandTrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Unique RegionID
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Version of this forecast with respect to the Effectivedate and Offerdate
    versionno: rust_decimal::Decimal,
    /// Tracking purposes only
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Identifier of authorising user
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DemandTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Intermittent Gen Limit
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentGenLimit1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// Trading interval number (1...48) within this TRADINGDATE for which UPPERMWLIMIT applies
    periodid: rust_decimal::Decimal,
    /// Maximum imposed MW limit (down regulation in ANEMOS). Value between 0 and the registered DUID Maximum Capacity. Value = -1 means no limit applies
    uppermwlimit: Option<i64>,
}
impl crate::GetTable<DemandIntermittentGenLimit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "INTERMITTENT_GEN_LIMIT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Operational Demand
/// File Name: Forecast
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OperationalDemandForecast1 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    load_date: Option<chrono::NaiveDateTime>,
    /// 10% probability of exceedance operational demand forecast value
    operational_demand_poe10: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance operational demand forecast value
    operational_demand_poe50: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance operational demand forecast value
    operational_demand_poe90: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OperationalDemandForecast1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OPERATIONAL_DEMAND".into(),
                        table_name: "FORECAST".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Rooftop
/// File Name: Actual
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct RooftopActual2 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    #[serde(rename = "type")]
    type_: String,
    /// Region identifier
    regionid: String,
    /// Estimated generation in MW at the interval end
    power: Option<rust_decimal::Decimal>,
    /// Quality indicator. Represents the quality of the estimate.
    qi: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<RooftopActual2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "ROOFTOP".into(),
                        table_name: "ACTUAL".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Mtpasa Intermittent Limit
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandMtpasaIntermittentLimit1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Maximum imposed MW limit. Value between 0 and the registered DUID Maximum Capacity.Value = -1 means no limit applies.
    uppermwlimit: Option<i64>,
    /// User entering the unit availability submission
    authorisedbyuser: Option<String>,
    /// Participant entering the unit availability submission
    authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable<DemandMtpasaIntermittentLimit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "MTPASA_INTERMITTENT_LIMIT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Intermittent Ds Run
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentDsRun1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    origin: String,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values.
    forecast_priority: rust_decimal::Decimal,
    /// Authorising officer of this forecast (applicable for participant forecasts only). This column is not made available to the public.
    authorisedby: Option<String>,
    /// Comments relating to the forecast. This column is not made available to the public.
    comments: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Metadata relating to the forecast. This column is not made available to the public.
    model: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    participant_timestamp: Option<chrono::NaiveDateTime>,
    /// Was this forecast suppressed by AEMO? Suppressed = 1,Not suppressed =0<br>
    suppressed_aemo: Option<rust_decimal::Decimal>,
    /// Was this forecast suppressed by the participant? Suppressed submissions may not be used,  Suppressed = 1, Not suppressed =0<br>
    suppressed_participant: Option<rust_decimal::Decimal>,
    /// Uniquely identifies this interaction
    transaction_id: Option<String>,
}
impl crate::GetTable<DemandIntermittentDsRun1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "INTERMITTENT_DS_RUN".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Intermittent Gen Limit Day
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentGenLimitDay1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit 
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// Unique participant identifier
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// User entering the unit availability submission
    authorisedbyuser: Option<String>,
    /// Participant entering the unit availability submission
    authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable<DemandIntermittentGenLimitDay1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "INTERMITTENT_GEN_LIMIT_DAY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Demand
/// File Name: Intermittent Cluster Avail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentClusterAvail1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit 
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID 
    clusterid: String,
    /// Trading interval number (1…48) within this TRADINGDATE for which ELEMENTS_UNAVAILABLE applies
    periodid: rust_decimal::Decimal,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE and PERIODID (scheduled maintenance in ANEMOS). Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable
    elements_unavailable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandIntermittentClusterAvail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DEMAND".into(),
                        table_name: "INTERMITTENT_CLUSTER_AVAIL".into(),
                        version: 1,
                    }
                    
    }
}
