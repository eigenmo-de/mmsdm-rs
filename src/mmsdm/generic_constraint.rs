/// Data Set Name: Geqrhs
/// File Name: Null
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqrhsNull1 {
    /// Generic Equation Identifier
    equationid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    versionno: rust_decimal::Decimal,
    /// The unique identifier for the a equation RHS term
    termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    spd_id: Option<String>,
    /// ID describing type of data source
    spd_type: Option<String>,
    /// Multiplier applied to operator result
    factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    operation: Option<String>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    parameterterm1: Option<String>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    parameterterm2: Option<String>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Gcrhs
/// File Name: Null
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GcrhsNull1 {
    /// Generic Constraint Identifier
    genconid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    versionno: rust_decimal::Decimal,
    /// Scope of RHS term (DS, PD, ST or EQ) 
    scope: String,
    /// The unique identifier for the a constraint RHS term
    termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    spd_id: Option<String>,
    /// ID describing type of data source
    spd_type: Option<String>,
    /// Multiplier applied to operator result
    factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    operation: Option<String>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    parameterterm1: Option<String>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    parameterterm2: Option<String>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Spdicc
/// File Name: Null
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdiccNull1 {
    /// Interconnector Identifier
    interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    genconid: String,
    /// Constraint factor
    factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Genconset
/// File Name: Null
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsetNull1 {
    /// Unique ID for the Constraint Set
    genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    versionno: rust_decimal::Decimal,
    /// Generic Contraint ID
    genconid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    genconeffdate: Option<chrono::NaiveDateTime>,
    /// Since market start in 1998 these fields have not been used and any data that has been populated in the fields should be ignored
    genconversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Spdcpc
/// File Name: Null
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdcpcNull2 {
    /// Connection Point Identifier
    connectionpointid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    genconid: String,
    /// Constraint factor
    factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Bid Type Identifier; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    bidtype: String,
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
/// Data Set Name: Genconsettrk
/// File Name: Null
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsettrkNull2 {
    /// Unique ID for the Constraint Set
    genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    versionno: rust_decimal::Decimal,
    /// Description of the constraint
    description: Option<String>,
    /// The person who authorised the constraint set
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The region the constraint set is located in or a special grouping (e.g. CHIMERA)
    coverage: Option<String>,
    /// Details of the changes made to this version of the constraint set
    modifications: Option<String>,
    /// Not used as of 2005 End of Year Release [was Flag to indicate if the constraint set is a system normal (1) or and an outage set (0)]
    systemnormal: Option<String>,
    /// Detail of the plant that is not in service
    outage: Option<String>,
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
/// Data Set Name: Geqdesc
/// File Name: Null
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqdescNull2 {
    /// Generic Equation Identifier
    equationid: String,
    /// Generic Equation Description
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The device(s) affected by the constraint (e.g. Interconnector, Generator(s) or Cutset) 
    impact: Option<String>,
    /// The source of the constraint formulation
    source: Option<String>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    limittype: Option<String>,
    /// The contingency or reason for the constraint
    reason: Option<String>,
    /// Details of the changes made to this version of the generic equation RHS
    modifications: Option<String>,
    /// Extra notes on the constraint
    additionalnotes: Option<String>,
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
/// Data Set Name: Generic Constraint
/// File Name: Emsmaster
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintEmsmaster1 {
    /// ID defining data source
    spd_id: String,
    /// ID describing type of data source
    spd_type: String,
    /// The detailed description of the SCADA point associated with the SPD_ID
    description: Option<String>,
    /// The Grouping associated with the SPD ID - most often a RegionID
    grouping_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Spdrc
/// File Name: Null
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdrcNull2 {
    /// Region Identifier
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    genconid: String,
    /// Constraint factor; one of (-1, 1)
    factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// AS Service type - relates to the BidType table; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    bidtype: String,
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
/// Data Set Name: Generic Constraint
/// File Name: Genconsetinvoke
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintGenconsetinvoke2 {
    /// Abstract unique identifier for the record. Allows Invocations to be modified without affecting PK values
    invocation_id: i64,
    #[serde(with = "crate::mms_datetime")]
    startdate: chrono::NaiveDateTime,
    /// The first dispatch interval of the invocation being the dispatch interval number starting from1 at 04:05.
    startperiod: rust_decimal::Decimal,
    /// Unique generic constraint set identifier
    genconsetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    /// Dispatch interval number end
    endperiod: Option<rust_decimal::Decimal>,
    /// User authorising invoke, indicating a constraint set invocation is applicable (i.e. non-null). A null value indicates inactive invocation.
    startauthorisedby: Option<String>,
    /// user authorising revoke.
    endauthorisedby: Option<String>,
    /// 0 is not intervention, 1 is intervention and causes dispatch to solve twice.
    intervention: Option<String>,
    /// Constraint type (e.g. ancillary services). This also flags where a constraint is an interconnector or intra-region network limit.
    asconstrainttype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startintervaldatetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    endintervaldatetime: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if the constraint set is a system normal (1) or an outage set (0)
    systemnormal: Option<String>,
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
/// Data Set Name: Gencondata
/// File Name: Null
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GencondataNull6 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    versionno: rust_decimal::Decimal,
    /// Unique ID for the constraint
    genconid: String,
    /// The logical operator (=, &gt;=, &lt;=)
    constrainttype: Option<String>,
    /// the RHS value used if there is no dynamic RHS defined in GenericConstraintRHS
    constraintvalue: Option<rust_decimal::Decimal>,
    /// Detail of the plant that is not in service
    description: Option<String>,
    /// Not used
    status: Option<String>,
    /// The constraint violation penalty factor
    genericconstraintweight: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    authorisedby: Option<String>,
    /// Not used
    dynamicrhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag: constraint RHS used for Dispatch? 1-used, 0-not used
    dispatch: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for PreDispatch, 1-used, 0-not used
    predispatch: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for ST PASA, 1-used, 0-not used
    stpasa: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for MT PASA, 1-used, 0-not used
    mtpasa: Option<String>,
    /// The device(s) that is affected by the constraint e.g. Interconnector, Generator(s) or Cutset
    impact: Option<String>,
    /// The source of the constraint formulation
    source: Option<String>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    limittype: Option<String>,
    /// The contingency or reason for the constraint
    reason: Option<String>,
    /// Details of the changes made to this version of the constraint
    modifications: Option<String>,
    /// Extra notes on the constraint
    additionalnotes: Option<String>,
    /// Extra notes on the constraint: NULL = Dispatch RHS applied in 5MPD, PD = PreDispatch RHS applied in 5MPD
    p5min_scope_override: Option<String>,
    /// Flag to indicate if PASA LRC run uses the constraint; 1-used, 0-not used
    lrc: Option<String>,
    /// Flag to indicate if PASA LOR run uses the constraint; 1-used, 0-not used
    lor: Option<String>,
    /// Flags Constraints for which NEMDE must use "InitialMW" values instead of "WhatOfInitialMW" for Intervention Pricing runs
    force_scada: Option<rust_decimal::Decimal>,
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
