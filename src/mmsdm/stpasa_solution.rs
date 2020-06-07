/// Data Set Name: Stpasa
/// File Name: Constraintsolution
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaConstraintsolution2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    constraintid: String,
    /// The RHS value in the capacity evaluation.
    capacityrhs: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment marginal value, 0 if not binding
    capacitymarginalvalue: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment violation degree for generic constraint; 0 if not violating
    capacityviolationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run.  Values are RELIABILITY_LRC and OUTAGE_LRC
    runtype: String,
}
impl crate::GetTable<StpasaConstraintsolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "STPASA".into(),
                        table_name: "CONSTRAINTSOLUTION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Stpasa
/// File Name: Interconnectorsoln
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaInterconnectorsoln2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Interconnector Identifier
    interconnectorid: String,
    /// Interconnector loading level (MW) that can be reached in case of capacity scarcity in neighbouring regions subject to network and energy constraints
    capacitymwflow: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment marginal value, 0 if not binding
    capacitymarginalvalue: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment violation degree for interconnector capacity; 0 if not violating
    capacityviolationdegree: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    calculatedexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    calculatedimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run.  Values are RELIABILITY_LRC and OUTAGE_LRC
    runtype: String,
    /// ID of the constraint that sets the Interconnector Export Limit
    exportlimitconstraintid: Option<String>,
    /// ID of the constraint that sets the Interconnector Import Limit
    importlimitconstraintid: Option<String>,
}
impl crate::GetTable<StpasaInterconnectorsoln2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "STPASA".into(),
                        table_name: "INTERCONNECTORSOLN".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Stpasa
/// File Name: Regionsolution
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaRegionsolution5 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    regionid: String,
    /// Input value for 10% probability demand
    demand10: Option<rust_decimal::Decimal>,
    /// Input value for 50% probability demand
    demand50: Option<rust_decimal::Decimal>,
    /// Input value for 90% probability demand
    demand90: Option<rust_decimal::Decimal>,
    /// Input reserve requirement
    reservereq: Option<rust_decimal::Decimal>,
    /// Demand + Reserve Requirement
    capacityreq: Option<rust_decimal::Decimal>,
    /// Sum of: (Region Period Demand - given Demand50)/Period (sum by trading day, entered in first period of trading day, GWh)
    energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Region energy unconstrained MW capacity subject to energy and network security constraints
    unconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Available capacity (MW) in this region energy constrained MW capacity subject to energy and network security constraints
    constrainedcapacity: Option<rust_decimal::Decimal>,
    /// Net export in MW out of this region in the capacity adequacy evaluation. Export if &gt; 0, Import if &lt; 0.
    netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Regional surplus capacity MW, +/- values indicate surplus/deficit capacity respectively
    surpluscapacity: Option<rust_decimal::Decimal>,
    /// Regional reserve surplus. +/- values indicate surplus/deficit reserve respectively
    surplusreserve: Option<rust_decimal::Decimal>,
    /// The regional reserve condition: 0  Adequate, 1  LRC
    reservecondition: Option<rust_decimal::Decimal>,
    /// The Maximum Surplus Reserve evaluated for this region in this period.  Calculated for each region in turn.
    maxsurplusreserve: Option<rust_decimal::Decimal>,
    /// The Maximum Spare Capacity evaluated for this region in this period. Calculated for each region in turn. 
    maxsparecapacity: Option<rust_decimal::Decimal>,
    /// The LOR Condition determined from the Maximum Spare Capacity value: 0 - no condition, 1 - LOR1 condition, 2 - LOR2 condition, 3 - LOR3 condition
    lorcondition: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities offered by all Scheduled Generators in a given Region for a given PERIODID.
    aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities bid by of all Scheduled Loads in a given Region for a given PERIODID.
    aggregatescheduledload: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of  PASAAVAILABILITY quantities offered by all Scheduled Generators in a given Region for a given PERIODID.
    aggregatepasaavailability: Option<rust_decimal::Decimal>,
    /// Type of run.  Values are RELIABILITY_LRC and OUTAGE_LRC
    runtype: String,
    /// Energy (GWh) required for this energy block based on the 10% probability of exceedance demand. Listed in the first interval of the energy block
    energyreqdemand10: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR1 used. Can be static value or calculated value if an interconnector is a credible contingency
    calculatedlor1level: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR2 used. Can be static value or calculated value if an interconnector is a credible contingency
    calculatedlor2level: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the MSR assessment
    msrnetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the LOR assessment
    lornetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    uigf: Option<rust_decimal::Decimal>,
    /// Aggregate Regional UIGF availability
    semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    /// Aggregate Regional UIGF availability for LOR
    lor_semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    /// Largest Credible Risk. MW value for highest credible contingency 
    lcr: Option<rust_decimal::Decimal>,
    /// Two Largest Creditable Risks. MW value for highest two credible contingencies.
    lcr2: Option<rust_decimal::Decimal>,
    /// Forecasting Uncertainty Measure. MW value of reserve calculated as defined in the Reserve Level Declaration Guidelines
    fum: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is solar
    ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is wind
    ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled UIGF availability where the primary fuel source is solar
    ss_solar_capacity: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled UIGF availability where the primary fuel source is wind
    ss_wind_capacity: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled cleared MW where the primary fuel source is solar and StudyRegion = Region
    ss_solar_cleared: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled cleared MW where the primary fuel source is wind and StudyRegion = Region
    ss_wind_cleared: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<StpasaRegionsolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "STPASA".into(),
                        table_name: "REGIONSOLUTION".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Stpasa
/// File Name: Casesolution
/// Data Version: 3
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaCasesolution3 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// Version of the PASA solver used to solve this case
    pasaversion: Option<String>,
    /// Low Reserve Condition (LRC) flag for the case (1 - LRC in the case, 0 - No LRCs in the case) for capacity run
    reservecondition: Option<rust_decimal::Decimal>,
    /// Lack of Reserve Condition (LOR) flag for the case indicates the most severe condition in the case  (3 = LOR3, 2 = LOR2, 1 = LOR1, 0 = No LOR)
    lorcondition: Option<rust_decimal::Decimal>,
    /// Objective Function from the Capacity Adequacy run
    capacityobjfunction: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for capacity adequacy assessment. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    capacityoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for assessment of Maximum surplus Reserve. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for assessment of Maximum Spare Capacity. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    maxsparecapacityoption: Option<rust_decimal::Decimal>,
    /// The penalty for non-zero interconnector flow
    interconnectorflowpenalty: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for Reliability LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    reliabilitylrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for outage LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    outagelrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for LOR assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    lordemandoption: Option<rust_decimal::Decimal>,
    /// Generation Availability to be used in Reliability LRC run (either PASA or MARKET)
    reliabilitylrccapacityoption: Option<String>,
    /// Generation Availability to be used in Outage LRC run (either PASA or MARKET)
    outagelrccapacityoption: Option<String>,
    /// Generation Availability to be used in LOR run (either PASA or MARKET)
    lorcapacityoption: Option<String>,
    /// UIGF POE forecast availability used for this option
    loruigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    reliability_lrcuigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    outage_lrcuigf_option: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<StpasaCasesolution3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "STPASA".into(),
                        table_name: "CASESOLUTION".into(),
                        version: 3,
                    }
                    
    }
}
