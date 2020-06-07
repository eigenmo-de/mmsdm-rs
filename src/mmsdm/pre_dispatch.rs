/// Data Set Name: Predispatch
/// File Name: Constraint Solution
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchConstraintSolution5 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: Option<String>,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    runno: Option<rust_decimal::Decimal>,
    /// Generic constraint identifier
    constraintid: String,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
    /// RHS value used.
    rhs: Option<rust_decimal::Decimal>,
    /// Marginal value of violated constraint
    marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of constraint violation
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    lhs: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchConstraintSolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "CONSTRAINT_SOLUTION".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Blocked Constraints
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchBlockedConstraints1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: String,
    /// Generic Constraint identifier (synonymous with GenConID)
    constraintid: String,
}
impl crate::GetTable<PredispatchBlockedConstraints1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "BLOCKED_CONSTRAINTS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Pricesensitivities
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchPricesensitivities1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: Option<String>,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    regionid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 1
    rrpeep1: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 2
    rrpeep2: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 3
    rrpeep3: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 4
    rrpeep4: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 5
    rrpeep5: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 6
    rrpeep6: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 7
    rrpeep7: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 8
    rrpeep8: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 9
    rrpeep9: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 10
    rrpeep10: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 11
    rrpeep11: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 12
    rrpeep12: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 13
    rrpeep13: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 14
    rrpeep14: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 15
    rrpeep15: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 16
    rrpeep16: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 17
    rrpeep17: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 18
    rrpeep18: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 19
    rrpeep19: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 20
    rrpeep20: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 21
    rrpeep21: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 22
    rrpeep22: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 23
    rrpeep23: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 24
    rrpeep24: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 25
    rrpeep25: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 26
    rrpeep26: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 27
    rrpeep27: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 28
    rrpeep28: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    /// Regional Energy Price for scenario 29
    rrpeep29: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 30
    rrpeep30: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 31
    rrpeep31: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 32
    rrpeep32: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 33
    rrpeep33: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 34
    rrpeep34: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 35
    rrpeep35: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this period has an active intervention constraint: 0= No, 1= Yes
    intervention_active: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 36
    rrpeep36: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 37
    rrpeep37: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 38
    rrpeep38: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 39
    rrpeep39: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 40
    rrpeep40: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 41
    rrpeep41: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 42
    rrpeep42: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchPricesensitivities1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "PRICESENSITIVITIES".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Scenario Demand
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemand1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    versionno: i64,
    /// The scenario identifier.
    scenario: i64,
    /// The region to which to apply the deltaMW for this SCENARIO.
    regionid: String,
    /// The MW offset that is applied for this scenario
    deltamw: Option<i64>,
}
impl crate::GetTable<PredispatchScenarioDemand1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "SCENARIO_DEMAND".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Scenario Demand Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemandTrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    versionno: i64,
    /// The user that authorised the scenario update
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchScenarioDemandTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "SCENARIO_DEMAND_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Local Price
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchLocalPrice1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: String,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    duid: String,
    /// A period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period
    periodid: Option<String>,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    locally_constrained: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchLocalPrice1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "LOCAL_PRICE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Regionfcasrequirement
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionfcasrequirement2 {
    /// PreDispatch Sequence number 
    predispatchseqno: Option<String>,
    /// Case Run number
    runno: Option<rust_decimal::Decimal>,
    /// Intervention Flag
    intervention: Option<rust_decimal::Decimal>,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    periodid: Option<String>,
    /// Generic Constraint ID - Join to table GenConData
    genconid: String,
    /// Region ID
    regionid: String,
    /// Bid Type Identifier
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    genconversionno: Option<rust_decimal::Decimal>,
    /// Marginal Value of generic constraint
    marginalvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The base cost of the constraint for this service, before the regulation/contingency split
    base_cost: Option<rust_decimal::Decimal>,
    /// The adjusted cost of the constraint for this service, before the regulation/contingency split
    adjusted_cost: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CMPF, based on dispatched data
    estimated_cmpf: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CRMPF, based on dispatched data
    estimated_crmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CMPF based recovery
    recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CRMPF based recovery
    recovery_factor_crmpf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchRegionfcasrequirement2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "REGIONFCASREQUIREMENT".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Interconnector Soln
/// Data Version: 3
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectorSoln3 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: Option<String>,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    runno: Option<rust_decimal::Decimal>,
    /// Interconnector identifier
    interconnectorid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
    /// Metered MW Flow from EMS. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run.
    meteredmwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Flow
    mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    mwlosses: Option<rust_decimal::Decimal>,
    /// $ Marginal value of interconnector constraint from SPD
    marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation of interconnector constraint in MW
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    /// Calculated export limit.
    exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit.
    importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust bids between reports.
    marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    exportgenconid: Option<String>,
    /// Generic Constraint setting the import limit
    importgenconid: Option<String>,
    /// Calculated export limit applying to energy + FCAS.
    fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + FCAS.
    fcasimportlimit: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    locally_constrained_import: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchInterconnectorSoln3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "INTERCONNECTOR_SOLN".into(),
                        version: 3,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Region Solution
/// Data Version: 4
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionSolution4 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: Option<String>,
    /// LP Solver Pre-Dispatch run no, typically 1. It increments if the case is re-run.
    runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    regionid: String,
    /// PERIODID is just a period count, starting from 1 for each Pre-Dispatch run. Use DATETIME to determine half hour period.
    periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
    /// Total demand in MW for period (less normally on loads)
    totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    availableload: Option<rust_decimal::Decimal>,
    /// Delta MW value only
    demandforecast: Option<rust_decimal::Decimal>,
    /// Generation dispatched in period
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Load dispatched in period
    dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    netinterchange: Option<rust_decimal::Decimal>,
    /// Excess generation in period / Deficit generation if VOLL
    excessgeneration: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 5 min
    lower5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 5 min
    lower5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 5 min
    lower5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 60 sec
    lower60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 60 sec
    lower60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 60 sec
    lower60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 6 sec
    lower6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 6 sec
    lower6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 6 sec
    lower6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW dispatch
    raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003.  Raise 5 min MW imported
    raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 5 min
    raise5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 5 min
    raise5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 5 min
    raise5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 60 sec local dispatch
    raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 60 sec
    raise60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    raise60seclocalreq: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchRegionSolution4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "REGION_SOLUTION".into(),
                        version: 4,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Interconnectr Sens
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectrSens1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: Option<String>,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    runno: Option<rust_decimal::Decimal>,
    /// Unique interconnector identifier
    interconnectorid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this period has an active intervention constraint: 0= No, 1= Yes
    intervention_active: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 1
    mwflow1: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 2
    mwflow2: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 3
    mwflow3: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 4
    mwflow4: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 5
    mwflow5: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 6
    mwflow6: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 7
    mwflow7: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 8
    mwflow8: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 9
    mwflow9: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 10
    mwflow10: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 11
    mwflow11: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 12
    mwflow12: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 13
    mwflow13: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 14
    mwflow14: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 15
    mwflow15: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 16
    mwflow16: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 17
    mwflow17: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 18
    mwflow18: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 19
    mwflow19: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 20
    mwflow20: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 21
    mwflow21: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 22
    mwflow22: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 23
    mwflow23: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 24
    mwflow24: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 25
    mwflow25: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 26
    mwflow26: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 27
    mwflow27: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 28
    mwflow28: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 29
    mwflow29: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 30
    mwflow30: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 31
    mwflow31: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 32
    mwflow32: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 33
    mwflow33: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 34
    mwflow34: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 35
    mwflow35: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 36
    mwflow36: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 37
    mwflow37: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 38
    mwflow38: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 39
    mwflow39: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 40
    mwflow40: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 41
    mwflow41: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 42
    mwflow42: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 43
    mwflow43: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchInterconnectrSens1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "INTERCONNECTR_SENS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Mnspbidtrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchMnspbidtrk1 {
    /// Predispatch run identifier
    predispatchseqno: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    linkid: String,
    /// Trading Interval number
    periodid: String,
    /// Participant Identifier
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    settlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// Version No. for given offer date and settlement date used
    versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchMnspbidtrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "MNSPBIDTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Unit Solution
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchUnitSolution2 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: Option<String>,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    runno: Option<rust_decimal::Decimal>,
    /// Dispatchable unit identifier for fast start
    duid: String,
    /// Not used
    tradetype: Option<rust_decimal::Decimal>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
    /// Connection point identifier
    connectionpointid: Option<String>,
    /// AGC Status from EMS
    agcstatus: Option<rust_decimal::Decimal>,
    /// Dispatch mode of unit for fast start (1-4)
    dispatchmode: Option<rust_decimal::Decimal>,
    /// Initial MW at start of first period. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run.
    initialmw: Option<rust_decimal::Decimal>,
    /// Target MW at end of period
    totalcleared: Option<rust_decimal::Decimal>,
    /// Lower 5 min MW target in period
    lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec MW target in period
    lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec MW target in period
    lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min MW target in period
    raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec MW target in period
    raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec MW target in period
    raise6sec: Option<rust_decimal::Decimal>,
    /// Ramp down rate in period in MW/minute
    rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate in period in MW/minute
    rampuprate: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    downepf: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min from LP Solver
    marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds from LP Solver
    marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds from LP Solver
    marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy from LP Solver
    marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation MW 5 min
    violation5mindegree: Option<rust_decimal::Decimal>,
    /// Violation MW 60 seconds
    violation60secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW 6 seconds
    violation6secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW energy
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    /// Lower Regulation reserve target
    lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag 
    raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag 
    raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag
    raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise reg status flag 
    raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag 
    lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag 
    lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  
    lowerregflags: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    raise5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise reg availability
    raiseregactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 6sec availability
    lower6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 60sec availability
    lower60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 5min availability
    lower5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower reg availability
    lowerregactualavailability: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchUnitSolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "UNIT_SOLUTION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Offertrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchOffertrk1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: String,
    /// Dispatchable Unit identifier
    duid: String,
    /// Bid type Identifier - the ancillary service to which the bid applies
    bidtype: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    periodid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    bidofferdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchOffertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "OFFERTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Region Prices
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionPrices1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: Option<String>,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    regionid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price
    eep: Option<rust_decimal::Decimal>,
    /// Not used
    rrp1: Option<rust_decimal::Decimal>,
    /// Not used
    eep1: Option<rust_decimal::Decimal>,
    /// Not used
    rrp2: Option<rust_decimal::Decimal>,
    /// Not used
    eep2: Option<rust_decimal::Decimal>,
    /// Not used
    rrp3: Option<rust_decimal::Decimal>,
    /// Not used
    eep3: Option<rust_decimal::Decimal>,
    /// Not used
    rrp4: Option<rust_decimal::Decimal>,
    /// Not used
    eep4: Option<rust_decimal::Decimal>,
    /// Not used
    rrp5: Option<rust_decimal::Decimal>,
    /// Not used
    eep5: Option<rust_decimal::Decimal>,
    /// Not used
    rrp6: Option<rust_decimal::Decimal>,
    /// Not used
    eep6: Option<rust_decimal::Decimal>,
    /// Not used
    rrp7: Option<rust_decimal::Decimal>,
    /// Not used
    eep7: Option<rust_decimal::Decimal>,
    /// Not used
    rrp8: Option<rust_decimal::Decimal>,
    /// Not used
    eep8: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    /// Regional reference price for this dispatch period
    raise6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    raise60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    raise5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    raiseregrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lower6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lower60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lower5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lowerregrrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchRegionPrices1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "REGION_PRICES".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Predispatch
/// File Name: Case Solution
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchCaseSolution1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    predispatchseqno: String,
    /// Predispatch run no, normally 1.
    runno: rust_decimal::Decimal,
    /// If non-zero indicated one of the following conditions: 1 = Supply Scarcity, Excess generation or constraint violations, -X = Model failure
    solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    spdversion: Option<String>,
    /// Non-Physical Losses algorithm invoked during this run
    nonphysicallosses: Option<rust_decimal::Decimal>,
    /// The Objective function from the LP
    totalobjective: Option<rust_decimal::Decimal>,
    /// Total Region Demand violations
    totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Total interconnector violations
    totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Total generic constraint violations
    totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Total ramp rate violations
    totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Total unit capacity violations
    totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Total of 5 minute ancillary service region violations
    total5minviolation: Option<rust_decimal::Decimal>,
    /// Total of Regulation ancillary service region violations
    totalregviolation: Option<rust_decimal::Decimal>,
    /// Total of 6 second ancillary service region violations
    total6secviolation: Option<rust_decimal::Decimal>,
    /// Total of 60 second ancillary service region violations
    total60secviolation: Option<rust_decimal::Decimal>,
    /// Total of ancillary service trader profile violations
    totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Total of Energy Constrained unit offer violations.
    totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    totalenergyofferviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Pre-Dispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchCaseSolution1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PREDISPATCH".into(),
                        table_name: "CASE_SOLUTION".into(),
                        version: 1,
                    }
                    
    }
}
