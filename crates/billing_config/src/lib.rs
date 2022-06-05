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
    /// Start Date of week
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// End Date of week
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Preliminary Statement Date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub preliminarystatementdate: Option<chrono::NaiveDateTime>,
    /// Final Statement Date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub finalstatementdate: Option<chrono::NaiveDateTime>,
    /// Payment Date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub paymentdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Revision 1 Statement Date for the billing week.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub revision1_statementdate: Option<chrono::NaiveDateTime>,
    /// Revision 2 Statement Date for the billing week.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub revision2_statementdate: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingConfigBillingcalendar2 {
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("BILLINGCALENDAR".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> BillingConfigBillingcalendar2PrimaryKey {
        BillingConfigBillingcalendar2PrimaryKey {
            contractyear: self.contractyear,
            weekno: self.weekno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "billing_config_billingcalendar_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingConfigBillingcalendar2PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingConfigBillingcalendar2PrimaryKey {
}
impl mmsdm_core::CompareWithRow for BillingConfigBillingcalendar2 {
    type Row = BillingConfigBillingcalendar2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
        && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigBillingcalendar2 {
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
        && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingConfigBillingcalendar2PrimaryKey {
    type Row = BillingConfigBillingcalendar2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
        && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigBillingcalendar2PrimaryKey {
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
        && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigBillingcalendar2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("contractyear", arrow2::datatypes::DataType::Decimal(4,0), false),
            arrow2::datatypes::Field::new("weekno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("preliminarystatementdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("finalstatementdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("paymentdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("revision1_statementdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("revision2_statementdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut preliminarystatementdate_array = Vec::new();
        let mut finalstatementdate_array = Vec::new();
        let mut paymentdate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut revision1_statementdate_array = Vec::new();
        let mut revision2_statementdate_array = Vec::new();
        for row in partition {
            contractyear_array.push({
                        let mut val = row.contractyear;
                        val.rescale(0);
                        val.mantissa()
                    });
            weekno_array.push({
                        let mut val = row.weekno;
                        val.rescale(0);
                        val.mantissa()
                    });
            startdate_array.push(row.startdate.map(|val| val.timestamp()));
            enddate_array.push(row.enddate.map(|val| val.timestamp()));
            preliminarystatementdate_array.push(row.preliminarystatementdate.map(|val| val.timestamp()));
            finalstatementdate_array.push(row.finalstatementdate.map(|val| val.timestamp()));
            paymentdate_array.push(row.paymentdate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            revision1_statementdate_array.push(row.revision1_statementdate.map(|val| val.timestamp()));
            revision2_statementdate_array.push(row.revision2_statementdate.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array).to(arrow2::datatypes::DataType::Decimal(4,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(startdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(preliminarystatementdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(finalstatementdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(paymentdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(revision1_statementdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(revision2_statementdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
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
    /// Last date and time the record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingConfigGstBasClass1 {
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_BAS_CLASS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstBasClass1PrimaryKey {
        BillingConfigGstBasClass1PrimaryKey {
            bas_class: self.bas_class.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_bas_class_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingConfigGstBasClass1PrimaryKey {
    pub bas_class: String,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstBasClass1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for BillingConfigGstBasClass1 {
    type Row = BillingConfigGstBasClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstBasClass1 {
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
    }
}
impl mmsdm_core::CompareWithRow for BillingConfigGstBasClass1PrimaryKey {
    type Row = BillingConfigGstBasClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstBasClass1PrimaryKey {
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigGstBasClass1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("bas_class", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("description", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut bas_class_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            bas_class_array.push(row.bas_class);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bas_class_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
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
    /// The effective date of the data set
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    pub versionno: rust_decimal::Decimal,
    /// The BAS classification
    pub bas_class: String,
    /// The GST rate that applies to this BAS classification
    pub gst_rate: Option<rust_decimal::Decimal>,
    /// Last date and time the record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingConfigGstRate1 {
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_RATE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstRate1PrimaryKey {
        BillingConfigGstRate1PrimaryKey {
            bas_class: self.bas_class.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_rate_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingConfigGstRate1PrimaryKey {
    pub bas_class: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstRate1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for BillingConfigGstRate1 {
    type Row = BillingConfigGstRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstRate1 {
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for BillingConfigGstRate1PrimaryKey {
    type Row = BillingConfigGstRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstRate1PrimaryKey {
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigGstRate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("bas_class", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("gst_rate", arrow2::datatypes::DataType::Decimal(8,5), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut bas_class_array = Vec::new();
        let mut gst_rate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            bas_class_array.push(row.bas_class);
            gst_rate_array.push({
                        row.gst_rate.map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bas_class_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(gst_rate_array).to(arrow2::datatypes::DataType::Decimal(8,5))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
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
    /// The effective date of the data set
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    pub versionno: rust_decimal::Decimal,
    /// NEM settlement transaction type
    pub transaction_type: String,
    /// The BAS classification that the transaction type corresponds to
    pub bas_class: String,
    /// Last date and time the record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingConfigGstTransactionClass1 {
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_TRANSACTION_CLASS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstTransactionClass1PrimaryKey {
        BillingConfigGstTransactionClass1PrimaryKey {
            bas_class: self.bas_class.clone(),
            effectivedate: self.effectivedate,
            transaction_type: self.transaction_type.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_transaction_class_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingConfigGstTransactionClass1PrimaryKey {
    pub bas_class: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub transaction_type: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstTransactionClass1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for BillingConfigGstTransactionClass1 {
    type Row = BillingConfigGstTransactionClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
        && self.effectivedate == row.effectivedate
        && self.transaction_type == row.transaction_type
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstTransactionClass1 {
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
        && self.effectivedate == key.effectivedate
        && self.transaction_type == key.transaction_type
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for BillingConfigGstTransactionClass1PrimaryKey {
    type Row = BillingConfigGstTransactionClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
        && self.effectivedate == row.effectivedate
        && self.transaction_type == row.transaction_type
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstTransactionClass1PrimaryKey {
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
        && self.effectivedate == key.effectivedate
        && self.transaction_type == key.transaction_type
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigGstTransactionClass1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("transaction_type", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("bas_class", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut transaction_type_array = Vec::new();
        let mut bas_class_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            transaction_type_array.push(row.transaction_type);
            bas_class_array.push(row.bas_class);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(transaction_type_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bas_class_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
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
    /// Last date and time the record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingConfigGstTransactionType1 {
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_TRANSACTION_TYPE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstTransactionType1PrimaryKey {
        BillingConfigGstTransactionType1PrimaryKey {
            transaction_type: self.transaction_type.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_transaction_type_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingConfigGstTransactionType1PrimaryKey {
    pub transaction_type: String,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstTransactionType1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for BillingConfigGstTransactionType1 {
    type Row = BillingConfigGstTransactionType1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.transaction_type == row.transaction_type
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstTransactionType1 {
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.transaction_type == key.transaction_type
    }
}
impl mmsdm_core::CompareWithRow for BillingConfigGstTransactionType1PrimaryKey {
    type Row = BillingConfigGstTransactionType1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.transaction_type == row.transaction_type
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstTransactionType1PrimaryKey {
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.transaction_type == key.transaction_type
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigGstTransactionType1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("transaction_type", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("description", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("gl_financialcode", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("gl_tcode", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut transaction_type_array = Vec::new();
        let mut description_array = Vec::new();
        let mut gl_financialcode_array = Vec::new();
        let mut gl_tcode_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            transaction_type_array.push(row.transaction_type);
            description_array.push(row.description);
            gl_financialcode_array.push(row.gl_financialcode);
            gl_tcode_array.push(row.gl_tcode);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(transaction_type_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gl_financialcode_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gl_tcode_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
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
    /// The effective date of the interest rate change
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Date Time this record was added
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// The interest rate for the interest account ID as on the effective date.
    pub interest_rate: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingConfigSecdepositInterestRate1 {
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("SECDEPOSIT_INTEREST_RATE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigSecdepositInterestRate1PrimaryKey {
        BillingConfigSecdepositInterestRate1PrimaryKey {
            effectivedate: self.effectivedate,
            interest_acct_id: self.interest_acct_id.clone(),
            version_datetime: self.version_datetime
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "billing_config_secdeposit_interest_rate_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingConfigSecdepositInterestRate1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interest_acct_id: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BillingConfigSecdepositInterestRate1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for BillingConfigSecdepositInterestRate1 {
    type Row = BillingConfigSecdepositInterestRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.interest_acct_id == row.interest_acct_id
        && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigSecdepositInterestRate1 {
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.interest_acct_id == key.interest_acct_id
        && self.version_datetime == key.version_datetime
    }
}
impl mmsdm_core::CompareWithRow for BillingConfigSecdepositInterestRate1PrimaryKey {
    type Row = BillingConfigSecdepositInterestRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.interest_acct_id == row.interest_acct_id
        && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigSecdepositInterestRate1PrimaryKey {
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.interest_acct_id == key.interest_acct_id
        && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigSecdepositInterestRate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("interest_acct_id", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("version_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("interest_rate", arrow2::datatypes::DataType::Decimal(18,8), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut interest_acct_id_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut interest_rate_array = Vec::new();
        for row in partition {
            interest_acct_id_array.push(row.interest_acct_id);
            effectivedate_array.push(row.effectivedate.timestamp());
            version_datetime_array.push(row.version_datetime.timestamp());
            interest_rate_array.push({
                        row.interest_rate.map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                    });
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(interest_acct_id_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(version_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interest_rate_array).to(arrow2::datatypes::DataType::Decimal(18,8))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
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
    /// The date the security deposit ID is entered and authorised by settlements
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
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
impl mmsdm_core::GetTable for BillingConfigSecdepositProvision1 {
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("SECDEPOSIT_PROVISION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigSecdepositProvision1PrimaryKey {
        BillingConfigSecdepositProvision1PrimaryKey {
            participantid: self.participantid.clone(),
            security_deposit_id: self.security_deposit_id.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "billing_config_secdeposit_provision_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingConfigSecdepositProvision1PrimaryKey {
    pub participantid: String,
    pub security_deposit_id: String,
}
impl mmsdm_core::PrimaryKey for BillingConfigSecdepositProvision1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for BillingConfigSecdepositProvision1 {
    type Row = BillingConfigSecdepositProvision1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
        && self.security_deposit_id == row.security_deposit_id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigSecdepositProvision1 {
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
        && self.security_deposit_id == key.security_deposit_id
    }
}
impl mmsdm_core::CompareWithRow for BillingConfigSecdepositProvision1PrimaryKey {
    type Row = BillingConfigSecdepositProvision1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
        && self.security_deposit_id == row.security_deposit_id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigSecdepositProvision1PrimaryKey {
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
        && self.security_deposit_id == key.security_deposit_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigSecdepositProvision1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("security_deposit_id", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("transaction_date", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("maturity_contractyear", arrow2::datatypes::DataType::Decimal(4,0), true),
            arrow2::datatypes::Field::new("maturity_weekno", arrow2::datatypes::DataType::Decimal(3,0), true),
            arrow2::datatypes::Field::new("amount", arrow2::datatypes::DataType::Decimal(18,8), true),
            arrow2::datatypes::Field::new("interest_rate", arrow2::datatypes::DataType::Decimal(18,8), true),
            arrow2::datatypes::Field::new("interest_calc_type", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("interest_acct_id", arrow2::datatypes::DataType::LargeUtf8, true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut security_deposit_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut transaction_date_array = Vec::new();
        let mut maturity_contractyear_array = Vec::new();
        let mut maturity_weekno_array = Vec::new();
        let mut amount_array = Vec::new();
        let mut interest_rate_array = Vec::new();
        let mut interest_calc_type_array = Vec::new();
        let mut interest_acct_id_array = Vec::new();
        for row in partition {
            security_deposit_id_array.push(row.security_deposit_id);
            participantid_array.push(row.participantid);
            transaction_date_array.push(row.transaction_date.map(|val| val.timestamp()));
            maturity_contractyear_array.push({
                        row.maturity_contractyear.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            maturity_weekno_array.push({
                        row.maturity_weekno.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            amount_array.push({
                        row.amount.map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                    });
            interest_rate_array.push({
                        row.interest_rate.map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                    });
            interest_calc_type_array.push(row.interest_calc_type);
            interest_acct_id_array.push(row.interest_acct_id);
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(security_deposit_id_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(transaction_date_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maturity_contractyear_array).to(arrow2::datatypes::DataType::Decimal(4,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maturity_weekno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(amount_array).to(arrow2::datatypes::DataType::Decimal(18,8))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interest_rate_array).to(arrow2::datatypes::DataType::Decimal(18,8))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(interest_calc_type_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(interest_acct_id_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
#[cfg(feature = "sql_server")]
pub async fn save<'a, S>(mms_file: &mut mmsdm_core::MmsFile<'a>, file_key: &mmsdm_core::FileKey, client: &mut tiberius::Client<S>, chunk_size: Option<usize>) -> mmsdm_core::Result<()>
where S: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin + Send,
{
    match (
        file_key.table_name.as_deref(),
        file_key.version,
    ) {
        (Some("BILLINGCALENDAR"), version) if version <= 2_i32 => {
            let d: Vec<BillingConfigBillingcalendar2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertBillingConfigBillingcalendar2 @P1, @P2", chunk_size).await?;
        }
        (Some("GST_BAS_CLASS"), version) if version <= 1_i32 => {
            let d: Vec<BillingConfigGstBasClass1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertBillingConfigGstBasClass1 @P1, @P2", chunk_size).await?;
        }
        (Some("GST_RATE"), version) if version <= 1_i32 => {
            let d: Vec<BillingConfigGstRate1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertBillingConfigGstRate1 @P1, @P2", chunk_size).await?;
        }
        (Some("GST_TRANSACTION_CLASS"), version) if version <= 1_i32 => {
            let d: Vec<BillingConfigGstTransactionClass1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertBillingConfigGstTransactionClass1 @P1, @P2", chunk_size).await?;
        }
        (Some("GST_TRANSACTION_TYPE"), version) if version <= 1_i32 => {
            let d: Vec<BillingConfigGstTransactionType1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertBillingConfigGstTransactionType1 @P1, @P2", chunk_size).await?;
        }
        (Some("SECDEPOSIT_INTEREST_RATE"), version) if version <= 1_i32 => {
            let d: Vec<BillingConfigSecdepositInterestRate1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertBillingConfigSecdepositInterestRate1 @P1, @P2", chunk_size).await?;
        }
        (Some("SECDEPOSIT_PROVISION"), version) if version <= 1_i32 => {
            let d: Vec<BillingConfigSecdepositProvision1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertBillingConfigSecdepositProvision1 @P1, @P2", chunk_size).await?;
        }
        _ => {
            log::error!("Unexpected file key {:?}", file_key);
        }
    }
    Ok(())
}
