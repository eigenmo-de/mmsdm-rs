/// Data Set Name: Billing Config
/// File Name: Gst Transaction Class
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstTransactionClass1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    versionno: rust_decimal::Decimal,
    /// NEM settlement transaction type
    transaction_type: String,
    /// The BAS classification that the transaction type corresponds to
    bas_class: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstTransactionClass1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING_CONFIG".into(),
                        table_name: "GST_TRANSACTION_CLASS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing Config
/// File Name: Secdeposit Provision
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigSecdepositProvision1 {
    /// The security deposit ID 
    security_deposit_id: String,
    /// The Participant ID linked to the security deposit ID
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    transaction_date: Option<chrono::NaiveDateTime>,
    /// The contract year of the billing week when the security deposit is maturing
    maturity_contractyear: Option<rust_decimal::Decimal>,
    /// The week no of the billing week when the security deposit is maturing
    maturity_weekno: Option<rust_decimal::Decimal>,
    /// The security deposit amount
    amount: Option<rust_decimal::Decimal>,
    /// The interest rate assigned to the security deposit ID. Null if INTEREST_CALC_TYPE &lt;&gt; FIXED
    interest_rate: Option<rust_decimal::Decimal>,
    /// FIXED OR DAILY
    interest_calc_type: Option<String>,
    /// The Interest Account ID for calculating the Interest Payment. This is NULL if the INTEREST_CALC_TYPE = FIXED
    interest_acct_id: Option<String>,
}
impl crate::GetTable<BillingConfigSecdepositProvision1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING_CONFIG".into(),
                        table_name: "SECDEPOSIT_PROVISION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing Config
/// File Name: Billingcalendar
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigBillingcalendar2 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    preliminarystatementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    finalstatementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    paymentdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    revision1_statementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    revision2_statementdate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigBillingcalendar2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING_CONFIG".into(),
                        table_name: "BILLINGCALENDAR".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Billing Config
/// File Name: Gst Rate
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstRate1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    versionno: rust_decimal::Decimal,
    /// The BAS classification
    bas_class: String,
    /// The GST rate that applies to this BAS classification
    gst_rate: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstRate1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING_CONFIG".into(),
                        table_name: "GST_RATE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing Config
/// File Name: Secdeposit Interest Rate
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigSecdepositInterestRate1 {
    /// The interest account ID for calculating the interest payment
    interest_acct_id: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// The interest rate for the interest account ID as on the effective date.
    interest_rate: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingConfigSecdepositInterestRate1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING_CONFIG".into(),
                        table_name: "SECDEPOSIT_INTEREST_RATE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing Config
/// File Name: Gst Bas Class
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstBasClass1 {
    /// The BAS classification
    bas_class: String,
    /// Description of the BAS classification
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstBasClass1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING_CONFIG".into(),
                        table_name: "GST_BAS_CLASS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing Config
/// File Name: Gst Transaction Type
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstTransactionType1 {
    /// The transaction type
    transaction_type: String,
    /// Description of the transaction type
    description: Option<String>,
    /// &nbsp; 
    gl_financialcode: Option<String>,
    /// &nbsp; 
    gl_tcode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstTransactionType1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING_CONFIG".into(),
                        table_name: "GST_TRANSACTION_TYPE".into(),
                        version: 1,
                    }
                    
    }
}
