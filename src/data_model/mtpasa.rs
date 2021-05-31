/// # Summary
///
/// ## MTPASA_CASERESULT
///  _MTPASA solution header table_
///
/// * Data Set Name: Mtpasa
/// * File Name: Caseresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_CASERESULT is public data. Holds one Record for entire solution
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
/// * RUN_NO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaCaseresult1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Version of PLEXOS used
    pub plexos_version: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaCaseresult1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("CASERESULT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_CONSTRAINTRESULT
///  _Constraint results for Binding or Violating Constraints_
///
/// * Data Set Name: Mtpasa
/// * File Name: Constraintresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_CONSTRAINTRESULT is public data. &nbsp;
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DAY
/// * DEMAND_POE_TYPE
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaConstraintresult1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    #[serde(with = "crate::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the constraint. Only binding or violating constraints are reported
    pub constraintid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// The version of the constraint used
    pub versionno: Option<rust_decimal::Decimal>,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// Proportion of a constraint binding, across iterations and reference years
    pub probabilityofbinding: Option<rust_decimal::Decimal>,
    /// Proportion of a constraint violating, across iterations and reference years
    pub probabilityofviolation: Option<rust_decimal::Decimal>,
    /// The 90th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation90: Option<rust_decimal::Decimal>,
    /// The 50th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation50: Option<rust_decimal::Decimal>,
    /// The 10th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaConstraintresult1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("CONSTRAINTRESULT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_CONSTRAINTSUMMARY
///  _Constraint Summary results over aggregation periods_
///
/// * Data Set Name: Mtpasa
/// * File Name: Constraintsummary
/// * Data Version: 1
///
/// # Description
///  MTPASA_CONSTRAINTSUMMARY is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * CONSTRAINTID
/// * DAY
/// * DEMAND_POE_TYPE
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaConstraintsummary1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    #[serde(with = "crate::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the constraint. Only binding or violating constraints are reported
    pub constraintid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// The version of the constraintID
    pub versionno: Option<rust_decimal::Decimal>,
    /// Period data is aggregated over. Values are PEAK, SHOULDER, OFFPEAK. PEAK = 14:00-19:59, SHOULDER = 07:00-13:59 and 20:00-21:59, OFFPEAK = 22:00-06:59
    pub aggregation_period: String,
    /// Constraint hours binding or violating for period, averaged across iterations and reference years
    pub constrainthoursbinding: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaConstraintsummary1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("CONSTRAINTSUMMARY".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_DUIDAVAILABILITY
///  _Offered PASA Availability of the scheduled generator DUID for each day over the Medium Term PASA period. The data in this table is input data to the MT PASA process it is not part of the MTPASA solution. The availability does not reflect any energy limitations in the MT PASA offers_
///
/// * Data Set Name: Mtpasa
/// * File Name: Duidavailability
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * DAY
/// * DUID
/// * PUBLISH_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaDuidavailability1 {
    #[serde(with = "crate::mms_datetime")]
    pub publish_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// NEM Region.
    pub regionid: String,
    /// NEM DUID.
    pub duid: String,
    /// Offered PASA Availability of Scheduled generator DUID for the day.
    pub pasaavailability: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaDuidavailability1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("DUIDAVAILABILITY".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_INTERCONNECTORRESULT
///  _Interconnector results for interval of max demand per day_
///
/// * Data Set Name: Mtpasa
/// * File Name: Interconnectorresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_INTERCONNECTORRESULT is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DEMAND_POE_TYPE
/// * INTERCONNECTORID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaInterconnectorresult1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    #[serde(with = "crate::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the interconnector
    pub interconnectorid: String,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// The 90th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow10: Option<rust_decimal::Decimal>,
    /// Proportion of iterations and reference years with interconnector constrained when exporting
    pub probabilityofbindingexport: Option<rust_decimal::Decimal>,
    /// Proportion of iterations and reference years with interconnector constrained when importing
    pub probabilityofbindingimport: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit, averaged across iterations and reference years
    pub calculatedexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit, averaged across iterations and reference years
    pub calculatedimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaInterconnectorresult1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("INTERCONNECTORRESULT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_LOLPRESULT
///  _Results for Loss of Load Probability (LOLP) run per day_
///
/// * Data Set Name: Mtpasa
/// * File Name: Lolpresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_LOLPRESULT is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaLolpresult1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always LOLP
    pub runtype: String,
    #[serde(with = "crate::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// The half hourly interval period with the highest LOLP, or highest region demand if LOLP = 0 for all intervals (1..48)
    pub worst_interval_periodid: Option<rust_decimal::Decimal>,
    /// The Abstract Operational Demand for the worst interval in this region (MW)
    pub worst_interval_demand: Option<rust_decimal::Decimal>,
    /// The half hourly aggregate intermittent generation for the worst interval in this region (MW)
    pub worst_interval_intgen: Option<rust_decimal::Decimal>,
    /// The half hourly aggregate demand side participation for the worst interval period in this region (MW)
    pub worst_interval_dsp: Option<rust_decimal::Decimal>,
    /// Loss of Load Probability for the worst interval in this region
    pub lossofloadprobability: Option<rust_decimal::Decimal>,
    /// Loss of Load Magnitude for the worst interval in this region. Values are LOW, MEDIUM, HIGH
    pub lossofloadmagnitude: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaLolpresult1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("LOLPRESULT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_REGIONAVAILABILITY
///  _Stores the Region-aggregate offered PASA Availability of scheduled generators for each day over the Medium Term PASA period. The data in this table is an aggregate of input data to the MT PASA process it is not part of the MTPASA solution. The aggregate availability does not reflect any energy limitations in the MT PASA offers._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionavailability
/// * Data Version: 3
///
/// # Description
///  MTPASA_REGIONAVAILABILITY is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * PUBLISH_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionavailability3 {
    #[serde(with = "crate::mms_datetime")]
    pub publish_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// NEM Region.
    pub regionid: String,
    /// Aggregate of the offered PASA Availability for all Scheduled generators in this region.
    pub pasaavailability_scheduled: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
    /// Region energy unconstrained MW capacity
    pub energyunconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Region energy constrained MW capacity
    pub energyconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW)
    pub nonscheduledgeneration: Option<rust_decimal::Decimal>,
    /// 10% probability demand (ex non-scheduled demand)
    pub demand10: Option<rust_decimal::Decimal>,
    /// 50% probability demand (ex non-scheduled demand)
    pub demand50: Option<rust_decimal::Decimal>,
    /// Total weekly operational as generated consumption (POE 10)
    pub energyreqdemand10: Option<rust_decimal::Decimal>,
    /// Total weekly operational as generated consumption (POE 50)
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Minimum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P10 traces (MW).
    pub demand10min: Option<rust_decimal::Decimal>,
    /// Maximum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P10 traces (MW).
    pub demand10max: Option<rust_decimal::Decimal>,
    /// Minimum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P50 traces (MW).
    pub demand50min: Option<rust_decimal::Decimal>,
    /// Maximum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P50 traces (MW).
    pub demand50max: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for MtpasaRegionavailability3 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONAVAILABILITY".into()),
            version: 3,
        }
    }
}
/// # Summary
///
/// ## MTPASA_REGIONAVAIL_TRK
///  _The tracking table to assist in versioning of the region-aggregate offered PASA Availability data published to the MTPASA_REGIONAVAILABILITY table._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionavailtrk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PUBLISH_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionavailtrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub publish_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaRegionavailtrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONAVAILTRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_REGIONITERATION
///  _Region results for Unserved Energy (USE)_
///
/// * Data Set Name: Mtpasa
/// * File Name: Regioniteration
/// * Data Version: 1
///
/// # Description
///  MTPASA_REGIONITERATION is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * DEMAND_POE_TYPE
/// * PERIOD_ENDING
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
/// * USE_ITERATION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegioniteration1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10 or POE50
    pub demand_poe_type: String,
    /// Period data is aggregated over. Values are YEAR
    pub aggregation_period: String,
    #[serde(with = "crate::mms_datetime")]
    pub period_ending: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// Iteration ID, only produced for iterations showing unserved energy&gt;0
    pub use_iteration_id: i64,
    /// Number of half hours showing unserved energy over year, for iteration
    pub use_iteration_event_number: Option<rust_decimal::Decimal>,
    /// Average unserved energy event size for iteration over year (MW)
    pub use_iteration_event_average: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaRegioniteration1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONITERATION".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_REGIONRESULT
///  _Region results for interval of max demand per day._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionresult
/// * Data Version: 2
///
/// # Description
///  MTPASA_REGIONRESULT is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DEMAND_POE_TYPE
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionresult2 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    #[serde(with = "crate::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// Demand value from selected half hourly interval (MW)
    pub demand: Option<rust_decimal::Decimal>,
    /// The total installed capacity of all generation (MW)
    pub aggregateinstalledcapacity: Option<rust_decimal::Decimal>,
    /// Total number of iterations and reference years performed
    pub numberofiterations: Option<rust_decimal::Decimal>,
    /// Number of iterations and reference years with unserved energy&gt;0
    pub use_numberofiterations: Option<rust_decimal::Decimal>,
    /// Maximum unserved energy, across iterations and reference years (MW)
    pub use_max: Option<rust_decimal::Decimal>,
    /// Upper quartile unserved energy, across iterations and reference years (MW)
    pub use_upperquartile: Option<rust_decimal::Decimal>,
    /// Median unserved energy, across iterations and reference years (MW)
    pub use_median: Option<rust_decimal::Decimal>,
    /// Lower quartile unserved energy, across iterations and reference years (MW)
    pub use_lowerquartile: Option<rust_decimal::Decimal>,
    /// Minimum unserved energy, across iterations and reference years (MW)
    pub use_min: Option<rust_decimal::Decimal>,
    /// Average unserved energy, across iterations and reference years (MW)
    pub use_average: Option<rust_decimal::Decimal>,
    /// Average unserved energy event size, across iterations and reference years (MW)
    pub use_event_average: Option<rust_decimal::Decimal>,
    /// The 90th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen10: Option<rust_decimal::Decimal>,
    /// The 90th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen10: Option<rust_decimal::Decimal>,
    /// The 90th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The 90% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen90: Option<rust_decimal::Decimal>,
    /// The 50% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen50: Option<rust_decimal::Decimal>,
    /// The 10% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen10: Option<rust_decimal::Decimal>,
    /// Minimum available capacity, across iterations and reference years (MW).
    pub totalavailablegenmin: Option<rust_decimal::Decimal>,
    /// The 10% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen10: Option<rust_decimal::Decimal>,
    /// The 50% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen50: Option<rust_decimal::Decimal>,
    /// The 90% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen90: Option<rust_decimal::Decimal>,
    /// Maximum available capacity, across iterations and reference years (MW).
    pub totalavailablegenmax: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for MtpasaRegionresult2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONRESULT".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## MTPASA_REGIONSUMMARY
///  _Region Results summary over aggregation periods._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionsummary
/// * Data Version: 1
///
/// # Description
///  MTPASA_REGIONSUMMARY is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * DEMAND_POE_TYPE
/// * PERIOD_ENDING
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionsummary1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value are POE10, POE50
    pub demand_poe_type: String,
    /// Period data is aggregated over. Values are YEAR, MONTH
    pub aggregation_period: String,
    #[serde(with = "crate::mms_datetime")]
    pub period_ending: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// Native demand calculated from Operational As Generated trace supplied by Energy Forecasting
    pub nativedemand: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 10th percentile of iterations and reference years (MWh)
    pub use_percentile10: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 20th percentile of iterations and reference years (MWh)
    pub use_percentile20: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 30th percentile of iterations and reference years (MWh)
    pub use_percentile30: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 40th percentile of iterations and reference years (MWh)
    pub use_percentile40: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 50th percentile of iterations and reference years (MWh)
    pub use_percentile50: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 60th percentile of iterations and reference years (MWh)
    pub use_percentile60: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 70th percentile of iterations and reference years (MWh)
    pub use_percentile70: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 80th percentile of iterations and reference years (MWh)
    pub use_percentile80: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 90th percentile of iterations and reference years (MWh)
    pub use_percentile90: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 100th percentile of iterations and reference years (MWh)
    pub use_percentile100: Option<rust_decimal::Decimal>,
    /// Average period unserved energy across iterations and reference years (MWh)
    pub use_average: Option<rust_decimal::Decimal>,
    /// Total number of iterations and reference years performed
    pub numberofiterations: Option<rust_decimal::Decimal>,
    /// Number of iterations and reference years showing unserved energy
    pub use_numberofiterations: Option<rust_decimal::Decimal>,
    /// Maximum unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_max: Option<rust_decimal::Decimal>,
    /// Upper quartile unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_upperquartile: Option<rust_decimal::Decimal>,
    /// Median unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_median: Option<rust_decimal::Decimal>,
    /// Lower quartile unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_lowerquartile: Option<rust_decimal::Decimal>,
    /// Minimum unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_min: Option<rust_decimal::Decimal>,
    /// Fixed Values of 0.696 for 50 POE and 0.304 for 10 POE.  
    pub weight: Option<rust_decimal::Decimal>,
    /// Weighted average USE per region = (USE_AVERAGE_POE10/NATIVE_DEMAND_POE_10*WEIGHT_POE_10 + USE_AVERAGE_POE50/NATIVE_DEMAND_POE_50*WEIGHT_POE_50)*100
    pub use_weighted_avg: Option<rust_decimal::Decimal>,
    /// LRC Condition reported (Value=1) if USE_WEIGHTED_AVG &gt;= 0.002% otherwise (Value=0)
    pub lrc: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MtpasaRegionsummary1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONSUMMARY".into()),
            version: 1,
        }
    }
}
