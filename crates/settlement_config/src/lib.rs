#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct SettlementConfigAncillaryRecoverySplit2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigAncillaryRecoverySplit2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigAncillaryRecoverySplit2 {
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
pub struct SettlementConfigAncillaryRecoverySplit2Mapping([usize; 7]);
/// # Summary
///
/// ## ANCILLARY_RECOVERY_SPLIT
///
/// ANCILLARY_RECOVERY_SPLIT holds the actual customer portion for each service and payment type. A single EFFECTIVEDATE/VERSIONNO combination applies to all services (i.e. the latest EFFECTIVEDATE/VERSIONNO is not retrieved for a single service, but applies to a data set).
///
/// * Data Set Name: Settlement Config
/// * File Name: Ancillary Recovery Split
/// * Data Version: 2
///
/// # Description
/// ANCILLARY_RECOVERY_SPLIT is public data, and is available to all participants.SourceThis table is updated infrequently.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PAYMENTTYPE
/// * SERVICE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigAncillaryRecoverySplit2Row<'data> {
    /// Calendar settlement date record becomes effective.
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the record for the given date.
    pub versionno: rust_decimal::Decimal,
    /// Ancillary service name (e.g. AGC, FCASCOMP)
    pub service: core::ops::Range<usize>,
    /// A payment type associated with the service (can be ENABLING, AVAILABILITY, USAGE, or COMPENSATION).
    pub paymenttype: core::ops::Range<usize>,
    /// The percentage value of the recovery funded by market customers.
    pub customer_portion: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The percentage value of the recovery funded using the ACE MWh Values. This field is only used for Settlement post IESS rule effective date.
    pub ace_portion: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigAncillaryRecoverySplit2Row<'data> {
    pub fn service(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.service.clone())
    }
    pub fn paymenttype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.paymenttype.clone())
    }
}
impl mmsdm_core::GetTable for SettlementConfigAncillaryRecoverySplit2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "ANCILLARY_RECOVERY_SPLIT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigAncillaryRecoverySplit2Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "SERVICE",
        "PAYMENTTYPE",
        "CUSTOMER_PORTION",
        "LASTCHANGED",
        "ACE_PORTION",
    ];
    type Row<'row> = SettlementConfigAncillaryRecoverySplit2Row<'row>;
    type FieldMapping = SettlementConfigAncillaryRecoverySplit2Mapping;
    type PrimaryKey = SettlementConfigAncillaryRecoverySplit2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigAncillaryRecoverySplit2Row {
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
            service: row.get_range("service", field_mapping.0[2])?,
            paymenttype: row.get_range("paymenttype", field_mapping.0[3])?,
            customer_portion: row
                .get_opt_custom_parsed_at_idx(
                    "customer_portion",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            ace_portion: row
                .get_opt_custom_parsed_at_idx(
                    "ace_portion",
                    field_mapping.0[6],
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
        Ok(SettlementConfigAncillaryRecoverySplit2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigAncillaryRecoverySplit2PrimaryKey {
        SettlementConfigAncillaryRecoverySplit2PrimaryKey {
            effectivedate: row.effectivedate,
            paymenttype: row.paymenttype().to_string(),
            service: row.service().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_ancillary_recovery_split_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigAncillaryRecoverySplit2Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            service: row.service.clone(),
            paymenttype: row.paymenttype.clone(),
            customer_portion: row.customer_portion.clone(),
            lastchanged: row.lastchanged.clone(),
            ace_portion: row.ace_portion.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigAncillaryRecoverySplit2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub paymenttype: alloc::string::String,
    pub service: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigAncillaryRecoverySplit2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigAncillaryRecoverySplit2Row<'data> {
    type Row<'other> = SettlementConfigAncillaryRecoverySplit2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.paymenttype() == row.paymenttype() && self.service() == row.service()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigAncillaryRecoverySplit2Row<'data> {
    type PrimaryKey = SettlementConfigAncillaryRecoverySplit2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.paymenttype() == key.paymenttype
            && self.service() == key.service && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigAncillaryRecoverySplit2PrimaryKey {
    type Row<'other> = SettlementConfigAncillaryRecoverySplit2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.paymenttype == row.paymenttype()
            && self.service == row.service() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigAncillaryRecoverySplit2PrimaryKey {
    type PrimaryKey = SettlementConfigAncillaryRecoverySplit2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.paymenttype == key.paymenttype
            && self.service == key.service && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigAncillaryRecoverySplit2 {
    type Builder = SettlementConfigAncillaryRecoverySplit2Builder;
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
                    "service",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "paymenttype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "customer_portion",
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
                arrow::datatypes::Field::new(
                    "ace_portion",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementConfigAncillaryRecoverySplit2Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            service_array: arrow::array::builder::StringBuilder::new(),
            paymenttype_array: arrow::array::builder::StringBuilder::new(),
            customer_portion_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            ace_portion_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
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
        builder.service_array.append_value(row.service());
        builder.paymenttype_array.append_value(row.paymenttype());
        builder
            .customer_portion_array
            .append_option({
                row.customer_portion
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .ace_portion_array
            .append_option({
                row.ace_portion
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
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.service_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.paymenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.customer_portion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ace_portion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigAncillaryRecoverySplit2Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    service_array: arrow::array::builder::StringBuilder,
    paymenttype_array: arrow::array::builder::StringBuilder,
    customer_portion_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    ace_portion_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementConfigMarketFeeCatExcl1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigMarketFeeCatExcl1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigMarketFeeCatExcl1 {
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
pub struct SettlementConfigMarketFeeCatExcl1Mapping([usize; 4]);
/// # Summary
///
/// ## MARKET_FEE_CAT_EXCL
///
/// Market fee exclusions for participant categories.
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Cat Excl
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANT_CATEGORYID
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigMarketFeeCatExcl1Row<'data> {
    /// The excluded market fee
    pub marketfeeid: core::ops::Range<usize>,
    /// The date the exclusion is effective from
    pub effectivedate: chrono::NaiveDateTime,
    /// The version information for this record
    pub version_datetime: chrono::NaiveDateTime,
    /// Participant category to be excluded from this market fee
    pub participant_categoryid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigMarketFeeCatExcl1Row<'data> {
    pub fn marketfeeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.marketfeeid.clone())
    }
    pub fn participant_categoryid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participant_categoryid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for SettlementConfigMarketFeeCatExcl1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "MARKET_FEE_CAT_EXCL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigMarketFeeCatExcl1Mapping([
        4, 5, 6, 7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MARKETFEEID",
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "PARTICIPANT_CATEGORYID",
    ];
    type Row<'row> = SettlementConfigMarketFeeCatExcl1Row<'row>;
    type FieldMapping = SettlementConfigMarketFeeCatExcl1Mapping;
    type PrimaryKey = SettlementConfigMarketFeeCatExcl1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigMarketFeeCatExcl1Row {
            marketfeeid: row.get_range("marketfeeid", field_mapping.0[0])?,
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
            participant_categoryid: row
                .get_range("participant_categoryid", field_mapping.0[3])?,
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
        Ok(SettlementConfigMarketFeeCatExcl1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementConfigMarketFeeCatExcl1PrimaryKey {
        SettlementConfigMarketFeeCatExcl1PrimaryKey {
            effectivedate: row.effectivedate,
            marketfeeid: row.marketfeeid().to_string(),
            participant_categoryid: row.participant_categoryid().to_string(),
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_market_fee_cat_excl_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigMarketFeeCatExcl1Row {
            marketfeeid: row.marketfeeid.clone(),
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            participant_categoryid: row.participant_categoryid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigMarketFeeCatExcl1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: alloc::string::String,
    pub participant_categoryid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementConfigMarketFeeCatExcl1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketFeeCatExcl1Row<'data> {
    type Row<'other> = SettlementConfigMarketFeeCatExcl1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid() == row.marketfeeid()
            && self.participant_categoryid() == row.participant_categoryid()
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketFeeCatExcl1Row<'data> {
    type PrimaryKey = SettlementConfigMarketFeeCatExcl1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid() == key.marketfeeid
            && self.participant_categoryid() == key.participant_categoryid
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketFeeCatExcl1PrimaryKey {
    type Row<'other> = SettlementConfigMarketFeeCatExcl1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.marketfeeid == row.marketfeeid()
            && self.participant_categoryid == row.participant_categoryid()
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementConfigMarketFeeCatExcl1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeCatExcl1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid == key.marketfeeid
            && self.participant_categoryid == key.participant_categoryid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigMarketFeeCatExcl1 {
    type Builder = SettlementConfigMarketFeeCatExcl1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "marketfeeid",
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
                    "participant_categoryid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementConfigMarketFeeCatExcl1Builder {
            marketfeeid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participant_categoryid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.marketfeeid_array.append_value(row.marketfeeid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder.participant_categoryid_array.append_value(row.participant_categoryid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.marketfeeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participant_categoryid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigMarketFeeCatExcl1Builder {
    marketfeeid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    participant_categoryid_array: arrow::array::builder::StringBuilder,
}
pub struct SettlementConfigMarketFeeCatExclTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigMarketFeeCatExclTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigMarketFeeCatExclTrk1 {
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
pub struct SettlementConfigMarketFeeCatExclTrk1Mapping([usize; 4]);
/// # Summary
///
/// ## MARKET_FEE_CAT_EXCL_TRK
///
/// Tracking table for market fee exclusions for participant categories.
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Cat Excl Trk
/// * Data Version: 1
///
/// # Description
/// MARKET_FEE_EXCLUSION data is confidential to the relevant participant.SourceMARKET_FEE_EXCLUSION updates only on change of participant configuration.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigMarketFeeCatExclTrk1Row<'data> {
    /// The excluded market fee
    pub marketfeeid: core::ops::Range<usize>,
    /// The date the exclusion is effective from
    pub effectivedate: chrono::NaiveDateTime,
    /// The version information for this record
    pub version_datetime: chrono::NaiveDateTime,
    /// Last date and time the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigMarketFeeCatExclTrk1Row<'data> {
    pub fn marketfeeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.marketfeeid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementConfigMarketFeeCatExclTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "MARKET_FEE_CAT_EXCL_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigMarketFeeCatExclTrk1Mapping([
        4, 5, 6, 7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MARKETFEEID",
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigMarketFeeCatExclTrk1Row<'row>;
    type FieldMapping = SettlementConfigMarketFeeCatExclTrk1Mapping;
    type PrimaryKey = SettlementConfigMarketFeeCatExclTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigMarketFeeCatExclTrk1Row {
            marketfeeid: row.get_range("marketfeeid", field_mapping.0[0])?,
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
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[3],
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
        Ok(SettlementConfigMarketFeeCatExclTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
        SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
            effectivedate: row.effectivedate,
            marketfeeid: row.marketfeeid().to_string(),
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_market_fee_cat_excl_trk_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigMarketFeeCatExclTrk1Row {
            marketfeeid: row.marketfeeid.clone(),
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementConfigMarketFeeCatExclTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigMarketFeeCatExclTrk1Row<'data> {
    type Row<'other> = SettlementConfigMarketFeeCatExclTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid() == row.marketfeeid()
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketFeeCatExclTrk1Row<'data> {
    type PrimaryKey = SettlementConfigMarketFeeCatExclTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid() == key.marketfeeid
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
    type Row<'other> = SettlementConfigMarketFeeCatExclTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.marketfeeid == row.marketfeeid()
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeCatExclTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid == key.marketfeeid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigMarketFeeCatExclTrk1 {
    type Builder = SettlementConfigMarketFeeCatExclTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "marketfeeid",
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
        SettlementConfigMarketFeeCatExclTrk1Builder {
            marketfeeid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.marketfeeid_array.append_value(row.marketfeeid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
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
                    alloc::sync::Arc::new(builder.marketfeeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigMarketFeeCatExclTrk1Builder {
    marketfeeid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementConfigMarketFeeExclusion1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigMarketFeeExclusion1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigMarketFeeExclusion1 {
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
pub struct SettlementConfigMarketFeeExclusion1Mapping([usize; 5]);
/// # Summary
///
/// ## MARKET_FEE_EXCLUSION
///
/// MARKET_FEE_EXCLUSION shows the list of market fees from which a participant is excluded from funding after a particular settlement date.
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Exclusion
/// * Data Version: 1
///
/// # Description
/// MARKET_FEE_EXCLUSION data is confidential to the relevant participant.SourceMARKET_FEE_EXCLUSION updates only on change of participant configuration.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigMarketFeeExclusion1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Date on which this data becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    pub versionno: rust_decimal::Decimal,
    /// Identifier for Market Fee
    pub marketfeeid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigMarketFeeExclusion1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn marketfeeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.marketfeeid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementConfigMarketFeeExclusion1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "MARKET_FEE_EXCLUSION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigMarketFeeExclusion1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "MARKETFEEID",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigMarketFeeExclusion1Row<'row>;
    type FieldMapping = SettlementConfigMarketFeeExclusion1Mapping;
    type PrimaryKey = SettlementConfigMarketFeeExclusion1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigMarketFeeExclusion1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marketfeeid: row.get_range("marketfeeid", field_mapping.0[3])?,
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
        Ok(SettlementConfigMarketFeeExclusion1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigMarketFeeExclusion1PrimaryKey {
        SettlementConfigMarketFeeExclusion1PrimaryKey {
            effectivedate: row.effectivedate,
            marketfeeid: row.marketfeeid().to_string(),
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_market_fee_exclusion_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigMarketFeeExclusion1Row {
            participantid: row.participantid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            marketfeeid: row.marketfeeid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigMarketFeeExclusion1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigMarketFeeExclusion1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigMarketFeeExclusion1Row<'data> {
    type Row<'other> = SettlementConfigMarketFeeExclusion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid() == row.marketfeeid()
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketFeeExclusion1Row<'data> {
    type PrimaryKey = SettlementConfigMarketFeeExclusion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid() == key.marketfeeid
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigMarketFeeExclusion1PrimaryKey {
    type Row<'other> = SettlementConfigMarketFeeExclusion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.marketfeeid == row.marketfeeid()
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketFeeExclusion1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeExclusion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid == key.marketfeeid
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigMarketFeeExclusion1 {
    type Builder = SettlementConfigMarketFeeExclusion1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marketfeeid",
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
        SettlementConfigMarketFeeExclusion1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            marketfeeid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
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
        builder.marketfeeid_array.append_value(row.marketfeeid());
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
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigMarketFeeExclusion1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    marketfeeid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementConfigMarketFeeExclusionTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigMarketFeeExclusionTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigMarketFeeExclusionTrk1 {
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
pub struct SettlementConfigMarketFeeExclusionTrk1Mapping([usize; 6]);
/// # Summary
///
/// ## MARKET_FEE_EXCLUSIONTRK
///
/// MARKET_FEE_EXCLUSIONTRK shows authorisation details of participant market fee exclusion data sets.
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Exclusion Trk
/// * Data Version: 1
///
/// # Description
/// MARKET_FEE_EXCLUSIONTRK is confidential to the participant.SourceMARKET_FEE_EXCLUSIONTRK updates only on change of participant configuration.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigMarketFeeExclusionTrk1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Date on which this data becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    pub versionno: rust_decimal::Decimal,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigMarketFeeExclusionTrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn authorisedby(&self) -> Option<&str> {
        if self.authorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for SettlementConfigMarketFeeExclusionTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "MARKET_FEE_EXCLUSION_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigMarketFeeExclusionTrk1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigMarketFeeExclusionTrk1Row<'row>;
    type FieldMapping = SettlementConfigMarketFeeExclusionTrk1Mapping;
    type PrimaryKey = SettlementConfigMarketFeeExclusionTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigMarketFeeExclusionTrk1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[3])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
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
        Ok(SettlementConfigMarketFeeExclusionTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
        SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
            effectivedate: row.effectivedate,
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_market_fee_exclusion_trk_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigMarketFeeExclusionTrk1Row {
            participantid: row.participantid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigMarketFeeExclusionTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigMarketFeeExclusionTrk1Row<'data> {
    type Row<'other> = SettlementConfigMarketFeeExclusionTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketFeeExclusionTrk1Row<'data> {
    type PrimaryKey = SettlementConfigMarketFeeExclusionTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
    type Row<'other> = SettlementConfigMarketFeeExclusionTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeExclusionTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigMarketFeeExclusionTrk1 {
    type Builder = SettlementConfigMarketFeeExclusionTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementConfigMarketFeeExclusionTrk1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
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
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
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
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigMarketFeeExclusionTrk1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementConfigMarketfee2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigMarketfee2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigMarketfee2 {
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
pub struct SettlementConfigMarketfee2Mapping([usize; 10]);
/// # Summary
///
/// ## MARKETFEE
///
/// MARKETFEE sets out fee type and period for each market fee.
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfee
/// * Data Version: 2
///
/// # Description
/// MARKETFEE data is public, so is available to all participants.SourceMARKETFEE updates when fees change.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * MARKETFEEID
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigMarketfee2Row<'data> {
    /// Identifier for Market Fee
    pub marketfeeid: core::ops::Range<usize>,
    /// Period type - PERIOD, DAILY, WEEKLY
    pub marketfeeperiod: core::ops::Range<usize>,
    /// Type - MW or $
    pub marketfeetype: core::ops::Range<usize>,
    /// Description of market fee
    pub description: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    pub gl_tcode: core::ops::Range<usize>,
    pub gl_financialcode: core::ops::Range<usize>,
    pub fee_class: core::ops::Range<usize>,
    /// The Energy Type for the Market Fees Calculation. E.g of Meter Types are CUSTOMER, GENERATOR, NREG, BDU etc. If Meter Type is mentioned as ALL then all the Meter Types for that Participant Category will be used in the Fee calculation
    pub meter_type: core::ops::Range<usize>,
    /// The Meter Sub Type values are ACE, ASOE or ALL. ACE represent ACE_MWH value , ASOE represent ASOE_MWH value and ALL represent sum of ACE_MWh and ASOE_MWh
    pub meter_subtype: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigMarketfee2Row<'data> {
    pub fn marketfeeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.marketfeeid.clone())
    }
    pub fn marketfeeperiod(&self) -> Option<&str> {
        if self.marketfeeperiod.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.marketfeeperiod.clone(),
                ),
            )
        }
    }
    pub fn marketfeetype(&self) -> Option<&str> {
        if self.marketfeetype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.marketfeetype.clone(),
                ),
            )
        }
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
    pub fn fee_class(&self) -> Option<&str> {
        if self.fee_class.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.fee_class.clone(),
                ),
            )
        }
    }
    pub fn meter_type(&self) -> Option<&str> {
        if self.meter_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.meter_type.clone(),
                ),
            )
        }
    }
    pub fn meter_subtype(&self) -> Option<&str> {
        if self.meter_subtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.meter_subtype.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for SettlementConfigMarketfee2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "MARKETFEE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigMarketfee2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MARKETFEEID",
        "MARKETFEEPERIOD",
        "MARKETFEETYPE",
        "DESCRIPTION",
        "LASTCHANGED",
        "GL_TCODE",
        "GL_FINANCIALCODE",
        "FEE_CLASS",
        "METER_TYPE",
        "METER_SUBTYPE",
    ];
    type Row<'row> = SettlementConfigMarketfee2Row<'row>;
    type FieldMapping = SettlementConfigMarketfee2Mapping;
    type PrimaryKey = SettlementConfigMarketfee2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigMarketfee2Row {
            marketfeeid: row.get_range("marketfeeid", field_mapping.0[0])?,
            marketfeeperiod: row.get_opt_range("marketfeeperiod", field_mapping.0[1])?,
            marketfeetype: row.get_opt_range("marketfeetype", field_mapping.0[2])?,
            description: row.get_opt_range("description", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            gl_tcode: row.get_opt_range("gl_tcode", field_mapping.0[5])?,
            gl_financialcode: row.get_opt_range("gl_financialcode", field_mapping.0[6])?,
            fee_class: row.get_opt_range("fee_class", field_mapping.0[7])?,
            meter_type: row.get_opt_range("meter_type", field_mapping.0[8])?,
            meter_subtype: row.get_opt_range("meter_subtype", field_mapping.0[9])?,
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
        Ok(SettlementConfigMarketfee2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementConfigMarketfee2PrimaryKey {
        SettlementConfigMarketfee2PrimaryKey {
            marketfeeid: row.marketfeeid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("settlement_config_marketfee_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigMarketfee2Row {
            marketfeeid: row.marketfeeid.clone(),
            marketfeeperiod: row.marketfeeperiod.clone(),
            marketfeetype: row.marketfeetype.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            gl_tcode: row.gl_tcode.clone(),
            gl_financialcode: row.gl_financialcode.clone(),
            fee_class: row.fee_class.clone(),
            meter_type: row.meter_type.clone(),
            meter_subtype: row.meter_subtype.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigMarketfee2PrimaryKey {
    pub marketfeeid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SettlementConfigMarketfee2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketfee2Row<'data> {
    type Row<'other> = SettlementConfigMarketfee2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.marketfeeid() == row.marketfeeid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementConfigMarketfee2Row<'data> {
    type PrimaryKey = SettlementConfigMarketfee2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid() == key.marketfeeid
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketfee2PrimaryKey {
    type Row<'other> = SettlementConfigMarketfee2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.marketfeeid == row.marketfeeid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementConfigMarketfee2PrimaryKey {
    type PrimaryKey = SettlementConfigMarketfee2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid == key.marketfeeid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigMarketfee2 {
    type Builder = SettlementConfigMarketfee2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "marketfeeid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marketfeeperiod",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marketfeetype",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
                arrow::datatypes::Field::new(
                    "gl_tcode",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gl_financialcode",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fee_class",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meter_type",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meter_subtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementConfigMarketfee2Builder {
            marketfeeid_array: arrow::array::builder::StringBuilder::new(),
            marketfeeperiod_array: arrow::array::builder::StringBuilder::new(),
            marketfeetype_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            gl_tcode_array: arrow::array::builder::StringBuilder::new(),
            gl_financialcode_array: arrow::array::builder::StringBuilder::new(),
            fee_class_array: arrow::array::builder::StringBuilder::new(),
            meter_type_array: arrow::array::builder::StringBuilder::new(),
            meter_subtype_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.marketfeeid_array.append_value(row.marketfeeid());
        builder.marketfeeperiod_array.append_option(row.marketfeeperiod());
        builder.marketfeetype_array.append_option(row.marketfeetype());
        builder.description_array.append_option(row.description());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.gl_tcode_array.append_option(row.gl_tcode());
        builder.gl_financialcode_array.append_option(row.gl_financialcode());
        builder.fee_class_array.append_option(row.fee_class());
        builder.meter_type_array.append_option(row.meter_type());
        builder.meter_subtype_array.append_option(row.meter_subtype());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.marketfeeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeeperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeetype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gl_tcode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gl_financialcode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fee_class_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_subtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigMarketfee2Builder {
    marketfeeid_array: arrow::array::builder::StringBuilder,
    marketfeeperiod_array: arrow::array::builder::StringBuilder,
    marketfeetype_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    gl_tcode_array: arrow::array::builder::StringBuilder,
    gl_financialcode_array: arrow::array::builder::StringBuilder,
    fee_class_array: arrow::array::builder::StringBuilder,
    meter_type_array: arrow::array::builder::StringBuilder,
    meter_subtype_array: arrow::array::builder::StringBuilder,
}
pub struct SettlementConfigMarketfeedata1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigMarketfeedata1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigMarketfeedata1 {
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
pub struct SettlementConfigMarketfeedata1Mapping([usize; 5]);
/// # Summary
///
/// ## MARKETFEEDATA
///
/// MARKETFEEDATA sets out actual fee rates, as adjusted from time to time.
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfeedata
/// * Data Version: 1
///
/// # Description
/// MARKETFEEDATA is public data, and is available to all participants.SourceMARKETFEEDATA updates whenever fee rates change.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * MARKETFEEVERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigMarketfeedata1Row<'data> {
    /// Identifier for Market Fee
    pub marketfeeid: core::ops::Range<usize>,
    /// Version of fees for this id
    pub marketfeeversionno: rust_decimal::Decimal,
    /// Date on which this data becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Market fee rate/MWh, a dollar amount
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigMarketfeedata1Row<'data> {
    pub fn marketfeeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.marketfeeid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementConfigMarketfeedata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "MARKETFEEDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigMarketfeedata1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MARKETFEEID",
        "MARKETFEEVERSIONNO",
        "EFFECTIVEDATE",
        "MARKETFEEVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigMarketfeedata1Row<'row>;
    type FieldMapping = SettlementConfigMarketfeedata1Mapping;
    type PrimaryKey = SettlementConfigMarketfeedata1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigMarketfeedata1Row {
            marketfeeid: row.get_range("marketfeeid", field_mapping.0[0])?,
            marketfeeversionno: row
                .get_custom_parsed_at_idx(
                    "marketfeeversionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            marketfeevalue: row
                .get_opt_custom_parsed_at_idx(
                    "marketfeevalue",
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
        Ok(SettlementConfigMarketfeedata1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementConfigMarketfeedata1PrimaryKey {
        SettlementConfigMarketfeedata1PrimaryKey {
            effectivedate: row.effectivedate,
            marketfeeid: row.marketfeeid().to_string(),
            marketfeeversionno: row.marketfeeversionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_marketfeedata_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigMarketfeedata1Row {
            marketfeeid: row.marketfeeid.clone(),
            marketfeeversionno: row.marketfeeversionno.clone(),
            effectivedate: row.effectivedate.clone(),
            marketfeevalue: row.marketfeevalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigMarketfeedata1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: alloc::string::String,
    pub marketfeeversionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigMarketfeedata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketfeedata1Row<'data> {
    type Row<'other> = SettlementConfigMarketfeedata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid() == row.marketfeeid()
            && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketfeedata1Row<'data> {
    type PrimaryKey = SettlementConfigMarketfeedata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid() == key.marketfeeid
            && self.marketfeeversionno == key.marketfeeversionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketfeedata1PrimaryKey {
    type Row<'other> = SettlementConfigMarketfeedata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.marketfeeid == row.marketfeeid()
            && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementConfigMarketfeedata1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketfeedata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid == key.marketfeeid
            && self.marketfeeversionno == key.marketfeeversionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigMarketfeedata1 {
    type Builder = SettlementConfigMarketfeedata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "marketfeeid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marketfeeversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
                    "marketfeevalue",
                    arrow::datatypes::DataType::Decimal128(22, 8),
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
        SettlementConfigMarketfeedata1Builder {
            marketfeeid_array: arrow::array::builder::StringBuilder::new(),
            marketfeeversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            marketfeevalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.marketfeeid_array.append_value(row.marketfeeid());
        builder
            .marketfeeversionno_array
            .append_value({
                let mut val = row.marketfeeversionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .marketfeevalue_array
            .append_option({
                row.marketfeevalue
                    .map(|mut val| {
                        val.rescale(8);
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
                    alloc::sync::Arc::new(builder.marketfeeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeeversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeevalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigMarketfeedata1Builder {
    marketfeeid_array: arrow::array::builder::StringBuilder,
    marketfeeversionno_array: arrow::array::builder::Decimal128Builder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    marketfeevalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementConfigMarketfeetrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigMarketfeetrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigMarketfeetrk1 {
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
pub struct SettlementConfigMarketfeetrk1Mapping([usize; 5]);
/// # Summary
///
/// ## MARKETFEETRK
///
/// MARKETFEETRK sets out versions of each market fee used and its effective date.
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfeetrk
/// * Data Version: 1
///
/// # Description
/// MARKETFEETRK data is public, so is available to all participants.SourceMARKETFEETRK updated infrequently, when new annual rates must be inserted.VolumeOne record inserted per year.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEVERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigMarketfeetrk1Row<'data> {
    /// Version of fees for this ID
    pub marketfeeversionno: rust_decimal::Decimal,
    /// Effective Date of Market notice
    pub effectivedate: chrono::NaiveDateTime,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigMarketfeetrk1Row<'data> {
    pub fn authorisedby(&self) -> Option<&str> {
        if self.authorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for SettlementConfigMarketfeetrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "MARKETFEETRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigMarketfeetrk1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MARKETFEEVERSIONNO",
        "EFFECTIVEDATE",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigMarketfeetrk1Row<'row>;
    type FieldMapping = SettlementConfigMarketfeetrk1Mapping;
    type PrimaryKey = SettlementConfigMarketfeetrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigMarketfeetrk1Row {
            marketfeeversionno: row
                .get_custom_parsed_at_idx(
                    "marketfeeversionno",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[2])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(SettlementConfigMarketfeetrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementConfigMarketfeetrk1PrimaryKey {
        SettlementConfigMarketfeetrk1PrimaryKey {
            effectivedate: row.effectivedate,
            marketfeeversionno: row.marketfeeversionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("settlement_config_marketfeetrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigMarketfeetrk1Row {
            marketfeeversionno: row.marketfeeversionno.clone(),
            effectivedate: row.effectivedate.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigMarketfeetrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeversionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigMarketfeetrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketfeetrk1Row<'data> {
    type Row<'other> = SettlementConfigMarketfeetrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigMarketfeetrk1Row<'data> {
    type PrimaryKey = SettlementConfigMarketfeetrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeversionno == key.marketfeeversionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementConfigMarketfeetrk1PrimaryKey {
    type Row<'other> = SettlementConfigMarketfeetrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementConfigMarketfeetrk1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketfeetrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeversionno == key.marketfeeversionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigMarketfeetrk1 {
    type Builder = SettlementConfigMarketfeetrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "marketfeeversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementConfigMarketfeetrk1Builder {
            marketfeeversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .marketfeeversionno_array
            .append_value({
                let mut val = row.marketfeeversionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
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
                    alloc::sync::Arc::new(builder.marketfeeversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigMarketfeetrk1Builder {
    marketfeeversionno_array: arrow::array::builder::Decimal128Builder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementConfigParticipantBandfeeAlloc1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigParticipantBandfeeAlloc1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigParticipantBandfeeAlloc1 {
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
pub struct SettlementConfigParticipantBandfeeAlloc1Mapping([usize; 7]);
/// # Summary
///
/// ## PARTICIPANT_BANDFEE_ALLOC
///
/// PARTICIPANT_BANDFEE_ALLOC shows the market fee for each Participant/Participant Category over time.
///
/// * Data Set Name: Settlement Config
/// * File Name: Participant Bandfee Alloc
/// * Data Version: 1
///
/// # Description
/// SourceThis view updates only on change of participant configuration.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigParticipantBandfeeAlloc1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Identifier for Market Fee
    pub marketfeeid: core::ops::Range<usize>,
    /// Date on which this data becomes effective.
    pub effectivedate: chrono::NaiveDateTime,
    /// Period identifier
    pub versionno: rust_decimal::Decimal,
    /// The participant category that the market fee recovery amount pertains to.
    pub participantcategoryid: core::ops::Range<usize>,
    /// The value of this market fee
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigParticipantBandfeeAlloc1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn marketfeeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.marketfeeid.clone())
    }
    pub fn participantcategoryid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participantcategoryid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for SettlementConfigParticipantBandfeeAlloc1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "PARTICIPANT_BANDFEE_ALLOC";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigParticipantBandfeeAlloc1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "MARKETFEEID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PARTICIPANTCATEGORYID",
        "MARKETFEEVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigParticipantBandfeeAlloc1Row<'row>;
    type FieldMapping = SettlementConfigParticipantBandfeeAlloc1Mapping;
    type PrimaryKey = SettlementConfigParticipantBandfeeAlloc1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigParticipantBandfeeAlloc1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            marketfeeid: row.get_range("marketfeeid", field_mapping.0[1])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantcategoryid: row
                .get_range("participantcategoryid", field_mapping.0[4])?,
            marketfeevalue: row
                .get_opt_custom_parsed_at_idx(
                    "marketfeevalue",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
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
        Ok(SettlementConfigParticipantBandfeeAlloc1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
        SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
            effectivedate: row.effectivedate,
            marketfeeid: row.marketfeeid().to_string(),
            participantcategoryid: row.participantcategoryid().to_string(),
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_participant_bandfee_alloc_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigParticipantBandfeeAlloc1Row {
            participantid: row.participantid.clone(),
            marketfeeid: row.marketfeeid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            participantcategoryid: row.participantcategoryid.clone(),
            marketfeevalue: row.marketfeevalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: alloc::string::String,
    pub participantcategoryid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigParticipantBandfeeAlloc1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigParticipantBandfeeAlloc1Row<'data> {
    type Row<'other> = SettlementConfigParticipantBandfeeAlloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid() == row.marketfeeid()
            && self.participantcategoryid() == row.participantcategoryid()
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigParticipantBandfeeAlloc1Row<'data> {
    type PrimaryKey = SettlementConfigParticipantBandfeeAlloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid() == key.marketfeeid
            && self.participantcategoryid() == key.participantcategoryid
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
    type Row<'other> = SettlementConfigParticipantBandfeeAlloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.marketfeeid == row.marketfeeid()
            && self.participantcategoryid == row.participantcategoryid()
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
    type PrimaryKey = SettlementConfigParticipantBandfeeAlloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigParticipantBandfeeAlloc1 {
    type Builder = SettlementConfigParticipantBandfeeAlloc1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marketfeeid",
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantcategoryid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marketfeevalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
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
        SettlementConfigParticipantBandfeeAlloc1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            marketfeeid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantcategoryid_array: arrow::array::builder::StringBuilder::new(),
            marketfeevalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
        builder.marketfeeid_array.append_value(row.marketfeeid());
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
        builder.participantcategoryid_array.append_value(row.participantcategoryid());
        builder
            .marketfeevalue_array
            .append_option({
                row.marketfeevalue
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
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantcategoryid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeevalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigParticipantBandfeeAlloc1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    marketfeeid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantcategoryid_array: arrow::array::builder::StringBuilder,
    marketfeevalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SetcfgReallocation2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SetcfgReallocation2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SetcfgReallocation2 {
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
pub struct SetcfgReallocation2Mapping([usize; 15]);
/// # Summary
///
/// ## REALLOCATION
///
/// The REALLOCATION table shows the financial transactions agreed between two participants that are settled through the AEMO pool settlements process.
///
/// * Data Set Name: Setcfg
/// * File Name: Reallocation
/// * Data Version: 2
///
/// # Description
/// NoteThe column REALLOCATION_TYPE can be used in conjunction with CREDITPARTICIPANT or DEBITPARTICIPANT to determine who submitted a reallocation.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * REALLOCATIONID
#[derive(Debug, PartialEq, Eq)]
pub struct SetcfgReallocation2Row<'data> {
    /// Reallocation identifier
    pub reallocationid: core::ops::Range<usize>,
    /// The participant to be credited for the reallocation
    pub creditparticipantid: core::ops::Range<usize>,
    /// The participant to be debited for the reallocation
    pub debitparticipantid: core::ops::Range<usize>,
    /// Region identifier, being the spot price reference node for this reallocation
    pub regionid: core::ops::Range<usize>,
    /// $, (Quantity) Mwh, SWAP, CAP or FLOOR
    pub agreementtype: core::ops::Range<usize>,
    /// Optional reference detail for credit participant
    pub creditreference: core::ops::Range<usize>,
    /// Optional reference detail for debit participant
    pub debitreference: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// First day of the Reallocation contract
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last day of the Reallocation contract
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Reallocation state. One of SUBMITTED, AUTHORISED, CANCELLED.
    pub current_stepid: core::ops::Range<usize>,
    /// The day type profile for which the reallocation applies over the start and end date range. Valid entries are BUSINESS, NON_BUSINESS or FLAT.
    pub daytype: core::ops::Range<usize>,
    /// Denotes a Credit or Debit reallocation with a value of "C"or "D"respectively
    pub reallocation_type: core::ops::Range<usize>,
    /// Unique ID of the calendar for which data is requested
    pub calendarid: core::ops::Range<usize>,
    /// The length of settlement intervals (in minutes) in the reallocation profile
    pub intervallength: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SetcfgReallocation2Row<'data> {
    pub fn reallocationid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.reallocationid.clone(),
        )
    }
    pub fn creditparticipantid(&self) -> Option<&str> {
        if self.creditparticipantid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.creditparticipantid.clone(),
                ),
            )
        }
    }
    pub fn debitparticipantid(&self) -> Option<&str> {
        if self.debitparticipantid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.debitparticipantid.clone(),
                ),
            )
        }
    }
    pub fn regionid(&self) -> Option<&str> {
        if self.regionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionid.clone(),
                ),
            )
        }
    }
    pub fn agreementtype(&self) -> Option<&str> {
        if self.agreementtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.agreementtype.clone(),
                ),
            )
        }
    }
    pub fn creditreference(&self) -> Option<&str> {
        if self.creditreference.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.creditreference.clone(),
                ),
            )
        }
    }
    pub fn debitreference(&self) -> Option<&str> {
        if self.debitreference.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.debitreference.clone(),
                ),
            )
        }
    }
    pub fn current_stepid(&self) -> Option<&str> {
        if self.current_stepid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.current_stepid.clone(),
                ),
            )
        }
    }
    pub fn daytype(&self) -> Option<&str> {
        if self.daytype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.daytype.clone(),
                ),
            )
        }
    }
    pub fn reallocation_type(&self) -> Option<&str> {
        if self.reallocation_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reallocation_type.clone(),
                ),
            )
        }
    }
    pub fn calendarid(&self) -> Option<&str> {
        if self.calendarid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.calendarid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for SetcfgReallocation2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "SETCFG";
    const TABLE_NAME: &'static str = "REALLOCATION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SetcfgReallocation2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "REALLOCATIONID",
        "CREDITPARTICIPANTID",
        "DEBITPARTICIPANTID",
        "REGIONID",
        "AGREEMENTTYPE",
        "CREDITREFERENCE",
        "DEBITREFERENCE",
        "LASTCHANGED",
        "STARTDATE",
        "ENDDATE",
        "CURRENT_STEPID",
        "DAYTYPE",
        "REALLOCATION_TYPE",
        "CALENDARID",
        "INTERVALLENGTH",
    ];
    type Row<'row> = SetcfgReallocation2Row<'row>;
    type FieldMapping = SetcfgReallocation2Mapping;
    type PrimaryKey = SetcfgReallocation2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SetcfgReallocation2Row {
            reallocationid: row.get_range("reallocationid", field_mapping.0[0])?,
            creditparticipantid: row
                .get_opt_range("creditparticipantid", field_mapping.0[1])?,
            debitparticipantid: row
                .get_opt_range("debitparticipantid", field_mapping.0[2])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[3])?,
            agreementtype: row.get_opt_range("agreementtype", field_mapping.0[4])?,
            creditreference: row.get_opt_range("creditreference", field_mapping.0[5])?,
            debitreference: row.get_opt_range("debitreference", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            current_stepid: row.get_opt_range("current_stepid", field_mapping.0[10])?,
            daytype: row.get_opt_range("daytype", field_mapping.0[11])?,
            reallocation_type: row
                .get_opt_range("reallocation_type", field_mapping.0[12])?,
            calendarid: row.get_opt_range("calendarid", field_mapping.0[13])?,
            intervallength: row
                .get_opt_custom_parsed_at_idx(
                    "intervallength",
                    field_mapping.0[14],
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
        Ok(SetcfgReallocation2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SetcfgReallocation2PrimaryKey {
        SetcfgReallocation2PrimaryKey {
            reallocationid: row.reallocationid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("setcfg_reallocation_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SetcfgReallocation2Row {
            reallocationid: row.reallocationid.clone(),
            creditparticipantid: row.creditparticipantid.clone(),
            debitparticipantid: row.debitparticipantid.clone(),
            regionid: row.regionid.clone(),
            agreementtype: row.agreementtype.clone(),
            creditreference: row.creditreference.clone(),
            debitreference: row.debitreference.clone(),
            lastchanged: row.lastchanged.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            current_stepid: row.current_stepid.clone(),
            daytype: row.daytype.clone(),
            reallocation_type: row.reallocation_type.clone(),
            calendarid: row.calendarid.clone(),
            intervallength: row.intervallength.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetcfgReallocation2PrimaryKey {
    pub reallocationid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SetcfgReallocation2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SetcfgReallocation2Row<'data> {
    type Row<'other> = SetcfgReallocation2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.reallocationid() == row.reallocationid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SetcfgReallocation2Row<'data> {
    type PrimaryKey = SetcfgReallocation2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.reallocationid() == key.reallocationid
    }
}
impl<'data> mmsdm_core::CompareWithRow for SetcfgReallocation2PrimaryKey {
    type Row<'other> = SetcfgReallocation2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.reallocationid == row.reallocationid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SetcfgReallocation2PrimaryKey {
    type PrimaryKey = SetcfgReallocation2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.reallocationid == key.reallocationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SetcfgReallocation2 {
    type Builder = SetcfgReallocation2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "reallocationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "creditparticipantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "debitparticipantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "agreementtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "creditreference",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "debitreference",
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
                    "current_stepid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "daytype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reallocation_type",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calendarid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervallength",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SetcfgReallocation2Builder {
            reallocationid_array: arrow::array::builder::StringBuilder::new(),
            creditparticipantid_array: arrow::array::builder::StringBuilder::new(),
            debitparticipantid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            agreementtype_array: arrow::array::builder::StringBuilder::new(),
            creditreference_array: arrow::array::builder::StringBuilder::new(),
            debitreference_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            current_stepid_array: arrow::array::builder::StringBuilder::new(),
            daytype_array: arrow::array::builder::StringBuilder::new(),
            reallocation_type_array: arrow::array::builder::StringBuilder::new(),
            calendarid_array: arrow::array::builder::StringBuilder::new(),
            intervallength_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.reallocationid_array.append_value(row.reallocationid());
        builder.creditparticipantid_array.append_option(row.creditparticipantid());
        builder.debitparticipantid_array.append_option(row.debitparticipantid());
        builder.regionid_array.append_option(row.regionid());
        builder.agreementtype_array.append_option(row.agreementtype());
        builder.creditreference_array.append_option(row.creditreference());
        builder.debitreference_array.append_option(row.debitreference());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder.current_stepid_array.append_option(row.current_stepid());
        builder.daytype_array.append_option(row.daytype());
        builder.reallocation_type_array.append_option(row.reallocation_type());
        builder.calendarid_array.append_option(row.calendarid());
        builder
            .intervallength_array
            .append_option({
                row.intervallength
                    .map(|mut val| {
                        val.rescale(0);
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
                    alloc::sync::Arc::new(builder.reallocationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.creditparticipantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.debitparticipantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.agreementtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.creditreference_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.debitreference_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.current_stepid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.daytype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reallocation_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calendarid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervallength_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SetcfgReallocation2Builder {
    reallocationid_array: arrow::array::builder::StringBuilder,
    creditparticipantid_array: arrow::array::builder::StringBuilder,
    debitparticipantid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    agreementtype_array: arrow::array::builder::StringBuilder,
    creditreference_array: arrow::array::builder::StringBuilder,
    debitreference_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    current_stepid_array: arrow::array::builder::StringBuilder,
    daytype_array: arrow::array::builder::StringBuilder,
    reallocation_type_array: arrow::array::builder::StringBuilder,
    calendarid_array: arrow::array::builder::StringBuilder,
    intervallength_array: arrow::array::builder::Decimal128Builder,
}
pub struct SetcfgReallocationinterval1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SetcfgReallocationinterval1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SetcfgReallocationinterval1 {
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
pub struct SetcfgReallocationinterval1Mapping([usize; 5]);
/// # Summary
///
/// ## REALLOCATIONINTERVAL
///
/// 30-minute or (5-minute for 5MS) data comprising a single reallocation transaction.
///
/// * Data Set Name: Setcfg
/// * File Name: Reallocationinterval
/// * Data Version: 1
///
/// # Description
/// SETCFG_PARTICIPANT_MPF data is available to all participants.VolumeApproximately 20,000 records per year
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REALLOCATIONID
#[derive(Debug, PartialEq, Eq)]
pub struct SetcfgReallocationinterval1Row<'data> {
    /// Reallocation identifier
    pub reallocationid: core::ops::Range<usize>,
    /// Trading Interval
    pub periodid: i64,
    /// Reallocation value in the units of the agreement type
    pub value: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Nominated Reallocation Price, only used in agreement types of SWAP, CAP and FLOOR, being the contract strike price in $/MWh
    pub nrp: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SetcfgReallocationinterval1Row<'data> {
    pub fn reallocationid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.reallocationid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for SetcfgReallocationinterval1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETCFG";
    const TABLE_NAME: &'static str = "REALLOCATIONINTERVAL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SetcfgReallocationinterval1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "REALLOCATIONID",
        "PERIODID",
        "VALUE",
        "LASTCHANGED",
        "NRP",
    ];
    type Row<'row> = SetcfgReallocationinterval1Row<'row>;
    type FieldMapping = SetcfgReallocationinterval1Mapping;
    type PrimaryKey = SetcfgReallocationinterval1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SetcfgReallocationinterval1Row {
            reallocationid: row.get_range("reallocationid", field_mapping.0[0])?,
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[1])?,
            value: row
                .get_opt_custom_parsed_at_idx(
                    "value",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            nrp: row
                .get_opt_custom_parsed_at_idx(
                    "nrp",
                    field_mapping.0[4],
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
        Ok(SetcfgReallocationinterval1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SetcfgReallocationinterval1PrimaryKey {
        SetcfgReallocationinterval1PrimaryKey {
            periodid: row.periodid,
            reallocationid: row.reallocationid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("setcfg_reallocationinterval_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SetcfgReallocationinterval1Row {
            reallocationid: row.reallocationid.clone(),
            periodid: row.periodid.clone(),
            value: row.value.clone(),
            lastchanged: row.lastchanged.clone(),
            nrp: row.nrp.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetcfgReallocationinterval1PrimaryKey {
    pub periodid: i64,
    pub reallocationid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SetcfgReallocationinterval1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SetcfgReallocationinterval1Row<'data> {
    type Row<'other> = SetcfgReallocationinterval1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.reallocationid() == row.reallocationid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SetcfgReallocationinterval1Row<'data> {
    type PrimaryKey = SetcfgReallocationinterval1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.reallocationid() == key.reallocationid
    }
}
impl<'data> mmsdm_core::CompareWithRow for SetcfgReallocationinterval1PrimaryKey {
    type Row<'other> = SetcfgReallocationinterval1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.reallocationid == row.reallocationid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SetcfgReallocationinterval1PrimaryKey {
    type PrimaryKey = SetcfgReallocationinterval1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.reallocationid == key.reallocationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SetcfgReallocationinterval1 {
    type Builder = SetcfgReallocationinterval1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "reallocationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "value",
                    arrow::datatypes::DataType::Decimal128(15, 5),
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
                    "nrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SetcfgReallocationinterval1Builder {
            reallocationid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Int64Builder::new(),
            value_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            nrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.reallocationid_array.append_value(row.reallocationid());
        builder.periodid_array.append_value(row.periodid);
        builder
            .value_array
            .append_option({
                row.value
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .nrp_array
            .append_option({
                row.nrp
                    .map(|mut val| {
                        val.rescale(5);
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
                    alloc::sync::Arc::new(builder.reallocationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.value_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SetcfgReallocationinterval1Builder {
    reallocationid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Int64Builder,
    value_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    nrp_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementConfigSetcfgParticipantMpf1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigSetcfgParticipantMpf1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigSetcfgParticipantMpf1 {
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
pub struct SettlementConfigSetcfgParticipantMpf1Mapping([usize; 7]);
/// # Summary
///
/// ## SETCFG_PARTICIPANT_MPF
///
/// SETCFG_PARTICIPANT_MPF shows the Market Participation Factors (MPF) for each participant for each connection point. The MPF values are used to determine recovery amounts for regulation FCAS.
///
/// * Data Set Name: Settlement Config
/// * File Name: Setcfg Participant Mpf
/// * Data Version: 1
///
/// # Description
/// SETCFG_PARTICIPANT_MPF data is available to all participants.VolumeApproximately 20,000 records per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigSetcfgParticipantMpf1Row<'data> {
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Effective date of the MPF data
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    pub versionno: rust_decimal::Decimal,
    /// Participant Category
    pub participantcategoryid: core::ops::Range<usize>,
    /// Connection point identifier
    pub connectionpointid: core::ops::Range<usize>,
    /// Market Participation Factor
    pub mpf: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigSetcfgParticipantMpf1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn participantcategoryid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participantcategoryid.clone(),
        )
    }
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for SettlementConfigSetcfgParticipantMpf1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "SETCFG_PARTICIPANT_MPF";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigSetcfgParticipantMpf1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PARTICIPANTCATEGORYID",
        "CONNECTIONPOINTID",
        "MPF",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigSetcfgParticipantMpf1Row<'row>;
    type FieldMapping = SettlementConfigSetcfgParticipantMpf1Mapping;
    type PrimaryKey = SettlementConfigSetcfgParticipantMpf1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigSetcfgParticipantMpf1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantcategoryid: row
                .get_range("participantcategoryid", field_mapping.0[3])?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[4])?,
            mpf: row
                .get_opt_custom_parsed_at_idx(
                    "mpf",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
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
        Ok(SettlementConfigSetcfgParticipantMpf1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigSetcfgParticipantMpf1PrimaryKey {
        SettlementConfigSetcfgParticipantMpf1PrimaryKey {
            connectionpointid: row.connectionpointid().to_string(),
            effectivedate: row.effectivedate,
            participantcategoryid: row.participantcategoryid().to_string(),
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_setcfg_participant_mpf_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigSetcfgParticipantMpf1Row {
            participantid: row.participantid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            participantcategoryid: row.participantcategoryid.clone(),
            connectionpointid: row.connectionpointid.clone(),
            mpf: row.mpf.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigSetcfgParticipantMpf1PrimaryKey {
    pub connectionpointid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub participantcategoryid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigSetcfgParticipantMpf1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigSetcfgParticipantMpf1Row<'data> {
    type Row<'other> = SettlementConfigSetcfgParticipantMpf1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid() == row.connectionpointid()
            && self.effectivedate == row.effectivedate
            && self.participantcategoryid() == row.participantcategoryid()
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigSetcfgParticipantMpf1Row<'data> {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpf1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid() == key.connectionpointid
            && self.effectivedate == key.effectivedate
            && self.participantcategoryid() == key.participantcategoryid
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigSetcfgParticipantMpf1PrimaryKey {
    type Row<'other> = SettlementConfigSetcfgParticipantMpf1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid == row.connectionpointid()
            && self.effectivedate == row.effectivedate
            && self.participantcategoryid == row.participantcategoryid()
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigSetcfgParticipantMpf1PrimaryKey {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpf1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigSetcfgParticipantMpf1 {
    type Builder = SettlementConfigSetcfgParticipantMpf1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantcategoryid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "mpf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
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
        SettlementConfigSetcfgParticipantMpf1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantcategoryid_array: arrow::array::builder::StringBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            mpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
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
        builder.participantcategoryid_array.append_value(row.participantcategoryid());
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder
            .mpf_array
            .append_option({
                row.mpf
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
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantcategoryid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigSetcfgParticipantMpf1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantcategoryid_array: arrow::array::builder::StringBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    mpf_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementConfigSetcfgParticipantMpftrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigSetcfgParticipantMpftrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigSetcfgParticipantMpftrk1 {
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
pub struct SettlementConfigSetcfgParticipantMpftrk1Mapping([usize; 6]);
/// # Summary
///
/// ## SETCFG_PARTICIPANT_MPFTRK
///
/// SETCFG_PARTICIPANT_MPFTRK is the tracking table for Market Participation Factors (MPF) data stored in the SETCFG_PARTICIPANT_MPF table for each participant.
///
/// * Data Set Name: Settlement Config
/// * File Name: Setcfg Participant Mpftrk
/// * Data Version: 1
///
/// # Description
/// SETCFG_PARTICIPANT_MPFTRK data is public, so is available to all participants.VolumeApproximately 2,000 records per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigSetcfgParticipantMpftrk1Row<'data> {
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Effective date of the MPF data
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    pub versionno: rust_decimal::Decimal,
    /// Authorising user
    pub authorisedby: core::ops::Range<usize>,
    /// Authorised date and time
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigSetcfgParticipantMpftrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn authorisedby(&self) -> Option<&str> {
        if self.authorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for SettlementConfigSetcfgParticipantMpftrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "SETCFG_PARTICIPANT_MPFTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigSetcfgParticipantMpftrk1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigSetcfgParticipantMpftrk1Row<'row>;
    type FieldMapping = SettlementConfigSetcfgParticipantMpftrk1Mapping;
    type PrimaryKey = SettlementConfigSetcfgParticipantMpftrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigSetcfgParticipantMpftrk1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[3])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
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
        Ok(SettlementConfigSetcfgParticipantMpftrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
        SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
            effectivedate: row.effectivedate,
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_setcfg_participant_mpftrk_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigSetcfgParticipantMpftrk1Row {
            participantid: row.participantid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigSetcfgParticipantMpftrk1Row<'data> {
    type Row<'other> = SettlementConfigSetcfgParticipantMpftrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigSetcfgParticipantMpftrk1Row<'data> {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpftrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
    type Row<'other> = SettlementConfigSetcfgParticipantMpftrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpftrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigSetcfgParticipantMpftrk1 {
    type Builder = SettlementConfigSetcfgParticipantMpftrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementConfigSetcfgParticipantMpftrk1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
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
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
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
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementConfigSetcfgParticipantMpftrk1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SetcfgSapsSettPrice1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SetcfgSapsSettPrice1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SetcfgSapsSettPrice1 {
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
pub struct SetcfgSapsSettPrice1Mapping([usize; 7]);
/// # Summary
///
/// ## SETCFG_SAPS_SETT_PRICE
///
/// The Settlement Price for SAPS Energy in each Region
///
/// * Data Set Name: Setcfg
/// * File Name: Saps Sett Price
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * FROMDATE
/// * REGIONID
/// * TODATE
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct SetcfgSapsSettPrice1Row<'data> {
    /// The From Date of the SAPS Pricing Application Period
    pub fromdate: chrono::NaiveDateTime,
    /// The To Date of the SAPS Pricing Application Period
    pub todate: chrono::NaiveDateTime,
    /// The Region ID for which the calculated SAPS Price is applicable
    pub regionid: core::ops::Range<usize>,
    /// The Date time of the record generation
    pub version_datetime: chrono::NaiveDateTime,
    /// The Region Reference Price for SAPS in the Region
    pub saps_rrp: Option<rust_decimal::Decimal>,
    /// Whether the SAPS Price is Firm or Non-Firm
    pub isfirm: Option<rust_decimal::Decimal>,
    /// The Last Changed Date time of the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SetcfgSapsSettPrice1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SetcfgSapsSettPrice1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETCFG";
    const TABLE_NAME: &'static str = "SAPS_SETT_PRICE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SetcfgSapsSettPrice1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "FROMDATE",
        "TODATE",
        "REGIONID",
        "VERSION_DATETIME",
        "SAPS_RRP",
        "ISFIRM",
        "LASTCHANGED",
    ];
    type Row<'row> = SetcfgSapsSettPrice1Row<'row>;
    type FieldMapping = SetcfgSapsSettPrice1Mapping;
    type PrimaryKey = SetcfgSapsSettPrice1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SetcfgSapsSettPrice1Row {
            fromdate: row
                .get_custom_parsed_at_idx(
                    "fromdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            todate: row
                .get_custom_parsed_at_idx(
                    "todate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            saps_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "saps_rrp",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            isfirm: row
                .get_opt_custom_parsed_at_idx(
                    "isfirm",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
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
        Ok(SetcfgSapsSettPrice1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SetcfgSapsSettPrice1PrimaryKey {
        SetcfgSapsSettPrice1PrimaryKey {
            fromdate: row.fromdate,
            regionid: row.regionid().to_string(),
            todate: row.todate,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("setcfg_saps_sett_price_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SetcfgSapsSettPrice1Row {
            fromdate: row.fromdate.clone(),
            todate: row.todate.clone(),
            regionid: row.regionid.clone(),
            version_datetime: row.version_datetime.clone(),
            saps_rrp: row.saps_rrp.clone(),
            isfirm: row.isfirm.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetcfgSapsSettPrice1PrimaryKey {
    pub fromdate: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub todate: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SetcfgSapsSettPrice1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SetcfgSapsSettPrice1Row<'data> {
    type Row<'other> = SetcfgSapsSettPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.fromdate == row.fromdate && self.regionid() == row.regionid()
            && self.todate == row.todate && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SetcfgSapsSettPrice1Row<'data> {
    type PrimaryKey = SetcfgSapsSettPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.fromdate == key.fromdate && self.regionid() == key.regionid
            && self.todate == key.todate && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for SetcfgSapsSettPrice1PrimaryKey {
    type Row<'other> = SetcfgSapsSettPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.fromdate == row.fromdate && self.regionid == row.regionid()
            && self.todate == row.todate && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SetcfgSapsSettPrice1PrimaryKey {
    type PrimaryKey = SetcfgSapsSettPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.fromdate == key.fromdate && self.regionid == key.regionid
            && self.todate == key.todate && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SetcfgSapsSettPrice1 {
    type Builder = SetcfgSapsSettPrice1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "fromdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "todate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
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
                    "saps_rrp",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "isfirm",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        SetcfgSapsSettPrice1Builder {
            fromdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            todate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            saps_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            isfirm_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.fromdate_array.append_value(row.fromdate.and_utc().timestamp_millis());
        builder.todate_array.append_value(row.todate.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .saps_rrp_array
            .append_option({
                row.saps_rrp
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .isfirm_array
            .append_option({
                row.isfirm
                    .map(|mut val| {
                        val.rescale(0);
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
                    alloc::sync::Arc::new(builder.fromdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.todate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.saps_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.isfirm_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SetcfgSapsSettPrice1Builder {
    fromdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    todate_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    saps_rrp_array: arrow::array::builder::Decimal128Builder,
    isfirm_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsConfigWdrReimburseRate1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementsConfigWdrReimburseRate1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementsConfigWdrReimburseRate1 {
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
pub struct SettlementsConfigWdrReimburseRate1Mapping([usize; 6]);
/// # Summary
///
/// ## SETCFG_WDR_REIMBURSE_RATE
///
/// Settlements WDR transactions
///
/// * Data Set Name: Settlements Config
/// * File Name: Wdr Reimburse Rate
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * VERSION_DATETIME
/// * WDRRRPERIOD
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsConfigWdrReimburseRate1Row<'data> {
    /// Unique identifier for the period to which the WDRRR applies. For quarter-based periods, this will be equal to YYYY[Q]NN, e.g. 2020Q3 for 2020 Quarter 3.
    pub wdrrrperiod: core::ops::Range<usize>,
    /// Unique identifier for the region
    pub regionid: core::ops::Range<usize>,
    /// The Version Date time of the latest changes.
    pub version_datetime: chrono::NaiveDateTime,
    /// WDRRR value for the period and region ($/MWh)
    pub wdrrr: Option<rust_decimal::Decimal>,
    /// A flag to indicate that the WDRRR value is FIRM for the period and region, i.e. it is based on a complete set of firm prices from dispatch. Possible Values are 1 and 0
    pub isfirm: Option<rust_decimal::Decimal>,
    /// Last changed date for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsConfigWdrReimburseRate1Row<'data> {
    pub fn wdrrrperiod(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.wdrrrperiod.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsConfigWdrReimburseRate1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS_CONFIG";
    const TABLE_NAME: &'static str = "WDR_REIMBURSE_RATE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsConfigWdrReimburseRate1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "WDRRRPERIOD",
        "REGIONID",
        "VERSION_DATETIME",
        "WDRRR",
        "ISFIRM",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsConfigWdrReimburseRate1Row<'row>;
    type FieldMapping = SettlementsConfigWdrReimburseRate1Mapping;
    type PrimaryKey = SettlementsConfigWdrReimburseRate1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsConfigWdrReimburseRate1Row {
            wdrrrperiod: row.get_range("wdrrrperiod", field_mapping.0[0])?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            wdrrr: row
                .get_opt_custom_parsed_at_idx(
                    "wdrrr",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            isfirm: row
                .get_opt_custom_parsed_at_idx(
                    "isfirm",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
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
        Ok(SettlementsConfigWdrReimburseRate1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsConfigWdrReimburseRate1PrimaryKey {
        SettlementsConfigWdrReimburseRate1PrimaryKey {
            regionid: row.regionid().to_string(),
            version_datetime: row.version_datetime,
            wdrrrperiod: row.wdrrrperiod().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_config_wdr_reimburse_rate_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsConfigWdrReimburseRate1Row {
            wdrrrperiod: row.wdrrrperiod.clone(),
            regionid: row.regionid.clone(),
            version_datetime: row.version_datetime.clone(),
            wdrrr: row.wdrrr.clone(),
            isfirm: row.isfirm.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsConfigWdrReimburseRate1PrimaryKey {
    pub regionid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
    pub wdrrrperiod: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SettlementsConfigWdrReimburseRate1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsConfigWdrReimburseRate1Row<'data> {
    type Row<'other> = SettlementsConfigWdrReimburseRate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid() == row.regionid()
            && self.version_datetime == row.version_datetime
            && self.wdrrrperiod() == row.wdrrrperiod()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementsConfigWdrReimburseRate1Row<'data> {
    type PrimaryKey = SettlementsConfigWdrReimburseRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid() == key.regionid && self.version_datetime == key.version_datetime
            && self.wdrrrperiod() == key.wdrrrperiod
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsConfigWdrReimburseRate1PrimaryKey {
    type Row<'other> = SettlementsConfigWdrReimburseRate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid == row.regionid() && self.version_datetime == row.version_datetime
            && self.wdrrrperiod == row.wdrrrperiod()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsConfigWdrReimburseRate1PrimaryKey {
    type PrimaryKey = SettlementsConfigWdrReimburseRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.version_datetime == key.version_datetime
            && self.wdrrrperiod == key.wdrrrperiod
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsConfigWdrReimburseRate1 {
    type Builder = SettlementsConfigWdrReimburseRate1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "wdrrrperiod",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
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
                    "wdrrr",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "isfirm",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        SettlementsConfigWdrReimburseRate1Builder {
            wdrrrperiod_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            wdrrr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            isfirm_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.wdrrrperiod_array.append_value(row.wdrrrperiod());
        builder.regionid_array.append_value(row.regionid());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .wdrrr_array
            .append_option({
                row.wdrrr
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .isfirm_array
            .append_option({
                row.isfirm
                    .map(|mut val| {
                        val.rescale(0);
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
                    alloc::sync::Arc::new(builder.wdrrrperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdrrr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.isfirm_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsConfigWdrReimburseRate1Builder {
    wdrrrperiod_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    wdrrr_array: arrow::array::builder::Decimal128Builder,
    isfirm_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsConfigWdrrrCalendar1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementsConfigWdrrrCalendar1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementsConfigWdrrrCalendar1 {
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
pub struct SettlementsConfigWdrrrCalendar1Mapping([usize; 6]);
/// # Summary
///
/// ## SETCFG_WDRRR_CALENDAR
///
/// Wholesale Demand Response Reimbursement Rate Calendar
///
/// * Data Set Name: Settlements Config
/// * File Name: Wdrrr Calendar
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * VERSION_DATETIME
/// * WDRRRPERIOD
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsConfigWdrrrCalendar1Row<'data> {
    /// Unique identifier for the period to which the WDRRR applies. For quarter-based periods, this will be equal to YYYY[Q]NN, for example,2020Q3 for 2020 Quarter 3.
    pub wdrrrperiod: core::ops::Range<usize>,
    /// Unique Identifier for the region id
    pub regionid: core::ops::Range<usize>,
    /// The Version Date time of the latest changes.
    pub version_datetime: chrono::NaiveDateTime,
    /// Start Date of Period (Inclusive).
    pub startdate: Option<chrono::NaiveDateTime>,
    /// End Date of Period (Inclusive).
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Last changed date for the record.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsConfigWdrrrCalendar1Row<'data> {
    pub fn wdrrrperiod(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.wdrrrperiod.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsConfigWdrrrCalendar1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS_CONFIG";
    const TABLE_NAME: &'static str = "WDRRR_CALENDAR";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsConfigWdrrrCalendar1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "WDRRRPERIOD",
        "REGIONID",
        "VERSION_DATETIME",
        "STARTDATE",
        "ENDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsConfigWdrrrCalendar1Row<'row>;
    type FieldMapping = SettlementsConfigWdrrrCalendar1Mapping;
    type PrimaryKey = SettlementsConfigWdrrrCalendar1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsConfigWdrrrCalendar1Row {
            wdrrrperiod: row.get_range("wdrrrperiod", field_mapping.0[0])?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
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
        Ok(SettlementsConfigWdrrrCalendar1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsConfigWdrrrCalendar1PrimaryKey {
        SettlementsConfigWdrrrCalendar1PrimaryKey {
            regionid: row.regionid().to_string(),
            version_datetime: row.version_datetime,
            wdrrrperiod: row.wdrrrperiod().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_config_wdrrr_calendar_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsConfigWdrrrCalendar1Row {
            wdrrrperiod: row.wdrrrperiod.clone(),
            regionid: row.regionid.clone(),
            version_datetime: row.version_datetime.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsConfigWdrrrCalendar1PrimaryKey {
    pub regionid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
    pub wdrrrperiod: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SettlementsConfigWdrrrCalendar1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsConfigWdrrrCalendar1Row<'data> {
    type Row<'other> = SettlementsConfigWdrrrCalendar1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid() == row.regionid()
            && self.version_datetime == row.version_datetime
            && self.wdrrrperiod() == row.wdrrrperiod()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementsConfigWdrrrCalendar1Row<'data> {
    type PrimaryKey = SettlementsConfigWdrrrCalendar1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid() == key.regionid && self.version_datetime == key.version_datetime
            && self.wdrrrperiod() == key.wdrrrperiod
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsConfigWdrrrCalendar1PrimaryKey {
    type Row<'other> = SettlementsConfigWdrrrCalendar1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid == row.regionid() && self.version_datetime == row.version_datetime
            && self.wdrrrperiod == row.wdrrrperiod()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsConfigWdrrrCalendar1PrimaryKey {
    type PrimaryKey = SettlementsConfigWdrrrCalendar1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.version_datetime == key.version_datetime
            && self.wdrrrperiod == key.wdrrrperiod
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsConfigWdrrrCalendar1 {
    type Builder = SettlementsConfigWdrrrCalendar1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "wdrrrperiod",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
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
        SettlementsConfigWdrrrCalendar1Builder {
            wdrrrperiod_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.wdrrrperiod_array.append_value(row.wdrrrperiod());
        builder.regionid_array.append_value(row.regionid());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.wdrrrperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsConfigWdrrrCalendar1Builder {
    wdrrrperiod_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
