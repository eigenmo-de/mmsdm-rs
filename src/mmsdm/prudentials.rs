/// Data Set Name: Prudential
/// File Name: Company Position
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialCompanyPosition1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    runno: i64,
    /// The company identifier
    company_id: String,
    /// The Maximum Credit Limit of the company at the time of the prudential run
    mcl: Option<rust_decimal::Decimal>,
    /// The Credit Support of the company at the time of the prudential run
    credit_support: Option<rust_decimal::Decimal>,
    /// The Trading Limit of the company at the time of the prudential run
    trading_limit: Option<rust_decimal::Decimal>,
    /// The balance of the company for all unpaid billing weeks at the time of the prudential run
    current_amount_balance: Option<rust_decimal::Decimal>,
    /// The sum of all active security deposit provision amounts at the time of the prudential run
    security_deposit_provision: Option<rust_decimal::Decimal>,
    /// The sum of all active security deposit application amounts at the time of the prudential run
    security_deposit_offset: Option<rust_decimal::Decimal>,
    /// The balance of all active security deposits at the time of the prudential run
    security_deposit_balance: Option<rust_decimal::Decimal>,
    /// The balance of all ex-post reallocations for the company that were calculated outside of billing runs at the time of the prudential run
    expost_realloc_balance: Option<rust_decimal::Decimal>,
    /// The balance of all defaults for the company at the time of the prudential run
    default_balance: Option<rust_decimal::Decimal>,
    /// The total outstandings for the company at the time of the prudential run
    outstandings: Option<rust_decimal::Decimal>,
    /// The trading margin for the company at the time of the prudential run
    trading_margin: Option<rust_decimal::Decimal>,
    /// The typical accrual for the company at the time of the prudential run
    typical_accrual: Option<rust_decimal::Decimal>,
    /// The prudential margin is the current value determined by AEMO for the registered participant. It represents the buffer below the value of credit support which is used to set the trading limit
    prudential_margin: Option<rust_decimal::Decimal>,
    /// The early payment amount deducted from Outstandings in the prudential run
    early_payment_amount: Option<rust_decimal::Decimal>,
    /// The percentage of outstandings calculated against the trading margin and prudential margin
    percentage_outstandings: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PrudentialCompanyPosition1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRUDENTIAL".into(),
                        table_name: "COMPANY_POSITION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Prudential
/// File Name: Runtrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialRuntrk1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    runno: i64,
    /// The user that authorised the prudential run
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PrudentialRuntrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PRUDENTIAL".into(),
                        table_name: "RUNTRK".into(),
                        version: 1,
                    }
                    
    }
}
