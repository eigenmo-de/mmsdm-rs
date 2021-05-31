/// # Summary
///
/// ## PRUDENTIALCOMPANYPOSITION
///  _The prudential position of each company as at the datetime of a specific prudential run_
///
/// * Data Set Name: Prudential
/// * File Name: Company Position
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * COMPANY_ID
/// * PRUDENTIAL_DATE
/// * RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialCompanyPosition1 {
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub runno: i64,
    /// The company identifier
    pub company_id: String,
    /// The Maximum Credit Limit of the company at the time of the prudential run
    pub mcl: Option<rust_decimal::Decimal>,
    /// The Credit Support of the company at the time of the prudential run
    pub credit_support: Option<rust_decimal::Decimal>,
    /// The Trading Limit of the company at the time of the prudential run
    pub trading_limit: Option<rust_decimal::Decimal>,
    /// The balance of the company for all unpaid billing weeks at the time of the prudential run
    pub current_amount_balance: Option<rust_decimal::Decimal>,
    /// The sum of all active security deposit provision amounts at the time of the prudential run
    pub security_deposit_provision: Option<rust_decimal::Decimal>,
    /// The sum of all active security deposit application amounts at the time of the prudential run
    pub security_deposit_offset: Option<rust_decimal::Decimal>,
    /// The balance of all active security deposits at the time of the prudential run
    pub security_deposit_balance: Option<rust_decimal::Decimal>,
    /// The balance of all ex-post reallocations for the company that were calculated outside of billing runs at the time of the prudential run
    pub expost_realloc_balance: Option<rust_decimal::Decimal>,
    /// The balance of all defaults for the company at the time of the prudential run
    pub default_balance: Option<rust_decimal::Decimal>,
    /// The total outstandings for the company at the time of the prudential run
    pub outstandings: Option<rust_decimal::Decimal>,
    /// The trading margin for the company at the time of the prudential run
    pub trading_margin: Option<rust_decimal::Decimal>,
    /// The typical accrual for the company at the time of the prudential run
    pub typical_accrual: Option<rust_decimal::Decimal>,
    /// The prudential margin is the current value determined by AEMO for the registered participant. It represents the buffer below the value of credit support which is used to set the trading limit
    pub prudential_margin: Option<rust_decimal::Decimal>,
    /// The early payment amount deducted from Outstandings in the prudential run
    pub early_payment_amount: Option<rust_decimal::Decimal>,
    /// The percentage of outstandings calculated against the trading margin and prudential margin
    pub percentage_outstandings: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PrudentialCompanyPosition1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRUDENTIAL".into(),
            table_name: Some("COMPANY_POSITION".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PRUDENTIALRUNTRK
///  _Records the prudential run accepted by Settlements staff for each prudential date_
///
/// * Data Set Name: Prudential
/// * File Name: Runtrk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PRUDENTIAL_DATE
/// * RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialRuntrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub runno: i64,
    /// The user that authorised the prudential run
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PrudentialRuntrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRUDENTIAL".into(),
            table_name: Some("RUNTRK".into()),
            version: 1,
        }
    }
}
