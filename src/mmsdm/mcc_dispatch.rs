/// Data Set Name: Mcc
/// File Name: Constraintsolution
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccConstraintsolution1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    constraintid: String,
    /// Generic Constraint RHS Value for this MCC run
    rhs: Option<rust_decimal::Decimal>,
    /// Generic Constraint Marginal Value for this MCC run
    marginalvalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<MccConstraintsolution1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MCC".into(),
                        table_name: "CONSTRAINTSOLUTION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Mcc
/// File Name: Casesolution
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccCasesolution1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
}
impl crate::GetTable<MccCasesolution1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MCC".into(),
                        table_name: "CASESOLUTION".into(),
                        version: 1,
                    }
                    
    }
}
