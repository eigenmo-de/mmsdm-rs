#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct MeterdataAggregateReads1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&MeterdataAggregateReads1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterdataAggregateReads1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MeterdataAggregateReads1Mapping([usize; 10]);
/// # Summary
///
/// ## METERDATA_AGGREGATE_READS
///  _Publishes aggregated metering data associated with a wholesale connection point for a given CASE_ID_
///
/// * Data Set Name: Meterdata
/// * File Name: Aggregate Reads
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * CONNECTIONPOINTID
/// * FRMP
/// * LR
/// * METER_TYPE
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct MeterdataAggregateReads1Row<'data> {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    /// Settlement date within the case
    pub settlementdate: chrono::NaiveDateTime,
    /// Connection Point ID
    pub connectionpointid: core::ops::Range<usize>,
    /// The meter type for the read, one of: CUSTOMER; GENERATOR; EMBEDDED_GENERATOR
    pub meter_type: core::ops::Range<usize>,
    /// The financially responsible market participantid
    pub frmp: core::ops::Range<usize>,
    /// The local retailer at the connection point id
    pub lr: core::ops::Range<usize>,
    /// Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The import(pool-centric) value for the meter read (MWh)
    pub importvalue: rust_decimal::Decimal,
    /// The export(pool-centric) value for the meter read (MWh)
    pub exportvalue: rust_decimal::Decimal,
    /// Last changed date for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterdataAggregateReads1Row<'data> {
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn meter_type(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.meter_type.clone())
    }
    pub fn frmp(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.frmp.clone())
    }
    pub fn lr(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.lr.clone())
    }
}
impl mmsdm_core::GetTable for MeterdataAggregateReads1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METERDATA";
    const TABLE_NAME: &'static str = "AGGREGATE_READS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterdataAggregateReads1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CASE_ID",
        "SETTLEMENTDATE",
        "CONNECTIONPOINTID",
        "METER_TYPE",
        "FRMP",
        "LR",
        "PERIODID",
        "IMPORTVALUE",
        "EXPORTVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = MeterdataAggregateReads1Row<'row>;
    type FieldMapping = MeterdataAggregateReads1Mapping;
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterdataAggregateReads1Row {
            case_id: row
                .get_custom_parsed_at_idx(
                    "case_id",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[2])?,
            meter_type: row.get_range("meter_type", field_mapping.0[3])?,
            frmp: row.get_range("frmp", field_mapping.0[4])?,
            lr: row.get_range("lr", field_mapping.0[5])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importvalue: row
                .get_custom_parsed_at_idx(
                    "importvalue",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportvalue: row
                .get_custom_parsed_at_idx(
                    "exportvalue",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
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
        Ok(MeterdataAggregateReads1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterdataAggregateReads1PrimaryKey {
        MeterdataAggregateReads1PrimaryKey {
            case_id: row.case_id,
            connectionpointid: row.connectionpointid().to_string(),
            frmp: row.frmp().to_string(),
            lr: row.lr().to_string(),
            meter_type: row.meter_type().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meterdata_aggregate_reads_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterdataAggregateReads1Row {
            case_id: row.case_id.clone(),
            settlementdate: row.settlementdate.clone(),
            connectionpointid: row.connectionpointid.clone(),
            meter_type: row.meter_type.clone(),
            frmp: row.frmp.clone(),
            lr: row.lr.clone(),
            periodid: row.periodid.clone(),
            importvalue: row.importvalue.clone(),
            exportvalue: row.exportvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataAggregateReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub connectionpointid: alloc::string::String,
    pub frmp: alloc::string::String,
    pub lr: alloc::string::String,
    pub meter_type: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataAggregateReads1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterdataAggregateReads1Row<'data> {
    type Row<'other> = MeterdataAggregateReads1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id
            && self.connectionpointid() == row.connectionpointid()
            && self.frmp() == row.frmp() && self.lr() == row.lr()
            && self.meter_type() == row.meter_type() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterdataAggregateReads1Row<'data> {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.connectionpointid() == key.connectionpointid
            && self.frmp() == key.frmp && self.lr() == key.lr
            && self.meter_type() == key.meter_type && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterdataAggregateReads1PrimaryKey {
    type Row<'other> = MeterdataAggregateReads1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id && self.connectionpointid == row.connectionpointid()
            && self.frmp == row.frmp() && self.lr == row.lr()
            && self.meter_type == row.meter_type() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataAggregateReads1PrimaryKey {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.connectionpointid == key.connectionpointid
            && self.frmp == key.frmp && self.lr == key.lr
            && self.meter_type == key.meter_type && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataAggregateReads1 {
    type Builder = MeterdataAggregateReads1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "case_id",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meter_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "frmp",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lr",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "importvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "exportvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        MeterdataAggregateReads1Builder {
            case_id_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            meter_type_array: arrow::array::builder::StringBuilder::new(),
            frmp_array: arrow::array::builder::StringBuilder::new(),
            lr_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            importvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .case_id_array
            .append_value({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder.meter_type_array.append_value(row.meter_type());
        builder.frmp_array.append_value(row.frmp());
        builder.lr_array.append_value(row.lr());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .importvalue_array
            .append_value({
                let mut val = row.importvalue;
                val.rescale(8);
                val.mantissa()
            });
        builder
            .exportvalue_array
            .append_value({
                let mut val = row.exportvalue;
                val.rescale(8);
                val.mantissa()
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.case_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.frmp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterdataAggregateReads1Builder {
    case_id_array: arrow::array::builder::Decimal128Builder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    meter_type_array: arrow::array::builder::StringBuilder,
    frmp_array: arrow::array::builder::StringBuilder,
    lr_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    importvalue_array: arrow::array::builder::Decimal128Builder,
    exportvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MeterdataIndividualReads1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&MeterdataIndividualReads1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterdataIndividualReads1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MeterdataIndividualReads1Mapping([usize; 12]);
/// # Summary
///
/// ## METERDATA_INDIVIDUAL_READS
///  _Publishes metering data associated with individual metering points for a given CASE_ID_
///
/// * Data Set Name: Meterdata
/// * File Name: Individual Reads
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * METER_ID
/// * METER_ID_SUFFIX
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct MeterdataIndividualReads1Row<'data> {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    /// Settlement date within the case
    pub settlementdate: chrono::NaiveDateTime,
    /// The National Metering Identifier (NMI)
    pub meter_id: core::ops::Range<usize>,
    /// The National Metering Identifier (NMI) data stream
    pub meter_id_suffix: core::ops::Range<usize>,
    /// The financially responsible market participantid
    pub frmp: core::ops::Range<usize>,
    /// The local retailer at the connection point id
    pub lr: core::ops::Range<usize>,
    /// Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Connection Point ID
    pub connectionpointid: core::ops::Range<usize>,
    /// The meter type for the read, one of: CUSTOMER; GENERATOR; EMBEDDED_GENERATOR
    pub meter_type: core::ops::Range<usize>,
    /// The import(pool-centric) value for the meter read (MWh)
    pub importvalue: rust_decimal::Decimal,
    /// The export(pool-centric) value for the meter read (MWh)
    pub exportvalue: rust_decimal::Decimal,
    /// Last changed date for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterdataIndividualReads1Row<'data> {
    pub fn meter_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.meter_id.clone())
    }
    pub fn meter_id_suffix(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.meter_id_suffix.clone(),
        )
    }
    pub fn frmp(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.frmp.clone())
    }
    pub fn lr(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.lr.clone())
    }
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn meter_type(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.meter_type.clone())
    }
}
impl mmsdm_core::GetTable for MeterdataIndividualReads1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METERDATA";
    const TABLE_NAME: &'static str = "INDIVIDUAL_READS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterdataIndividualReads1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CASE_ID",
        "SETTLEMENTDATE",
        "METER_ID",
        "METER_ID_SUFFIX",
        "FRMP",
        "LR",
        "PERIODID",
        "CONNECTIONPOINTID",
        "METER_TYPE",
        "IMPORTVALUE",
        "EXPORTVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = MeterdataIndividualReads1Row<'row>;
    type FieldMapping = MeterdataIndividualReads1Mapping;
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterdataIndividualReads1Row {
            case_id: row
                .get_custom_parsed_at_idx(
                    "case_id",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            meter_id: row.get_range("meter_id", field_mapping.0[2])?,
            meter_id_suffix: row.get_range("meter_id_suffix", field_mapping.0[3])?,
            frmp: row.get_range("frmp", field_mapping.0[4])?,
            lr: row.get_range("lr", field_mapping.0[5])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[7])?,
            meter_type: row.get_range("meter_type", field_mapping.0[8])?,
            importvalue: row
                .get_custom_parsed_at_idx(
                    "importvalue",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportvalue: row
                .get_custom_parsed_at_idx(
                    "exportvalue",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[11],
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
        Ok(MeterdataIndividualReads1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterdataIndividualReads1PrimaryKey {
        MeterdataIndividualReads1PrimaryKey {
            case_id: row.case_id,
            meter_id: row.meter_id().to_string(),
            meter_id_suffix: row.meter_id_suffix().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meterdata_individual_reads_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterdataIndividualReads1Row {
            case_id: row.case_id.clone(),
            settlementdate: row.settlementdate.clone(),
            meter_id: row.meter_id.clone(),
            meter_id_suffix: row.meter_id_suffix.clone(),
            frmp: row.frmp.clone(),
            lr: row.lr.clone(),
            periodid: row.periodid.clone(),
            connectionpointid: row.connectionpointid.clone(),
            meter_type: row.meter_type.clone(),
            importvalue: row.importvalue.clone(),
            exportvalue: row.exportvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataIndividualReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub meter_id: alloc::string::String,
    pub meter_id_suffix: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataIndividualReads1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterdataIndividualReads1Row<'data> {
    type Row<'other> = MeterdataIndividualReads1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id && self.meter_id() == row.meter_id()
            && self.meter_id_suffix() == row.meter_id_suffix()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterdataIndividualReads1Row<'data> {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.meter_id() == key.meter_id
            && self.meter_id_suffix() == key.meter_id_suffix
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterdataIndividualReads1PrimaryKey {
    type Row<'other> = MeterdataIndividualReads1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id && self.meter_id == row.meter_id()
            && self.meter_id_suffix == row.meter_id_suffix()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataIndividualReads1PrimaryKey {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.meter_id == key.meter_id
            && self.meter_id_suffix == key.meter_id_suffix
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataIndividualReads1 {
    type Builder = MeterdataIndividualReads1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "case_id",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meter_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meter_id_suffix",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "frmp",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lr",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meter_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "importvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "exportvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        MeterdataIndividualReads1Builder {
            case_id_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            meter_id_array: arrow::array::builder::StringBuilder::new(),
            meter_id_suffix_array: arrow::array::builder::StringBuilder::new(),
            frmp_array: arrow::array::builder::StringBuilder::new(),
            lr_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            meter_type_array: arrow::array::builder::StringBuilder::new(),
            importvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .case_id_array
            .append_value({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.meter_id_array.append_value(row.meter_id());
        builder.meter_id_suffix_array.append_value(row.meter_id_suffix());
        builder.frmp_array.append_value(row.frmp());
        builder.lr_array.append_value(row.lr());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder.meter_type_array.append_value(row.meter_type());
        builder
            .importvalue_array
            .append_value({
                let mut val = row.importvalue;
                val.rescale(8);
                val.mantissa()
            });
        builder
            .exportvalue_array
            .append_value({
                let mut val = row.exportvalue;
                val.rescale(8);
                val.mantissa()
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.case_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_id_suffix_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.frmp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterdataIndividualReads1Builder {
    case_id_array: arrow::array::builder::Decimal128Builder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    meter_id_array: arrow::array::builder::StringBuilder,
    meter_id_suffix_array: arrow::array::builder::StringBuilder,
    frmp_array: arrow::array::builder::StringBuilder,
    lr_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    meter_type_array: arrow::array::builder::StringBuilder,
    importvalue_array: arrow::array::builder::Decimal128Builder,
    exportvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MeterdataInterconnector1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&MeterdataInterconnector1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterdataInterconnector1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MeterdataInterconnector1Mapping([usize; 7]);
/// # Summary
///
/// ## METERDATA_INTERCONNECTOR
///  _Publishes metering data associated with wholesale interconnectors for a given CASE_ID_
///
/// * Data Set Name: Meterdata
/// * File Name: Interconnector
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * INTERCONNECTORID
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct MeterdataInterconnector1Row<'data> {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    /// Settlement date within the case
    pub settlementdate: chrono::NaiveDateTime,
    /// Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The import direction value for the meter read (MWh)
    pub importvalue: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportvalue: Option<rust_decimal::Decimal>,
    /// Last changed date for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterdataInterconnector1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for MeterdataInterconnector1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METERDATA";
    const TABLE_NAME: &'static str = "INTERCONNECTOR";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterdataInterconnector1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CASE_ID",
        "SETTLEMENTDATE",
        "INTERCONNECTORID",
        "PERIODID",
        "IMPORTVALUE",
        "EXPORTVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = MeterdataInterconnector1Row<'row>;
    type FieldMapping = MeterdataInterconnector1Mapping;
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterdataInterconnector1Row {
            case_id: row
                .get_custom_parsed_at_idx(
                    "case_id",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importvalue: row
                .get_opt_custom_parsed_at_idx(
                    "importvalue",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportvalue: row
                .get_opt_custom_parsed_at_idx(
                    "exportvalue",
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
        Ok(MeterdataInterconnector1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterdataInterconnector1PrimaryKey {
        MeterdataInterconnector1PrimaryKey {
            case_id: row.case_id,
            interconnectorid: row.interconnectorid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meterdata_interconnector_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterdataInterconnector1Row {
            case_id: row.case_id.clone(),
            settlementdate: row.settlementdate.clone(),
            interconnectorid: row.interconnectorid.clone(),
            periodid: row.periodid.clone(),
            importvalue: row.importvalue.clone(),
            exportvalue: row.exportvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataInterconnector1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub interconnectorid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataInterconnector1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterdataInterconnector1Row<'data> {
    type Row<'other> = MeterdataInterconnector1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id && self.interconnectorid() == row.interconnectorid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterdataInterconnector1Row<'data> {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.interconnectorid() == key.interconnectorid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterdataInterconnector1PrimaryKey {
    type Row<'other> = MeterdataInterconnector1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id && self.interconnectorid == row.interconnectorid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataInterconnector1PrimaryKey {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataInterconnector1 {
    type Builder = MeterdataInterconnector1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "case_id",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "importvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        MeterdataInterconnector1Builder {
            case_id_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            importvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .case_id_array
            .append_value({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .importvalue_array
            .append_option({
                row.importvalue
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .exportvalue_array
            .append_option({
                row.exportvalue
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.case_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterdataInterconnector1Builder {
    case_id_array: arrow::array::builder::Decimal128Builder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    importvalue_array: arrow::array::builder::Decimal128Builder,
    exportvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MeterdataMeterdataSaps1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&MeterdataMeterdataSaps1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterdataMeterdataSaps1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MeterdataMeterdataSaps1Mapping([usize; 10]);
/// # Summary
///
/// ## METERDATA_SAPS
///  _The SAPS Meter data for MSRP and Retailer used in the Settlement Calculation_
///
/// * Data Set Name: Meterdata
/// * File Name: Meterdata Saps
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * CONNECTIONPOINT_ID
/// * FRMP
/// * LR
/// * METER_TYPE
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct MeterdataMeterdataSaps1Row<'data> {
    /// The Metering Case ID used for Settlements
    pub case_id: rust_decimal::Decimal,
    /// The Settlement Date for that Week
    pub settlementdate: chrono::NaiveDateTime,
    /// The SAPS Connection Point Id
    pub connectionpoint_id: core::ops::Range<usize>,
    /// The Meter Type Identifier , CUSTOMER or MSRP
    pub meter_type: core::ops::Range<usize>,
    /// The Financial Responsible Market Participant
    pub frmp: core::ops::Range<usize>,
    /// The Local Retailer
    pub lr: core::ops::Range<usize>,
    /// The Period ID Identifier
    pub periodid: rust_decimal::Decimal,
    /// The Sent Out Energy in MWh
    pub importvalue: Option<rust_decimal::Decimal>,
    /// The Consumed Energy in MWh
    pub exportvalue: Option<rust_decimal::Decimal>,
    /// The Date time of the record last updated or inserted.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterdataMeterdataSaps1Row<'data> {
    pub fn connectionpoint_id(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpoint_id.clone(),
        )
    }
    pub fn meter_type(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.meter_type.clone())
    }
    pub fn frmp(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.frmp.clone())
    }
    pub fn lr(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.lr.clone())
    }
}
impl mmsdm_core::GetTable for MeterdataMeterdataSaps1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METERDATA";
    const TABLE_NAME: &'static str = "METERDATA_SAPS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterdataMeterdataSaps1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CASE_ID",
        "SETTLEMENTDATE",
        "CONNECTIONPOINT_ID",
        "METER_TYPE",
        "FRMP",
        "LR",
        "PERIODID",
        "IMPORTVALUE",
        "EXPORTVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = MeterdataMeterdataSaps1Row<'row>;
    type FieldMapping = MeterdataMeterdataSaps1Mapping;
    type PrimaryKey = MeterdataMeterdataSaps1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterdataMeterdataSaps1Row {
            case_id: row
                .get_custom_parsed_at_idx(
                    "case_id",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            connectionpoint_id: row.get_range("connectionpoint_id", field_mapping.0[2])?,
            meter_type: row.get_range("meter_type", field_mapping.0[3])?,
            frmp: row.get_range("frmp", field_mapping.0[4])?,
            lr: row.get_range("lr", field_mapping.0[5])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importvalue: row
                .get_opt_custom_parsed_at_idx(
                    "importvalue",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportvalue: row
                .get_opt_custom_parsed_at_idx(
                    "exportvalue",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
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
        Ok(MeterdataMeterdataSaps1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterdataMeterdataSaps1PrimaryKey {
        MeterdataMeterdataSaps1PrimaryKey {
            case_id: row.case_id,
            connectionpoint_id: row.connectionpoint_id().to_string(),
            frmp: row.frmp().to_string(),
            lr: row.lr().to_string(),
            meter_type: row.meter_type().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meterdata_meterdata_saps_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterdataMeterdataSaps1Row {
            case_id: row.case_id.clone(),
            settlementdate: row.settlementdate.clone(),
            connectionpoint_id: row.connectionpoint_id.clone(),
            meter_type: row.meter_type.clone(),
            frmp: row.frmp.clone(),
            lr: row.lr.clone(),
            periodid: row.periodid.clone(),
            importvalue: row.importvalue.clone(),
            exportvalue: row.exportvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataMeterdataSaps1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub connectionpoint_id: alloc::string::String,
    pub frmp: alloc::string::String,
    pub lr: alloc::string::String,
    pub meter_type: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataMeterdataSaps1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterdataMeterdataSaps1Row<'data> {
    type Row<'other> = MeterdataMeterdataSaps1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id
            && self.connectionpoint_id() == row.connectionpoint_id()
            && self.frmp() == row.frmp() && self.lr() == row.lr()
            && self.meter_type() == row.meter_type() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterdataMeterdataSaps1Row<'data> {
    type PrimaryKey = MeterdataMeterdataSaps1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
            && self.connectionpoint_id() == key.connectionpoint_id
            && self.frmp() == key.frmp && self.lr() == key.lr
            && self.meter_type() == key.meter_type && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterdataMeterdataSaps1PrimaryKey {
    type Row<'other> = MeterdataMeterdataSaps1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id
            && self.connectionpoint_id == row.connectionpoint_id()
            && self.frmp == row.frmp() && self.lr == row.lr()
            && self.meter_type == row.meter_type() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataMeterdataSaps1PrimaryKey {
    type PrimaryKey = MeterdataMeterdataSaps1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.connectionpoint_id == key.connectionpoint_id
            && self.frmp == key.frmp && self.lr == key.lr
            && self.meter_type == key.meter_type && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataMeterdataSaps1 {
    type Builder = MeterdataMeterdataSaps1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "case_id",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpoint_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meter_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "frmp",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lr",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "importvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        MeterdataMeterdataSaps1Builder {
            case_id_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            connectionpoint_id_array: arrow::array::builder::StringBuilder::new(),
            meter_type_array: arrow::array::builder::StringBuilder::new(),
            frmp_array: arrow::array::builder::StringBuilder::new(),
            lr_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            importvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .case_id_array
            .append_value({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.connectionpoint_id_array.append_value(row.connectionpoint_id());
        builder.meter_type_array.append_value(row.meter_type());
        builder.frmp_array.append_value(row.frmp());
        builder.lr_array.append_value(row.lr());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .importvalue_array
            .append_option({
                row.importvalue
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .exportvalue_array
            .append_option({
                row.exportvalue
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.case_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpoint_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.frmp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterdataMeterdataSaps1Builder {
    case_id_array: arrow::array::builder::Decimal128Builder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    connectionpoint_id_array: arrow::array::builder::StringBuilder,
    meter_type_array: arrow::array::builder::StringBuilder,
    frmp_array: arrow::array::builder::StringBuilder,
    lr_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    importvalue_array: arrow::array::builder::Decimal128Builder,
    exportvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MeterdataWdrReads1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&MeterdataWdrReads1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterdataWdrReads1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MeterdataWdrReads1Mapping([usize; 14]);
/// # Summary
///
/// ## METERDATA_WDR_READS
///  _Metering Data WDR Readings_
///
/// * Data Set Name: Meterdata
/// * File Name: Wdr Reads
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * MARKET_ID
/// * METER_ID
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct MeterdataWdrReads1Row<'data> {
    /// Unique identifier for the market to which this metering record applies.  Always equal to NEM in the current system.
    pub market_id: core::ops::Range<usize>,
    /// Unique identifier for the metering case.
    pub case_id: rust_decimal::Decimal,
    /// The settlement date for the metering record
    pub settlementdate: chrono::NaiveDateTime,
    /// Unique identifier for the meter to which the metering record applies
    pub meter_id: core::ops::Range<usize>,
    /// Unique identifier for the transmission node to which this meter belongs on the settlement date
    pub tni: core::ops::Range<usize>,
    /// Unique identifier for the participant acting as the FRMP for this NMI on the settlement date
    pub frmp: core::ops::Range<usize>,
    /// Unique identifier for the participant acting as the DRSP for this NMI on the settlement date
    pub drsp: core::ops::Range<usize>,
    /// Trading interval identifier, with Period 1 being the first TI for the calendar day, i.e interval ending 00:05.
    pub periodid: rust_decimal::Decimal,
    /// Metered quantity Import in MWh for the NMI in the trading interval.  A negative value indicates net consumption, while a positive value indicates net generation
    pub meteredquantityimport: Option<rust_decimal::Decimal>,
    /// Metered quantity Export in MWh for the NMI in the trading interval.  A negative value indicates net consumption, while a positive value indicates net generation
    pub meteredquantityexport: Option<rust_decimal::Decimal>,
    /// Baseline quantity in MWh for the NMI in the trading interval.  A negative value indicates net consumption, while a positive value indicates the net generation
    pub baselinequantity: Option<rust_decimal::Decimal>,
    /// Quality flag for the meter read.  Where multiple datastreams exist against the NMI with different quality flags for each read, the lowest quality flag will be published against the NMI for the interval.
    pub qualityflag: core::ops::Range<usize>,
    /// A value of TRUE (indicated by 1) for this column indicates that financial settlement of WDR transactions for this NMI should not proceed for the settlement date and trading interval. Possible values are 1 and 0.
    pub isnoncompliant: Option<rust_decimal::Decimal>,
    /// A reference to the baseline run that produced the baseline quantity for this NMI and interval
    pub baselinecalculationid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterdataWdrReads1Row<'data> {
    pub fn market_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.market_id.clone())
    }
    pub fn meter_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.meter_id.clone())
    }
    pub fn tni(&self) -> Option<&str> {
        if self.tni.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.tni.clone()))
        }
    }
    pub fn frmp(&self) -> Option<&str> {
        if self.frmp.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.frmp.clone()),
            )
        }
    }
    pub fn drsp(&self) -> Option<&str> {
        if self.drsp.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.drsp.clone()),
            )
        }
    }
    pub fn qualityflag(&self) -> Option<&str> {
        if self.qualityflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.qualityflag.clone(),
                ),
            )
        }
    }
    pub fn baselinecalculationid(&self) -> Option<&str> {
        if self.baselinecalculationid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.baselinecalculationid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MeterdataWdrReads1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METERDATA";
    const TABLE_NAME: &'static str = "WDR_READS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterdataWdrReads1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MARKET_ID",
        "CASE_ID",
        "SETTLEMENTDATE",
        "METER_ID",
        "TNI",
        "FRMP",
        "DRSP",
        "PERIODID",
        "METEREDQUANTITYIMPORT",
        "METEREDQUANTITYEXPORT",
        "BASELINEQUANTITY",
        "QUALITYFLAG",
        "ISNONCOMPLIANT",
        "BASELINECALCULATIONID",
    ];
    type Row<'row> = MeterdataWdrReads1Row<'row>;
    type FieldMapping = MeterdataWdrReads1Mapping;
    type PrimaryKey = MeterdataWdrReads1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterdataWdrReads1Row {
            market_id: row.get_range("market_id", field_mapping.0[0])?,
            case_id: row
                .get_custom_parsed_at_idx(
                    "case_id",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            meter_id: row.get_range("meter_id", field_mapping.0[3])?,
            tni: row.get_opt_range("tni", field_mapping.0[4])?,
            frmp: row.get_opt_range("frmp", field_mapping.0[5])?,
            drsp: row.get_opt_range("drsp", field_mapping.0[6])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meteredquantityimport: row
                .get_opt_custom_parsed_at_idx(
                    "meteredquantityimport",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meteredquantityexport: row
                .get_opt_custom_parsed_at_idx(
                    "meteredquantityexport",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            baselinequantity: row
                .get_opt_custom_parsed_at_idx(
                    "baselinequantity",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            qualityflag: row.get_opt_range("qualityflag", field_mapping.0[11])?,
            isnoncompliant: row
                .get_opt_custom_parsed_at_idx(
                    "isnoncompliant",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            baselinecalculationid: row
                .get_opt_range("baselinecalculationid", field_mapping.0[13])?,
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
        Ok(MeterdataWdrReads1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterdataWdrReads1PrimaryKey {
        MeterdataWdrReads1PrimaryKey {
            case_id: row.case_id,
            market_id: row.market_id().to_string(),
            meter_id: row.meter_id().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meterdata_wdr_reads_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterdataWdrReads1Row {
            market_id: row.market_id.clone(),
            case_id: row.case_id.clone(),
            settlementdate: row.settlementdate.clone(),
            meter_id: row.meter_id.clone(),
            tni: row.tni.clone(),
            frmp: row.frmp.clone(),
            drsp: row.drsp.clone(),
            periodid: row.periodid.clone(),
            meteredquantityimport: row.meteredquantityimport.clone(),
            meteredquantityexport: row.meteredquantityexport.clone(),
            baselinequantity: row.baselinequantity.clone(),
            qualityflag: row.qualityflag.clone(),
            isnoncompliant: row.isnoncompliant.clone(),
            baselinecalculationid: row.baselinecalculationid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataWdrReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub market_id: alloc::string::String,
    pub meter_id: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataWdrReads1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterdataWdrReads1Row<'data> {
    type Row<'other> = MeterdataWdrReads1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id && self.market_id() == row.market_id()
            && self.meter_id() == row.meter_id() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterdataWdrReads1Row<'data> {
    type PrimaryKey = MeterdataWdrReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.market_id() == key.market_id
            && self.meter_id() == key.meter_id && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterdataWdrReads1PrimaryKey {
    type Row<'other> = MeterdataWdrReads1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id && self.market_id == row.market_id()
            && self.meter_id == row.meter_id() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataWdrReads1PrimaryKey {
    type PrimaryKey = MeterdataWdrReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.market_id == key.market_id
            && self.meter_id == key.meter_id && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataWdrReads1 {
    type Builder = MeterdataWdrReads1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "market_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "case_id",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meter_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tni",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "frmp",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "drsp",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meteredquantityimport",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meteredquantityexport",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "baselinequantity",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "qualityflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "isnoncompliant",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "baselinecalculationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MeterdataWdrReads1Builder {
            market_id_array: arrow::array::builder::StringBuilder::new(),
            case_id_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            meter_id_array: arrow::array::builder::StringBuilder::new(),
            tni_array: arrow::array::builder::StringBuilder::new(),
            frmp_array: arrow::array::builder::StringBuilder::new(),
            drsp_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            meteredquantityimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            meteredquantityexport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            baselinequantity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            qualityflag_array: arrow::array::builder::StringBuilder::new(),
            isnoncompliant_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            baselinecalculationid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.market_id_array.append_value(row.market_id());
        builder
            .case_id_array
            .append_value({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.meter_id_array.append_value(row.meter_id());
        builder.tni_array.append_option(row.tni());
        builder.frmp_array.append_option(row.frmp());
        builder.drsp_array.append_option(row.drsp());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .meteredquantityimport_array
            .append_option({
                row.meteredquantityimport
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .meteredquantityexport_array
            .append_option({
                row.meteredquantityexport
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .baselinequantity_array
            .append_option({
                row.baselinequantity
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder.qualityflag_array.append_option(row.qualityflag());
        builder
            .isnoncompliant_array
            .append_option({
                row.isnoncompliant
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.baselinecalculationid_array.append_option(row.baselinecalculationid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.market_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.case_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meter_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tni_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.frmp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.drsp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meteredquantityimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meteredquantityexport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.baselinequantity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.qualityflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.isnoncompliant_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.baselinecalculationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterdataWdrReads1Builder {
    market_id_array: arrow::array::builder::StringBuilder,
    case_id_array: arrow::array::builder::Decimal128Builder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    meter_id_array: arrow::array::builder::StringBuilder,
    tni_array: arrow::array::builder::StringBuilder,
    frmp_array: arrow::array::builder::StringBuilder,
    drsp_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    meteredquantityimport_array: arrow::array::builder::Decimal128Builder,
    meteredquantityexport_array: arrow::array::builder::Decimal128Builder,
    baselinequantity_array: arrow::array::builder::Decimal128Builder,
    qualityflag_array: arrow::array::builder::StringBuilder,
    isnoncompliant_array: arrow::array::builder::Decimal128Builder,
    baselinecalculationid_array: arrow::array::builder::StringBuilder,
}
