/// Data Set Name: Voltage Instruction
/// File Name: Instruction
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct VoltageInstructionInstruction2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for reference within AEMO –matches equipment names between NOS and EMS
    ems_id: String,
    /// The NEM id of the participant who owns the equipment
    participantid: Option<String>,
    /// The id of the station where the control equipment resides
    station_id: Option<String>,
    /// The company/participant preferred name of an equipment
    device_id: Option<String>,
    /// One of REACTOR, CAPACITOR, GEN, SVC, TRANS or GRPGEN but may be extended to other types
    device_type: Option<String>,
    /// One of VOLTAGE, TAP, MVAR, SWITCH or COMMIT but may be extended to other types
    control_type: Option<String>,
    /// Instruction for the device, for this interval – null denotes no instruction
    target: Option<rust_decimal::Decimal>,
    /// [0,1] Denotes if the Device is currently conforming
    conforming: Option<rust_decimal::Decimal>,
    /// Verbose summary of instruction
    instruction_summary: Option<String>,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// Order for execution of Instruction
    instruction_sequence: Option<rust_decimal::Decimal>,
    /// Additional information pertaining to a particular instruction, e.g. Previously issued instruction revoked
    additional_notes: Option<String>,
}
impl crate::GetTable<VoltageInstructionInstruction2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "VOLTAGE_INSTRUCTION".into(),
                        table_name: "INSTRUCTION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Voltage Instruction
/// File Name: Track
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct VoltageInstructionTrack2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// Either 'SIGNAL' (childless) or 'INSTRUCTION'
    file_type: Option<String>,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    se_datetime: Option<chrono::NaiveDateTime>,
    /// VDS solver solution category. Valid values: SUCCESS, WARNING, FAILURE
    solution_category: Option<String>,
    /// VDS solver solution status. Valid values: NOACTCNV [Solved with no instructions], NOVIOACT, CONVERGE, UNMANAGE, UNMANCTG, CTGDIV, SENHDIV [Failed with too many violations], BCDIV
    solution_status: Option<String>,
    /// The current VDS operating mode. Valid values: AUTO, AUTO-VERFIED, MANUAL
    operating_mode: Option<String>,
    /// Unstructured code and message from AEMO
    operating_status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    est_expiry: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    est_next_instruction: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<VoltageInstructionTrack2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "VOLTAGE_INSTRUCTION".into(),
                        table_name: "TRACK".into(),
                        version: 2,
                    }
                    
    }
}
