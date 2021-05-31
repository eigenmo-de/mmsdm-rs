/// # Summary
///
/// ## STPASA_CASESOLUTION
///  _STPASA_CASESOLUTION holds one record containing results pertaining to each entire solution_
///
/// * Data Set Name: Stpasa
/// * File Name: Casesolution
/// * Data Version: 3
///
/// # Description
///  STPASA_CASESOLUTION is public data. Source STPASA_CASESOLUTION is updated each STPASA run (i.e. every 2 hours). Volume Rows per day: 12 Mb per month: &lt;1
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaCasesolution3 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Version of the PASA solver used to solve this case
    pub pasaversion: Option<String>,
    /// Low Reserve Condition (LRC) flag for the case (1 - LRC in the case, 0 - No LRCs in the case) for capacity run
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// Lack of Reserve Condition (LOR) flag for the case indicates the most severe condition in the case  (3 = LOR3, 2 = LOR2, 1 = LOR1, 0 = No LOR)
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Objective Function from the Capacity Adequacy run
    pub capacityobjfunction: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for capacity adequacy assessment. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    pub capacityoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for assessment of Maximum surplus Reserve. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    pub maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for assessment of Maximum Spare Capacity. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    pub maxsparecapacityoption: Option<rust_decimal::Decimal>,
    /// The penalty for non-zero interconnector flow
    pub interconnectorflowpenalty: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for Reliability LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub reliabilitylrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for outage LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub outagelrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for LOR assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub lordemandoption: Option<rust_decimal::Decimal>,
    /// Generation Availability to be used in Reliability LRC run (either PASA or MARKET)
    pub reliabilitylrccapacityoption: Option<String>,
    /// Generation Availability to be used in Outage LRC run (either PASA or MARKET)
    pub outagelrccapacityoption: Option<String>,
    /// Generation Availability to be used in LOR run (either PASA or MARKET)
    pub lorcapacityoption: Option<String>,
    /// UIGF POE forecast availability used for this option
    pub loruigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    pub reliability_lrcuigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    pub outage_lrcuigf_option: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for StpasaCasesolution3 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: Some("CASESOLUTION".into()),
            version: 3,
        }
    }
}
/// # Summary
///
/// ## STPASA_CONSTRAINTSOLUTION
///  _STPASA_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value._
///
/// * Data Set Name: Stpasa
/// * File Name: Constraintsolution
/// * Data Version: 3
///
/// # Description
///  STPASA_CONSTRAINTSOLUTION is public data. Source STPASA_CONSTRAINTSOLUTION is updated each STPASA run (i.e. every 2 hours). Volume Rows per day: 19000 (est.) Mb per month: 90
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
/// * STUDYREGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaConstraintsolution3 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// The RHS value in the capacity evaluation.
    pub capacityrhs: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment marginal value, 0 if not binding
    pub capacitymarginalvalue: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment violation degree for generic constraint; 0 if not violating
    pub capacityviolationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run.  Values are RELIABILITY_LRC and OUTAGE_LRC
    pub runtype: String,
    /// Primary Region for LP Solve (or MARKET if none).
    pub studyregionid: String,
}
impl crate::GetTable for StpasaConstraintsolution3 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: Some("CONSTRAINTSOLUTION".into()),
            version: 3,
        }
    }
}
/// # Summary
///
/// ## STPASA_INTERCONNECTORSOLN
///  _STPASA_INTERCONNECTORSOLN shows the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval._
///
/// * Data Set Name: Stpasa
/// * File Name: Interconnectorsoln
/// * Data Version: 3
///
/// # Description
///  STPASA_INTERCONNECTORSOLN is public so is available to all participants. Source STPASA_INTERCONNECTORSOLN is updated each STPASA run (i.e. every 2 hours). Volume Rows per day: 576 Mb per month: 4
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
/// * STUDYREGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaInterconnectorsoln3 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Interconnector Identifier
    pub interconnectorid: String,
    /// Interconnector loading level (MW) that can be reached in case of capacity scarcity in neighbouring regions subject to network and energy constraints
    pub capacitymwflow: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment marginal value, 0 if not binding
    pub capacitymarginalvalue: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment violation degree for interconnector capacity; 0 if not violating
    pub capacityviolationdegree: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    pub calculatedexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    pub calculatedimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run.  Values are RELIABILITY_LRC and OUTAGE_LRC
    pub runtype: String,
    /// ID of the constraint that sets the Interconnector Export Limit
    pub exportlimitconstraintid: Option<String>,
    /// ID of the constraint that sets the Interconnector Import Limit
    pub importlimitconstraintid: Option<String>,
    /// Primary Region for LP Solve (or MARKET if none).
    pub studyregionid: String,
}
impl crate::GetTable for StpasaInterconnectorsoln3 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: Some("INTERCONNECTORSOLN".into()),
            version: 3,
        }
    }
}
/// # Summary
///
/// ## STPASA_REGIONSOLUTION
///  _STPASA_REGIONSOLUTION shows the results of the regional capacity, maximum surplus reserve and maximum spare capacity evaluations for each period of the study._
///
/// * Data Set Name: Stpasa
/// * File Name: Regionsolution
/// * Data Version: 6
///
/// # Description
///  STPASA_REGIONSOLUTION is public so is available to all participants. Source STPASA_REGIONSOLUTION is updated each STPASA run (i.e every 2 hours). Volume Rows per day: 480 Mb per month: 8
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaRegionsolution6 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: String,
    /// Input value for 10% probability demand
    pub demand10: Option<rust_decimal::Decimal>,
    /// Input value for 50% probability demand
    pub demand50: Option<rust_decimal::Decimal>,
    /// Input value for 90% probability demand
    pub demand90: Option<rust_decimal::Decimal>,
    /// Input reserve requirement
    pub reservereq: Option<rust_decimal::Decimal>,
    /// Demand + Reserve Requirement
    pub capacityreq: Option<rust_decimal::Decimal>,
    /// Sum of: (Region Period Demand - given Demand50)/Period (sum by trading day, entered in first period of trading day, GWh)
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Region energy unconstrained MW capacity subject to energy and network security constraints
    pub unconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Available capacity (MW) in this region energy constrained MW capacity subject to energy and network security constraints
    pub constrainedcapacity: Option<rust_decimal::Decimal>,
    /// Net export in MW out of this region in the capacity adequacy evaluation. Export if &gt; 0, Import if &lt; 0.
    pub netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Regional surplus capacity MW, +/- values indicate surplus/deficit capacity respectively
    pub surpluscapacity: Option<rust_decimal::Decimal>,
    /// Regional reserve surplus. +/- values indicate surplus/deficit reserve respectively
    pub surplusreserve: Option<rust_decimal::Decimal>,
    /// The regional reserve condition: 0  Adequate, 1  LRC
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// The Maximum Surplus Reserve evaluated for this region in this period.  Calculated for each region in turn.
    pub maxsurplusreserve: Option<rust_decimal::Decimal>,
    /// The Maximum Spare Capacity evaluated for this region in this period. Calculated for each region in turn.
    pub maxsparecapacity: Option<rust_decimal::Decimal>,
    /// The LOR Condition determined from the Maximum Spare Capacity value: 0 - no condition, 1 - LOR1 condition, 2 - LOR2 condition, 3 - LOR3 condition
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities offered by all Scheduled Generators in a given Region for a given PERIODID.
    pub aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities bid by of all Scheduled Loads in a given Region for a given PERIODID.
    pub aggregatescheduledload: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of  PASAAVAILABILITY quantities offered by all Scheduled Generators in a given Region for a given PERIODID.
    pub aggregatepasaavailability: Option<rust_decimal::Decimal>,
    /// Type of run.  Values are RELIABILITY_LRC and OUTAGE_LRC
    pub runtype: String,
    /// Energy (GWh) required for this energy block based on the 10% probability of exceedance demand. Listed in the first interval of the energy block
    pub energyreqdemand10: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR1 used. Can be static value or calculated value if an interconnector is a credible contingency
    pub calculatedlor1level: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR2 used. Can be static value or calculated value if an interconnector is a credible contingency
    pub calculatedlor2level: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the MSR assessment
    pub msrnetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the LOR assessment
    pub lornetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    /// Aggregate Regional UIGF availability
    pub semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    /// Aggregate Regional UIGF availability for LOR
    pub lor_semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    /// Largest Credible Risk. MW value for highest credible contingency
    pub lcr: Option<rust_decimal::Decimal>,
    /// Two Largest Creditable Risks. MW value for highest two credible contingencies.
    pub lcr2: Option<rust_decimal::Decimal>,
    /// Forecasting Uncertainty Measure. MW value of reserve calculated as defined in the Reserve Level Declaration Guidelines
    pub fum: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is solar
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is wind
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled UIGF availability where the primary fuel source is solar
    pub ss_solar_capacity: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled UIGF availability where the primary fuel source is wind
    pub ss_wind_capacity: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled cleared MW where the primary fuel source is solar and StudyRegion = Region
    pub ss_solar_cleared: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled cleared MW where the primary fuel source is wind and StudyRegion = Region
    pub ss_wind_cleared: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for StpasaRegionsolution6 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: Some("REGIONSOLUTION".into()),
            version: 6,
        }
    }
}
