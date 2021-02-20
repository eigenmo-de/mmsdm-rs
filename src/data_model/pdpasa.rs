/// # Summary
/// 
/// ## PDPASA_CASESOLUTION
///  _The top-level table identifying a PDPASA case, reporting options applied in the case and summary results_
/// 
/// * Data Set Name: Pdpasa
/// * File Name: Casesolution
/// * Data Version: 3
/// 
/// # Description
///  PDPASA_CASESOLUTION is public data. Source PDPASA_CASESOLUTION is updated each PDPASA run (i.e. half-hourly). Volume Rows per day: 48 Mb per month: &lt;1
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaCasesolution3 {
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
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for capacity adequacy (LRC) assessment. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE.
    pub capacityoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for assessment of Maximum surplus Reserve. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE
    pub maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for assessment of Maximum Spare Capacity. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE
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
impl crate::GetTable for PdpasaCasesolution3 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PDPASA".into(),
                        table_name: Some("CASESOLUTION".into()),
                        version: 3,
                    }
                    
    }
}
/// # Summary
/// 
/// ## PDPASA_REGIONSOLUTION
///  _The PDPASA region solution data_
/// 
/// * Data Set Name: Pdpasa
/// * File Name: Regionsolution
/// * Data Version: 5
/// 
/// # Description
///  PDPASA_REGIONSOLUTION is public so is available to all participants. Source PDPASA_REGIONSOLUTION is updated each PDPASA run (i.e. half-hourly). Volume Rows per day: 32000 Notes LRC Determination SURPLUSRESERVE is the surplus reserve in a region based on meeting the demand plus the reserve requirement in all regions simultaneously. Note that any surplus above the network restrictions and system reserve requirements is reported in the region it is generated, thus a surplus of zero can mean that a region is importing to meet a requirement or that it has exported all surplus to meet an adjacent region’s requirement. &nbsp; The PASA processes also calculate a regionally optimised surplus called the Maximum LRC Surplus (MAXSURPLUSRESERVE) being a figure on how much generation could be brought to this region subject to meeting requirements in other regions. &nbsp; LOR Determination MAXSPARECAPACITY is a regionally optimised figure representing the surplus generation able to be brought to a region subject to meeting the demand in all other regions. &nbsp; Participants are directed to the first half hour of the Predispatch PASA (PDPASA) reports as NEMMCO's latest reserve determination for a given half hour.
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
pub struct PdpasaRegionsolution5 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    /// 10% Probability of Exceedance demand forecast
    pub demand10: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast
    pub demand50: Option<rust_decimal::Decimal>,
    /// 90% Probability of Exceedance demand forecast
    pub demand90: Option<rust_decimal::Decimal>,
    /// Region reserve requirement (MW)
    pub reservereq: Option<rust_decimal::Decimal>,
    /// Capacity required to meet the demand and reserve levels in the capacity adequacy assessment.
    pub capacityreq: Option<rust_decimal::Decimal>,
    /// Energy (GWh) required for this energy block based on the 50% probability of exceedance demand. Listed in the first interval of the energy block.
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Aggregate generator capability from Non Energy Constrained plant including restrictions due to network constraints from the capacity adequacy (LRC) assessment.
    pub unconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Aggregate generator capability from Energy Constrained plant including restrictions due to network constraints
    pub constrainedcapacity: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the capacity adequacy (LRC) assessment.
    pub netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Surplus capacity (MW) above the demand, scheduled load and net interchange in this region from the capacity adequacy (LRC) assessment.
    pub surpluscapacity: Option<rust_decimal::Decimal>,
    /// Surplus reserve (MW) above the demand, scheduled load,  net interchange and reserve requirement in this region from the capacity adequacy (LRC) assessment.
    pub surplusreserve: Option<rust_decimal::Decimal>,
    /// Low Reserve Condition (LRC) flag for this region in this interval (1 - LRC, 0 - No LRC)
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// Maximum surplus reserve (MW) above the demand + reserve requirement able to be sourced to this region while meeting demand + reserve requirements in other regions.
    pub maxsurplusreserve: Option<rust_decimal::Decimal>,
    /// Maximum spare capacity (MW) above the demand able to be sourced to this region while meeting demands in other regions.
    pub maxsparecapacity: Option<rust_decimal::Decimal>,
    /// Lack of Reserve Condition (LOR) flag for this region and interval   (3 = LOR3, 2 = LOR2, 1 = LOR1, 0 = No LOR)
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
impl crate::GetTable for PdpasaRegionsolution5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PDPASA".into(),
                        table_name: Some("REGIONSOLUTION".into()),
                        version: 5,
                    }
                    
    }
}
