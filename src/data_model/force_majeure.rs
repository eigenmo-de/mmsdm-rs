/// # Summary
///
/// ## MARKET_SUSPEND_SCHEDULE
///  _Trading prices that will apply in the event of a market suspension event updated weekly._
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Schedule
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND_SCHEDULE is public data, so is available to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY_TYPE
/// * EFFECTIVEDATE
/// * PERIODID
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendSchedule1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Distinguishes which record set to apply - at time of writing this was Business or Non-business day but may change in the future depending on outcome of consultation
    pub day_type: String,
    /// Region affected.
    pub regionid: String,
    /// 48 intervals for a day, midnight base (equates to 00:30 - 00:00)
    pub periodid: rust_decimal::Decimal,
    /// Energy Price applied for this period for this Day Type
    pub energy_rrp: Option<rust_decimal::Decimal>,
    /// Raise 6Sec contingency Price applied for this period for this Day Type
    pub r6_rrp: Option<rust_decimal::Decimal>,
    /// Raise 60Sec contingency Price applied for this period for this Day Type
    pub r60_rrp: Option<rust_decimal::Decimal>,
    /// Raise 5Min contingency Price applied for this period for this Day Type
    pub r5_rrp: Option<rust_decimal::Decimal>,
    /// Raise Regulation contingency Price applied for this period for this Day Type
    pub rreg_rrp: Option<rust_decimal::Decimal>,
    /// Lower 6Sec contingency Price applied for this period for this Day Type
    pub l6_rrp: Option<rust_decimal::Decimal>,
    /// Lower 60Sec contingency Price applied for this period for this Day Type
    pub l60_rrp: Option<rust_decimal::Decimal>,
    /// Lower 5Min contingency Price applied for this period for this Day Type
    pub l5_rrp: Option<rust_decimal::Decimal>,
    /// Lower Regulation Price applied for this period for this Day Type
    pub lreg_rrp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendSchedule1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_SCHEDULE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## IRFMAMOUNT
///  _IRFMAMOUNT sets out settlement amounts associated with Industrial Relations Forced Majeure events._
///
/// * Data Set Name: Force Majeure
/// * File Name: Irfmamount
/// * Data Version: 1
///
/// # Description
///  IRFMAMOUNTis public data. Source IRFMAMOUNT is obsolete; was updated with each settlement run as required.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * IRFMID
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmamount1 {
    /// Unique Industrial Relations Force Majeure event
    pub irfmid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of record of event
    pub versionno: rust_decimal::Decimal,
    /// Settlement period
    pub periodid: rust_decimal::Decimal,
    /// Total settlement amount in $
    pub amount: Option<rust_decimal::Decimal>,
    /// Person authorising amount
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureIrfmamount1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("IRFMAMOUNT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKET_SUSPEND_REGIME_SUM
///  _Tracks the evolution of pricing regimes applied to the suspended region and from which Dispatch Interval_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Regime Sum
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND_REGIME_SUM is public data, so is available to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * START_INTERVAL
/// * SUSPENSION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegimeSum1 {
    /// Unique identifier for this suspension event
    pub suspension_id: String,
    /// Region(s) covered by this evolution of the event
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub start_interval: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub end_interval: Option<chrono::NaiveDateTime>,
    /// Pricing Regime applied
    pub pricing_regime: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendRegimeSum1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_REGIME_SUM".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## REGIONAPC
///  _REGIONAPC defines Administered Price profiles (Energy and FCAS) for a region._
///
/// * Data Set Name: Ap
/// * File Name: Regionapc
/// * Data Version: 1
///
/// # Description
///  REGIONAPC data is public, so is available to all participants. Source REGIONAPC updates when a change is ever made to the Administered Price Cap details. Changes to this table are infrequent.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapc1 {
    /// Region Identifier
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ApRegionapc1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("REGIONAPC".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## REGIONAPCINTERVALS
///  _REGIONAPCINTERVALS contains Administered Price profiles (Energy and FCAS) applicable to each interval for a region._
///
/// * Data Set Name: Ap
/// * File Name: Regionapcintervals
/// * Data Version: 1
///
/// # Description
///  REGIONAPCINTERVALS data is public, so is available to all participants. Source REGIONAPCINTERVALS is updated whenever an Administered Price Cap occurs.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PERIODID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapcintervals1 {
    /// Region Identifier
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    pub versionno: rust_decimal::Decimal,
    /// Period number where 1 is the 00:30 EST
    pub periodid: rust_decimal::Decimal,
    /// Administered price cap in $
    pub apcvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// not used
    pub apctype: Option<rust_decimal::Decimal>,
    /// FCAS Administered price cap in $
    pub fcasapcvalue: Option<rust_decimal::Decimal>,
    /// Administered price floor in $
    pub apfvalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ApRegionapcintervals1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("REGIONAPCINTERVALS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## IRFMEVENTS
///  _IRFMEVENTS sets out specific Industrial Relations Forced Majeure events._
///
/// * Data Set Name: Force Majeure
/// * File Name: Irfmevents
/// * Data Version: 1
///
/// # Description
///  IRFMEVENTS is public data. Source IRFMEVENTS updates with the occurrence of any such events.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * IRFMID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmevents1 {
    /// &nbsp;
    pub irfmid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub startperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub endperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureIrfmevents1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("IRFMEVENTS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKET_SUSPEND_SCHEDULE_TRK
///  _Parent table for pricing regimes used in suspensions_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Schedule Trk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendScheduleTrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub source_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub source_end_date: Option<chrono::NaiveDateTime>,
    /// Reason why this regime was applied
    pub comments: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendScheduleTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_SCHEDULE_TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKET_SUSPEND_REGION_SUM
///  _Summary of Market Suspension timings_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Region Sum
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND is public data, so is available to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * SUSPENSION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegionSum1 {
    /// Unique identifier for this suspension event
    pub suspension_id: String,
    /// Region(s) covered by the Suspension event
    pub regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub initial_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub end_region_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub end_suspension_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendRegionSum1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_REGION_SUM".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## APEVENT
///  _APEVENT is the driving data defining the existence and timeframes of an administered pricing event._
///
/// * Data Set Name: Ap
/// * File Name: Apevent
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * APEVENTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApevent1 {
    /// Unique identifier for this administered pricing event
    pub apeventid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivefrominterval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivetointerval: Option<chrono::NaiveDateTime>,
    /// Description of the driver for the Event
    pub reason: Option<String>,
    /// Authorising staff for start of AP event
    pub startauthorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startauthoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising staff for end of AP event
    pub endauthorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub endauthoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ApApevent1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("APEVENT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## OVERRIDERRP
///  _OVERRIDERRP shows details of override price periods._
///
/// * Data Set Name: Force Majeure
/// * File Name: Overriderrp
/// * Data Version: 1
///
/// # Description
///  OVERRIDERRP data is public, so is available to all participants. Source OVERRIDERRP updates every five minutes when override prices apply for the period.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * STARTDATE
/// * STARTPERIOD
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureOverriderrp1 {
    /// Region Identifier
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub startdate: chrono::NaiveDateTime,
    /// Starting period of override
    pub startperiod: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Terminate period of override
    pub endperiod: Option<rust_decimal::Decimal>,
    /// Dispatch Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Description of reason for override
    pub description: Option<String>,
    /// Authorise Start of Override
    pub authorisestart: Option<String>,
    /// Authorise End of Override
    pub authoriseend: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureOverriderrp1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("OVERRIDERRP".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## APEVENTREGION
///  _APEVENTREGION is the Region detail for an administered pricing event defined through APEVENT._
///
/// * Data Set Name: Ap
/// * File Name: Apeventregion
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApeventregion1 {
    /// Unique identifier for this administered pricing event
    pub apeventid: rust_decimal::Decimal,
    /// Date-Time of the first Dispatch Interval to which the administered event applies
    pub regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// flag indicating if the apevent covers an energy AP
    pub energyapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise6sec AP
    pub raise6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise60sec AP
    pub raise60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise5min AP
    pub raise5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raisereg AP
    pub raiseregapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower6sec AP
    pub lower6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower60sec AP<br>flag indicating if the apevent covers a lower5min AP<br>flag indicating if the apevent covers a lowerreg AP<br>flag indicating if the apevent covers a lower60sec AP
    pub lower60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower5min AP
    pub lower5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lowerreg AP
    pub lowerregapflag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ApApeventregion1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("APEVENTREGION".into()),
            version: 1,
        }
    }
}
