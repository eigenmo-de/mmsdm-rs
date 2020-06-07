/// Data Set Name: Ap
/// File Name: Regionapc
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapc1 {
    /// Region Identifier
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ApRegionapc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "AP".into(),
                        table_name: "REGIONAPC".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Force Majeure
/// File Name: Market Suspend Region Sum
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegionSum1 {
    /// Unique identifier for this suspension event
    suspension_id: String,
    /// Region(s) covered by the Suspension event
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    initial_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    end_region_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    end_suspension_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendRegionSum1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORCE_MAJEURE".into(),
                        table_name: "MARKET_SUSPEND_REGION_SUM".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Ap
/// File Name: Apeventregion
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApeventregion1 {
    /// Unique identifier for this administered pricing event
    apeventid: rust_decimal::Decimal,
    /// Date-Time of the first Dispatch Interval to which the administered event applies
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// flag indicating if the apevent covers an energy AP
    energyapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise6sec AP
    raise6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise60sec AP
    raise60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise5min AP
    raise5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raisereg AP
    raiseregapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower6sec AP
    lower6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower60sec AP<br>flag indicating if the apevent covers a lower5min AP<br>flag indicating if the apevent covers a lowerreg AP<br>flag indicating if the apevent covers a lower60sec AP
    lower60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower5min AP
    lower5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lowerreg AP
    lowerregapflag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ApApeventregion1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "AP".into(),
                        table_name: "APEVENTREGION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Force Majeure
/// File Name: Market Suspend Schedule Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendScheduleTrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    source_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    source_end_date: Option<chrono::NaiveDateTime>,
    /// Reason why this regime was applied
    comments: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendScheduleTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORCE_MAJEURE".into(),
                        table_name: "MARKET_SUSPEND_SCHEDULE_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Ap
/// File Name: Regionapcintervals
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapcintervals1 {
    /// Region Identifier
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    versionno: rust_decimal::Decimal,
    /// Period number where 1 is the 00:30 EST
    periodid: rust_decimal::Decimal,
    /// Administered price cap in $
    apcvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// not used
    apctype: Option<rust_decimal::Decimal>,
    /// FCAS Administered price cap in $
    fcasapcvalue: Option<rust_decimal::Decimal>,
    /// Administered price floor in $
    apfvalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ApRegionapcintervals1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "AP".into(),
                        table_name: "REGIONAPCINTERVALS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Force Majeure
/// File Name: Market Suspend Regime Sum
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegimeSum1 {
    /// Unique identifier for this suspension event
    suspension_id: String,
    /// Region(s) covered by this evolution of the event
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    start_interval: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    end_interval: Option<chrono::NaiveDateTime>,
    /// Pricing Regime applied
    pricing_regime: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendRegimeSum1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORCE_MAJEURE".into(),
                        table_name: "MARKET_SUSPEND_REGIME_SUM".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Force Majeure
/// File Name: Market Suspend Schedule
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendSchedule1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Distinguishes which record set to apply - at time of writing this was Business or Non-business day but may change in the future depending on outcome of consultation
    day_type: String,
    /// Region affected. 
    regionid: String,
    /// 48 intervals for a day, midnight base (equates to 00:30 - 00:00)
    periodid: rust_decimal::Decimal,
    /// Energy Price applied for this period for this Day Type
    energy_rrp: Option<rust_decimal::Decimal>,
    /// Raise 6Sec contingency Price applied for this period for this Day Type
    r6_rrp: Option<rust_decimal::Decimal>,
    /// Raise 60Sec contingency Price applied for this period for this Day Type
    r60_rrp: Option<rust_decimal::Decimal>,
    /// Raise 5Min contingency Price applied for this period for this Day Type
    r5_rrp: Option<rust_decimal::Decimal>,
    /// Raise Regulation contingency Price applied for this period for this Day Type
    rreg_rrp: Option<rust_decimal::Decimal>,
    /// Lower 6Sec contingency Price applied for this period for this Day Type
    l6_rrp: Option<rust_decimal::Decimal>,
    /// Lower 60Sec contingency Price applied for this period for this Day Type
    l60_rrp: Option<rust_decimal::Decimal>,
    /// Lower 5Min contingency Price applied for this period for this Day Type
    l5_rrp: Option<rust_decimal::Decimal>,
    /// Lower Regulation Price applied for this period for this Day Type
    lreg_rrp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendSchedule1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORCE_MAJEURE".into(),
                        table_name: "MARKET_SUSPEND_SCHEDULE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Ap
/// File Name: Apevent
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApevent1 {
    /// Unique identifier for this administered pricing event
    apeventid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivefrominterval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivetointerval: Option<chrono::NaiveDateTime>,
    /// Description of the driver for the Event
    reason: Option<String>,
    /// Authorising staff for start of AP event
    startauthorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    startauthoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising staff for end of AP event
    endauthorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    endauthoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ApApevent1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "AP".into(),
                        table_name: "APEVENT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Force Majeure
/// File Name: Irfmamount
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmamount1 {
    /// Unique Industrial Relations Force Majeure event
    irfmid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of record of event
    versionno: rust_decimal::Decimal,
    /// Settlement period
    periodid: rust_decimal::Decimal,
    /// Total settlement amount in $
    amount: Option<rust_decimal::Decimal>,
    /// Person authorising amount
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureIrfmamount1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORCE_MAJEURE".into(),
                        table_name: "IRFMAMOUNT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Force Majeure
/// File Name: Overriderrp
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureOverriderrp1 {
    /// Region Identifier
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    startdate: chrono::NaiveDateTime,
    /// Starting period of override
    startperiod: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    /// Terminate period of override
    endperiod: Option<rust_decimal::Decimal>,
    /// Dispatch Price
    rrp: Option<rust_decimal::Decimal>,
    /// Description of reason for override
    description: Option<String>,
    /// Authorise Start of Override
    authorisestart: Option<String>,
    /// Authorise End of Override
    authoriseend: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureOverriderrp1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORCE_MAJEURE".into(),
                        table_name: "OVERRIDERRP".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Force Majeure
/// File Name: Irfmevents
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmevents1 {
    /// &nbsp; 
    irfmid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    /// &nbsp; 
    startperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    /// &nbsp; 
    endperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureIrfmevents1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "FORCE_MAJEURE".into(),
                        table_name: "IRFMEVENTS".into(),
                        version: 1,
                    }
                    
    }
}
