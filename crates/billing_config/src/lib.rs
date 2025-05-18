#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct BillingConfigBillingcalendar2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingConfigBillingcalendar2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingConfigBillingcalendar2 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BillingConfigBillingcalendar2Mapping([usize; 10]);
/// # Summary
///
/// ## BILLINGCALENDAR
///
/// BILLINGCALENDAR sets out the billing calendar for the year, with week number 1 starting on 1 January. BILLINGCALENDAR advises preliminary and final statement posting date and corresponding  settlement for each billing week.
///
/// * Data Set Name: Billing Config
/// * File Name: Billingcalendar
/// * Data Version: 2
///
/// # Description
/// BILLINGCALENDAR is public data, and is available to all participants.SourceInfrequently, only when inserting billing weeks for a future contractyear.Volume52-53 records inserted per contractyear
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, PartialEq, Eq)]
pub struct BillingConfigBillingcalendar2Row<'data> {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Start Date of week
    pub startdate: Option<chrono::NaiveDateTime>,
    /// End Date of week
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Preliminary Statement Date
    pub preliminarystatementdate: Option<chrono::NaiveDateTime>,
    /// Final Statement Date
    pub finalstatementdate: Option<chrono::NaiveDateTime>,
    /// Payment Date
    pub paymentdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Revision 1 Statement Date for the billing week.
    pub revision1_statementdate: Option<chrono::NaiveDateTime>,
    /// Revision 2 Statement Date for the billing week.
    pub revision2_statementdate: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> BillingConfigBillingcalendar2Row<'data> {}
impl mmsdm_core::GetTable for BillingConfigBillingcalendar2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "BILLING_CONFIG";
    const TABLE_NAME: &'static str = "BILLINGCALENDAR";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingConfigBillingcalendar2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTYEAR",
        "WEEKNO",
        "STARTDATE",
        "ENDDATE",
        "PRELIMINARYSTATEMENTDATE",
        "FINALSTATEMENTDATE",
        "PAYMENTDATE",
        "LASTCHANGED",
        "REVISION1_STATEMENTDATE",
        "REVISION2_STATEMENTDATE",
    ];
    type Row<'row> = BillingConfigBillingcalendar2Row<'row>;
    type FieldMapping = BillingConfigBillingcalendar2Mapping;
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingConfigBillingcalendar2Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            weekno: row
                .get_custom_parsed_at_idx(
                    "weekno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            preliminarystatementdate: row
                .get_opt_custom_parsed_at_idx(
                    "preliminarystatementdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            finalstatementdate: row
                .get_opt_custom_parsed_at_idx(
                    "finalstatementdate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            paymentdate: row
                .get_opt_custom_parsed_at_idx(
                    "paymentdate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            revision1_statementdate: row
                .get_opt_custom_parsed_at_idx(
                    "revision1_statementdate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            revision2_statementdate: row
                .get_opt_custom_parsed_at_idx(
                    "revision2_statementdate",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: core::marker::PhantomData,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(BillingConfigBillingcalendar2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingConfigBillingcalendar2PrimaryKey {
        BillingConfigBillingcalendar2PrimaryKey {
            contractyear: row.contractyear,
            weekno: row.weekno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("billing_config_billingcalendar_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingConfigBillingcalendar2Row {
            contractyear: row.contractyear.clone(),
            weekno: row.weekno.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            preliminarystatementdate: row.preliminarystatementdate.clone(),
            finalstatementdate: row.finalstatementdate.clone(),
            paymentdate: row.paymentdate.clone(),
            lastchanged: row.lastchanged.clone(),
            revision1_statementdate: row.revision1_statementdate.clone(),
            revision2_statementdate: row.revision2_statementdate.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigBillingcalendar2PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingConfigBillingcalendar2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigBillingcalendar2Row<'data> {
    type Row<'other> = BillingConfigBillingcalendar2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.weekno == row.weekno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for BillingConfigBillingcalendar2Row<'data> {
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.weekno == key.weekno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigBillingcalendar2PrimaryKey {
    type Row<'other> = BillingConfigBillingcalendar2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigBillingcalendar2PrimaryKey {
    type PrimaryKey = BillingConfigBillingcalendar2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigBillingcalendar2 {
    type Builder = BillingConfigBillingcalendar2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "weekno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "startdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "preliminarystatementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "finalstatementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "paymentdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "revision1_statementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "revision2_statementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingConfigBillingcalendar2Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            weekno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            preliminarystatementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            finalstatementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            paymentdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            revision1_statementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            revision2_statementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .weekno_array
            .append_value({
                let mut val = row.weekno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .preliminarystatementdate_array
            .append_option(
                row.preliminarystatementdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .finalstatementdate_array
            .append_option(
                row.finalstatementdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .paymentdate_array
            .append_option(row.paymentdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .revision1_statementdate_array
            .append_option(
                row.revision1_statementdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .revision2_statementdate_array
            .append_option(
                row.revision2_statementdate.map(|val| val.and_utc().timestamp_millis()),
            );
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.weekno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.preliminarystatementdate_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.finalstatementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.paymentdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.revision1_statementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.revision2_statementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingConfigBillingcalendar2Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    weekno_array: arrow::array::builder::Decimal128Builder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    preliminarystatementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    finalstatementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    paymentdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    revision1_statementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    revision2_statementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BillingConfigGstBasClass1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingConfigGstBasClass1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingConfigGstBasClass1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BillingConfigGstBasClass1Mapping([usize; 3]);
/// # Summary
///
/// ## GST_BAS_CLASS
///
/// GST_BAS_CLASS contains a static list of BAS (Business Activity Statement) classifications supported by the MMS.
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Bas Class
/// * Data Version: 1
///
/// # Description
/// GST_BAS_CLASS data is public to all participants.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BAS_CLASS
#[derive(Debug, PartialEq, Eq)]
pub struct BillingConfigGstBasClass1Row<'data> {
    /// The BAS classification
    pub bas_class: core::ops::Range<usize>,
    /// Description of the BAS classification
    pub description: core::ops::Range<usize>,
    /// Last date and time the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingConfigGstBasClass1Row<'data> {
    pub fn bas_class(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bas_class.clone())
    }
    pub fn description(&self) -> Option<&str> {
        if self.description.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.description.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for BillingConfigGstBasClass1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BILLING_CONFIG";
    const TABLE_NAME: &'static str = "GST_BAS_CLASS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingConfigGstBasClass1Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "BAS_CLASS",
        "DESCRIPTION",
        "LASTCHANGED",
    ];
    type Row<'row> = BillingConfigGstBasClass1Row<'row>;
    type FieldMapping = BillingConfigGstBasClass1Mapping;
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingConfigGstBasClass1Row {
            bas_class: row.get_range("bas_class", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(BillingConfigGstBasClass1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingConfigGstBasClass1PrimaryKey {
        BillingConfigGstBasClass1PrimaryKey {
            bas_class: row.bas_class().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("billing_config_gst_bas_class_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingConfigGstBasClass1Row {
            bas_class: row.bas_class.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstBasClass1PrimaryKey {
    pub bas_class: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstBasClass1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstBasClass1Row<'data> {
    type Row<'other> = BillingConfigGstBasClass1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bas_class() == row.bas_class()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BillingConfigGstBasClass1Row<'data> {
    type PrimaryKey = BillingConfigGstBasClass1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class() == key.bas_class
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstBasClass1PrimaryKey {
    type Row<'other> = BillingConfigGstBasClass1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bas_class == row.bas_class()
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
    type Builder = BillingConfigGstBasClass1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "bas_class",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingConfigGstBasClass1Builder {
            bas_class_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.bas_class_array.append_value(row.bas_class());
        builder.description_array.append_option(row.description());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.bas_class_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingConfigGstBasClass1Builder {
    bas_class_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BillingConfigGstRate1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingConfigGstRate1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingConfigGstRate1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BillingConfigGstRate1Mapping([usize; 5]);
/// # Summary
///
/// ## GST_RATE
///
/// GST_RATE maintains the GST rates on a BAS (Business Activity Statement) class basis.
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Rate
/// * Data Version: 1
///
/// # Description
/// GST_RATE data is public to all participants.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BAS_CLASS
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct BillingConfigGstRate1Row<'data> {
    /// The effective date of the data set
    pub effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    pub versionno: rust_decimal::Decimal,
    /// The BAS classification
    pub bas_class: core::ops::Range<usize>,
    /// The GST rate that applies to this BAS classification
    pub gst_rate: Option<rust_decimal::Decimal>,
    /// Last date and time the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingConfigGstRate1Row<'data> {
    pub fn bas_class(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bas_class.clone())
    }
}
impl mmsdm_core::GetTable for BillingConfigGstRate1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BILLING_CONFIG";
    const TABLE_NAME: &'static str = "GST_RATE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingConfigGstRate1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "BAS_CLASS",
        "GST_RATE",
        "LASTCHANGED",
    ];
    type Row<'row> = BillingConfigGstRate1Row<'row>;
    type FieldMapping = BillingConfigGstRate1Mapping;
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingConfigGstRate1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bas_class: row.get_range("bas_class", field_mapping.0[2])?,
            gst_rate: row
                .get_opt_custom_parsed_at_idx(
                    "gst_rate",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(BillingConfigGstRate1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingConfigGstRate1PrimaryKey {
        BillingConfigGstRate1PrimaryKey {
            bas_class: row.bas_class().to_string(),
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("billing_config_gst_rate_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingConfigGstRate1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            bas_class: row.bas_class.clone(),
            gst_rate: row.gst_rate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstRate1PrimaryKey {
    pub bas_class: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstRate1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstRate1Row<'data> {
    type Row<'other> = BillingConfigGstRate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bas_class() == row.bas_class() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BillingConfigGstRate1Row<'data> {
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class() == key.bas_class && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstRate1PrimaryKey {
    type Row<'other> = BillingConfigGstRate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bas_class == row.bas_class() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstRate1PrimaryKey {
    type PrimaryKey = BillingConfigGstRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigGstRate1 {
    type Builder = BillingConfigGstRate1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bas_class",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "gst_rate",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingConfigGstRate1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            bas_class_array: arrow::array::builder::StringBuilder::new(),
            gst_rate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.bas_class_array.append_value(row.bas_class());
        builder
            .gst_rate_array
            .append_option({
                row.gst_rate
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bas_class_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gst_rate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingConfigGstRate1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    bas_class_array: arrow::array::builder::StringBuilder,
    gst_rate_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BillingConfigGstTransactionClass1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingConfigGstTransactionClass1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingConfigGstTransactionClass1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BillingConfigGstTransactionClass1Mapping([usize; 5]);
/// # Summary
///
/// ## GST_TRANSACTION_CLASS
///
/// GST_TRANSACTION_CLASS maps NEM settlement transaction types with BAS (Business Activity Statement) classifications.
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Transaction Class
/// * Data Version: 1
///
/// # Description
/// GST_TRANSACTION_CLASS data is public to all participants.SourceGST_TRANSACTION_CLASS updates infrequently, when new transactions are introduced to the NEM.VolumeGenerally volume is fewer than one hundred records.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BAS_CLASS
/// * EFFECTIVEDATE
/// * TRANSACTION_TYPE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct BillingConfigGstTransactionClass1Row<'data> {
    /// The effective date of the data set
    pub effectivedate: chrono::NaiveDateTime,
    /// The version number of the data set
    pub versionno: rust_decimal::Decimal,
    /// NEM settlement transaction type
    pub transaction_type: core::ops::Range<usize>,
    /// The BAS classification that the transaction type corresponds to
    pub bas_class: core::ops::Range<usize>,
    /// Last date and time the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingConfigGstTransactionClass1Row<'data> {
    pub fn transaction_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.transaction_type.clone(),
        )
    }
    pub fn bas_class(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bas_class.clone())
    }
}
impl mmsdm_core::GetTable for BillingConfigGstTransactionClass1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BILLING_CONFIG";
    const TABLE_NAME: &'static str = "GST_TRANSACTION_CLASS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingConfigGstTransactionClass1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "TRANSACTION_TYPE",
        "BAS_CLASS",
        "LASTCHANGED",
    ];
    type Row<'row> = BillingConfigGstTransactionClass1Row<'row>;
    type FieldMapping = BillingConfigGstTransactionClass1Mapping;
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingConfigGstTransactionClass1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            transaction_type: row.get_range("transaction_type", field_mapping.0[2])?,
            bas_class: row.get_range("bas_class", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(BillingConfigGstTransactionClass1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingConfigGstTransactionClass1PrimaryKey {
        BillingConfigGstTransactionClass1PrimaryKey {
            bas_class: row.bas_class().to_string(),
            effectivedate: row.effectivedate,
            transaction_type: row.transaction_type().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "billing_config_gst_transaction_class_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingConfigGstTransactionClass1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            transaction_type: row.transaction_type.clone(),
            bas_class: row.bas_class.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstTransactionClass1PrimaryKey {
    pub bas_class: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub transaction_type: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstTransactionClass1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstTransactionClass1Row<'data> {
    type Row<'other> = BillingConfigGstTransactionClass1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bas_class() == row.bas_class() && self.effectivedate == row.effectivedate
            && self.transaction_type() == row.transaction_type()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for BillingConfigGstTransactionClass1Row<'data> {
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class() == key.bas_class && self.effectivedate == key.effectivedate
            && self.transaction_type() == key.transaction_type
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstTransactionClass1PrimaryKey {
    type Row<'other> = BillingConfigGstTransactionClass1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bas_class == row.bas_class() && self.effectivedate == row.effectivedate
            && self.transaction_type == row.transaction_type()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingConfigGstTransactionClass1PrimaryKey {
    type PrimaryKey = BillingConfigGstTransactionClass1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class && self.effectivedate == key.effectivedate
            && self.transaction_type == key.transaction_type
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigGstTransactionClass1 {
    type Builder = BillingConfigGstTransactionClass1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "transaction_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bas_class",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingConfigGstTransactionClass1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            transaction_type_array: arrow::array::builder::StringBuilder::new(),
            bas_class_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.transaction_type_array.append_value(row.transaction_type());
        builder.bas_class_array.append_value(row.bas_class());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transaction_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bas_class_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingConfigGstTransactionClass1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    transaction_type_array: arrow::array::builder::StringBuilder,
    bas_class_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BillingConfigGstTransactionType1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingConfigGstTransactionType1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingConfigGstTransactionType1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BillingConfigGstTransactionType1Mapping([usize; 5]);
/// # Summary
///
/// ## GST_TRANSACTION_TYPE
///
/// GST_TRANSACTION_TYPE shows a static list of transaction types supported by the MMS.
///
/// * Data Set Name: Billing Config
/// * File Name: Gst Transaction Type
/// * Data Version: 1
///
/// # Description
/// GST_TRANSACTION_TYPE data is public to all participants.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * TRANSACTION_TYPE
#[derive(Debug, PartialEq, Eq)]
pub struct BillingConfigGstTransactionType1Row<'data> {
    /// The transaction type
    pub transaction_type: core::ops::Range<usize>,
    /// Description of the transaction type
    pub description: core::ops::Range<usize>,
    pub gl_financialcode: core::ops::Range<usize>,
    pub gl_tcode: core::ops::Range<usize>,
    /// Last date and time the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingConfigGstTransactionType1Row<'data> {
    pub fn transaction_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.transaction_type.clone(),
        )
    }
    pub fn description(&self) -> Option<&str> {
        if self.description.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.description.clone(),
                ),
            )
        }
    }
    pub fn gl_financialcode(&self) -> Option<&str> {
        if self.gl_financialcode.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.gl_financialcode.clone(),
                ),
            )
        }
    }
    pub fn gl_tcode(&self) -> Option<&str> {
        if self.gl_tcode.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.gl_tcode.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for BillingConfigGstTransactionType1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BILLING_CONFIG";
    const TABLE_NAME: &'static str = "GST_TRANSACTION_TYPE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingConfigGstTransactionType1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRANSACTION_TYPE",
        "DESCRIPTION",
        "GL_FINANCIALCODE",
        "GL_TCODE",
        "LASTCHANGED",
    ];
    type Row<'row> = BillingConfigGstTransactionType1Row<'row>;
    type FieldMapping = BillingConfigGstTransactionType1Mapping;
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingConfigGstTransactionType1Row {
            transaction_type: row.get_range("transaction_type", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            gl_financialcode: row.get_opt_range("gl_financialcode", field_mapping.0[2])?,
            gl_tcode: row.get_opt_range("gl_tcode", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(BillingConfigGstTransactionType1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingConfigGstTransactionType1PrimaryKey {
        BillingConfigGstTransactionType1PrimaryKey {
            transaction_type: row.transaction_type().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "billing_config_gst_transaction_type_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingConfigGstTransactionType1Row {
            transaction_type: row.transaction_type.clone(),
            description: row.description.clone(),
            gl_financialcode: row.gl_financialcode.clone(),
            gl_tcode: row.gl_tcode.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigGstTransactionType1PrimaryKey {
    pub transaction_type: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for BillingConfigGstTransactionType1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstTransactionType1Row<'data> {
    type Row<'other> = BillingConfigGstTransactionType1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.transaction_type() == row.transaction_type()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for BillingConfigGstTransactionType1Row<'data> {
    type PrimaryKey = BillingConfigGstTransactionType1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.transaction_type() == key.transaction_type
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigGstTransactionType1PrimaryKey {
    type Row<'other> = BillingConfigGstTransactionType1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.transaction_type == row.transaction_type()
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
    type Builder = BillingConfigGstTransactionType1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "transaction_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gl_financialcode",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gl_tcode",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingConfigGstTransactionType1Builder {
            transaction_type_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            gl_financialcode_array: arrow::array::builder::StringBuilder::new(),
            gl_tcode_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.transaction_type_array.append_value(row.transaction_type());
        builder.description_array.append_option(row.description());
        builder.gl_financialcode_array.append_option(row.gl_financialcode());
        builder.gl_tcode_array.append_option(row.gl_tcode());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.transaction_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gl_financialcode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gl_tcode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingConfigGstTransactionType1Builder {
    transaction_type_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    gl_financialcode_array: arrow::array::builder::StringBuilder,
    gl_tcode_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BillingConfigSecdepositInterestRate1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingConfigSecdepositInterestRate1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingConfigSecdepositInterestRate1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BillingConfigSecdepositInterestRate1Mapping([usize; 4]);
/// # Summary
///
/// ## SECDEPOSIT_INTEREST_RATE
///
/// The security deposit interest rate on a daily basis. This is the public table published when the business enter and authorise a new daily interest rate
///
/// * Data Set Name: Billing Config
/// * File Name: Secdeposit Interest Rate
/// * Data Version: 1
///
/// # Description
/// SECDEPOSIT_INTEREST_RATE data is public to all participants.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTEREST_ACCT_ID
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct BillingConfigSecdepositInterestRate1Row<'data> {
    /// The interest account ID for calculating the interest payment
    pub interest_acct_id: core::ops::Range<usize>,
    /// The effective date of the interest rate change
    pub effectivedate: chrono::NaiveDateTime,
    /// Date Time this record was added
    pub version_datetime: chrono::NaiveDateTime,
    /// The interest rate for the interest account ID as on the effective date.
    pub interest_rate: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingConfigSecdepositInterestRate1Row<'data> {
    pub fn interest_acct_id(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interest_acct_id.clone(),
        )
    }
}
impl mmsdm_core::GetTable for BillingConfigSecdepositInterestRate1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BILLING_CONFIG";
    const TABLE_NAME: &'static str = "SECDEPOSIT_INTEREST_RATE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingConfigSecdepositInterestRate1Mapping([
        4, 5, 6, 7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTEREST_ACCT_ID",
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "INTEREST_RATE",
    ];
    type Row<'row> = BillingConfigSecdepositInterestRate1Row<'row>;
    type FieldMapping = BillingConfigSecdepositInterestRate1Mapping;
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingConfigSecdepositInterestRate1Row {
            interest_acct_id: row.get_range("interest_acct_id", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interest_rate: row
                .get_opt_custom_parsed_at_idx(
                    "interest_rate",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(BillingConfigSecdepositInterestRate1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> BillingConfigSecdepositInterestRate1PrimaryKey {
        BillingConfigSecdepositInterestRate1PrimaryKey {
            effectivedate: row.effectivedate,
            interest_acct_id: row.interest_acct_id().to_string(),
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "billing_config_secdeposit_interest_rate_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingConfigSecdepositInterestRate1Row {
            interest_acct_id: row.interest_acct_id.clone(),
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            interest_rate: row.interest_rate.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigSecdepositInterestRate1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interest_acct_id: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BillingConfigSecdepositInterestRate1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for BillingConfigSecdepositInterestRate1Row<'data> {
    type Row<'other> = BillingConfigSecdepositInterestRate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interest_acct_id() == row.interest_acct_id()
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for BillingConfigSecdepositInterestRate1Row<'data> {
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interest_acct_id() == key.interest_acct_id
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow
for BillingConfigSecdepositInterestRate1PrimaryKey {
    type Row<'other> = BillingConfigSecdepositInterestRate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interest_acct_id == row.interest_acct_id()
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for BillingConfigSecdepositInterestRate1PrimaryKey {
    type PrimaryKey = BillingConfigSecdepositInterestRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interest_acct_id == key.interest_acct_id
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingConfigSecdepositInterestRate1 {
    type Builder = BillingConfigSecdepositInterestRate1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "interest_acct_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "version_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interest_rate",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingConfigSecdepositInterestRate1Builder {
            interest_acct_id_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interest_rate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.interest_acct_id_array.append_value(row.interest_acct_id());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .interest_rate_array
            .append_option({
                row.interest_rate
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.interest_acct_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interest_rate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingConfigSecdepositInterestRate1Builder {
    interest_acct_id_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interest_rate_array: arrow::array::builder::Decimal128Builder,
}
pub struct BillingConfigSecdepositProvision1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingConfigSecdepositProvision1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingConfigSecdepositProvision1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BillingConfigSecdepositProvision1Mapping([usize; 9]);
/// # Summary
///
/// ## SECDEPOSIT_PROVISION
///
/// The security deposit provision entry details
///
/// * Data Set Name: Billing Config
/// * File Name: Secdeposit Provision
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SECURITY_DEPOSIT_ID
#[derive(Debug, PartialEq, Eq)]
pub struct BillingConfigSecdepositProvision1Row<'data> {
    /// The security deposit ID
    pub security_deposit_id: core::ops::Range<usize>,
    /// The Participant ID linked to the security deposit ID
    pub participantid: core::ops::Range<usize>,
    /// The date the security deposit ID is entered and authorised by settlements
    pub transaction_date: Option<chrono::NaiveDateTime>,
    /// The contract year of the billing week when the security deposit is maturing
    pub maturity_contractyear: Option<rust_decimal::Decimal>,
    /// The week no of the billing week when the security deposit is maturing
    pub maturity_weekno: Option<rust_decimal::Decimal>,
    /// The security deposit amount
    pub amount: Option<rust_decimal::Decimal>,
    /// The interest rate assigned to the security deposit ID. Null if INTEREST_CALC_TYPE <>FIXED
    pub interest_rate: Option<rust_decimal::Decimal>,
    /// FIXED OR DAILY
    pub interest_calc_type: core::ops::Range<usize>,
    /// The Interest Account ID for calculating the Interest Payment. This is NULL if the INTEREST_CALC_TYPE = FIXED
    pub interest_acct_id: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingConfigSecdepositProvision1Row<'data> {
    pub fn security_deposit_id(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.security_deposit_id.clone(),
        )
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interest_calc_type(&self) -> Option<&str> {
        if self.interest_calc_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.interest_calc_type.clone(),
                ),
            )
        }
    }
    pub fn interest_acct_id(&self) -> Option<&str> {
        if self.interest_acct_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.interest_acct_id.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for BillingConfigSecdepositProvision1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BILLING_CONFIG";
    const TABLE_NAME: &'static str = "SECDEPOSIT_PROVISION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingConfigSecdepositProvision1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SECURITY_DEPOSIT_ID",
        "PARTICIPANTID",
        "TRANSACTION_DATE",
        "MATURITY_CONTRACTYEAR",
        "MATURITY_WEEKNO",
        "AMOUNT",
        "INTEREST_RATE",
        "INTEREST_CALC_TYPE",
        "INTEREST_ACCT_ID",
    ];
    type Row<'row> = BillingConfigSecdepositProvision1Row<'row>;
    type FieldMapping = BillingConfigSecdepositProvision1Mapping;
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingConfigSecdepositProvision1Row {
            security_deposit_id: row
                .get_range("security_deposit_id", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            transaction_date: row
                .get_opt_custom_parsed_at_idx(
                    "transaction_date",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            maturity_contractyear: row
                .get_opt_custom_parsed_at_idx(
                    "maturity_contractyear",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maturity_weekno: row
                .get_opt_custom_parsed_at_idx(
                    "maturity_weekno",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            amount: row
                .get_opt_custom_parsed_at_idx(
                    "amount",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interest_rate: row
                .get_opt_custom_parsed_at_idx(
                    "interest_rate",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interest_calc_type: row
                .get_opt_range("interest_calc_type", field_mapping.0[7])?,
            interest_acct_id: row.get_opt_range("interest_acct_id", field_mapping.0[8])?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(BillingConfigSecdepositProvision1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingConfigSecdepositProvision1PrimaryKey {
        BillingConfigSecdepositProvision1PrimaryKey {
            participantid: row.participantid().to_string(),
            security_deposit_id: row.security_deposit_id().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "billing_config_secdeposit_provision_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingConfigSecdepositProvision1Row {
            security_deposit_id: row.security_deposit_id.clone(),
            participantid: row.participantid.clone(),
            transaction_date: row.transaction_date.clone(),
            maturity_contractyear: row.maturity_contractyear.clone(),
            maturity_weekno: row.maturity_weekno.clone(),
            amount: row.amount.clone(),
            interest_rate: row.interest_rate.clone(),
            interest_calc_type: row.interest_calc_type.clone(),
            interest_acct_id: row.interest_acct_id.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingConfigSecdepositProvision1PrimaryKey {
    pub participantid: alloc::string::String,
    pub security_deposit_id: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for BillingConfigSecdepositProvision1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigSecdepositProvision1Row<'data> {
    type Row<'other> = BillingConfigSecdepositProvision1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid()
            && self.security_deposit_id() == row.security_deposit_id()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for BillingConfigSecdepositProvision1Row<'data> {
    type PrimaryKey = BillingConfigSecdepositProvision1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid
            && self.security_deposit_id() == key.security_deposit_id
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingConfigSecdepositProvision1PrimaryKey {
    type Row<'other> = BillingConfigSecdepositProvision1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid()
            && self.security_deposit_id == row.security_deposit_id()
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
    type Builder = BillingConfigSecdepositProvision1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "security_deposit_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "transaction_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maturity_contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maturity_weekno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interest_rate",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interest_calc_type",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interest_acct_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingConfigSecdepositProvision1Builder {
            security_deposit_id_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            transaction_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            maturity_contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            maturity_weekno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            interest_rate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            interest_calc_type_array: arrow::array::builder::StringBuilder::new(),
            interest_acct_id_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.security_deposit_id_array.append_value(row.security_deposit_id());
        builder.participantid_array.append_value(row.participantid());
        builder
            .transaction_date_array
            .append_option(
                row.transaction_date.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .maturity_contractyear_array
            .append_option({
                row.maturity_contractyear
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maturity_weekno_array
            .append_option({
                row.maturity_weekno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .amount_array
            .append_option({
                row.amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .interest_rate_array
            .append_option({
                row.interest_rate
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder.interest_calc_type_array.append_option(row.interest_calc_type());
        builder.interest_acct_id_array.append_option(row.interest_acct_id());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.security_deposit_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transaction_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maturity_contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maturity_weekno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interest_rate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interest_calc_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interest_acct_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingConfigSecdepositProvision1Builder {
    security_deposit_id_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    transaction_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    maturity_contractyear_array: arrow::array::builder::Decimal128Builder,
    maturity_weekno_array: arrow::array::builder::Decimal128Builder,
    amount_array: arrow::array::builder::Decimal128Builder,
    interest_rate_array: arrow::array::builder::Decimal128Builder,
    interest_calc_type_array: arrow::array::builder::StringBuilder,
    interest_acct_id_array: arrow::array::builder::StringBuilder,
}
