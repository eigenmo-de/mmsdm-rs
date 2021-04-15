/// # Summary
///
/// ## GST_RATE
///  _GST_RATE maintains the GST rates on a BAS (Business Activity Statement) class basis._
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Rate
/// * Data Version: 1
///
/// # Description
///  GST_RATE data is public to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BAS_CLASS
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstRate1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    pub versionno: rust_decimal::Decimal,
    /// The BAS classification
    pub bas_class: String,
    /// The GST rate that applies to this BAS classification
    pub gst_rate: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for BillingConfigGstRate1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_RATE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SECDEPOSIT_INTEREST_RATE
///  _The security deposit interest rate on a daily basis. This is the public table published when the business enter and authorise a new daily interest rate_
///
/// * Data Set Name: Billing Config
/// * File Name: Secdeposit Interest Rate
/// * Data Version: 1
///
/// # Description
///  SECDEPOSIT_INTEREST_RATE data is public to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTEREST_ACCT_ID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigSecdepositInterestRate1 {
    /// The interest account ID for calculating the interest payment
    pub interest_acct_id: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// The interest rate for the interest account ID as on the effective date.
    pub interest_rate: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for BillingConfigSecdepositInterestRate1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("SECDEPOSIT_INTEREST_RATE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## GST_TRANSACTION_TYPE
///  _GST_TRANSACTION_TYPE shows a static list of transaction types supported by the MMS. _
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Transaction Type
/// * Data Version: 1
///
/// # Description
///  GST_TRANSACTION_TYPE data is public to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * TRANSACTION_TYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstTransactionType1 {
    /// The transaction type
    pub transaction_type: String,
    /// Description of the transaction type
    pub description: Option<String>,
    /// &nbsp;
    pub gl_financialcode: Option<String>,
    /// &nbsp;
    pub gl_tcode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for BillingConfigGstTransactionType1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_TRANSACTION_TYPE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## GST_BAS_CLASS
///  _GST_BAS_CLASS contains a static list of BAS (Business Activity Statement) classifications supported by the MMS. _
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Bas Class
/// * Data Version: 1
///
/// # Description
///  GST_BAS_CLASS data is public to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BAS_CLASS
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstBasClass1 {
    /// The BAS classification
    pub bas_class: String,
    /// Description of the BAS classification
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for BillingConfigGstBasClass1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_BAS_CLASS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## BILLINGCALENDAR
///  _BILLINGCALENDAR sets out the billing calendar for the year, with week number 1 starting on 1 January. BILLINGCALENDAR advises preliminary and final statement posting date and corresponding  settlement for each billing week._
///
/// * Data Set Name: Billing Config
/// * File Name: Billingcalendar
/// * Data Version: 2
///
/// # Description
///  BILLINGCALENDAR is public data, and is available to all participants. Source Infrequently, only when inserting billing weeks for a future contractyear. Volume 52-53 records inserted per contractyear
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigBillingcalendar2 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub preliminarystatementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub finalstatementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub paymentdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub revision1_statementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub revision2_statementdate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for BillingConfigBillingcalendar2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("BILLINGCALENDAR".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## GST_TRANSACTION_CLASS
///  _GST_TRANSACTION_CLASS maps NEM settlement transaction types with BAS (Business Activity Statement) classifications._
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Transaction Class
/// * Data Version: 1
///
/// # Description
///  GST_TRANSACTION_CLASS data is public to all participants. Source GST_TRANSACTION_CLASS updates infrequently, when new transactions are introduced to the NEM. Volume Generally volume is fewer than one hundred records.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BAS_CLASS
/// * EFFECTIVEDATE
/// * TRANSACTION_TYPE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstTransactionClass1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    pub versionno: rust_decimal::Decimal,
    /// NEM settlement transaction type
    pub transaction_type: String,
    /// The BAS classification that the transaction type corresponds to
    pub bas_class: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for BillingConfigGstTransactionClass1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_TRANSACTION_CLASS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SECDEPOSIT_PROVISION
///  _The security deposit provision entry details_
///
/// * Data Set Name: Billing Config
/// * File Name: Secdeposit Provision
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SECURITY_DEPOSIT_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigSecdepositProvision1 {
    /// The security deposit ID
    pub security_deposit_id: String,
    /// The Participant ID linked to the security deposit ID
    pub participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub transaction_date: Option<chrono::NaiveDateTime>,
    /// The contract year of the billing week when the security deposit is maturing
    pub maturity_contractyear: Option<rust_decimal::Decimal>,
    /// The week no of the billing week when the security deposit is maturing
    pub maturity_weekno: Option<rust_decimal::Decimal>,
    /// The security deposit amount
    pub amount: Option<rust_decimal::Decimal>,
    /// The interest rate assigned to the security deposit ID. Null if INTEREST_CALC_TYPE &lt;&gt; FIXED
    pub interest_rate: Option<rust_decimal::Decimal>,
    /// FIXED OR DAILY
    pub interest_calc_type: Option<String>,
    /// The Interest Account ID for calculating the Interest Payment. This is NULL if the INTEREST_CALC_TYPE = FIXED
    pub interest_acct_id: Option<String>,
}
impl crate::GetTable for BillingConfigSecdepositProvision1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("SECDEPOSIT_PROVISION".into()),
            version: 1,
        }
    }
}
