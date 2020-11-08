/// # Summary
///
/// ## INTERMITTENT_GEN_LIMIT_DAY
///  _Summary record for an Upper MW Limit submission for an intermittent generating unit for a Trading Day_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Gen Limit Day
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentGenLimitDay1 {
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// User entering the unit availability submission
    pub authorisedbyuser: Option<String>,
    /// Participant entering the unit availability submission
    pub authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable for IntermittentGenLimitDay1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_GEN_LIMIT_DAY".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## INTERMITTENT_GEN_FCST_DATA
///  _Stores the forecast generation (MW) for each interval within a given forecast of an intermittent generator._
///
/// * Data Set Name: Forecast
/// * File Name: Intermittent Gen Data
/// * Data Version: 1
///
/// # Description
///  Source INTERMITTENT_GEN_FCST_DATA updates every 30 minutes when AEMO issues a new 30-minute forecast of wind generation out to 40 hours ahead. Volume ~1,500,000 rows per generator per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentGenData1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// &nbsp;
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// &nbsp;
    pub powermean: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub powerpoe50: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub powerpoelow: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub powerpoehigh: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IntermittentGenData1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORECAST".into(),
            table_name: Some("INTERMITTENT_GEN_DATA".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## ROOFTOP_PV_ACTUAL
///  _Estimate of regional Rooftop Solar actual generation for each half-hour interval in a day_
///
/// * Data Set Name: Rooftop
/// * File Name: Actual
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * TYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Actual2 {
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    #[serde(rename = "type")]
    pub type_: String,
    /// Region identifier
    pub regionid: String,
    /// Estimated generation in MW at the interval end
    pub power: Option<rust_decimal::Decimal>,
    /// Quality indicator. Represents the quality of the estimate.
    pub qi: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for Actual2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ROOFTOP".into(),
            table_name: Some("ACTUAL".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## MTPASA_INTERMITTENT_AVAIL
///  _A submission of expected plant availability for intermittent generators for use in MTPASA intermittent generation forecasts_
///
/// * Data Set Name: Demand
/// * File Name: Mtpasa Intermittent Avail
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaIntermittentAvail1 {
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE. Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable.
    pub elements_unavailable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for MtpasaIntermittentAvail1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("MTPASA_INTERMITTENT_AVAIL".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_INTERMITTENT_LIMIT
///  _A submission of expected maximum availability for intermittent generators for use in MTPASA intermittent generation<br>forecasts_
///
/// * Data Set Name: Demand
/// * File Name: Mtpasa Intermittent Limit
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaIntermittentLimit1 {
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Maximum imposed MW limit. Value between 0 and the registered DUID Maximum Capacity.Value = -1 means no limit applies.
    pub uppermwlimit: Option<i64>,
    /// User entering the unit availability submission
    pub authorisedbyuser: Option<String>,
    /// Participant entering the unit availability submission
    pub authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable for MtpasaIntermittentLimit1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("MTPASA_INTERMITTENT_LIMIT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## INTERMITTENT_GEN_LIMIT
///  _A submission of Upper MW Limit for an intermittent generating unit, by Trading Day and Trading Interval_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Gen Limit
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentGenLimit1 {
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Trading interval number (1...48) within this TRADINGDATE for which UPPERMWLIMIT applies
    pub periodid: rust_decimal::Decimal,
    /// Maximum imposed MW limit (down regulation in ANEMOS). Value between 0 and the registered DUID Maximum Capacity. Value = -1 means no limit applies
    pub uppermwlimit: Option<i64>,
}
impl crate::GetTable for IntermittentGenLimit1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_GEN_LIMIT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## DEMANDOPERATIONALACTUAL
///  _Shows Actual Operational Demand for a particular date time interval._
///
/// * Data Set Name: Operational Demand
/// * File Name: Actual
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Actual1 {
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    /// Operational demand value
    pub operational_demand: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for Actual1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OPERATIONAL_DEMAND".into(),
            table_name: Some("ACTUAL".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## INTERMITTENT_GEN_FCST
///  _Identifying record for a given forecast of an intermittent generation. This table is the version table for the INTERMITTENT_GEN_FCST_DATA table which stores the individual forecast values_
///
/// * Data Set Name: Forecast
/// * File Name: Intermittent Gen
/// * Data Version: 1
///
/// # Description
///  Source INTERMITTENT_GEN_FCST updates every 30 minutes when AEMO issues a new 30-minute forecast of wind generation out to 40 hours ahead. Volume ~18,000 rows per generator per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentGen1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Identifier of the intermittent generator.
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub start_interval_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub end_interval_datetime: chrono::NaiveDateTime,
    /// Versioning information for resolution back to AEMO's wind generation forecasting system.
    pub versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IntermittentGen1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORECAST".into(),
            table_name: Some("INTERMITTENT_GEN".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## INTERMITTENT_CLUSTER_AVAIL
///  _A submission of Elements Unavailable for an intermittent generating unit cluster, by Trading Day and Trading Interval_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Cluster Avail
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentClusterAvail1 {
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: String,
    /// Trading interval number (1…48) within this TRADINGDATE for which ELEMENTS_UNAVAILABLE applies
    pub periodid: rust_decimal::Decimal,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE and PERIODID (scheduled maintenance in ANEMOS). Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable
    pub elements_unavailable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IntermittentClusterAvail1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_CLUSTER_AVAIL".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## INTERMITTENT_CLUSTER_AVAIL_DAY
///  _Summary record for an Elements Unavailable submission for an intermittent generating unit cluster for a Trading Day_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Cluster Avail Day
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentClusterAvailDay1 {
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: String,
}
impl crate::GetTable for IntermittentClusterAvailDay1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_CLUSTER_AVAIL_DAY".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PERDEMAND
///  _PERDEMAND sets out the regional demands and MR schedule data for each half-hour period. PERDEMAND is a child table to RESDEMANDTRK._
///
/// * Data Set Name: Demand
/// * File Name: Period
/// * Data Version: 1
///
/// # Description
///  The RESDEMANDTRK and PERDEMAND tables have a parent/child relationship, and define forecast regional demands since market start. RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date. PERDEMAND defines the numerical forecast values for each trading interval of a the trading day for that region. A complete trading day forecast for one region consists of one RESDEMANDTRK record and 48 PERDEMAND records. Source PERDEMAND updates whenever AEMO issues a new or revised forecast. ST PASA forecasts update seven days at a time. Predispatch updates one date. Volume 1296000 rows per year Note In the context of a mandatory restrictions event the forecast schedule (MW) of restrictions are reported through the RESDEMANDTRK and PERDEMAND tables using the new field PerDemand.MR_Schedule. The relationship between fields and mandatory restriction terms for the 50% probability of exceedence forecast are: ·	 UnRestricted Profile  = ResDemand + MR_Schedule ·	 Restricted Profile  = ResDemand
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * OFFERDATE
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Period1 {
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Differentiates this region from all other regions
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Half hourly trading intervals from 04:30.
    pub periodid: rust_decimal::Decimal,
    /// The version of the RESDEMAND file for this date
    pub versionno: rust_decimal::Decimal,
    /// Base Demand forecast for period
    pub resdemand: Option<rust_decimal::Decimal>,
    /// Demand at 90% probability of exceedance
    pub demand90probability: Option<rust_decimal::Decimal>,
    /// Demand level for a 10% probability of exceedance
    pub demand10probability: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// MR_Schedule = Unrestricted Demand - POE
    pub mr_schedule: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for Period1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("PERIOD".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## INTERMITTENT_DS_RUN
///  _Unconstrained Intermittent Generation Forecasts (UIGF) for Dispatch._
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Ds Run
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * FORECAST_PRIORITY
/// * OFFERDATETIME
/// * ORIGIN
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentDsRun1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    pub origin: String,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values.
    pub forecast_priority: rust_decimal::Decimal,
    /// Authorising officer of this forecast (applicable for participant forecasts only). This column is not made available to the public.
    pub authorisedby: Option<String>,
    /// Comments relating to the forecast. This column is not made available to the public.
    pub comments: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Metadata relating to the forecast. This column is not made available to the public.
    pub model: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub participant_timestamp: Option<chrono::NaiveDateTime>,
    /// Was this forecast suppressed by AEMO? Suppressed = 1,Not suppressed =0<br>
    pub suppressed_aemo: Option<rust_decimal::Decimal>,
    /// Was this forecast suppressed by the participant? Suppressed submissions may not be used,  Suppressed = 1, Not suppressed =0<br>
    pub suppressed_participant: Option<rust_decimal::Decimal>,
    /// Uniquely identifies this interaction
    pub transaction_id: Option<String>,
}
impl crate::GetTable for IntermittentDsRun1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_DS_RUN".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## ROOFTOP_PV_FORECAST
///  _Regional forecasts of Rooftop Solar generation across the half-hour intervals over 8 days_
///
/// * Data Set Name: Rooftop
/// * File Name: Forecast
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Forecast1 {
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// The average forecast value in MW at the interval end
    pub powermean: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance forecast value in MW at the interval end
    pub powerpoe50: Option<rust_decimal::Decimal>,
    /// 10% probability of exceedance forecast value in MW at the interval end
    pub powerpoelow: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance forecast value in MW at the interval end
    pub powerpoehigh: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for Forecast1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ROOFTOP".into(),
            table_name: Some("FORECAST".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## DEMANDOPERATIONALFORECAST
///  _Shows Forecast Operational Demand for a particular date time interval._
///
/// * Data Set Name: Operational Demand
/// * File Name: Forecast
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Forecast1 {
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub load_date: Option<chrono::NaiveDateTime>,
    /// 10% probability of exceedance operational demand forecast value
    pub operational_demand_poe10: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance operational demand forecast value
    pub operational_demand_poe50: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance operational demand forecast value
    pub operational_demand_poe90: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for Forecast1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OPERATIONAL_DEMAND".into(),
            table_name: Some("FORECAST".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESDEMANDTRK
///  _RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date.<br>RESDEMANDTRK and PERDEMAND have a parent/child relationship, and are for defined forecast regional demands since market start. RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date. PERDEMAND defines the numerical forecast values for each trading interval of a the trading day for that region. A complete trading day forecast for one region consists of one RESDEMANDTRK record and 48 PERDEMAND records.<br>_
///
/// * Data Set Name: Demand
/// * File Name: Trk
/// * Data Version: 1
///
/// # Description
///  RESDEMANDTRK data is public, so is available to all participants. Source RESDEMANDTRK updates are ad hoc. Volume 27000 rows per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * OFFERDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Trk1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Version of this forecast with respect to the Effectivedate and Offerdate
    pub versionno: rust_decimal::Decimal,
    /// Tracking purposes only
    pub filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Identifier of authorising user
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for Trk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## INTERMITTENT_DS_PRED
///  _Unconstrained Intermittent Generation Forecasts (UIGF) for Dispatch_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Ds Pred
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * FORECAST_PRIORITY
/// * INTERVAL_DATETIME
/// * OFFERDATETIME
/// * ORIGIN
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IntermittentDsPred1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    pub origin: String,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values<br>
    pub forecast_priority: rust_decimal::Decimal,
    /// Forecast MW value for this interval_DateTime
    pub forecast_mean: Option<rust_decimal::Decimal>,
    /// Forecast 10% POE MW value for this interval_DateTime
    pub forecast_poe10: Option<rust_decimal::Decimal>,
    /// Forecast 50% POE MW value for this interval_DateTime. Used in Dispatch.
    pub forecast_poe50: Option<rust_decimal::Decimal>,
    /// Forecast 90% POE MW value for this interval_DateTime
    pub forecast_poe90: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IntermittentDsPred1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_DS_PRED".into()),
            version: 1,
        }
    }
}
