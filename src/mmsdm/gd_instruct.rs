/// Data Set Name: Gd Instruct
/// File Name: Instructiontype
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructiontype1 {
    /// Dispatch instruction type for example FCAS service.
    instructiontypeid: String,
    /// Description of instruction type
    description: Option<String>,
    /// Region id if regional instruction only.
    regionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GdInstructInstructiontype1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GD_INSTRUCT".into(),
                        table_name: "INSTRUCTIONTYPE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Gd Instruct
/// File Name: Instructionsubtype
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructionsubtype1 {
    /// Instruction type
    instructiontypeid: String,
    /// Subtype for each dispatch instruction type, for example governor off.
    instructionsubtypeid: String,
    /// Description of instruction subtype
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GdInstructInstructionsubtype1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GD_INSTRUCT".into(),
                        table_name: "INSTRUCTIONSUBTYPE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Gd Instruct
/// File Name: Gdinstruct
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructGdinstruct1 {
    /// Dispatchable unit identifier
    duid: Option<String>,
    /// Station Identifier
    stationid: Option<String>,
    /// Region Identifier
    regionid: Option<String>,
    /// Instruction ID (sequential number)
    id: rust_decimal::Decimal,
    /// Instruction type
    instructiontypeid: Option<String>,
    /// Instruction sub type
    instructionsubtypeid: Option<String>,
    /// Instruction class
    instructionclassid: Option<String>,
    /// Reason
    reason: Option<String>,
    /// Instruction target level
    instlevel: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorised by
    authorisedby: Option<String>,
    /// Unique participant identifier
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    issuedtime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    targettime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GdInstructGdinstruct1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GD_INSTRUCT".into(),
                        table_name: "GDINSTRUCT".into(),
                        version: 1,
                    }
                    
    }
}
