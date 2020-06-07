/// Data Set Name: Dispatch
/// File Name: Mnspbidtrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMnspbidtrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Participant that owns unit during effective record period
    participantid: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    linkid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    offersettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offereffectivedate: Option<chrono::NaiveDateTime>,
    /// VersionNo of the bid/offer used
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchMnspbidtrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "MNSPBIDTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Local Price
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchLocalPrice1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    duid: String,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchLocalPrice1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "LOCAL_PRICE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Regionsum
/// Data Version: 4
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchRegionsum4 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    dispatchinterval: rust_decimal::Decimal,
    /// Manual Intervention flag
    intervention: rust_decimal::Decimal,
    /// Demand (less loads)
    totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    availableload: Option<rust_decimal::Decimal>,
    /// 5 minute forecast adjust
    demandforecast: Option<rust_decimal::Decimal>,
    /// Dispatched Generation
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Dispatched Load (add to total demand to get inherent region demand).
    dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    netinterchange: Option<rust_decimal::Decimal>,
    /// MW quantity of excess
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
    /// Not used since Dec 2003. Raise 5 min MW imported
    raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise price of lower 5 min
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
impl crate::GetTable<DispatchRegionsum4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "REGIONSUM".into(),
                        version: 4,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Unit Scada
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitScada1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    duid: String,
    /// Instantaneous MW reading from SCADA at the start of the Dispatch interval
    scadavalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchUnitScada1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "UNIT_SCADA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Constraint
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchConstraint5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    constraintid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    dispatchinterval: rust_decimal::Decimal,
    /// Manual Intervention flag, which, if set (1), causes predispatch to solve twice.
    intervention: rust_decimal::Decimal,
    /// Right hand Side value as used in dispatch.
    rhs: Option<rust_decimal::Decimal>,
    /// $ Value of binding constraint
    marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation in MW
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
}
impl crate::GetTable<DispatchConstraint5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "CONSTRAINT".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Unit Solution
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitSolution2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    duid: String,
    /// Not used
    tradetype: Option<rust_decimal::Decimal>,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    dispatchinterval: Option<rust_decimal::Decimal>,
    /// Intervention flag if intervention run
    intervention: rust_decimal::Decimal,
    /// Connection point identifier for DUID
    connectionpointid: Option<String>,
    /// Dispatch mode for fast start plant (0 to 4).
    dispatchmode: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS<br>* 1 = on<br>* 0 = off
    agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period
    initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period
    totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate used in dispatch (lesser of bid or telemetered rate).
    rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lesser of bid or telemetered rate).
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
    /// Not Used
    downepf: Option<rust_decimal::Decimal>,
    /// Not Used
    upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min
    marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds
    marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds
    marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy
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
    /// Lower Regulation reserve target
    lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag  - see 
    raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag  - see 
    raise60secflags: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag  - see 
    raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag  - see 
    lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag  
    lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  - see 
    lowerregflags: Option<rust_decimal::Decimal>,
    /// RaiseReg availability - minimum of bid and telemetered value
    raiseregavailability: Option<rust_decimal::Decimal>,
    /// RaiseReg enablement max point - minimum of bid and telemetered value
    raiseregenablementmax: Option<rust_decimal::Decimal>,
    /// RaiseReg Enablement Min point - maximum of bid and telemetered value
    raiseregenablementmin: Option<rust_decimal::Decimal>,
    /// Lower Reg availability - minimum of bid and telemetered value
    lowerregavailability: Option<rust_decimal::Decimal>,
    /// Lower Reg enablement Max point - minimum of bid and telemetered value
    lowerregenablementmax: Option<rust_decimal::Decimal>,
    /// Lower Reg Enablement Min point - maximum of bid and telemetered value
    lowerregenablementmin: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    raise5minactualavailability: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchUnitSolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "UNIT_SOLUTION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Blocked Constraints
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchBlockedConstraints1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    constraintid: String,
}
impl crate::GetTable<DispatchBlockedConstraints1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "BLOCKED_CONSTRAINTS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Price
/// Data Version: 4
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchPrice4 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Dispatch interval identifier 001 to 288 in format YYYYMMDDPPP
    dispatchinterval: String,
    /// Manual intervention flag
    intervention: rust_decimal::Decimal,
    /// Regional Reference Price for this dispatch period. RRP is the price used to settle the market
    rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price - no longer used
    eep: Option<rust_decimal::Decimal>,
    /// Regional Override Price, being the original price prior to any price scaling, price capping or VoLL override being applied. The APC flag allows the determination of whether capping, scaling or override occurred
    rop: Option<rust_decimal::Decimal>,
    /// APC Active flag (see note)
    apcflag: Option<rust_decimal::Decimal>,
    /// Market suspended flag
    marketsuspendedflag: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp; 
    raise6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raise5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raiseregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raiseregrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    raiseregapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lower5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lowerregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lowerregrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    lowerregapcflag: Option<rust_decimal::Decimal>,
    /// Status of regional prices for this dispatch interval "NOT FIRM" or "FIRM"
    price_status: Option<String>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    cumul_pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    cumul_pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    cumul_pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    cumul_pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    cumul_pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchPrice4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "PRICE".into(),
                        version: 4,
                    }
                    
    }
}
/// Data Set Name: Priceload
/// File Name: Constraintrelaxation
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintrelaxation1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no
    runno: rust_decimal::Decimal,
    /// Constraint identifier
    constraintid: String,
    /// Relaxed RHS used in attempt to avoid constraint violation
    rhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Version Number
    versionno: rust_decimal::Decimal,
}
impl crate::GetTable<PriceloadConstraintrelaxation1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRICELOAD".into(),
                        table_name: "CONSTRAINTRELAXATION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Unit Conformance
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitConformance1 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    duid: String,
    /// Dispatch Target - MW
    totalcleared: Option<rust_decimal::Decimal>,
    /// Unit output measured at the conclusion of the dispatch interval - MW (MWB)
    actualmw: Option<rust_decimal::Decimal>,
    /// Rate of Change in direction of error MW per hour
    roc: Option<rust_decimal::Decimal>,
    /// Offered unit capacity - MW (MWO)
    availability: Option<rust_decimal::Decimal>,
    /// Lower Regulation FCAS enabled - MW (FCL)
    lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation FCAS enabled - MW (FCR)
    raisereg: Option<rust_decimal::Decimal>,
    /// Calculated small trigger error limit in MW
    striglm: Option<rust_decimal::Decimal>,
    /// Calculated large trigger error limit in MW
    ltriglm: Option<rust_decimal::Decimal>,
    /// Calculated actual error
    mwerror: Option<rust_decimal::Decimal>,
    /// Max of mwerror while that unit was not in a normal state
    max_mwerror: Option<rust_decimal::Decimal>,
    /// Large trigger error count. Reset when mwerror changes sign
    lecount: Option<i64>,
    /// Small trigger error count.  Reset when mwerror changes sign
    secount: Option<i64>,
    /// Unit conformance status.<br>NORMAL<br>OFF-TARGET<br>NOT-RESPONDING<br>NC-PENDING<br>NON-CONFORMING<br>SUSPENDED
    status: Option<String>,
    /// Participant action required in response to current STATUS
    participant_status_action: Option<String>,
    /// conformance operating mode<br>MANUAL<br>AUTO
    operating_mode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchUnitConformance1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "UNIT_CONFORMANCE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Negative Residue
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchNegativeResidue1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    nrm_datetime: chrono::NaiveDateTime,
    /// Negative residue related direction interconnector id
    directional_interconnectorid: String,
    /// Is 1 if negative residue process is on, else is 0
    nrm_activated_flag: Option<rust_decimal::Decimal>,
    /// Negative residue triggering amount
    cumul_negresidue_amount: Option<rust_decimal::Decimal>,
    /// Previous trading interval cumulative negative residue amount
    cumul_negresidue_prev_ti: Option<rust_decimal::Decimal>,
    /// Current trading interval negative residue amount
    negresidue_current_ti: Option<rust_decimal::Decimal>,
    /// The cumulative negative residue for the next trading interval (PD)
    negresidue_pd_next_ti: Option<rust_decimal::Decimal>,
    /// SubjectToReview, Indeterminate, Accepted or Rejected
    price_revision: Option<String>,
    /// Predispatch sequence number
    predispatchseqno: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    event_activated_di: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    event_deactivated_di: Option<chrono::NaiveDateTime>,
    /// Count of the number of DIs not binding by this constraint
    di_notbinding_count: Option<rust_decimal::Decimal>,
    /// Count of the number of DIs violated by this constraint
    di_violated_count: Option<rust_decimal::Decimal>,
    /// 1 if constraint is blocked, else 0
    nrmconstraint_blocked_flag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchNegativeResidue1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "NEGATIVE_RESIDUE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Case Solution
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchCaseSolution2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Intervention flag - refer to package documentation for definition and practical query examples
    intervention: rust_decimal::Decimal,
    /// Overconstrained dispatch indicator: <br>* OCD = detecting over-constrained dispatch<br>* null = no special condition
    casesubtype: Option<String>,
    /// If non-zero indicated one of the following conditions:<br>* 1 = Supply Scarcity, Excess generation or constraint violations<br>* X = Model failure
    solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    spdversion: Option<String>,
    /// Non-Physical Losses algorithm invoked occurred during this run
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
    /// Total of fast start trader profile violations
    totalfaststartviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    totalenergyofferviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag indicating the SCADA status for FCAS Interconnector dead-band. "0" if SCADA Status or requesting Constraint not invoked. "1" if SCADA Status AND requesting Constraint is invoked
    switchruninitialstatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Solution – from PeriodSolution
    switchrunbeststatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Intervention Physical Solution - from PeriodSolution
    switchrunbeststatus_int: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchCaseSolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "CASE_SOLUTION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Interconnectorres
/// Data Version: 3
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnectorres3 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Interconnector identifier
    interconnectorid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    dispatchinterval: rust_decimal::Decimal,
    /// Intervention case or not
    intervention: rust_decimal::Decimal,
    /// Metered MW Flow from SCADA.
    meteredmwflow: Option<rust_decimal::Decimal>,
    /// Target MW Flow for next 5 mins.
    mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    mwlosses: Option<rust_decimal::Decimal>,
    /// Shadow price resulting from thermal or reserve sharing constraints on Interconnector import/export (0 unless binding) - NEMDE Solution InterconnectorSolution element "Price" attribute
    marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation on interconnector constraints
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Calculated export limit applying to energy only.
    exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy only.
    importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust prices between regions.
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
impl crate::GetTable<DispatchInterconnectorres3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "INTERCONNECTORRES".into(),
                        version: 3,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Fcas Req
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchFcasReq2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Intervention Flag
    intervention: rust_decimal::Decimal,
    /// Generic Constraint ID - Join to table GenConData
    genconid: String,
    /// &nbsp; 
    regionid: String,
    /// DUID offered type
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    genconversionno: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    marginalvalue: Option<rust_decimal::Decimal>,
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
impl crate::GetTable<DispatchFcasReq2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "FCAS_REQ".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Intermittent Forecast Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchIntermittentForecastTrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Tracks to INTERMITTENT_DS_RUN.DUID
    duid: String,
    /// Tracks to INTERMITTENT_DS_RUN.ORIGIN, SCADA is written to ORIGIN if no forecast is discovered.
    origin: Option<String>,
    /// Tracks to INTERMITTENT_DS_RUN.FORECAST_PRIORITY - except for -1 and 0, which denote ''Last Target'' and ''SCADA'' respectively
    forecast_priority: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdatetime: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchIntermittentForecastTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "INTERMITTENT_FORECAST_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Priceload
/// File Name: Constraint Fcas Ocd
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintFcasOcd1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: i64,
    /// Intervention 0/1
    intervention: i64,
    /// ConstraintID/GenconID
    constraintid: String,
    /// VersionNo
    versionno: i64,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// RHS from OCD re-run
    rhs: Option<rust_decimal::Decimal>,
    /// marginalvalue from OCD re-run
    marginalvalue: Option<rust_decimal::Decimal>,
    /// The violation degree of this constraint in the solution result
    violationdegree: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PriceloadConstraintFcasOcd1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRICELOAD".into(),
                        table_name: "CONSTRAINT_FCAS_OCD".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Interconnection
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnection1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Intervention case or not
    intervention: rust_decimal::Decimal,
    /// Nominated RegionID from which the energy flows
    from_regionid: String,
    /// Nominated RegionID to which the energy flows
    to_regionid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP
    dispatchinterval: Option<rust_decimal::Decimal>,
    /// Inter-Regional Loss Factor. Calculated based on the MWFLOW and the nominal From and To Region losses.  
    irlf: Option<rust_decimal::Decimal>,
    /// Summed MW flow of the parallel regulated Interconnectors
    mwflow: Option<rust_decimal::Decimal>,
    /// Summed Metered MW flow of the parallel regulated Interconnectors
    meteredmwflow: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal From Region
    from_region_mw_losses: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal To Region
    to_region_mw_losses: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchInterconnection1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "INTERCONNECTION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Mr Schedule Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMrScheduleTrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Unique RegionID; Key reference to MR_Event_Schedule
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    mr_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    version_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchMrScheduleTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "MR_SCHEDULE_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Dispatch
/// File Name: Offertrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchOffertrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    duid: String,
    /// Bid type Identifier - the ancillary service to which the bid applies
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    bidofferdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchOffertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: "OFFERTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Priceload
/// File Name: Price Revision
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadPriceRevision1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    runno: rust_decimal::Decimal,
    /// Manual intervention flag; always 0
    intervention: rust_decimal::Decimal,
    /// Affected Region Identifier
    regionid: String,
    /// Affected Bid Type Identifier
    bidtype: String,
    /// Version No of price revision for this settlement date
    versionno: i64,
    /// New RRP in DISPATCHPRICE table
    rrp_new: Option<rust_decimal::Decimal>,
    /// Old RRP from DISPATCHPRICE table
    rrp_old: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PriceloadPriceRevision1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRICELOAD".into(),
                        table_name: "PRICE_REVISION".into(),
                        version: 1,
                    }
                    
    }
}
