/// Data Set Name: Pdpasa
/// File Name: Casesolution
/// Data Version: 3
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaCasesolution3 {
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
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for capacity adequacy (LRC) assessment. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE.
    capacityoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for assessment of Maximum surplus Reserve. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE
    maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for assessment of Maximum Spare Capacity. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE
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
impl crate::GetTable<PdpasaCasesolution3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PDPASA".into(),
                        table_name: "CASESOLUTION".into(),
                        version: 3,
                    }
                    
    }
}
/// Data Set Name: Pdpasa
/// File Name: Regionsolution
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaRegionsolution5 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    regionid: String,
    /// 10% Probability of Exceedance demand forecast
    demand10: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast
    demand50: Option<rust_decimal::Decimal>,
    /// 90% Probability of Exceedance demand forecast
    demand90: Option<rust_decimal::Decimal>,
    /// Region reserve requirement (MW)
    reservereq: Option<rust_decimal::Decimal>,
    /// Capacity required to meet the demand and reserve levels in the capacity adequacy assessment.
    capacityreq: Option<rust_decimal::Decimal>,
    /// Energy (GWh) required for this energy block based on the 50% probability of exceedance demand. Listed in the first interval of the energy block.
    energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Aggregate generator capability from Non Energy Constrained plant including restrictions due to network constraints from the capacity adequacy (LRC) assessment.
    unconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Aggregate generator capability from Energy Constrained plant including restrictions due to network constraints
    constrainedcapacity: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the capacity adequacy (LRC) assessment.
    netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Surplus capacity (MW) above the demand, scheduled load and net interchange in this region from the capacity adequacy (LRC) assessment.
    surpluscapacity: Option<rust_decimal::Decimal>,
    /// Surplus reserve (MW) above the demand, scheduled load,  net interchange and reserve requirement in this region from the capacity adequacy (LRC) assessment.
    surplusreserve: Option<rust_decimal::Decimal>,
    /// Low Reserve Condition (LRC) flag for this region in this interval (1 - LRC, 0 - No LRC)
    reservecondition: Option<rust_decimal::Decimal>,
    /// Maximum surplus reserve (MW) above the demand + reserve requirement able to be sourced to this region while meeting demand + reserve requirements in other regions.
    maxsurplusreserve: Option<rust_decimal::Decimal>,
    /// Maximum spare capacity (MW) above the demand able to be sourced to this region while meeting demands in other regions.
    maxsparecapacity: Option<rust_decimal::Decimal>,
    /// Lack of Reserve Condition (LOR) flag for this region and interval   (3 = LOR3, 2 = LOR2, 1 = LOR1, 0 = No LOR)
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
impl crate::GetTable<PdpasaRegionsolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PDPASA".into(),
                        table_name: "REGIONSOLUTION".into(),
                        version: 5,
                    }
                    
    }
}
