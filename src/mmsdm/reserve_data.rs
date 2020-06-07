/// Data Set Name: Mtpasa
/// File Name: Reservelimit Set
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimitSet1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// MT PASA LRC Reserve Requirement Set Identifier
    reservelimit_set_id: Option<String>,
    /// Description of this set of Reserve Requirements
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising this requirement set
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MtpasaReservelimitSet1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MTPASA".into(),
                        table_name: "RESERVELIMIT_SET".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Reserve Data
/// File Name: Reserve
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ReserveDataReserve1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Version No of record for this date, the version of the file loaded to produce these reserve figures
    versionno: rust_decimal::Decimal,
    /// Differentiates this region from all other regions
    regionid: String,
    /// Market Trading Interval
    periodid: rust_decimal::Decimal,
    /// Lower 5 minute reserve requirement
    lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 second reserve requirement
    lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 second reserve requirement
    lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 minute reserve requirement
    raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 second reserve requirement
    raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 second reserve requirement
    raise6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// PASA reserve requirement
    pasareserve: Option<rust_decimal::Decimal>,
    /// PASA Load rejection reserve requirement
    loadrejectionreservereq: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve requirement
    raisereg: Option<rust_decimal::Decimal>,
    /// Lower Regulation reserve requirement
    lowerreg: Option<rust_decimal::Decimal>,
    /// PASA Lack of Reserve 1 Level
    lor1level: Option<rust_decimal::Decimal>,
    /// PASA Lack of Reserve 1 Level
    lor2level: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ReserveDataReserve1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "RESERVE_DATA".into(),
                        table_name: "RESERVE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Mtpasa
/// File Name: Reservelimit
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimit1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve Requirement identifier
    reservelimitid: String,
    /// Description of this Reserve Requirement
    description: Option<String>,
    /// Right hand side value for this Reserve requirement
    rhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MtpasaReservelimit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MTPASA".into(),
                        table_name: "RESERVELIMIT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Mtpasa
/// File Name: Reservelimit Region
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimitRegion1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve requirement identifier
    reservelimitid: String,
    /// Region ID - identifier of a NEM region included in this requirement
    regionid: String,
    /// Coefficient for the region in this reserve requirement
    coef: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MtpasaReservelimitRegion1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MTPASA".into(),
                        table_name: "RESERVELIMIT_REGION".into(),
                        version: 1,
                    }
                    
    }
}
