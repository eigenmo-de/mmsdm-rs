/// # Summary
/// 
/// ## GENCONSETINVOKE
///  _GENCONSETINVOKE provides details of invoked and revoked generic constraints. GENCONSETINVOKE is the key table for determining what constraints are active in dispatch, predispatch and PASA.<br>GENCONSETINVOKE also indicates whether constraints are for interconnector limits, ancillary services, etc.<br>_
/// 
/// * Data Set Name: Generic Constraint
/// * File Name: Genconsetinvoke
/// * Data Version: 2
/// 
/// # Description
///  GENCONSETINVOKE is public data. All participants have access to this data. Source GENCONSETINVOKE updates each time a generic constraint is invoked or revoke time is altered. Once past the time, these times cannot be altered. Note The Replica software does not handle the deletion of GENCONSETINVOKE records. To workaround this problem, the field STARTAUTHORISEDBY indicates whether a constraint set invocation is applicable. A non-null value for the STARTAUTHORISEDBY field indicates that the constraint invocation is active. Essentially inactive invocations have a null value for the STARTAUTHORISEDBY field. To remove inactive invocations from queries on the GENCONSETINVOKE table, add the following text to the where clause "and STARTAUTHORISEDBY is not null".
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INVOCATION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintGenconsetinvoke2 {
    /// Abstract unique identifier for the record. Allows Invocations to be modified without affecting PK values
    pub invocation_id: i64,
    #[serde(with = "crate::mms_datetime")]
    pub startdate: chrono::NaiveDateTime,
    /// The first dispatch interval of the invocation being the dispatch interval number starting from1 at 04:05.
    pub startperiod: rust_decimal::Decimal,
    /// Unique generic constraint set identifier
    pub genconsetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Dispatch interval number end
    pub endperiod: Option<rust_decimal::Decimal>,
    /// User authorising invoke, indicating a constraint set invocation is applicable (i.e. non-null). A null value indicates inactive invocation.
    pub startauthorisedby: Option<String>,
    /// user authorising revoke.
    pub endauthorisedby: Option<String>,
    /// 0 is not intervention, 1 is intervention and causes dispatch to solve twice.
    pub intervention: Option<String>,
    /// Constraint type (e.g. ancillary services). This also flags where a constraint is an interconnector or intra-region network limit.
    pub asconstrainttype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startintervaldatetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub endintervaldatetime: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if the constraint set is a system normal (1) or an outage set (0)
    pub systemnormal: Option<String>,
}
impl crate::GetTable<GenericConstraintGenconsetinvoke2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GENERIC_CONSTRAINT".into(),
                        table_name: "GENCONSETINVOKE".into(),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## EMSMASTER
///  _EMSMASTER provides a description of the SCADA measurements that are associated with the SPD_ID points utilised in generic equation RHS terms_
/// 
/// * Data Set Name: Generic Constraint
/// * File Name: Emsmaster
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * SPD_ID
/// * SPD_TYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintEmsmaster1 {
    /// ID defining data source
    pub spd_id: String,
    /// ID describing type of data source
    pub spd_type: String,
    /// The detailed description of the SCADA point associated with the SPD_ID
    pub description: Option<String>,
    /// The Grouping associated with the SPD ID - most often a RegionID
    pub grouping_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GenericConstraintEmsmaster1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GENERIC_CONSTRAINT".into(),
                        table_name: "EMSMASTER".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENCONSET
///  _GENCONSET sets out generic constraint sets that are invoked and revoked, and may contain many generic constraints (GENCONDATA)._
/// 
/// * Data Set Name: Genconset
/// * File Name: Null
/// * Data Version: 1
/// 
/// # Description
///  GENCONSET is public data, and is available to all participants. Source GENCONSET updates as sets are updated by AEMO.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * GENCONID
/// * GENCONSETID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsetNull1 {
    /// Unique ID for the Constraint Set
    pub genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Contraint ID
    pub genconid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconeffdate: Option<chrono::NaiveDateTime>,
    /// Since market start in 1998 these fields have not been used and any data that has been populated in the fields should be ignored
    pub genconversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GenconsetNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GENCONSET".into(),
                        table_name: "NULL".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENERICEQUATIONDESC
///  _GENERICEQUATIONDESC defines a generic equation identifier with a description. The formulation of the generic equation is detailed in GENERICEQUATIONRHS._
/// 
/// * Data Set Name: Geqdesc
/// * File Name: Null
/// * Data Version: 2
/// 
/// # Description
///  GENERICEQUATIONDESC data is public to all participants. Source GENERICEQUATIONDESC updates when new a generic equation is created for the first time. Volume Approximately 100 records per year Note GENERICEQUATIONRHS and GENERICEQUATIONDESC allow commonly used constraint right hand side formulations to be defined as a generic equation. Once defined, the generic equation can be referenced from any Generic constraint RHS formulation defined in GENERICCONSTRAINTRHS.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EQUATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqdescNull2 {
    /// Generic Equation Identifier
    pub equationid: String,
    /// Generic Equation Description
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The device(s) affected by the constraint (e.g. Interconnector, Generator(s) or Cutset) 
    pub impact: Option<String>,
    /// The source of the constraint formulation
    pub source: Option<String>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    pub limittype: Option<String>,
    /// The contingency or reason for the constraint
    pub reason: Option<String>,
    /// Details of the changes made to this version of the generic equation RHS
    pub modifications: Option<String>,
    /// Extra notes on the constraint
    pub additionalnotes: Option<String>,
}
impl crate::GetTable<GeqdescNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GEQDESC".into(),
                        table_name: "NULL".into(),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENERICEQUATIONRHS
///  _GENERICEQUATIONRHS stores the formulation of commonly used Generic Constraint Right Hand Side Equations referenced from Generic Constraint Right Hand Side definitions stored in GENERICCONSTRAINTRHS. The Generic Equation definitions are versioned and the latest effective version is applied to the dispatch process._
/// 
/// * Data Set Name: Geqrhs
/// * File Name: Null
/// * Data Version: 1
/// 
/// # Description
///  GENERICEQUATIONRHS data is public to all participants. Source GENERICEQUATIONRHS updates whenever a generic equation is created or modified. Volume Approximately 1,000 records per year Note GENERICEQUATIONRHS and GENERICEQUATIONDESC allow commonly used constraint right hand side formulations to be defined as a generic equation. Once defined, the generic equation can be referenced from any Generic constraint RHS formulation defined in GENERICCONSTRAINTRHS. To reference a generic equation from a generic constraint RHS definition, specify a SPD_TYPE of ‘X’ and the SPD_ID equivalent to the EQUATIONID field in GENERICEQUATIONRHS.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * EQUATIONID
/// * TERMID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqrhsNull1 {
    /// Generic Equation Identifier
    pub equationid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// The unique identifier for the a equation RHS term
    pub termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    pub groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    pub spd_id: Option<String>,
    /// ID describing type of data source
    pub spd_type: Option<String>,
    /// Multiplier applied to operator result
    pub factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    pub operation: Option<String>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    pub defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    pub parameterterm1: Option<String>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    pub parameterterm2: Option<String>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    pub parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GeqrhsNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GEQRHS".into(),
                        table_name: "NULL".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SPDINTERCONNECTORCONSTRAINT
///  _SPDINTERCONNECTORCONSTRAINT contains details on the interconnector constraint factors used in dispatch, predispatch and STPASA. The details set a LHS value._
/// 
/// * Data Set Name: Spdicc
/// * File Name: Null
/// * Data Version: 1
/// 
/// # Description
///  SPDINTERCONNECTORCONSTRAINT is public data, and is available to all participants. Source SPDINTERCONNECTORCONSTRAINT updates whenever new connection point constraints are created.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * GENCONID
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdiccNull1 {
    /// Interconnector Identifier
    pub interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: String,
    /// Constraint factor
    pub factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SpdiccNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SPDICC".into(),
                        table_name: "NULL".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SPDREGIONCONSTRAINT
///  _SPDREGIONCONSTRAINT contains details on region demand constraint factors used in dispatch. SPDREGIONCONSTRAINTsets a LHS value._
/// 
/// * Data Set Name: Spdrc
/// * File Name: Null
/// * Data Version: 2
/// 
/// # Description
///  SPDREGIONCONSTRAINT is public data, and is available to all participants. Source SPDREGIONCONSTRAINT is updated whenever AEMO creates new regional constraints.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * EFFECTIVEDATE
/// * GENCONID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdrcNull2 {
    /// Region Identifier
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: String,
    /// Constraint factor; one of (-1, 1)
    pub factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// AS Service type - relates to the BidType table; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    pub bidtype: String,
}
impl crate::GetTable<SpdrcNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SPDRC".into(),
                        table_name: "NULL".into(),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENCONDATA
///  _GENCONDATA sets out the generic constraints contained within a generic constraint set invoked in PASA, predispatch and dispatch.<br>Fields enable selective application of invoked constraints in the Dispatch, Predispatch, ST PASA or MT PASA processes.<br>_
/// 
/// * Data Set Name: Gencondata
/// * File Name: Null
/// * Data Version: 6
/// 
/// # Description
///  GENCONDATA is a public data, and is available to all participants. Source GENCONDATA updates as constraint details are updated by AEMO. Note The following fields enable selective application of invoked constraints in the Dispatch, Predispatch, ST PASA or MT PASA processes: ·	 DISPATCH ·	 PREDISPATCH ·	 STPASA ·	 MTPASA The flag P5MIN_SCOPE_OVERRIDE indicates for each constraint whether 5MPD makes use of the default Dispatch (P5MIN_SCOPE_OVERRIDE = NULL) or Pre-dispatch (P5MIN_SCOPE_OVERRIDE = ‘PD’) style RHS definition. GENERICCONSTRAINTRHS stores generic constraint RHS definitions. Constraints without records in GENERICCONSTRAINTRHS only make use of the static RHS defined in the CONSTRAINTVALUE column in GENCONDATA . The default value for the P5MIN_SCOPE_OVERRIDE column is NULL, so constraints existing before implementing the column use the DISPATCH RHS definition by default, as was the case before the implementation of the change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * GENCONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GencondataNull6 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    pub versionno: rust_decimal::Decimal,
    /// Unique ID for the constraint
    pub genconid: String,
    /// The logical operator (=, &gt;=, &lt;=)
    pub constrainttype: Option<String>,
    /// the RHS value used if there is no dynamic RHS defined in GenericConstraintRHS
    pub constraintvalue: Option<rust_decimal::Decimal>,
    /// Detail of the plant that is not in service
    pub description: Option<String>,
    /// Not used
    pub status: Option<String>,
    /// The constraint violation penalty factor
    pub genericconstraintweight: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Not used
    pub dynamicrhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag: constraint RHS used for Dispatch? 1-used, 0-not used
    pub dispatch: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for PreDispatch, 1-used, 0-not used
    pub predispatch: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for ST PASA, 1-used, 0-not used
    pub stpasa: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for MT PASA, 1-used, 0-not used
    pub mtpasa: Option<String>,
    /// The device(s) that is affected by the constraint e.g. Interconnector, Generator(s) or Cutset
    pub impact: Option<String>,
    /// The source of the constraint formulation
    pub source: Option<String>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    pub limittype: Option<String>,
    /// The contingency or reason for the constraint
    pub reason: Option<String>,
    /// Details of the changes made to this version of the constraint
    pub modifications: Option<String>,
    /// Extra notes on the constraint
    pub additionalnotes: Option<String>,
    /// Extra notes on the constraint: NULL = Dispatch RHS applied in 5MPD, PD = PreDispatch RHS applied in 5MPD
    pub p5min_scope_override: Option<String>,
    /// Flag to indicate if PASA LRC run uses the constraint; 1-used, 0-not used
    pub lrc: Option<String>,
    /// Flag to indicate if PASA LOR run uses the constraint; 1-used, 0-not used
    pub lor: Option<String>,
    /// Flags Constraints for which NEMDE must use "InitialMW" values instead of "WhatOfInitialMW" for Intervention Pricing runs
    pub force_scada: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<GencondataNull6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GENCONDATA".into(),
                        table_name: "NULL".into(),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENCONSETTRK
///  _GENCONSETTRK assists in determining the correct version of a generic constraint set that has been invoked in GENCONSETINVOKE._
/// 
/// * Data Set Name: Genconsettrk
/// * File Name: Null
/// * Data Version: 2
/// 
/// # Description
///  GENCONSETTRK data is public to all participants. Source Ad hoc updates occur to GENCONSETTRK.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * GENCONSETID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsettrkNull2 {
    /// Unique ID for the Constraint Set
    pub genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// Description of the constraint
    pub description: Option<String>,
    /// The person who authorised the constraint set
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The region the constraint set is located in or a special grouping (e.g. CHIMERA)
    pub coverage: Option<String>,
    /// Details of the changes made to this version of the constraint set
    pub modifications: Option<String>,
    /// Not used as of 2005 End of Year Release [was Flag to indicate if the constraint set is a system normal (1) or and an outage set (0)]
    pub systemnormal: Option<String>,
    /// Detail of the plant that is not in service
    pub outage: Option<String>,
}
impl crate::GetTable<GenconsettrkNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GENCONSETTRK".into(),
                        table_name: "NULL".into(),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENERICCONSTRAINTRHS
///  _GENERICCONSTRAINTRHS sets out details of generic constraint Right Hand Side (RHS) formulations for dispatch (DS), predispatch (PD) and Short Term PASA (ST). GENERICCONSTRAINTRHS also includes general expressions (EQ) used in the dispatch, predispatch and PASA time frames.<br>GENERICCONSTRAINTRHS replaces data previously available via the "Constraint Library” Excel spreadsheet.<br>_
/// 
/// * Data Set Name: Gcrhs
/// * File Name: Null
/// * Data Version: 1
/// 
/// # Description
///  GENERICCONSTRAINTRHS is public data, and is available to all participants. Source GENERICCONSTRAINTRHS updates whenever a new generic constraint RHS or expression is created or modified Volume Approximately 70,000 records per year Note GENERICEQUATIONRHS and GENERICEQUATIONDESC allow commonly used constraint right hand side formulations to be defined as a generic equation. Once defined, the generic equation can be referenced from any Generic constraint RHS formulation defined in GENERICCONSTRAINTRHS.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * GENCONID
/// * SCOPE
/// * TERMID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GcrhsNull1 {
    /// Generic Constraint Identifier
    pub genconid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Scope of RHS term (DS, PD, ST or EQ) 
    pub scope: String,
    /// The unique identifier for the a constraint RHS term
    pub termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    pub groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    pub spd_id: Option<String>,
    /// ID describing type of data source
    pub spd_type: Option<String>,
    /// Multiplier applied to operator result
    pub factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    pub operation: Option<String>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    pub defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    pub parameterterm1: Option<String>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    pub parameterterm2: Option<String>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    pub parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GcrhsNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GCRHS".into(),
                        table_name: "NULL".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SPDCONNECTIONPOINTCONSTRAINT
///  _SPDCONNECTIONPOINTCONSTRAINT sets out details of connections point constraints issued in dispatch, predispatch and STPASA._
/// 
/// * Data Set Name: Spdcpc
/// * File Name: Null
/// * Data Version: 2
/// 
/// # Description
///  The addition of the BIDTYPE field to SPDCONNECTIONPOINTCONSTRAINT allows constraints to be applied to a dispatchable unit energy and/or Frequency Controlled Ancillary Services dispatch. SPDCONNECTIONPOINTCONSTRAINTdata is public, so is available to all participants. Source SPDCONNECTIONPOINTCONSTRAINT updates whenever new connection point constraints are created.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * GENCONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdcpcNull2 {
    /// Connection Point Identifier
    pub connectionpointid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: String,
    /// Constraint factor
    pub factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Bid Type Identifier; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    pub bidtype: String,
}
impl crate::GetTable<SpdcpcNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SPDCPC".into(),
                        table_name: "NULL".into(),
                        version: 2,
                    }
                    
    }
}
