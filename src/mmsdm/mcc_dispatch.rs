/// # Summary
/// 
/// ## MCC_CASESOLUTION
///  _Top level table for each MCC dispatch rerun process. Note there will be one record for each dispatch interval_
/// 
/// * Data Set Name: Mcc
/// * File Name: Casesolution
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccCasesolution1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
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
/// # Summary
/// 
/// ## MCC_CONSTRAINTSOLUTION
///  _Constraint solution data from the MCC dispatch rerun process. Note only constraints with a non-zero marginal value are published._
/// 
/// * Data Set Name: Mcc
/// * File Name: Constraintsolution
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * CONSTRAINTID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccConstraintsolution1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// Generic Constraint RHS Value for this MCC run
    pub rhs: Option<rust_decimal::Decimal>,
    /// Generic Constraint Marginal Value for this MCC run
    pub marginalvalue: Option<rust_decimal::Decimal>,
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
