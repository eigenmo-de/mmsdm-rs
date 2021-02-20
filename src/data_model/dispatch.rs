/// # Summary
/// 
/// ## DISPATCHCONSTRAINT
///  _DISPATCHCONSTRAINT sets out details of all binding and interregion constraints in each dispatch run. Note: invoked constraints can be established from GENCONSETINVOKE. Binding constraints show as marginal value &gt;$0. Interconnector constraints are listed so RHS (SCADA calculated limits) can be reported._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Constraint
/// * Data Version: 5
/// 
/// # Description
///  DISPATCHCONSTRAINT is public data, and is available to all participants. Source DISPATCHCONSTRAINT updates every five minutes.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * CONSTRAINTID
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchConstraint5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Manual Intervention flag, which, if set (1), causes predispatch to solve twice.
    pub intervention: rust_decimal::Decimal,
    /// Right hand Side value as used in dispatch.
    pub rhs: Option<rust_decimal::Decimal>,
    /// $ Value of binding constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation in MW
    pub violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    pub duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    pub genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    pub lhs: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchConstraint5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("CONSTRAINT".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_PRICE_REVISION
///  _An audit trail of price changes on the DISPATCHPRICE table (i.e. for 5 minute dispatch prices for energy and FCAS)._
/// 
/// * Data Set Name: Priceload
/// * File Name: Price Revision
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadPriceRevision1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Manual intervention flag; always 0
    pub intervention: rust_decimal::Decimal,
    /// Affected Region Identifier
    pub regionid: String,
    /// Affected Bid Type Identifier
    pub bidtype: String,
    /// Version No of price revision for this settlement date
    pub versionno: i64,
    /// New RRP in DISPATCHPRICE table
    pub rrp_new: Option<rust_decimal::Decimal>,
    /// Old RRP from DISPATCHPRICE table
    pub rrp_old: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PriceloadPriceRevision1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRICELOAD".into(),
                        table_name: Some("PRICE_REVISION".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NEGATIVE_RESIDUE
///  _Shows the inputs provided to the Negative Residue Constraints in the Dispatch horizon_
/// 
/// * Data Set Name: Dispatch
/// * File Name: Negative Residue
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DIRECTIONAL_INTERCONNECTORID
/// * NRM_DATETIME
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchNegativeResidue1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub nrm_datetime: chrono::NaiveDateTime,
    /// Negative residue related direction interconnector id
    pub directional_interconnectorid: String,
    /// Is 1 if negative residue process is on, else is 0
    pub nrm_activated_flag: Option<rust_decimal::Decimal>,
    /// Negative residue triggering amount
    pub cumul_negresidue_amount: Option<rust_decimal::Decimal>,
    /// Previous trading interval cumulative negative residue amount
    pub cumul_negresidue_prev_ti: Option<rust_decimal::Decimal>,
    /// Current trading interval negative residue amount
    pub negresidue_current_ti: Option<rust_decimal::Decimal>,
    /// The cumulative negative residue for the next trading interval (PD)
    pub negresidue_pd_next_ti: Option<rust_decimal::Decimal>,
    /// SubjectToReview, Indeterminate, Accepted or Rejected
    pub price_revision: Option<String>,
    /// Predispatch sequence number
    pub predispatchseqno: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub event_activated_di: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub event_deactivated_di: Option<chrono::NaiveDateTime>,
    /// Count of the number of DIs not binding by this constraint
    pub di_notbinding_count: Option<rust_decimal::Decimal>,
    /// Count of the number of DIs violated by this constraint
    pub di_violated_count: Option<rust_decimal::Decimal>,
    /// 1 if constraint is blocked, else 0
    pub nrmconstraint_blocked_flag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchNegativeResidue1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("NEGATIVE_RESIDUE".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_UNIT_SCADA
///  _Dispatchable unit MW from SCADA at the start of the dispatch interval. The table includes all scheduled and semi-scheduled (and non-scheduled units where SCADA is available)_
/// 
/// * Data Set Name: Dispatch
/// * File Name: Unit Scada
/// * Data Version: 1
/// 
/// # Description
///  DISPATCH_UNIT_SCADA data  is public data, and is available to all participants. Source DISPATCH_UNIT_SCADA shows data for every 5 minutes for all scheduled units Volume Rows per day: 288 per each scheduled, semi-scheduled (and non-scheduled unit where SCADA is available)
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitScada1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Instantaneous MW reading from SCADA at the start of the Dispatch interval
    pub scadavalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchUnitScada1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("UNIT_SCADA".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## CONSTRAINTRELAXATION_OCD
///  _CONSTRAINTRELAXATION_OCD contains details of interconnector constraints and unit ancillary service constraints relaxed in the over-constrained dispatch (OCD) re-run for this interval (if there was one).<br>Note: INTERVENTION is not included in CONSTRAINTRELAXATION_OCD, since the relaxation of the same constraint is the same amount in both intervened and non-intervened cases.<br>_
/// 
/// * Data Set Name: Priceload
/// * File Name: Constraintrelaxation
/// * Data Version: 1
/// 
/// # Description
///  Source The occurrences of Over-Constrained Dispatch (OCD) re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Over-constrained dispatch (OCD) re-run (if there was one). Volume Rows per day: ~2 Mb per month: &lt;1 The estimates on the number of rows are based on a 1% occurrence rate for OCD runs.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * CONSTRAINTID
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintrelaxation1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no
    pub runno: rust_decimal::Decimal,
    /// Constraint identifier
    pub constraintid: String,
    /// Relaxed RHS used in attempt to avoid constraint violation
    pub rhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Version Number
    pub versionno: rust_decimal::Decimal,
}
impl crate::GetTable for PriceloadConstraintrelaxation1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRICELOAD".into(),
                        table_name: Some("CONSTRAINTRELAXATION".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_MR_SCHEDULE_TRK
///  _DISPATCH_MR_SCHEDULE_TRK records the Mandatory Restrictions Acceptance Schedule applied to this dispatch interval for this region.<br>DISPATCH_MR_SCHEDULE_TRK is populated by the Dispatch process and records the MR Offer Stack applied in each dispatch interval. DISPATCH_MR_SCHEDULE_TRK is used by Settlements to calculate payments according to the correct MR offer stack.<br>_
/// 
/// * Data Set Name: Dispatch
/// * File Name: Mr Schedule Trk
/// * Data Version: 1
/// 
/// # Description
///  DISPATCH_MR_SCHEDULE_TRK  data is public to all participants. Source DISPATCH_MR_SCHEDULE_TRK updates are ad hoc. Volume 2 rows per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * REGIONID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMrScheduleTrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Unique RegionID; Key reference to MR_Event_Schedule
    pub regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub mr_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub version_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchMrScheduleTrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("MR_SCHEDULE_TRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHINTERCONNECTORRES
///  _DISPATCHINTERCONNECTORRES sets out MW flow and losses on each interconnector for each dispatch period, including fields for the Frequency Controlled Ancillary Services export and import limits and extra reporting of the generic constraints set the energy import and export limits._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Interconnectorres
/// * Data Version: 3
/// 
/// # Description
///  DISPATCHINTERCONNECTORRES is public data, and is available to all participants. Source DISPATCHINTERCONNECTORRES updates every 5 minutes. Note MW losses can be negative depending on the flow. The definition of direction of flow for an interconnector is that positive flow starts from the FROMREGION in the INTERCONNECTOR table.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DISPATCHINTERVAL
/// * INTERCONNECTORID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnectorres3 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Intervention case or not
    pub intervention: rust_decimal::Decimal,
    /// Metered MW Flow from SCADA.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Target MW Flow for next 5 mins.
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Shadow price resulting from thermal or reserve sharing constraints on Interconnector import/export (0 unless binding) - NEMDE Solution InterconnectorSolution element "Price" attribute
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation on interconnector constraints
    pub violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Calculated export limit applying to energy only.
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy only.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust prices between regions.
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
impl crate::GetTable for DispatchInterconnectorres3 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("INTERCONNECTORRES".into()),
                        version: 3,
                    }
                    
    }
}
/// # Summary
/// 
/// ## INTERMITTENT_FORECAST_TRK
///  _Uniquely tracks which Intermittent Generation forecast was used for the DUID in which Dispatch run_
/// 
/// * Data Set Name: Dispatch
/// * File Name: Intermittent Forecast Trk
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchIntermittentForecastTrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Tracks to INTERMITTENT_DS_RUN.DUID
    pub duid: String,
    /// Tracks to INTERMITTENT_DS_RUN.ORIGIN, SCADA is written to ORIGIN if no forecast is discovered.
    pub origin: Option<String>,
    /// Tracks to INTERMITTENT_DS_RUN.FORECAST_PRIORITY - except for -1 and 0, which denote ''Last Target'' and ''SCADA'' respectively
    pub forecast_priority: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdatetime: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchIntermittentForecastTrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("INTERMITTENT_FORECAST_TRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHLOAD
///  _DISPATCHLOAD set out the current SCADA MW and target MW for each dispatchable unit, including relevant Frequency Control Ancillary Services (FCAS) enabling targets for each five minutes and additional fields to handle the new Ancillary Services functionality. Fast Start Plant status is indicated by dispatch mode._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Unit Solution
/// * Data Version: 2
/// 
/// # Description
///  DISPATCHLOAD data is confidential for the current day, showing own details for participant and becomes public after close of business yesterday, and is available to all participants. Source DISPATCHLOAD shows data for every 5 minutes for all units, even zero targets. Volume Expect 40-50,000 records per day. All units are repeated, even zero targets. Note ** A flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined using the low 3 bits as follows: Flag Name Bit Description Enabled 0 The unit is enabled to provide this ancillary service type. Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. Interpretation of the bit-flags as a number gives the following possibilities (i.e. other combinations are not possible): Numeric Value Bit (2,1,0) Meaning 0 000 Not stranded, not trapped, not enabled (i.e. is unavailable). 1 001 Not stranded, not trapped, is enabled (i.e. available). 3 011 Not stranded, is trapped, is enabled (i.e. trapped). 4 100 Is stranded, not trapped, not enabled (i.e. stranded). For example, testing for availability can be done by checking for odd (=available) or even (=unavailable) number (e.g.  mod(flag,2)  results in 0 for unavailable and 1 for available). *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitSolution2 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Not used
    pub tradetype: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Intervention flag if intervention run
    pub intervention: rust_decimal::Decimal,
    /// Connection point identifier for DUID
    pub connectionpointid: Option<String>,
    /// Dispatch mode for fast start plant (0 to 4).
    pub dispatchmode: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS<br>* 1 = on<br>* 0 = off
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate used in dispatch (lesser of bid or telemetered rate).
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lesser of bid or telemetered rate).
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Lower 5 min reserve target
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec reserve target
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec reserve target
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min reserve target
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec reserve target
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec reserve target
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Not Used
    pub downepf: Option<rust_decimal::Decimal>,
    /// Not Used
    pub upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min
    pub marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds
    pub marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds
    pub marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy
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
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag  - see 
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag  - see 
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag  - see 
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag  - see 
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag  
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  - see 
    pub lowerregflags: Option<rust_decimal::Decimal>,
    /// RaiseReg availability - minimum of bid and telemetered value
    pub raiseregavailability: Option<rust_decimal::Decimal>,
    /// RaiseReg enablement max point - minimum of bid and telemetered value
    pub raiseregenablementmax: Option<rust_decimal::Decimal>,
    /// RaiseReg Enablement Min point - maximum of bid and telemetered value
    pub raiseregenablementmin: Option<rust_decimal::Decimal>,
    /// Lower Reg availability - minimum of bid and telemetered value
    pub lowerregavailability: Option<rust_decimal::Decimal>,
    /// Lower Reg enablement Max point - minimum of bid and telemetered value
    pub lowerregenablementmax: Option<rust_decimal::Decimal>,
    /// Lower Reg Enablement Min point - maximum of bid and telemetered value
    pub lowerregenablementmin: Option<rust_decimal::Decimal>,
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
impl crate::GetTable for DispatchUnitSolution2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("UNIT_SOLUTION".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period. Note that from 2014 Mid year release only records with non-zero Local_Price_Adjustment values are issued_
/// 
/// * Data Set Name: Dispatch
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
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchLocalPrice1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchLocalPrice1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("LOCAL_PRICE".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_INTERCONNECTION
///  _Inter-regional flow information common to or aggregated for regulated (i.e. not MNSP) Interconnectors spanning the From-Region and To-Region - NB only the physical run is calculated'_
/// 
/// * Data Set Name: Dispatch
/// * File Name: Interconnection
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * FROM_REGIONID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
/// * TO_REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnection1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention case or not
    pub intervention: rust_decimal::Decimal,
    /// Nominated RegionID from which the energy flows
    pub from_regionid: String,
    /// Nominated RegionID to which the energy flows
    pub to_regionid: String,
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Inter-Regional Loss Factor. Calculated based on the MWFLOW and the nominal From and To Region losses.  
    pub irlf: Option<rust_decimal::Decimal>,
    /// Summed MW flow of the parallel regulated Interconnectors
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Summed Metered MW flow of the parallel regulated Interconnectors
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal From Region
    pub from_region_mw_losses: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal To Region
    pub to_region_mw_losses: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchInterconnection1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("INTERCONNECTION".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHCASESOLUTION
///  _DISPATCHCASESOLUTION shows information relating to the complete dispatch run. The fields in DISPATCHCASESOLUTION provide an overview of the dispatch run results allowing immediate identification of conditions such as energy or FCAS deficiencies._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Case Solution
/// * Data Version: 2
/// 
/// # Description
///  The DISPATCHCASESOLUTION data is public. Source DISPATCHCASESOLUTION updates every 5 minutes. Volume Approximately 288 records per day.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchCaseSolution2 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention flag - refer to package documentation for definition and practical query examples
    pub intervention: rust_decimal::Decimal,
    /// Overconstrained dispatch indicator: <br>* OCD = detecting over-constrained dispatch<br>* null = no special condition
    pub casesubtype: Option<String>,
    /// If non-zero indicated one of the following conditions:<br>* 1 = Supply Scarcity, Excess generation or constraint violations<br>* X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: Option<String>,
    /// Non-Physical Losses algorithm invoked occurred during this run
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
    /// Total of fast start trader profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag indicating the SCADA status for FCAS Interconnector dead-band. "0" if SCADA Status or requesting Constraint not invoked. "1" if SCADA Status AND requesting Constraint is invoked
    pub switchruninitialstatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Solution – from PeriodSolution
    pub switchrunbeststatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Intervention Physical Solution - from PeriodSolution
    pub switchrunbeststatus_int: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchCaseSolution2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("CASE_SOLUTION".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHOFFERTRK
///  _DISPATCHOFFERTRK is the energy and ancillary service bid tracking table for the Dispatch process. The table identifies which bids from BIDDAYOFFER and BIDPEROFFER were applied for a given unit and bid type for each dispatch interval._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Offertrk
/// * Data Version: 1
/// 
/// # Description
///  DISPATCHOFFERTRK  data is confidential to each participant until the next trading day, when the data is public to all participants.  Source DISPATCHOFFERTRK updates every 5 minutes. Volume Approximately 250,000 records per day.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchOffertrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Bid type Identifier - the ancillary service to which the bid applies
    pub bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidofferdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchOffertrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("OFFERTRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_MNSPBIDTRK
///  _DISPATCH_MNSPBIDTRK shows the MNSP bid tracking, including the bid version used in each dispatch run for each MNSP Interconnector Link. DISPATCH_MNSPBIDTRK is the audit trail of the bids actually used for each dispatch run._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Mnspbidtrk
/// * Data Version: 1
/// 
/// # Description
///  DISPATCH_MNSPBIDTRK shows own details for participant as they occur, with all details until close of business yesterday being available to all participants after end of day. Source DISPATCH_MNSPBIDTRK potentially updates every 5 minutes. Volume 220, 000 per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * LINKID
/// * PARTICIPANTID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMnspbidtrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Participant that owns unit during effective record period
    pub participantid: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offersettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offereffectivedate: Option<chrono::NaiveDateTime>,
    /// VersionNo of the bid/offer used
    pub offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchMnspbidtrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("MNSPBIDTRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_CONSTRAINT_FCAS_OCD
///  _FCAS constraint solution from OCD re-run._
/// 
/// * Data Set Name: Priceload
/// * File Name: Constraint Fcas Ocd
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
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintFcasOcd1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: i64,
    /// Intervention 0/1
    pub intervention: i64,
    /// ConstraintID/GenconID
    pub constraintid: String,
    /// VersionNo
    pub versionno: i64,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// RHS from OCD re-run
    pub rhs: Option<rust_decimal::Decimal>,
    /// marginalvalue from OCD re-run
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// The violation degree of this constraint in the solution result
    pub violationdegree: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PriceloadConstraintFcasOcd1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRICELOAD".into(),
                        table_name: Some("CONSTRAINT_FCAS_OCD".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_FCAS_REQ
///  _DISPATCH_FCAS_REQ shows Dispatch Constraint tracking for Regional FCAS recovery._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Fcas Req
/// * Data Version: 2
/// 
/// # Description
///  DISPATCH_FCAS_REQ is public data and is available to all participants. Source DISPATCH_FCAS_REQ updates with each dispatch run (5 minutes). Volume Approximately 10,000 rows per day
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * GENCONID
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchFcasReq2 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention Flag
    pub intervention: rust_decimal::Decimal,
    /// Generic Constraint ID - Join to table GenConData
    pub genconid: String,
    /// &nbsp; 
    pub regionid: String,
    /// DUID offered type
    pub bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    pub genconversionno: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub marginalvalue: Option<rust_decimal::Decimal>,
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
impl crate::GetTable for DispatchFcasReq2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("FCAS_REQ".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHPRICE
///  _DISPATCHPRICE records 5 minute dispatch prices for energy and FCAS, including whether an intervention has occurred, or price override (e.g. for Administered Price Cap). DISPATCHPRICE updates when price adjustments occur, in which case the new price is written to the RRP field, and the old price to the ROP field as an audit trail._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Price
/// * Data Version: 4
/// 
/// # Description
///  Source DISPATCHPRICE updates every 5 minutes. Note APCFLAG is a 5-bit Region-based field indicating that the original Dispatch Price (ROP) calculated by the Dispatch Algorithm for a region has undergone modification by one of more of the following processes: Bit Value Description 5 16 Price Scaling via Inter-regional Loss Factor (IRLF) 4 8 Price manually overwritten 3 4 MPC or MPF binding (ROP was outside of MPC/MPF) 2 2 VoLL Override applied 1 1 APC or APF binding (ROP was outside of APC/APF) Where: ·	 MPC = Market Price Cap ·	 MPF = Market Price Floor ·	 APC = Administered Price Cap ·	 APF = Administered Price Floor xxxAPCFLAGs are each a 5-bit Region-based field indicating FCAS price post-processing (where "ROP" is the original NEMDE Solver price): Bit Cum Value Description 5 16 Not applicable 4 8 Price manually overwritten 3 4 MPC ($VoLL) or MPF ($zero) binding (xxFCAS ROP was outside of MPC/MPF) 2 2 Not applicable 1 1 APC or APF binding (ROP was outside of APC/APF)
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchPrice4 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Manual intervention flag
    pub intervention: rust_decimal::Decimal,
    /// Regional Reference Price for this dispatch period. RRP is the price used to settle the market
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price - no longer used
    pub eep: Option<rust_decimal::Decimal>,
    /// Regional Override Price, being the original price prior to any price scaling, price capping or VoLL override being applied. The APC flag allows the determination of whether capping, scaling or override occurred
    pub rop: Option<rust_decimal::Decimal>,
    /// APC Active flag (see note)
    pub apcflag: Option<rust_decimal::Decimal>,
    /// Market suspended flag
    pub marketsuspendedflag: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp; 
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raise5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub raiseregapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lower5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub lowerregapcflag: Option<rust_decimal::Decimal>,
    /// Status of regional prices for this dispatch interval "NOT FIRM" or "FIRM"
    pub price_status: Option<String>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    /// Communicates the current OCD status for this dispatch interval.  Values of: 'NOT_OCD', 'OCD_UNRESOLVED', 'OCD_RESOLVED'.
    pub ocd_status: Option<String>,
    /// Communicates the current MII status for this dispatch interval.  Values of: 'NOT_MII', 'MII_SUBJECT_TO_REVIEW', 'MII_PRICE_REJECTED', 'MII_PRICE_ACCEPTED'.
    pub mii_status: Option<String>,
}
impl crate::GetTable for DispatchPrice4 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("PRICE".into()),
                        version: 4,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCH_UNIT_CONFORMANCE
///  _DISPATCH_UNIT_CONFORMANCE details the conformance of a scheduled units operation with respect to a cleared target on dispatch interval basis.<br>Data is confidential_
/// 
/// * Data Set Name: Dispatch
/// * File Name: Unit Conformance
/// * Data Version: 1
/// 
/// # Description
///  DISPATCH_UNIT_CONFORMANCE data is confidential. Source DISPATCH_UNIT_CONFORMANCE shows data for every 5 minutes for all scheduled units Volume Rows per day: 288 per scheduled unit
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * INTERVAL_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitConformance1 {
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Dispatch Target - MW
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Unit output measured at the conclusion of the dispatch interval - MW (MWB)
    pub actualmw: Option<rust_decimal::Decimal>,
    /// Rate of Change in direction of error MW per hour
    pub roc: Option<rust_decimal::Decimal>,
    /// Offered unit capacity - MW (MWO)
    pub availability: Option<rust_decimal::Decimal>,
    /// Lower Regulation FCAS enabled - MW (FCL)
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation FCAS enabled - MW (FCR)
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Calculated small trigger error limit in MW
    pub striglm: Option<rust_decimal::Decimal>,
    /// Calculated large trigger error limit in MW
    pub ltriglm: Option<rust_decimal::Decimal>,
    /// Calculated actual error
    pub mwerror: Option<rust_decimal::Decimal>,
    /// Max of mwerror while that unit was not in a normal state
    pub max_mwerror: Option<rust_decimal::Decimal>,
    /// Large trigger error count. Reset when mwerror changes sign
    pub lecount: Option<i64>,
    /// Small trigger error count.  Reset when mwerror changes sign
    pub secount: Option<i64>,
    /// Unit conformance status.<br>NORMAL<br>OFF-TARGET<br>NOT-RESPONDING<br>NC-PENDING<br>NON-CONFORMING<br>SUSPENDED
    pub status: Option<String>,
    /// Participant action required in response to current STATUS
    pub participant_status_action: Option<String>,
    /// conformance operating mode<br>MANUAL<br>AUTO
    pub operating_mode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchUnitConformance1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("UNIT_CONFORMANCE".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHREGIONSUM
///  _DISPATCHREGIONSUM sets out the 5-minute solution for each dispatch run for each region, including the Frequency Control Ancillary Services (FCAS) services provided. Additional fields are for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations._
/// 
/// * Data Set Name: Dispatch
/// * File Name: Regionsum
/// * Data Version: 4
/// 
/// # Description
///  DISPATCHREGIONSUM is public data, and is available to all participants. Source DISPATCHREGIONSUM updates every 5 minutes. Note For details of calculations about load calculations, refer to Chapter 3 of the "Statement of Opportunities" *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values. From 16 February 2006, the old reserve values are no longer populated (i.e. are null), being LORSurplus and LRCSurplus. For more details on the changes to Reporting of Reserve Condition Data, refer to AEMO Communication 2042. For the best available indicator of reserve condition in each of the regions of the NEM for each trading interval, refer to the latest run of the Pre-Dispatch PASA (see table PDPASA_REGIONSOLUTION).
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchRegionsum4 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Manual Intervention flag
    pub intervention: rust_decimal::Decimal,
    /// Demand (less loads)
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    pub availableload: Option<rust_decimal::Decimal>,
    /// 5 minute forecast adjust
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Dispatched Generation
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Dispatched Load (add to total demand to get inherent region demand).
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// MW quantity of excess
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
    /// Not used since Dec 2003. Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise price of lower 5 min
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
    /// Calculated dispatch error
    pub aggegatedispatcherror: Option<rust_decimal::Decimal>,
    /// Calculated dispatch error
    pub aggregatedispatcherror: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
    /// Not in use after 17 Feb 2006. Total short term generation capacity reserve used in assessing lack of reserve condition
    pub lorsurplus: Option<rust_decimal::Decimal>,
    /// Not in use after 17 Feb 2006. Total short term generation capacity reserve above the stated low reserve condition requirement
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
impl crate::GetTable for DispatchRegionsum4 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("REGIONSUM".into()),
                        version: 4,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHBLOCKEDCONSTRAINT
///  _DISPATCH Blocked Constraints lists any constraints that were blocked in a dispatch run. If no constraints are blocked, there will be no rows for that dispatch run._
/// 
/// * Data Set Name: Dispatch
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
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchBlockedConstraints1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
}
impl crate::GetTable for DispatchBlockedConstraints1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "DISPATCH".into(),
                        table_name: Some("BLOCKED_CONSTRAINTS".into()),
                        version: 1,
                    }
                    
    }
}
