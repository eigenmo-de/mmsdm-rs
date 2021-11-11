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
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("BILLINGCALENDAR".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> BillingConfigBillingcalendar2PrimaryKey {
        BillingConfigBillingcalendar2PrimaryKey {
            contractyear: self.contractyear.clone(),
            weekno: self.weekno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "billing_config_billingcalendar_v2".to_string()
    }
}
impl crate::CompareWithRow for BillingConfigBillingcalendar2 {
    type Row = BillingConfigBillingcalendar2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.weekno == row.weekno
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigBillingcalendar2 {
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.weekno == key.weekno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigBillingcalendar2PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for BillingConfigBillingcalendar2PrimaryKey {
    type Row = BillingConfigBillingcalendar2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.weekno == row.weekno
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigBillingcalendar2PrimaryKey {
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.weekno == key.weekno
    }
}
impl crate::PrimaryKey for BillingConfigBillingcalendar2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BillingConfigBillingcalendar2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "weekno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "preliminarystatementdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "finalstatementdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("paymentdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "revision1_statementdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "revision2_statementdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
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
            startdate_array.push(row.startdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            enddate_array.push(row.enddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            preliminarystatementdate_array.push(row.preliminarystatementdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            finalstatementdate_array.push(row.finalstatementdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            paymentdate_array.push(row.paymentdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            revision1_statementdate_array.push(row.revision1_statementdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            revision2_statementdate_array.push(row.revision2_statementdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(weekno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(preliminarystatementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(finalstatementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(paymentdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(revision1_statementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(revision2_statementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
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
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_BAS_CLASS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstBasClass1PrimaryKey {
        BillingConfigGstBasClass1PrimaryKey {
            bas_class: self.bas_class.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_bas_class_v1".to_string()
    }
}
impl crate::CompareWithRow for BillingConfigGstBasClass1 {
    type Row = BillingConfigGstBasClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstBasClass1 {
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstBasClass1PrimaryKey {
    pub bas_class: String,
}
impl crate::CompareWithRow for BillingConfigGstBasClass1PrimaryKey {
    type Row = BillingConfigGstBasClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstBasClass1PrimaryKey {
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
    }
}
impl crate::PrimaryKey for BillingConfigGstBasClass1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BillingConfigGstBasClass1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "bas_class",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut bas_class_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            bas_class_array.push(row.bas_class);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bas_class_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
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
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_RATE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstRate1PrimaryKey {
        BillingConfigGstRate1PrimaryKey {
            bas_class: self.bas_class.clone(),
            effectivedate: self.effectivedate.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_rate_v1".to_string()
    }
}
impl crate::CompareWithRow for BillingConfigGstRate1 {
    type Row = BillingConfigGstRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstRate1 {
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstRate1PrimaryKey {
    pub bas_class: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for BillingConfigGstRate1PrimaryKey {
    type Row = BillingConfigGstRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstRate1PrimaryKey {
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for BillingConfigGstRate1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BillingConfigGstRate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "bas_class",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "gst_rate",
                arrow2::datatypes::DataType::Decimal(8, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut bas_class_array = Vec::new();
        let mut gst_rate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bas_class_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(gst_rate_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
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
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_TRANSACTION_CLASS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstTransactionClass1PrimaryKey {
        BillingConfigGstTransactionClass1PrimaryKey {
            bas_class: self.bas_class.clone(),
            effectivedate: self.effectivedate.clone(),
            transaction_type: self.transaction_type.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_transaction_class_v1".to_string()
    }
}
impl crate::CompareWithRow for BillingConfigGstTransactionClass1 {
    type Row = BillingConfigGstTransactionClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
            && self.effectivedate == row.effectivedate
            && self.transaction_type == row.transaction_type
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstTransactionClass1 {
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
            && self.effectivedate == key.effectivedate
            && self.transaction_type == key.transaction_type
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstTransactionClass1PrimaryKey {
    pub bas_class: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub transaction_type: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for BillingConfigGstTransactionClass1PrimaryKey {
    type Row = BillingConfigGstTransactionClass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class
            && self.effectivedate == row.effectivedate
            && self.transaction_type == row.transaction_type
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstTransactionClass1PrimaryKey {
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class
            && self.effectivedate == key.effectivedate
            && self.transaction_type == key.transaction_type
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for BillingConfigGstTransactionClass1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BillingConfigGstTransactionClass1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "transaction_type",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "bas_class",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut transaction_type_array = Vec::new();
        let mut bas_class_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            transaction_type_array.push(row.transaction_type);
            bas_class_array.push(row.bas_class);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    transaction_type_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bas_class_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
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
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("GST_TRANSACTION_TYPE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigGstTransactionType1PrimaryKey {
        BillingConfigGstTransactionType1PrimaryKey {
            transaction_type: self.transaction_type.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "billing_config_gst_transaction_type_v1".to_string()
    }
}
impl crate::CompareWithRow for BillingConfigGstTransactionType1 {
    type Row = BillingConfigGstTransactionType1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.transaction_type == row.transaction_type
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstTransactionType1 {
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.transaction_type == key.transaction_type
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstTransactionType1PrimaryKey {
    pub transaction_type: String,
}
impl crate::CompareWithRow for BillingConfigGstTransactionType1PrimaryKey {
    type Row = BillingConfigGstTransactionType1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.transaction_type == row.transaction_type
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigGstTransactionType1PrimaryKey {
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.transaction_type == key.transaction_type
    }
}
impl crate::PrimaryKey for BillingConfigGstTransactionType1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BillingConfigGstTransactionType1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "transaction_type",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "gl_financialcode",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("gl_tcode", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut transaction_type_array = Vec::new();
        let mut description_array = Vec::new();
        let mut gl_financialcode_array = Vec::new();
        let mut gl_tcode_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            transaction_type_array.push(row.transaction_type);
            description_array.push(row.description);
            gl_financialcode_array.push(row.gl_financialcode);
            gl_tcode_array.push(row.gl_tcode);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    transaction_type_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    gl_financialcode_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gl_tcode_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
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
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("SECDEPOSIT_INTEREST_RATE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigSecdepositInterestRate1PrimaryKey {
        BillingConfigSecdepositInterestRate1PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            interest_acct_id: self.interest_acct_id.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "billing_config_secdeposit_interest_rate_v1".to_string()
    }
}
impl crate::CompareWithRow for BillingConfigSecdepositInterestRate1 {
    type Row = BillingConfigSecdepositInterestRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interest_acct_id == row.interest_acct_id
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigSecdepositInterestRate1 {
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interest_acct_id == key.interest_acct_id
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigSecdepositInterestRate1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interest_acct_id: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for BillingConfigSecdepositInterestRate1PrimaryKey {
    type Row = BillingConfigSecdepositInterestRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interest_acct_id == row.interest_acct_id
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigSecdepositInterestRate1PrimaryKey {
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interest_acct_id == key.interest_acct_id
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for BillingConfigSecdepositInterestRate1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BillingConfigSecdepositInterestRate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "interest_acct_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interest_rate",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut interest_acct_id_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut interest_rate_array = Vec::new();
        for (_, row) in partition {
            interest_acct_id_array.push(row.interest_acct_id);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            interest_rate_array.push({
                row.interest_rate.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interest_acct_id_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(interest_rate_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
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
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: Some("SECDEPOSIT_PROVISION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BillingConfigSecdepositProvision1PrimaryKey {
        BillingConfigSecdepositProvision1PrimaryKey {
            participantid: self.participantid.clone(),
            security_deposit_id: self.security_deposit_id.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "billing_config_secdeposit_provision_v1".to_string()
    }
}
impl crate::CompareWithRow for BillingConfigSecdepositProvision1 {
    type Row = BillingConfigSecdepositProvision1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.security_deposit_id == row.security_deposit_id
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigSecdepositProvision1 {
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.security_deposit_id == key.security_deposit_id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigSecdepositProvision1PrimaryKey {
    pub participantid: String,
    pub security_deposit_id: String,
}
impl crate::CompareWithRow for BillingConfigSecdepositProvision1PrimaryKey {
    type Row = BillingConfigSecdepositProvision1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.security_deposit_id == row.security_deposit_id
    }
}
impl crate::CompareWithPrimaryKey for BillingConfigSecdepositProvision1PrimaryKey {
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.security_deposit_id == key.security_deposit_id
    }
}
impl crate::PrimaryKey for BillingConfigSecdepositProvision1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BillingConfigSecdepositProvision1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "security_deposit_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "transaction_date",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "maturity_contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "maturity_weekno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "interest_rate",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "interest_calc_type",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "interest_acct_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut security_deposit_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut transaction_date_array = Vec::new();
        let mut maturity_contractyear_array = Vec::new();
        let mut maturity_weekno_array = Vec::new();
        let mut amount_array = Vec::new();
        let mut interest_rate_array = Vec::new();
        let mut interest_calc_type_array = Vec::new();
        let mut interest_acct_id_array = Vec::new();
        for (_, row) in partition {
            security_deposit_id_array.push(row.security_deposit_id);
            participantid_array.push(row.participantid);
            transaction_date_array.push(row.transaction_date.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
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

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    security_deposit_id_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(transaction_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maturity_contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maturity_weekno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(interest_rate_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    interest_calc_type_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    interest_acct_id_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
