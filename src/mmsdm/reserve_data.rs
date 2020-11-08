/// # Summary
///
/// ## MTPASA_RESERVELIMIT
///  _MT PASA input table defining a MT PASA Reserve Requirement within a single set. An MT PASA Reserve Requirement can span more than one region._
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit
/// * Data Version: 1
///
/// # Description
///  Source MTPASA_RESERVELIMIT is updated on an ad hoc basis when a new Reserve Requirement is published. Volume ~20 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * RESERVELIMITID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Reservelimit1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve Requirement identifier
    pub reservelimitid: String,
    /// Description of this Reserve Requirement
    pub description: Option<String>,
    /// Right hand side value for this Reserve requirement
    pub rhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for Reservelimit1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("RESERVELIMIT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_RESERVELIMIT_SET
///  _MT PASA input table defining a set of MT PASA Reserve Requirements. Note only one set can be active on a given date._
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit Set
/// * Data Version: 1
///
/// # Description
///  Source MTPASA_RESERVELIMIT_SET is updated on an ad hoc basis when a new Reserve Requirement is published. Volume ~2 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ReservelimitSet1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA LRC Reserve Requirement Set Identifier
    pub reservelimit_set_id: Option<String>,
    /// Description of this set of Reserve Requirements
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising this requirement set
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ReservelimitSet1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("RESERVELIMIT_SET".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESERVE
///  _RESERVE sets out specific reserve requirements for dispatch, predispatch and STPASA, for each half-hour interval by region. Updates show as new versions for a date.<br>_
///
/// * Data Set Name: Reserve Data
/// * File Name: Reserve
/// * Data Version: 1
///
/// # Description
///  Two fields specify Frequency Controlled Ancillary Services requirements for the regulation ancillary services. Another two fields specify the Lack of Reserve levels to be applied in the ST PASA solver.  Change Notice 324 (for the FCAS Constraint enhancements project) means that Dispatch no longer utilises the static FCAS requirements defined in the DELTAMW and RESERVE tables. These tables are replaced with constraint data as a source of FCAS requirements. RESERVE data is public, so is available to all participants. Source RESERVE updates as AEMO updates forecasts, daily.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Reserve1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version No of record for this date, the version of the file loaded to produce these reserve figures
    pub versionno: rust_decimal::Decimal,
    /// Differentiates this region from all other regions
    pub regionid: String,
    /// Market Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Lower 5 minute reserve requirement
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 second reserve requirement
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 second reserve requirement
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 minute reserve requirement
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 second reserve requirement
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 second reserve requirement
    pub raise6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// PASA reserve requirement
    pub pasareserve: Option<rust_decimal::Decimal>,
    /// PASA Load rejection reserve requirement
    pub loadrejectionreservereq: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve requirement
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Lower Regulation reserve requirement
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// PASA Lack of Reserve 1 Level
    pub lor1level: Option<rust_decimal::Decimal>,
    /// PASA Lack of Reserve 1 Level
    pub lor2level: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for Reserve1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "RESERVE_DATA".into(),
            table_name: Some("RESERVE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MTPASA_RESERVELIMIT_REGION
///  _MT PASA input table to define the regions that are part of a single MT PASA Reserve Requirement_
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit Region
/// * Data Version: 1
///
/// # Description
///  Source MTPASA_RESERVELIMIT_REGION is updated on an ad hoc basis when a new Reserve Requirement is published. Volume ~50 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * RESERVELIMITID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ReservelimitRegion1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve requirement identifier
    pub reservelimitid: String,
    /// Region ID - identifier of a NEM region included in this requirement
    pub regionid: String,
    /// Coefficient for the region in this reserve requirement
    pub coef: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ReservelimitRegion1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("RESERVELIMIT_REGION".into()),
            version: 1,
        }
    }
}
