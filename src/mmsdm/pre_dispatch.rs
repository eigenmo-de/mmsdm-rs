/// # Summary
///
/// ## PREDISPATCH_FCAS_REQ
///  _PREDISPATCH_FCAS_REQ shows Predispatch Constraint tracking for Regional FCAS Requirements._
///
/// * Data Set Name: Predispatch
/// * File Name: Regionfcasrequirement
/// * Data Version: 2
///
/// # Description
///  Source PREDISPATCH_FCAS_REQ updates with each pre-dispatch run (half hourly) Volume Approximately 2,000 rows per day. Note The PERIODID columns in tables PREDISPATCHCONSTRAINT and PREDISPATCH_FCAS_REQ have no consistent relationship with the other PERIODID values in the other tables in the PRE-DISPATCH package (such as PREDISPATCHPRICE). AEMO and many Participants appreciate the data model is inconsistent, but the cost of changing existing systems has been judged as being unjustifiable. An additional field DATETIME was added to allow joins between these data sets.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DATETIME
/// * GENCONID
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionfcasrequirement2 {
    /// PreDispatch Sequence number
    pub predispatchseqno: Option<String>,
    /// Case Run number
    pub runno: Option<rust_decimal::Decimal>,
    /// Intervention Flag
    pub intervention: Option<rust_decimal::Decimal>,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    pub periodid: Option<String>,
    /// Generic Constraint ID - Join to table GenConData
    pub genconid: String,
    /// Region ID
    pub regionid: String,
    /// Bid Type Identifier
    pub bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    pub genconversionno: Option<rust_decimal::Decimal>,
    /// Marginal Value of generic constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The base cost of the constraint for this service, before the regulation/contingency split
    pub base_cost: Option<rust_decimal::Decimal>,
    /// The adjusted cost of the constraint for this service, before the regulation/contingency split
    pub adjusted_cost: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CMPF, based on dispatched data
    pub estimated_cmpf: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CRMPF, based on dispatched data
    pub estimated_crmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CMPF based recovery
    pub recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CRMPF based recovery
    pub recovery_factor_crmpf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchRegionfcasrequirement2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "REGIONFCASREQUIREMENT".into(),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHCASESOLUTION
///  _PREDISPATCHCASESOLUTION provides information relating to the complete predispatch run. The fields provide an overview of the dispatch run results allowing immediate identification of conditions such as energy or FCAS deficiencies._
///
/// * Data Set Name: Predispatch
/// * File Name: Case Solution
/// * Data Version: 1
///
/// # Description
///  PREDISPATCHCASESOLUTION data is public, so is available to all participants. Source PREDISPATCHCASESOLUTION updates every half-hour. Volume Approximately 48 records per day.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PREDISPATCHSEQNO
/// * RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchCaseSolution1 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// Predispatch run no, normally 1.
    pub runno: rust_decimal::Decimal,
    /// If non-zero indicated one of the following conditions: 1 = Supply Scarcity, Excess generation or constraint violations, -X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: Option<String>,
    /// Non-Physical Losses algorithm invoked during this run
    pub nonphysicallosses: Option<rust_decimal::Decimal>,
    /// The Objective function from the LP
    pub totalobjective: Option<rust_decimal::Decimal>,
    /// Total Region Demand violations
    pub totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Total interconnector violations
    pub totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Total generic constraint violations
    pub totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Total ramp rate violations
    pub totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Total unit capacity violations
    pub totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Total of 5 minute ancillary service region violations
    pub total5minviolation: Option<rust_decimal::Decimal>,
    /// Total of Regulation ancillary service region violations
    pub totalregviolation: Option<rust_decimal::Decimal>,
    /// Total of 6 second ancillary service region violations
    pub total6secviolation: Option<rust_decimal::Decimal>,
    /// Total of 60 second ancillary service region violations
    pub total60secviolation: Option<rust_decimal::Decimal>,
    /// Total of ancillary service trader profile violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Total of Energy Constrained unit offer violations.
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Pre-Dispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchCaseSolution1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "CASE_SOLUTION".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCH_MNSPBIDTRK
///  _PREDISPATCH_MNSPBIDTRK shows the MNSP bid tracking, including the bid version used in each predispatch run for each MNSP Interconnector Link. PREDISPATCH_MNSPBIDTRK shows the audit trail of the bid used for each predispatch run._
///
/// * Data Set Name: Predispatch
/// * File Name: Mnspbidtrk
/// * Data Version: 1
///
/// # Description
///  Source Own (confidential) data updates every predispatch run. All bids are available to all participants as part of next day market data. Volume 1, 700, 000 per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * LINKID
/// * PERIODID
/// * PREDISPATCHSEQNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchMnspbidtrk1 {
    /// Predispatch run identifier
    pub predispatchseqno: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    /// Trading Interval number
    pub periodid: String,
    /// Participant Identifier
    pub participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub settlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Version No. for given offer date and settlement date used
    pub versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchMnspbidtrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "MNSPBIDTRK".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHBLOCKEDCONSTRAINT
///  _PREDISPATCH Blocked Constraints lists any constraints that were blocked in a Predispatch run. If no constraints are blocked, there will be no rows for that predispatch run._
///
/// * Data Set Name: Predispatch
/// * File Name: Blocked Constraints
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * PREDISPATCHSEQNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchBlockedConstraints1 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
}
impl crate::GetTable for PredispatchBlockedConstraints1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "BLOCKED_CONSTRAINTS".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHREGIONSUM
///  _PREDISPATCHREGIONSUM sets out the overall regional Pre-Dispatch results for base case details (excluding price). _
///
/// * Data Set Name: Predispatch
/// * File Name: Region Solution
/// * Data Version: 4
///
/// # Description
///  PREDISPATCHREGIONSUM includes the forecast demand (total demand) and Frequency Control Ancillary Services (FCAS) requirements (specifically, for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations). PREDISPATCHREGIONSUM updates each half-hour with the latest Pre-Dispatch details for the remaining period. Regional demand can be calculated as total demand plus dispatchable load (i.e. Regional demand = Total Demand + Dispatchable Load) Source PREDISPATCHREGIONSUM updates every thirty minutes. Note *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values. From 16 February 2006, the old reserve values are no longer populated (i.e. are null), being LORSurplus and LRCSurplus. For more details on the changes to Reporting of Reserve Condition Data, refer to AEMO Communication 2042. For the best available indicator of reserve condition in each of the regions of the NEM for each trading interval, refer to the latest run of the Pre-Dispatch PASA (see table PDPASA_REGIONSOLUTION).
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionSolution4 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Pre-Dispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: String,
    /// PERIODID is just a period count, starting from 1 for each Pre-Dispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Total demand in MW for period (less normally on loads)
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    pub availableload: Option<rust_decimal::Decimal>,
    /// Delta MW value only
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Generation dispatched in period
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Load dispatched in period
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Excess generation in period / Deficit generation if VOLL
    pub excessgeneration: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    pub lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    pub lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    pub lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 5 min
    pub lower5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    pub lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 5 min
    pub lower5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    pub lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 5 min
    pub lower5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    pub lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    pub lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    pub lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 60 sec
    pub lower60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    pub lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 60 sec
    pub lower60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    pub lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 60 sec
    pub lower60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    pub lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    pub lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    pub lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 6 sec
    pub lower6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    pub lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 6 sec
    pub lower6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    pub lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 6 sec
    pub lower6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW dispatch
    pub raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003.  Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 5 min
    pub raise5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    pub raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 5 min
    pub raise5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    pub raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 5 min
    pub raise5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    pub raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    pub raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 60 sec local dispatch
    pub raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 60 sec
    pub raise60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    pub raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 60 sec
    pub raise60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec total requirement
    pub raise60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 60 sec
    pub raise60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW dispatch
    pub raise6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW imported
    pub raise6secimport: Option<rust_decimal::Decimal>,
    /// Raise 6 sec local dispatch
    pub raise6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 6 sec
    pub raise6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec local requirement
    pub raise6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 6 sec
    pub raise6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec total requirement
    pub raise6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 6 sec
    pub raise6secsupplyprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Sum of initial generation and import for region
    pub initialsupply: Option<rust_decimal::Decimal>,
    /// Sum of cleared generation and import for region
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation MW imported
    pub lowerregimport: Option<rust_decimal::Decimal>,
    /// Lower Regulation local dispatch
    pub lowerreglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation local requirement
    pub lowerreglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation total requirement
    pub lowerregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation MW imported
    pub raiseregimport: Option<rust_decimal::Decimal>,
    /// Raise Regulation local dispatch
    pub raisereglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation local requirement
    pub raisereglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation total requirement
    pub raiseregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min local requirement
    pub raise5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg local requirement
    pub raisereglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 sec local requirement
    pub raise60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 sec local requirement
    pub raise6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min local requirement
    pub lower5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg local requirement
    pub lowerreglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 sec local requirement
    pub lower60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 sec local requirement
    pub lower6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min requirement
    pub raise5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg requirement
    pub raiseregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 seconds requirement
    pub raise60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 seconds requirement
    pub raise6secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min requirement
    pub lower5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg requirement
    pub lowerregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 seconds requirement
    pub lower60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 seconds requirement
    pub lower6secviolation: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    pub raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    pub raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    pub raise5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise reg availability
    pub raiseregactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 6sec availability
    pub lower6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 60sec availability
    pub lower60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 5min availability
    pub lower5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower reg availability
    pub lowerregactualavailability: Option<rust_decimal::Decimal>,
    /// generation availability taking into account daily energy constraints
    pub decavailability: Option<rust_decimal::Decimal>,
    /// Not used after Feb 2006. Total short term generation capacity reserve used in assessing lack of reserve condition
    pub lorsurplus: Option<rust_decimal::Decimal>,
    /// Not used after Feb 2006. Total short term generation capacity reserve above the stated low reserve condition requirement
    pub lrcsurplus: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW
    pub semischedule_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced
    pub semischedule_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is solar
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is wind
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is solar
    pub ss_solar_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is wind
    pub ss_wind_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is solar
    pub ss_solar_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is wind
    pub ss_wind_compliancemw: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchRegionSolution4 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "REGION_SOLUTION".into(),
            version: 4,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHOFFERTRK
///  _PREDISPATCHOFFERTRK is for the ancillary service bid tracking of predispatch processing. PREDISPATCHOFFERTRK identifies which bids from BIDDAYOFFER and BIDPEROFFER were applied for a given unit and ancillary service for each predispatch run._
///
/// * Data Set Name: Predispatch
/// * File Name: Offertrk
/// * Data Version: 1
///
/// # Description
///  Source PREDISPATCHOFFERTRK updates every 30 minutes. The data is confidential to each participant until the next trading day.  Volume Approximately 45,000 records per day.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * PERIODID
/// * PREDISPATCHSEQNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchOffertrk1 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Bid type Identifier - the ancillary service to which the bid applies
    pub bidtype: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidofferdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchOffertrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "OFFERTRK".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHPRICESENSITIVITIES
///  _PREDISPATCHPRICESENSITIVITIES sets out the sensitivity prices for each region by period._
///
/// * Data Set Name: Predispatch
/// * File Name: Pricesensitivities
/// * Data Version: 1
///
/// # Description
///  Source The plan is to provide this data every half-hour.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchPricesensitivities1 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 1
    pub rrpeep1: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 2
    pub rrpeep2: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 3
    pub rrpeep3: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 4
    pub rrpeep4: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 5
    pub rrpeep5: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 6
    pub rrpeep6: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 7
    pub rrpeep7: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 8
    pub rrpeep8: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 9
    pub rrpeep9: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 10
    pub rrpeep10: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 11
    pub rrpeep11: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 12
    pub rrpeep12: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 13
    pub rrpeep13: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 14
    pub rrpeep14: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 15
    pub rrpeep15: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 16
    pub rrpeep16: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 17
    pub rrpeep17: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 18
    pub rrpeep18: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 19
    pub rrpeep19: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 20
    pub rrpeep20: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 21
    pub rrpeep21: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 22
    pub rrpeep22: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 23
    pub rrpeep23: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 24
    pub rrpeep24: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 25
    pub rrpeep25: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 26
    pub rrpeep26: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 27
    pub rrpeep27: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 28
    pub rrpeep28: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Regional Energy Price for scenario 29
    pub rrpeep29: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 30
    pub rrpeep30: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 31
    pub rrpeep31: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 32
    pub rrpeep32: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 33
    pub rrpeep33: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 34
    pub rrpeep34: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 35
    pub rrpeep35: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this period has an active intervention constraint: 0= No, 1= Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 36
    pub rrpeep36: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 37
    pub rrpeep37: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 38
    pub rrpeep38: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 39
    pub rrpeep39: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 40
    pub rrpeep40: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 41
    pub rrpeep41: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 42
    pub rrpeep42: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 43
    pub rrpeep43: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchPricesensitivities1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "PRICESENSITIVITIES".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHSCENARIODEMAND
///  _PREDISPATCHSCENARIODEMAND defines the demand offsets that are applied for each of the predispatch sensitivity scenarios._
///
/// * Data Set Name: Predispatch
/// * File Name: Scenario Demand
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
/// * REGIONID
/// * SCENARIO
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemand1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub versionno: i64,
    /// The scenario identifier.
    pub scenario: i64,
    /// The region to which to apply the deltaMW for this SCENARIO.
    pub regionid: String,
    /// The MW offset that is applied for this scenario
    pub deltamw: Option<i64>,
}
impl crate::GetTable for PredispatchScenarioDemand1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "SCENARIO_DEMAND".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHCONSTRAINT
///  _PREDISPATCHCONSTRAINT sets out constraints that are binding in each predispatch run and interconnector constraints (whether binding or not). Only binding and interconnector constraints are reported. Binding contracts have marginal value greater than $0. Interconnector constraints are listed so RHS values can be reported for ST PASA.<br>Constraint solutions only report fixed loading /MR constraints on the next day.<br>_
///
/// * Data Set Name: Predispatch
/// * File Name: Constraint Solution
/// * Data Version: 5
///
/// # Description
///  PREDISPATCHCONSTRAINT data is confidential on the day of creation, and public to all participants after the end of the market day. Source PREDISPATCHCONSTRAINT updates with every thirty-minute predispatch run. Note The PERIODID columns in tables PREDISPATCHCONSTRAINT and PREDISPATCH_FCAS_REQ have no consistent relationship with the other PERIODID values in the other tables in the PRE-DISPATCH package (such as PREDISPATCHPRICE). AEMO and many Participants appreciate the data model is inconsistent, but the cost of changing existing systems has been judged as being unjustifiable. An additional field DATETIME was added to allow joins between these data sets.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchConstraintSolution5 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Generic constraint identifier
    pub constraintid: String,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// RHS value used.
    pub rhs: Option<rust_decimal::Decimal>,
    /// Marginal value of violated constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of constraint violation
    pub violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    pub duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    pub genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    pub lhs: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchConstraintSolution5 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "CONSTRAINT_SOLUTION".into(),
            version: 5,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHINTERSENSITIVITIES
///  _PREDISPATCHINTERSENSITIVITIES sets out the sensitivity flows for each interconnector by period._
///
/// * Data Set Name: Predispatch
/// * File Name: Interconnectr Sens
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectrSens1 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique interconnector identifier
    pub interconnectorid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this period has an active intervention constraint: 0= No, 1= Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 1
    pub mwflow1: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 2
    pub mwflow2: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 3
    pub mwflow3: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 4
    pub mwflow4: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 5
    pub mwflow5: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 6
    pub mwflow6: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 7
    pub mwflow7: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 8
    pub mwflow8: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 9
    pub mwflow9: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 10
    pub mwflow10: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 11
    pub mwflow11: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 12
    pub mwflow12: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 13
    pub mwflow13: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 14
    pub mwflow14: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 15
    pub mwflow15: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 16
    pub mwflow16: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 17
    pub mwflow17: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 18
    pub mwflow18: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 19
    pub mwflow19: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 20
    pub mwflow20: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 21
    pub mwflow21: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 22
    pub mwflow22: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 23
    pub mwflow23: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 24
    pub mwflow24: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 25
    pub mwflow25: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 26
    pub mwflow26: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 27
    pub mwflow27: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 28
    pub mwflow28: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 29
    pub mwflow29: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 30
    pub mwflow30: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 31
    pub mwflow31: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 32
    pub mwflow32: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 33
    pub mwflow33: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 34
    pub mwflow34: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 35
    pub mwflow35: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 36
    pub mwflow36: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 37
    pub mwflow37: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 38
    pub mwflow38: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 39
    pub mwflow39: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 40
    pub mwflow40: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 41
    pub mwflow41: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 42
    pub mwflow42: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 43
    pub mwflow43: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchInterconnectrSens1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "INTERCONNECTR_SENS".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHSCENARIODEMANDTRK
///  _Tracks the predispatch scenario offset updates across time_
///
/// * Data Set Name: Predispatch
/// * File Name: Scenario Demand Trk
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
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemandTrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub versionno: i64,
    /// The user that authorised the scenario update
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchScenarioDemandTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "SCENARIO_DEMAND_TRK".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHLOAD
///  _PREDISPATCHLOAD shows pre-dispatch targets for each dispatchable unit, including additional fields to handle the Ancillary Services functionality. No record is written where a unit is not dispatched. PREDISPATCHLOAD shows all the results for each period._
///
/// * Data Set Name: Predispatch
/// * File Name: Unit Solution
/// * Data Version: 2
///
/// # Description
///  Source Own (confidential) data updates every thirty minutes, with whole market data for the day before available as part of next day market data. Note ** A flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined using the low 3 bits as follows: Flag Name Bit Description Enabled 0 The unit is enabled to provide this ancillary service type. Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. Interpretation of the bit-flags as a number gives the following possibilities (i.e. other combinations are not possible): Numeric Value Bit (2,1,0) Meaning 0 000 Not stranded, not trapped, not enabled (i.e. is unavailable). 1 001 Not stranded, not trapped, is enabled (i.e. available). 3 011 Not stranded, is trapped, is enabled (i.e. trapped). 4 100 Is stranded, not trapped, not enabled (i.e. stranded). For example, testing for availability can be done by checking for odd (=available) or even (=unavailable) number (e.g.  mod(flag,2)  results in 0 for unavailable and 1 for available). *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DATETIME
/// * DUID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchUnitSolution2 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Dispatchable unit identifier for fast start
    pub duid: String,
    /// Not used
    pub tradetype: Option<rust_decimal::Decimal>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Connection point identifier
    pub connectionpointid: Option<String>,
    /// AGC Status from EMS
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Dispatch mode of unit for fast start (1-4)
    pub dispatchmode: Option<rust_decimal::Decimal>,
    /// Initial MW at start of first period. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run.
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW at end of period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Lower 5 min MW target in period
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec MW target in period
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec MW target in period
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min MW target in period
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec MW target in period
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec MW target in period
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Ramp down rate in period in MW/minute
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate in period in MW/minute
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    pub downepf: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    pub upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min from LP Solver
    pub marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds from LP Solver
    pub marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds from LP Solver
    pub marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy from LP Solver
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation MW 5 min
    pub violation5mindegree: Option<rust_decimal::Decimal>,
    /// Violation MW 60 seconds
    pub violation60secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW 6 seconds
    pub violation6secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW energy
    pub violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise reg status flag
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  
    pub lowerregflags: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    pub raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    pub raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    pub raise5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise reg availability
    pub raiseregactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 6sec availability
    pub lower6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 60sec availability
    pub lower60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 5min availability
    pub lower5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower reg availability
    pub lowerregactualavailability: Option<rust_decimal::Decimal>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchUnitSolution2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "UNIT_SOLUTION".into(),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHINTERCONNECTORRES
///  _PREDISPATCHINTERCONNECTORRES records Interconnector flows and losses for the periods calculated in each predispatch run. Only binding and interconnector constraints are reported.<br>Some fields are for the Frequency Controlled Ancillary Services export and import limits and extra reporting of the generic constraint setting the energy import and export limits.<br>_
///
/// * Data Set Name: Predispatch
/// * File Name: Interconnector Soln
/// * Data Version: 3
///
/// # Description
///  Source PREDISPATCHINTERCONNECTORRES updates with every thirty-minute predispatch run. Note MW losses can be negative depending on the flow. The definition of direction of flow for an interconnector is that positive flow starts from the FROMREGION in INTERCONNECTOR.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectorSoln3 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Metered MW Flow from EMS. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Flow
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// $ Marginal value of interconnector constraint from SPD
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation of interconnector constraint in MW
    pub violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Calculated export limit.
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust bids between reports.
    pub marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    pub exportgenconid: Option<String>,
    /// Generic Constraint setting the import limit
    pub importgenconid: Option<String>,
    /// Calculated export limit applying to energy + FCAS.
    pub fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + FCAS.
    pub fcasimportlimit: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    pub local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    pub local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_import: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchInterconnectorSoln3 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "INTERCONNECTOR_SOLN".into(),
            version: 3,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCHPRICE
///  _PREDISPATCHPRICE records predispatch prices for each region by period for each predispatch run, including fields to handle the Ancillary Services functionality._
///
/// * Data Set Name: Predispatch
/// * File Name: Region Prices
/// * Data Version: 1
///
/// # Description
///  PREDISPATCHPRICE data is public, so is available to all participants. Source PREDISPATCHPRICE updates with every thirty-minute predispatch run.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionPrices1 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price
    pub eep: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp1: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep1: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp2: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep2: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp3: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep3: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp4: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep4: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp5: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep5: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp6: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep6: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp7: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep7: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp8: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep8: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Regional reference price for this dispatch period
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lowerregrrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchRegionPrices1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "REGION_PRICES".into(),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PREDISPATCH_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period_
///
/// * Data Set Name: Predispatch
/// * File Name: Local Price
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DATETIME
/// * DUID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchLocalPrice1 {
    #[serde(with = "crate::trading_period")]
    pub predispatchseqno: crate::TradingPeriod,
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// A period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period
    pub periodid: Option<String>,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchLocalPrice1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "LOCAL_PRICE".into(),
            version: 1,
        }
    }
}
