/// Data Set Name: P5min
/// File Name: Casesolution
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minCasesolution2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// Date and Time of first interval in study
    startinterval_datetime: Option<String>,
    /// The Objective function from the LP
    totalobjective: Option<rust_decimal::Decimal>,
    /// Flag to indicate non-physical losses occurred in this study
    nonphysicallosses: Option<rust_decimal::Decimal>,
    /// Sum of Regional Energy balance violations
    totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Sum of Interconnector violations of standing data limits
    totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Sum of Generic Constraint violations
    totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Sum of Unit Ramp Rate violations
    totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit capacity violations
    totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 5 min FCAS violations
    total5minviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional regulation FCAS violations
    totalregviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 6 sec FCAS violations
    total6secviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 60 sec FCAS violations
    total60secviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit energy constrained violations
    totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit offer violations
    totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit FCAS profile offer violations
    totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit Fast start profile violations
    totalfaststartviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minCasesolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: "CASESOLUTION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: P5min
/// File Name: Unitsolution
/// Data Version: 3
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minUnitsolution3 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    duid: String,
    /// Connection point identifier for DUID
    connectionpointid: Option<String>,
    /// Generator or Load
    tradetype: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS: 1 = on, 0 = off
    agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run.
    initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period
    totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate (lessor of bid or telemetered rate).
    rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lessor of bid or telemetered rate).
    rampuprate: Option<rust_decimal::Decimal>,
    /// Lower 5 min reserve target
    lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec reserve target
    lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec reserve target
    lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min reserve target
    raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec reserve target
    raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec reserve target
    raise6sec: Option<rust_decimal::Decimal>,
    /// Lower Regulation reserve target
    lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    raisereg: Option<rust_decimal::Decimal>,
    /// Energy Availability (MW)
    availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag 
    raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag  
    raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag  
    raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag  
    raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag 
    lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag  
    lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  
    lowerregflags: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Boolean representation flagging if the Target is Capped
    semidispatchcap: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minUnitsolution3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: "UNITSOLUTION".into(),
                        version: 3,
                    }
                    
    }
}
/// Data Set Name: P5min
/// File Name: Local Price
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minLocalPrice1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    duid: String,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minLocalPrice1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: "LOCAL_PRICE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: P5min
/// File Name: Constraintsolution
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minConstraintsolution6 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    constraintid: String,
    /// Right Hand Side value in the capacity evaluation
    rhs: Option<rust_decimal::Decimal>,
    /// Marginal cost of constraint (&gt;0 if binding)
    marginalvalue: Option<rust_decimal::Decimal>,
    /// Amount of Violation (&gt;0 if  violating)
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    lhs: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minConstraintsolution6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: "CONSTRAINTSOLUTION".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: P5min
/// File Name: Interconnectorsoln
/// Data Version: 4
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minInterconnectorsoln4 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// SCADA MW Flow measured at Run start. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run.
    meteredmwflow: Option<rust_decimal::Decimal>,
    /// Cleared Interconnector loading level (MW)
    mwflow: Option<rust_decimal::Decimal>,
    /// Interconnector Losses at cleared flow
    mwlosses: Option<rust_decimal::Decimal>,
    /// Marginal cost of Interconnector standing data limits (if binding)
    marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation of Interconnector standing data limits
    violationdegree: Option<rust_decimal::Decimal>,
    /// Flag indicating MNSP registration
    mnsp: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor at the cleared flow
    marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    exportgenconid: Option<String>,
    /// Generic Constraint setting the import limit
    importgenconid: Option<String>,
    /// Calculated export limit applying to energy + Frequency Controlled Ancillary Services.
    fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + Frequency Controlled Ancillary Services.
    fcasimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    locally_constrained_import: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minInterconnectorsoln4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: "INTERCONNECTORSOLN".into(),
                        version: 4,
                    }
                    
    }
}
/// Data Set Name: P5min
/// File Name: Blocked Constraints
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minBlockedConstraints1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    constraintid: String,
}
impl crate::GetTable<P5minBlockedConstraints1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: "BLOCKED_CONSTRAINTS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: P5min
/// File Name: Regionsolution
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minRegionsolution5 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    regionid: String,
    /// Region Reference Price (Energy)
    rrp: Option<rust_decimal::Decimal>,
    /// Region Override Price (Energy)
    rop: Option<rust_decimal::Decimal>,
    /// Total Energy Imbalance (MW)
    excessgeneration: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise6Sec)
    raise6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise6Sec)
    raise6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise60Sec)
    raise60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise60Sec)
    raise60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise5Min)
    raise5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise5Min)
    raise5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (RaiseReg)
    raiseregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (RaiseReg)
    raiseregrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower6Sec)
    lower6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower6Sec)
    lower6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower60Sec)
    lower60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower60Sec)
    lower60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower5Min)
    lower5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower5Min)
    lower5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (LowerReg)
    lowerregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (LowerReg)
    lowerregrop: Option<rust_decimal::Decimal>,
    /// Regional Demand - NB NOT net of Interconnector flows or Loads
    totaldemand: Option<rust_decimal::Decimal>,
    /// Regional Available generation
    availablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Available Load
    availableload: Option<rust_decimal::Decimal>,
    /// Predicted change in regional demand for this interval
    demandforecast: Option<rust_decimal::Decimal>,
    /// Regional Generation Dispatched
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Load Dispatched
    dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector Flows
    netinterchange: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Raise 5 min MW dispatch
    raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    raise60secdispatch: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minRegionsolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: "REGIONSOLUTION".into(),
                        version: 5,
                    }
                    
    }
}
