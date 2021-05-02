/// # Summary
/// 
/// ## INSTRUCTIONSUBTYPE
///  _Each Dispatch instruction (GD instruct) has a type and subtype. INSTRUCTIONSUBTYPE, together with INSTRUCTIONTYPE, sets out valid instruction types._
/// 
/// * Data Set Name: Gd Instruct
/// * File Name: Instructionsubtype
/// * Data Version: 1
/// 
/// # Description
///  INSTRUCTIONSUBTYPE is public data, and is available to all participants. Source INSTRUCTIONSUBTYPE shows ad hoc updates to market configuration. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INSTRUCTIONSUBTYPEID
/// * INSTRUCTIONTYPEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructionsubtype1 {
    /// Instruction type
    pub instructiontypeid: String,
    /// Subtype for each dispatch instruction type, for example governor off.
    pub instructionsubtypeid: String,
    /// Description of instruction subtype
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for GdInstructInstructionsubtype1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GD_INSTRUCT".into(),
                        table_name: Some("INSTRUCTIONSUBTYPE".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## INSTRUCTIONTYPE
///  _Dispatch instruction (GD instruct) has types and subtypes. INSTRUCTIONTYPE, together with INSTRUCTIONSUBTYPE, sets out valid instruction types._
/// 
/// * Data Set Name: Gd Instruct
/// * File Name: Instructiontype
/// * Data Version: 1
/// 
/// # Description
///  INSTRUCTIONTYPE data is public to all participants. Source INSTRUCTIONTYPE shows ad hoc updates to market configuration. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INSTRUCTIONTYPEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructiontype1 {
    /// Dispatch instruction type for example FCAS service.
    pub instructiontypeid: String,
    /// Description of instruction type
    pub description: Option<String>,
    /// Region id if regional instruction only.
    pub regionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for GdInstructInstructiontype1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GD_INSTRUCT".into(),
                        table_name: Some("INSTRUCTIONTYPE".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GDINSTRUCT
///  _GDINSTRUCT shows all manually issued dispatch instructions for a dispatchable unit. Ancillary Service instructions are to enable and to disable (i.e. 2 separate instructions) a service. Non-conforming units are also instructed via this facility. However, this facility is not the same as the market notice._
/// 
/// * Data Set Name: Gd Instruct
/// * File Name: Gdinstruct
/// * Data Version: 1
/// 
/// # Description
///  Source GDINSTRUCT updates on issue of an instruction by AEMO, with visibility restricted on the day of issue to the relevant participant. All participants have previous days' data available.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructGdinstruct1 {
    /// Dispatchable unit identifier
    pub duid: Option<String>,
    /// Station Identifier
    pub stationid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Instruction ID (sequential number)
    pub id: rust_decimal::Decimal,
    /// Instruction type
    pub instructiontypeid: Option<String>,
    /// Instruction sub type
    pub instructionsubtypeid: Option<String>,
    /// Instruction class
    pub instructionclassid: Option<String>,
    /// Reason
    pub reason: Option<String>,
    /// Instruction target level
    pub instlevel: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorised by
    pub authorisedby: Option<String>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub issuedtime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub targettime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for GdInstructGdinstruct1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "GD_INSTRUCT".into(),
                        table_name: Some("GDINSTRUCT".into()),
                        version: 1,
                    }
                    
    }
}
