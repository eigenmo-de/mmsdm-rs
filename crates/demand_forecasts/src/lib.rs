#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct OperationalDemandActual3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &OperationalDemandActual3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl OperationalDemandActual3 {
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
pub struct OperationalDemandActual3Mapping([usize; 6]);
/// # Summary
///
/// ## DEMANDOPERATIONALACTUAL
///
/// Shows Actual Operational Demand for a particular date time interval.
///
/// * Data Set Name: Operational Demand
/// * File Name: Actual
/// * Data Version: 3
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct OperationalDemandActual3Row<'data> {
    /// Date time interval for operational demand value
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: core::ops::Range<usize>,
    /// Average 30-minute measured operational demand MW value (unadjusted)
    pub operational_demand: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Adjustment value containing the estimated amount of activated RERT and involuntary load shedding that occurred as a result of a NER 4.8.9 instruction for load shedding from AEMO.
    pub operational_demand_adjustment: Option<rust_decimal::Decimal>,
    /// Estimated average 30-minute MW amount of Wholesale Demand Response that occurred
    pub wdr_estimate: Option<i64>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> OperationalDemandActual3Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for OperationalDemandActual3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "OPERATIONAL_DEMAND";
    const TABLE_NAME: &'static str = "ACTUAL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = OperationalDemandActual3Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERVAL_DATETIME",
        "REGIONID",
        "OPERATIONAL_DEMAND",
        "LASTCHANGED",
        "OPERATIONAL_DEMAND_ADJUSTMENT",
        "WDR_ESTIMATE",
    ];
    type Row<'row> = OperationalDemandActual3Row<'row>;
    type FieldMapping = OperationalDemandActual3Mapping;
    type PrimaryKey = OperationalDemandActual3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(OperationalDemandActual3Row {
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            operational_demand: row
                .get_opt_custom_parsed_at_idx(
                    "operational_demand",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            operational_demand_adjustment: row
                .get_opt_custom_parsed_at_idx(
                    "operational_demand_adjustment",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_estimate: row.get_opt_parsed_at_idx("wdr_estimate", field_mapping.0[5])?,
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
        Ok(OperationalDemandActual3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> OperationalDemandActual3PrimaryKey {
        OperationalDemandActual3PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("operational_demand_actual_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        OperationalDemandActual3Row {
            interval_datetime: row.interval_datetime.clone(),
            regionid: row.regionid.clone(),
            operational_demand: row.operational_demand.clone(),
            lastchanged: row.lastchanged.clone(),
            operational_demand_adjustment: row.operational_demand_adjustment.clone(),
            wdr_estimate: row.wdr_estimate.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationalDemandActual3PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for OperationalDemandActual3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for OperationalDemandActual3Row<'data> {
    type Row<'other> = OperationalDemandActual3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for OperationalDemandActual3Row<'data> {
    type PrimaryKey = OperationalDemandActual3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for OperationalDemandActual3PrimaryKey {
    type Row<'other> = OperationalDemandActual3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for OperationalDemandActual3PrimaryKey {
    type PrimaryKey = OperationalDemandActual3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for OperationalDemandActual3 {
    type Builder = OperationalDemandActual3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "interval_datetime",
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
                    "operational_demand",
                    arrow::datatypes::DataType::Decimal128(10, 0),
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
                    "operational_demand_adjustment",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdr_estimate",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        OperationalDemandActual3Builder {
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            operational_demand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            operational_demand_adjustment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            wdr_estimate_array: arrow::array::builder::Int64Builder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .operational_demand_array
            .append_option({
                row.operational_demand
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .operational_demand_adjustment_array
            .append_option({
                row.operational_demand_adjustment
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.wdr_estimate_array.append_option(row.wdr_estimate);
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.operational_demand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.operational_demand_adjustment_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdr_estimate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct OperationalDemandActual3Builder {
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    operational_demand_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    operational_demand_adjustment_array: arrow::array::builder::Decimal128Builder,
    wdr_estimate_array: arrow::array::builder::Int64Builder,
}
pub struct OperationalDemandForecast1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &OperationalDemandForecast1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl OperationalDemandForecast1 {
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
pub struct OperationalDemandForecast1Mapping([usize; 7]);
/// # Summary
///
/// ## DEMANDOPERATIONALFORECAST
///
/// Shows Forecast Operational Demand for a particular date time interval.
///
/// * Data Set Name: Operational Demand
/// * File Name: Forecast
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
/// * INTERVAL_DATETIME
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct OperationalDemandForecast1Row<'data> {
    /// Forecast for a particular date time interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: core::ops::Range<usize>,
    /// Date time this forecast was produced
    pub load_date: Option<chrono::NaiveDateTime>,
    /// 10% probability of exceedance operational demand forecast value
    pub operational_demand_poe10: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance operational demand forecast value
    pub operational_demand_poe50: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance operational demand forecast value
    pub operational_demand_poe90: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> OperationalDemandForecast1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for OperationalDemandForecast1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "OPERATIONAL_DEMAND";
    const TABLE_NAME: &'static str = "FORECAST";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = OperationalDemandForecast1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERVAL_DATETIME",
        "REGIONID",
        "LOAD_DATE",
        "OPERATIONAL_DEMAND_POE10",
        "OPERATIONAL_DEMAND_POE50",
        "OPERATIONAL_DEMAND_POE90",
        "LASTCHANGED",
    ];
    type Row<'row> = OperationalDemandForecast1Row<'row>;
    type FieldMapping = OperationalDemandForecast1Mapping;
    type PrimaryKey = OperationalDemandForecast1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(OperationalDemandForecast1Row {
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            load_date: row
                .get_opt_custom_parsed_at_idx(
                    "load_date",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            operational_demand_poe10: row
                .get_opt_custom_parsed_at_idx(
                    "operational_demand_poe10",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            operational_demand_poe50: row
                .get_opt_custom_parsed_at_idx(
                    "operational_demand_poe50",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            operational_demand_poe90: row
                .get_opt_custom_parsed_at_idx(
                    "operational_demand_poe90",
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
        Ok(OperationalDemandForecast1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> OperationalDemandForecast1PrimaryKey {
        OperationalDemandForecast1PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("operational_demand_forecast_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        OperationalDemandForecast1Row {
            interval_datetime: row.interval_datetime.clone(),
            regionid: row.regionid.clone(),
            load_date: row.load_date.clone(),
            operational_demand_poe10: row.operational_demand_poe10.clone(),
            operational_demand_poe50: row.operational_demand_poe50.clone(),
            operational_demand_poe90: row.operational_demand_poe90.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationalDemandForecast1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for OperationalDemandForecast1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for OperationalDemandForecast1Row<'data> {
    type Row<'other> = OperationalDemandForecast1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for OperationalDemandForecast1Row<'data> {
    type PrimaryKey = OperationalDemandForecast1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for OperationalDemandForecast1PrimaryKey {
    type Row<'other> = OperationalDemandForecast1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for OperationalDemandForecast1PrimaryKey {
    type PrimaryKey = OperationalDemandForecast1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for OperationalDemandForecast1 {
    type Builder = OperationalDemandForecast1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "interval_datetime",
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
                    "load_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operational_demand_poe10",
                    arrow::datatypes::DataType::Decimal128(15, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operational_demand_poe50",
                    arrow::datatypes::DataType::Decimal128(15, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operational_demand_poe90",
                    arrow::datatypes::DataType::Decimal128(15, 2),
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
        OperationalDemandForecast1Builder {
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            load_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            operational_demand_poe10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 2)),
            operational_demand_poe50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 2)),
            operational_demand_poe90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .load_date_array
            .append_option(row.load_date.map(|val| val.and_utc().timestamp_millis()));
        builder
            .operational_demand_poe10_array
            .append_option({
                row.operational_demand_poe10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .operational_demand_poe50_array
            .append_option({
                row.operational_demand_poe50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .operational_demand_poe90_array
            .append_option({
                row.operational_demand_poe90
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.operational_demand_poe10_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.operational_demand_poe50_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.operational_demand_poe90_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct OperationalDemandForecast1Builder {
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    load_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    operational_demand_poe10_array: arrow::array::builder::Decimal128Builder,
    operational_demand_poe50_array: arrow::array::builder::Decimal128Builder,
    operational_demand_poe90_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DemandIntermittentClusterAvail2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandIntermittentClusterAvail2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandIntermittentClusterAvail2 {
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
pub struct DemandIntermittentClusterAvail2Mapping([usize; 7]);
/// # Summary
///
/// ## INTERMITTENT_CLUSTER_AVAIL
///
/// A submission of expected plant availability for an intermittent generating unit cluster, by Trading Day and Trading Interval.
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Cluster Avail
/// * Data Version: 2
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DemandIntermittentClusterAvail2Row<'data> {
    /// The trading day to which the availability submission applies
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: core::ops::Range<usize>,
    /// Date and Time when this cluster availability submission was loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: core::ops::Range<usize>,
    /// Trading interval number (1…48) within this TRADINGDATE for which ELEMENTS_UNAVAILABLE applies
    pub periodid: rust_decimal::Decimal,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE and PERIODID (scheduled maintenance in AWEFS/ASEFS). Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable
    pub elements_unavailable: Option<rust_decimal::Decimal>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are available for this TRADINGDATE and PERIODID (scheduled maintenance in AWEFS/ASEFS). Value between 0 and the registered Number of Cluster Elements. Value = 0 means no elements available
    pub elements_available: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandIntermittentClusterAvail2Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn clusterid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.clusterid.clone())
    }
}
impl mmsdm_core::GetTable for DemandIntermittentClusterAvail2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "INTERMITTENT_CLUSTER_AVAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandIntermittentClusterAvail2Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRADINGDATE",
        "DUID",
        "OFFERDATETIME",
        "CLUSTERID",
        "PERIODID",
        "ELEMENTS_UNAVAILABLE",
        "ELEMENTS_AVAILABLE",
    ];
    type Row<'row> = DemandIntermittentClusterAvail2Row<'row>;
    type FieldMapping = DemandIntermittentClusterAvail2Mapping;
    type PrimaryKey = DemandIntermittentClusterAvail2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandIntermittentClusterAvail2Row {
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            clusterid: row.get_range("clusterid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            elements_unavailable: row
                .get_opt_custom_parsed_at_idx(
                    "elements_unavailable",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            elements_available: row
                .get_opt_custom_parsed_at_idx(
                    "elements_available",
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
        Ok(DemandIntermittentClusterAvail2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandIntermittentClusterAvail2PrimaryKey {
        DemandIntermittentClusterAvail2PrimaryKey {
            clusterid: row.clusterid().to_string(),
            duid: row.duid().to_string(),
            offerdatetime: row.offerdatetime,
            periodid: row.periodid,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "demand_intermittent_cluster_avail_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandIntermittentClusterAvail2Row {
            tradingdate: row.tradingdate.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            clusterid: row.clusterid.clone(),
            periodid: row.periodid.clone(),
            elements_unavailable: row.elements_unavailable.clone(),
            elements_available: row.elements_available.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandIntermittentClusterAvail2PrimaryKey {
    pub clusterid: alloc::string::String,
    pub duid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandIntermittentClusterAvail2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentClusterAvail2Row<'data> {
    type Row<'other> = DemandIntermittentClusterAvail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.clusterid() == row.clusterid() && self.duid() == row.duid()
            && self.offerdatetime == row.offerdatetime && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DemandIntermittentClusterAvail2Row<'data> {
    type PrimaryKey = DemandIntermittentClusterAvail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid() == key.clusterid && self.duid() == key.duid
            && self.offerdatetime == key.offerdatetime && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentClusterAvail2PrimaryKey {
    type Row<'other> = DemandIntermittentClusterAvail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.clusterid == row.clusterid() && self.duid == row.duid()
            && self.offerdatetime == row.offerdatetime && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandIntermittentClusterAvail2PrimaryKey {
    type PrimaryKey = DemandIntermittentClusterAvail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandIntermittentClusterAvail2 {
    type Builder = DemandIntermittentClusterAvail2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "clusterid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "elements_unavailable",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "elements_available",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandIntermittentClusterAvail2Builder {
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            clusterid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            elements_unavailable_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            elements_available_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .tradingdate_array
            .append_value(row.tradingdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder.clusterid_array.append_value(row.clusterid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .elements_unavailable_array
            .append_option({
                row.elements_unavailable
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .elements_available_array
            .append_option({
                row.elements_available
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
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clusterid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.elements_unavailable_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.elements_available_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandIntermittentClusterAvail2Builder {
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    clusterid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    elements_unavailable_array: arrow::array::builder::Decimal128Builder,
    elements_available_array: arrow::array::builder::Decimal128Builder,
}
pub struct DemandIntermittentClusterAvailDay1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandIntermittentClusterAvailDay1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandIntermittentClusterAvailDay1 {
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
pub struct DemandIntermittentClusterAvailDay1Mapping([usize; 4]);
/// # Summary
///
/// ## INTERMITTENT_CLUSTER_AVAIL_DAY
///
/// Summary record for an availability submission for an intermittent generating unit cluster for a Trading Day.
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Cluster Avail Day
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DemandIntermittentClusterAvailDay1Row<'data> {
    /// Trading Day for which this cluster availability submission applies
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: core::ops::Range<usize>,
    /// Date and Time when this cluster availability submission was loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandIntermittentClusterAvailDay1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn clusterid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.clusterid.clone())
    }
}
impl mmsdm_core::GetTable for DemandIntermittentClusterAvailDay1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "INTERMITTENT_CLUSTER_AVAIL_DAY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandIntermittentClusterAvailDay1Mapping([
        4, 5, 6, 7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRADINGDATE",
        "DUID",
        "OFFERDATETIME",
        "CLUSTERID",
    ];
    type Row<'row> = DemandIntermittentClusterAvailDay1Row<'row>;
    type FieldMapping = DemandIntermittentClusterAvailDay1Mapping;
    type PrimaryKey = DemandIntermittentClusterAvailDay1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandIntermittentClusterAvailDay1Row {
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            clusterid: row.get_range("clusterid", field_mapping.0[3])?,
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
        Ok(DemandIntermittentClusterAvailDay1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandIntermittentClusterAvailDay1PrimaryKey {
        DemandIntermittentClusterAvailDay1PrimaryKey {
            clusterid: row.clusterid().to_string(),
            duid: row.duid().to_string(),
            offerdatetime: row.offerdatetime,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "demand_intermittent_cluster_avail_day_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandIntermittentClusterAvailDay1Row {
            tradingdate: row.tradingdate.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            clusterid: row.clusterid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandIntermittentClusterAvailDay1PrimaryKey {
    pub clusterid: alloc::string::String,
    pub duid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandIntermittentClusterAvailDay1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentClusterAvailDay1Row<'data> {
    type Row<'other> = DemandIntermittentClusterAvailDay1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.clusterid() == row.clusterid() && self.duid() == row.duid()
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DemandIntermittentClusterAvailDay1Row<'data> {
    type PrimaryKey = DemandIntermittentClusterAvailDay1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid() == key.clusterid && self.duid() == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentClusterAvailDay1PrimaryKey {
    type Row<'other> = DemandIntermittentClusterAvailDay1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.clusterid == row.clusterid() && self.duid == row.duid()
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandIntermittentClusterAvailDay1PrimaryKey {
    type PrimaryKey = DemandIntermittentClusterAvailDay1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandIntermittentClusterAvailDay1 {
    type Builder = DemandIntermittentClusterAvailDay1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "clusterid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandIntermittentClusterAvailDay1Builder {
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            clusterid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .tradingdate_array
            .append_value(row.tradingdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder.clusterid_array.append_value(row.clusterid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clusterid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandIntermittentClusterAvailDay1Builder {
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    clusterid_array: arrow::array::builder::StringBuilder,
}
pub struct DemandIntermittentDsPred1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandIntermittentDsPred1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandIntermittentDsPred1 {
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
pub struct DemandIntermittentDsPred1Mapping([usize; 10]);
/// # Summary
///
/// ## INTERMITTENT_DS_PRED
///
/// Unconstrained Intermittent Generation Forecasts (UIGF) for Dispatch
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Ds Pred
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * FORECAST_PRIORITY
/// * INTERVAL_DATETIME
/// * OFFERDATETIME
/// * ORIGIN
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct DemandIntermittentDsPred1Row<'data> {
    /// Date and Time when the forecast applies (dispatch interval ending)
    pub run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    pub duid: core::ops::Range<usize>,
    /// Date and Time when this forecast submission was loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// Date and Time when the forecast applies (dispatch interval ending)
    pub interval_datetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    pub origin: core::ops::Range<usize>,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values
    pub forecast_priority: rust_decimal::Decimal,
    /// Forecast MW value for this interval_DateTime
    pub forecast_mean: Option<rust_decimal::Decimal>,
    /// Forecast 10% POE MW value for this interval_DateTime
    pub forecast_poe10: Option<rust_decimal::Decimal>,
    /// Forecast 50% POE MW value for this interval_DateTime. Used in Dispatch.
    pub forecast_poe50: Option<rust_decimal::Decimal>,
    /// Forecast 90% POE MW value for this interval_DateTime
    pub forecast_poe90: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandIntermittentDsPred1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn origin(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.origin.clone())
    }
}
impl mmsdm_core::GetTable for DemandIntermittentDsPred1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "INTERMITTENT_DS_PRED";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandIntermittentDsPred1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "DUID",
        "OFFERDATETIME",
        "INTERVAL_DATETIME",
        "ORIGIN",
        "FORECAST_PRIORITY",
        "FORECAST_MEAN",
        "FORECAST_POE10",
        "FORECAST_POE50",
        "FORECAST_POE90",
    ];
    type Row<'row> = DemandIntermittentDsPred1Row<'row>;
    type FieldMapping = DemandIntermittentDsPred1Mapping;
    type PrimaryKey = DemandIntermittentDsPred1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandIntermittentDsPred1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            origin: row.get_range("origin", field_mapping.0[4])?,
            forecast_priority: row
                .get_custom_parsed_at_idx(
                    "forecast_priority",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            forecast_mean: row
                .get_opt_custom_parsed_at_idx(
                    "forecast_mean",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            forecast_poe10: row
                .get_opt_custom_parsed_at_idx(
                    "forecast_poe10",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            forecast_poe50: row
                .get_opt_custom_parsed_at_idx(
                    "forecast_poe50",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            forecast_poe90: row
                .get_opt_custom_parsed_at_idx(
                    "forecast_poe90",
                    field_mapping.0[9],
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
        Ok(DemandIntermittentDsPred1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandIntermittentDsPred1PrimaryKey {
        DemandIntermittentDsPred1PrimaryKey {
            duid: row.duid().to_string(),
            forecast_priority: row.forecast_priority,
            interval_datetime: row.interval_datetime,
            offerdatetime: row.offerdatetime,
            origin: row.origin().to_string(),
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("demand_intermittent_ds_pred_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandIntermittentDsPred1Row {
            run_datetime: row.run_datetime.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            origin: row.origin.clone(),
            forecast_priority: row.forecast_priority.clone(),
            forecast_mean: row.forecast_mean.clone(),
            forecast_poe10: row.forecast_poe10.clone(),
            forecast_poe50: row.forecast_poe50.clone(),
            forecast_poe90: row.forecast_poe90.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandIntermittentDsPred1PrimaryKey {
    pub duid: alloc::string::String,
    pub forecast_priority: rust_decimal::Decimal,
    pub interval_datetime: chrono::NaiveDateTime,
    pub offerdatetime: chrono::NaiveDateTime,
    pub origin: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandIntermittentDsPred1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentDsPred1Row<'data> {
    type Row<'other> = DemandIntermittentDsPred1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.forecast_priority == row.forecast_priority
            && self.interval_datetime == row.interval_datetime
            && self.offerdatetime == row.offerdatetime && self.origin() == row.origin()
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DemandIntermittentDsPred1Row<'data> {
    type PrimaryKey = DemandIntermittentDsPred1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.forecast_priority == key.forecast_priority
            && self.interval_datetime == key.interval_datetime
            && self.offerdatetime == key.offerdatetime && self.origin() == key.origin
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentDsPred1PrimaryKey {
    type Row<'other> = DemandIntermittentDsPred1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.forecast_priority == row.forecast_priority
            && self.interval_datetime == row.interval_datetime
            && self.offerdatetime == row.offerdatetime && self.origin == row.origin()
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandIntermittentDsPred1PrimaryKey {
    type PrimaryKey = DemandIntermittentDsPred1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.forecast_priority == key.forecast_priority
            && self.interval_datetime == key.interval_datetime
            && self.offerdatetime == key.offerdatetime && self.origin == key.origin
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandIntermittentDsPred1 {
    type Builder = DemandIntermittentDsPred1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "origin",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "forecast_priority",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "forecast_mean",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "forecast_poe10",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "forecast_poe50",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "forecast_poe90",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandIntermittentDsPred1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            origin_array: arrow::array::builder::StringBuilder::new(),
            forecast_priority_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            forecast_mean_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            forecast_poe10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            forecast_poe50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            forecast_poe90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.origin_array.append_value(row.origin());
        builder
            .forecast_priority_array
            .append_value({
                let mut val = row.forecast_priority;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .forecast_mean_array
            .append_option({
                row.forecast_mean
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .forecast_poe10_array
            .append_option({
                row.forecast_poe10
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .forecast_poe50_array
            .append_option({
                row.forecast_poe50
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .forecast_poe90_array
            .append_option({
                row.forecast_poe90
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.origin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.forecast_priority_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.forecast_mean_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.forecast_poe10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.forecast_poe50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.forecast_poe90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandIntermittentDsPred1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    origin_array: arrow::array::builder::StringBuilder,
    forecast_priority_array: arrow::array::builder::Decimal128Builder,
    forecast_mean_array: arrow::array::builder::Decimal128Builder,
    forecast_poe10_array: arrow::array::builder::Decimal128Builder,
    forecast_poe50_array: arrow::array::builder::Decimal128Builder,
    forecast_poe90_array: arrow::array::builder::Decimal128Builder,
}
pub struct DemandIntermittentDsRun1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandIntermittentDsRun1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandIntermittentDsRun1 {
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
pub struct DemandIntermittentDsRun1Mapping([usize; 13]);
/// # Summary
///
/// ## INTERMITTENT_DS_RUN
///
/// Unconstrained Intermittent Generation Forecasts (UIGF) for Dispatch.
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Ds Run
/// * Data Version: 1
///
/// # Description
/// SourceINTERMITTENT_GEN_FCST_DATA updates every 30 minutes when AEMO issues a new 30-minute forecast of intermittent generation out to 8 days ahead.Volume~18,000 rows per generator per year
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * FORECAST_PRIORITY
/// * OFFERDATETIME
/// * ORIGIN
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct DemandIntermittentDsRun1Row<'data> {
    /// Date and Time where the forecast applies (dispatch interval ending)
    pub run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    pub duid: core::ops::Range<usize>,
    /// Date and Time when this forecast submission was loaded.
    pub offerdatetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    pub origin: core::ops::Range<usize>,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values.
    pub forecast_priority: rust_decimal::Decimal,
    /// Authorising officer of this forecast (applicable for participant forecasts only). This column is not made available to the public.
    pub authorisedby: core::ops::Range<usize>,
    /// Comments relating to the forecast. This column is not made available to the public.
    pub comments: core::ops::Range<usize>,
    /// Last date and time the record changed.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Metadata relating to the forecast. This column is not made available to the public.
    pub model: core::ops::Range<usize>,
    /// Participant can document when the forecast was created
    pub participant_timestamp: Option<chrono::NaiveDateTime>,
    /// Was this forecast suppressed by AEMO? Suppressed = 1,Not suppressed =0
    pub suppressed_aemo: Option<rust_decimal::Decimal>,
    /// Was this forecast suppressed by the participant? Suppressed submissions may not be used,  Suppressed = 1, Not suppressed =0
    pub suppressed_participant: Option<rust_decimal::Decimal>,
    /// Uniquely identifies this interaction
    pub transaction_id: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandIntermittentDsRun1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn origin(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.origin.clone())
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
    pub fn comments(&self) -> Option<&str> {
        if self.comments.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.comments.clone(),
                ),
            )
        }
    }
    pub fn model(&self) -> Option<&str> {
        if self.model.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.model.clone()),
            )
        }
    }
    pub fn transaction_id(&self) -> Option<&str> {
        if self.transaction_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.transaction_id.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DemandIntermittentDsRun1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "INTERMITTENT_DS_RUN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandIntermittentDsRun1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "DUID",
        "OFFERDATETIME",
        "ORIGIN",
        "FORECAST_PRIORITY",
        "AUTHORISEDBY",
        "COMMENTS",
        "LASTCHANGED",
        "MODEL",
        "PARTICIPANT_TIMESTAMP",
        "SUPPRESSED_AEMO",
        "SUPPRESSED_PARTICIPANT",
        "TRANSACTION_ID",
    ];
    type Row<'row> = DemandIntermittentDsRun1Row<'row>;
    type FieldMapping = DemandIntermittentDsRun1Mapping;
    type PrimaryKey = DemandIntermittentDsRun1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandIntermittentDsRun1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            origin: row.get_range("origin", field_mapping.0[3])?,
            forecast_priority: row
                .get_custom_parsed_at_idx(
                    "forecast_priority",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[5])?,
            comments: row.get_opt_range("comments", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            model: row.get_opt_range("model", field_mapping.0[8])?,
            participant_timestamp: row
                .get_opt_custom_parsed_at_idx(
                    "participant_timestamp",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            suppressed_aemo: row
                .get_opt_custom_parsed_at_idx(
                    "suppressed_aemo",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            suppressed_participant: row
                .get_opt_custom_parsed_at_idx(
                    "suppressed_participant",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            transaction_id: row.get_opt_range("transaction_id", field_mapping.0[12])?,
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
        Ok(DemandIntermittentDsRun1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandIntermittentDsRun1PrimaryKey {
        DemandIntermittentDsRun1PrimaryKey {
            duid: row.duid().to_string(),
            forecast_priority: row.forecast_priority,
            offerdatetime: row.offerdatetime,
            origin: row.origin().to_string(),
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("demand_intermittent_ds_run_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandIntermittentDsRun1Row {
            run_datetime: row.run_datetime.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            origin: row.origin.clone(),
            forecast_priority: row.forecast_priority.clone(),
            authorisedby: row.authorisedby.clone(),
            comments: row.comments.clone(),
            lastchanged: row.lastchanged.clone(),
            model: row.model.clone(),
            participant_timestamp: row.participant_timestamp.clone(),
            suppressed_aemo: row.suppressed_aemo.clone(),
            suppressed_participant: row.suppressed_participant.clone(),
            transaction_id: row.transaction_id.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandIntermittentDsRun1PrimaryKey {
    pub duid: alloc::string::String,
    pub forecast_priority: rust_decimal::Decimal,
    pub offerdatetime: chrono::NaiveDateTime,
    pub origin: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandIntermittentDsRun1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentDsRun1Row<'data> {
    type Row<'other> = DemandIntermittentDsRun1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.forecast_priority == row.forecast_priority
            && self.offerdatetime == row.offerdatetime && self.origin() == row.origin()
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DemandIntermittentDsRun1Row<'data> {
    type PrimaryKey = DemandIntermittentDsRun1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.forecast_priority == key.forecast_priority
            && self.offerdatetime == key.offerdatetime && self.origin() == key.origin
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentDsRun1PrimaryKey {
    type Row<'other> = DemandIntermittentDsRun1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.forecast_priority == row.forecast_priority
            && self.offerdatetime == row.offerdatetime && self.origin == row.origin()
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandIntermittentDsRun1PrimaryKey {
    type PrimaryKey = DemandIntermittentDsRun1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.forecast_priority == key.forecast_priority
            && self.offerdatetime == key.offerdatetime && self.origin == key.origin
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandIntermittentDsRun1 {
    type Builder = DemandIntermittentDsRun1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "origin",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "forecast_priority",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "comments",
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
                    "model",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participant_timestamp",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "suppressed_aemo",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "suppressed_participant",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "transaction_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandIntermittentDsRun1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            origin_array: arrow::array::builder::StringBuilder::new(),
            forecast_priority_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            comments_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            model_array: arrow::array::builder::StringBuilder::new(),
            participant_timestamp_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            suppressed_aemo_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            suppressed_participant_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            transaction_id_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder.origin_array.append_value(row.origin());
        builder
            .forecast_priority_array
            .append_value({
                let mut val = row.forecast_priority;
                val.rescale(0);
                val.mantissa()
            });
        builder.authorisedby_array.append_option(row.authorisedby());
        builder.comments_array.append_option(row.comments());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.model_array.append_option(row.model());
        builder
            .participant_timestamp_array
            .append_option(
                row.participant_timestamp.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .suppressed_aemo_array
            .append_option({
                row.suppressed_aemo
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .suppressed_participant_array
            .append_option({
                row.suppressed_participant
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.transaction_id_array.append_option(row.transaction_id());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.origin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.forecast_priority_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.comments_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.model_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participant_timestamp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.suppressed_aemo_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.suppressed_participant_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transaction_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandIntermittentDsRun1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    origin_array: arrow::array::builder::StringBuilder,
    forecast_priority_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    comments_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    model_array: arrow::array::builder::StringBuilder,
    participant_timestamp_array: arrow::array::builder::TimestampMillisecondBuilder,
    suppressed_aemo_array: arrow::array::builder::Decimal128Builder,
    suppressed_participant_array: arrow::array::builder::Decimal128Builder,
    transaction_id_array: arrow::array::builder::StringBuilder,
}
pub struct ForecastIntermittentGen1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ForecastIntermittentGen1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ForecastIntermittentGen1 {
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
pub struct ForecastIntermittentGen1Mapping([usize; 6]);
/// # Summary
///
/// ## INTERMITTENT_GEN_FCST
///
/// Identifying record for a given forecast of an intermittent generation. This table is the version table for the INTERMITTENT_GEN_FCST_DATA table which stores the individual forecast values
///
/// * Data Set Name: Forecast
/// * File Name: Intermittent Gen
/// * Data Version: 1
///
/// # Description
/// SourceINTERMITTENT_GEN_FCST_DATA updates every 30 minutes when AEMO issues a new 30-minute forecast of intermittent generation out to 8 days ahead.Volume~18,000 rows per generator per year
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * DUID
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct ForecastIntermittentGen1Row<'data> {
    /// Date Time of forecast (AEST).
    pub run_datetime: chrono::NaiveDateTime,
    /// Identifier of the intermittent generator.
    pub duid: core::ops::Range<usize>,
    /// Date Time (AEST) of the first half-hour interval being forecast.
    pub start_interval_datetime: chrono::NaiveDateTime,
    /// Date Time (AEST) of the final half-hour interval being forecast.
    pub end_interval_datetime: chrono::NaiveDateTime,
    /// Versioning information for resolution back to AEMO's wind generation forecasting system.
    pub versionno: Option<rust_decimal::Decimal>,
    /// Date Time record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForecastIntermittentGen1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for ForecastIntermittentGen1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORECAST";
    const TABLE_NAME: &'static str = "INTERMITTENT_GEN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForecastIntermittentGen1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "DUID",
        "START_INTERVAL_DATETIME",
        "END_INTERVAL_DATETIME",
        "VERSIONNO",
        "LASTCHANGED",
    ];
    type Row<'row> = ForecastIntermittentGen1Row<'row>;
    type FieldMapping = ForecastIntermittentGen1Mapping;
    type PrimaryKey = ForecastIntermittentGen1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForecastIntermittentGen1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            start_interval_datetime: row
                .get_custom_parsed_at_idx(
                    "start_interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            end_interval_datetime: row
                .get_custom_parsed_at_idx(
                    "end_interval_datetime",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
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
        Ok(ForecastIntermittentGen1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ForecastIntermittentGen1PrimaryKey {
        ForecastIntermittentGen1PrimaryKey {
            duid: row.duid().to_string(),
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("forecast_intermittent_gen_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForecastIntermittentGen1Row {
            run_datetime: row.run_datetime.clone(),
            duid: row.duid.clone(),
            start_interval_datetime: row.start_interval_datetime.clone(),
            end_interval_datetime: row.end_interval_datetime.clone(),
            versionno: row.versionno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForecastIntermittentGen1PrimaryKey {
    pub duid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for ForecastIntermittentGen1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ForecastIntermittentGen1Row<'data> {
    type Row<'other> = ForecastIntermittentGen1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ForecastIntermittentGen1Row<'data> {
    type PrimaryKey = ForecastIntermittentGen1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for ForecastIntermittentGen1PrimaryKey {
    type Row<'other> = ForecastIntermittentGen1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ForecastIntermittentGen1PrimaryKey {
    type PrimaryKey = ForecastIntermittentGen1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForecastIntermittentGen1 {
    type Builder = ForecastIntermittentGen1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "start_interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "end_interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(10, 0),
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
        ForecastIntermittentGen1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            start_interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            end_interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .start_interval_datetime_array
            .append_value(row.start_interval_datetime.and_utc().timestamp_millis());
        builder
            .end_interval_datetime_array
            .append_value(row.end_interval_datetime.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_option({
                row.versionno
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.start_interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.end_interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForecastIntermittentGen1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    start_interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    end_interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ForecastIntermittentGenData1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ForecastIntermittentGenData1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ForecastIntermittentGenData1 {
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
pub struct ForecastIntermittentGenData1Mapping([usize; 8]);
/// # Summary
///
/// ## INTERMITTENT_GEN_FCST_DATA
///
/// Stores the forecast generation (MW) for each interval within a given forecast of an intermittent generator.
///
/// * Data Set Name: Forecast
/// * File Name: Intermittent Gen Data
/// * Data Version: 1
///
/// # Description
/// SourceINTERMITTENT_GEN_FCST_DATA updates every 30 minutes when AEMO issues a new 30-minute forecast of wind generation out to 8 days ahead.Volume~1,500,000 rows per generator per year
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct ForecastIntermittentGenData1Row<'data> {
    /// Date Time of forecast (AEST).
    pub run_datetime: chrono::NaiveDateTime,
    /// Identifier of the intermittent generator
    pub duid: core::ops::Range<usize>,
    /// Date Time (AEST) of the halfhour interval being forecast
    pub interval_datetime: chrono::NaiveDateTime,
    /// The average forecast value in MW at the interval end
    pub powermean: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance forecast value in MW at the interval end
    pub powerpoe50: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance forecast value in MW at the interval end
    pub powerpoelow: Option<rust_decimal::Decimal>,
    /// 10% probability of exceedance forecast value in MW at the interval end
    pub powerpoehigh: Option<rust_decimal::Decimal>,
    /// Date Time record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForecastIntermittentGenData1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for ForecastIntermittentGenData1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORECAST";
    const TABLE_NAME: &'static str = "INTERMITTENT_GEN_DATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForecastIntermittentGenData1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "DUID",
        "INTERVAL_DATETIME",
        "POWERMEAN",
        "POWERPOE50",
        "POWERPOELOW",
        "POWERPOEHIGH",
        "LASTCHANGED",
    ];
    type Row<'row> = ForecastIntermittentGenData1Row<'row>;
    type FieldMapping = ForecastIntermittentGenData1Mapping;
    type PrimaryKey = ForecastIntermittentGenData1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForecastIntermittentGenData1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            powermean: row
                .get_opt_custom_parsed_at_idx(
                    "powermean",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            powerpoe50: row
                .get_opt_custom_parsed_at_idx(
                    "powerpoe50",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            powerpoelow: row
                .get_opt_custom_parsed_at_idx(
                    "powerpoelow",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            powerpoehigh: row
                .get_opt_custom_parsed_at_idx(
                    "powerpoehigh",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
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
        Ok(ForecastIntermittentGenData1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ForecastIntermittentGenData1PrimaryKey {
        ForecastIntermittentGenData1PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("forecast_intermittent_gen_data_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForecastIntermittentGenData1Row {
            run_datetime: row.run_datetime.clone(),
            duid: row.duid.clone(),
            interval_datetime: row.interval_datetime.clone(),
            powermean: row.powermean.clone(),
            powerpoe50: row.powerpoe50.clone(),
            powerpoelow: row.powerpoelow.clone(),
            powerpoehigh: row.powerpoehigh.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForecastIntermittentGenData1PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for ForecastIntermittentGenData1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ForecastIntermittentGenData1Row<'data> {
    type Row<'other> = ForecastIntermittentGenData1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ForecastIntermittentGenData1Row<'data> {
    type PrimaryKey = ForecastIntermittentGenData1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for ForecastIntermittentGenData1PrimaryKey {
    type Row<'other> = ForecastIntermittentGenData1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ForecastIntermittentGenData1PrimaryKey {
    type PrimaryKey = ForecastIntermittentGenData1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForecastIntermittentGenData1 {
    type Builder = ForecastIntermittentGenData1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "powermean",
                    arrow::datatypes::DataType::Decimal128(9, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "powerpoe50",
                    arrow::datatypes::DataType::Decimal128(9, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "powerpoelow",
                    arrow::datatypes::DataType::Decimal128(9, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "powerpoehigh",
                    arrow::datatypes::DataType::Decimal128(9, 3),
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
        ForecastIntermittentGenData1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            powermean_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 3)),
            powerpoe50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 3)),
            powerpoelow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 3)),
            powerpoehigh_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 3)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder
            .powermean_array
            .append_option({
                row.powermean
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .powerpoe50_array
            .append_option({
                row.powerpoe50
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .powerpoelow_array
            .append_option({
                row.powerpoelow
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .powerpoehigh_array
            .append_option({
                row.powerpoehigh
                    .map(|mut val| {
                        val.rescale(3);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powermean_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powerpoe50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powerpoelow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powerpoehigh_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForecastIntermittentGenData1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    powermean_array: arrow::array::builder::Decimal128Builder,
    powerpoe50_array: arrow::array::builder::Decimal128Builder,
    powerpoelow_array: arrow::array::builder::Decimal128Builder,
    powerpoehigh_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DemandIntermittentGenLimit1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandIntermittentGenLimit1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandIntermittentGenLimit1 {
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
pub struct DemandIntermittentGenLimit1Mapping([usize; 5]);
/// # Summary
///
/// ## INTERMITTENT_GEN_LIMIT
///
/// A submission of Upper MW Limit for an intermittent generating unit, by Trading Day and Trading Interval
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Gen Limit
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DemandIntermittentGenLimit1Row<'data> {
    /// Trading Day for which this unit availability submission applies
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: core::ops::Range<usize>,
    /// Date and Time when this unit availability submission was loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// Trading interval number (1...48) within this TRADINGDATE for which UPPERMWLIMIT applies
    pub periodid: rust_decimal::Decimal,
    /// Maximum imposed MW limit (down regulation in AWEFS/ASEFS). Value between 0 and the registered DUID Maximum Capacity. Value = -1 means no limit applies.
    pub uppermwlimit: Option<i64>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandIntermittentGenLimit1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for DemandIntermittentGenLimit1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "INTERMITTENT_GEN_LIMIT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandIntermittentGenLimit1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRADINGDATE",
        "DUID",
        "OFFERDATETIME",
        "PERIODID",
        "UPPERMWLIMIT",
    ];
    type Row<'row> = DemandIntermittentGenLimit1Row<'row>;
    type FieldMapping = DemandIntermittentGenLimit1Mapping;
    type PrimaryKey = DemandIntermittentGenLimit1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandIntermittentGenLimit1Row {
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uppermwlimit: row.get_opt_parsed_at_idx("uppermwlimit", field_mapping.0[4])?,
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
        Ok(DemandIntermittentGenLimit1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandIntermittentGenLimit1PrimaryKey {
        DemandIntermittentGenLimit1PrimaryKey {
            duid: row.duid().to_string(),
            offerdatetime: row.offerdatetime,
            periodid: row.periodid,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("demand_intermittent_gen_limit_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandIntermittentGenLimit1Row {
            tradingdate: row.tradingdate.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            periodid: row.periodid.clone(),
            uppermwlimit: row.uppermwlimit.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandIntermittentGenLimit1PrimaryKey {
    pub duid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandIntermittentGenLimit1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentGenLimit1Row<'data> {
    type Row<'other> = DemandIntermittentGenLimit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DemandIntermittentGenLimit1Row<'data> {
    type PrimaryKey = DemandIntermittentGenLimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentGenLimit1PrimaryKey {
    type Row<'other> = DemandIntermittentGenLimit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandIntermittentGenLimit1PrimaryKey {
    type PrimaryKey = DemandIntermittentGenLimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandIntermittentGenLimit1 {
    type Builder = DemandIntermittentGenLimit1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "uppermwlimit",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandIntermittentGenLimit1Builder {
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            uppermwlimit_array: arrow::array::builder::Int64Builder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .tradingdate_array
            .append_value(row.tradingdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.uppermwlimit_array.append_option(row.uppermwlimit);
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.uppermwlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandIntermittentGenLimit1Builder {
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    uppermwlimit_array: arrow::array::builder::Int64Builder,
}
pub struct DemandIntermittentGenLimitDay1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandIntermittentGenLimitDay1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandIntermittentGenLimitDay1 {
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
pub struct DemandIntermittentGenLimitDay1Mapping([usize; 7]);
/// # Summary
///
/// ## INTERMITTENT_GEN_LIMIT_DAY
///
/// Summary record for an Upper MW Limit submission for an intermittent generating unit for a Trading Day
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Gen Limit Day
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DemandIntermittentGenLimitDay1Row<'data> {
    /// Trading Day for which this unit availability submission applies
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: core::ops::Range<usize>,
    /// Date and Time when this unit availability submission was loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// User entering the unit availability submission
    pub authorisedbyuser: core::ops::Range<usize>,
    /// Participant entering the unit availability submission
    pub authorisedbyparticipantid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandIntermittentGenLimitDay1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn participantid(&self) -> Option<&str> {
        if self.participantid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.participantid.clone(),
                ),
            )
        }
    }
    pub fn authorisedbyuser(&self) -> Option<&str> {
        if self.authorisedbyuser.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedbyuser.clone(),
                ),
            )
        }
    }
    pub fn authorisedbyparticipantid(&self) -> Option<&str> {
        if self.authorisedbyparticipantid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedbyparticipantid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DemandIntermittentGenLimitDay1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "INTERMITTENT_GEN_LIMIT_DAY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandIntermittentGenLimitDay1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRADINGDATE",
        "DUID",
        "OFFERDATETIME",
        "PARTICIPANTID",
        "LASTCHANGED",
        "AUTHORISEDBYUSER",
        "AUTHORISEDBYPARTICIPANTID",
    ];
    type Row<'row> = DemandIntermittentGenLimitDay1Row<'row>;
    type FieldMapping = DemandIntermittentGenLimitDay1Mapping;
    type PrimaryKey = DemandIntermittentGenLimitDay1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandIntermittentGenLimitDay1Row {
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            participantid: row.get_opt_range("participantid", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedbyuser: row.get_opt_range("authorisedbyuser", field_mapping.0[5])?,
            authorisedbyparticipantid: row
                .get_opt_range("authorisedbyparticipantid", field_mapping.0[6])?,
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
        Ok(DemandIntermittentGenLimitDay1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandIntermittentGenLimitDay1PrimaryKey {
        DemandIntermittentGenLimitDay1PrimaryKey {
            duid: row.duid().to_string(),
            offerdatetime: row.offerdatetime,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "demand_intermittent_gen_limit_day_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandIntermittentGenLimitDay1Row {
            tradingdate: row.tradingdate.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            participantid: row.participantid.clone(),
            lastchanged: row.lastchanged.clone(),
            authorisedbyuser: row.authorisedbyuser.clone(),
            authorisedbyparticipantid: row.authorisedbyparticipantid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandIntermittentGenLimitDay1PrimaryKey {
    pub duid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandIntermittentGenLimitDay1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentGenLimitDay1Row<'data> {
    type Row<'other> = DemandIntermittentGenLimitDay1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DemandIntermittentGenLimitDay1Row<'data> {
    type PrimaryKey = DemandIntermittentGenLimitDay1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentGenLimitDay1PrimaryKey {
    type Row<'other> = DemandIntermittentGenLimitDay1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandIntermittentGenLimitDay1PrimaryKey {
    type PrimaryKey = DemandIntermittentGenLimitDay1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandIntermittentGenLimitDay1 {
    type Builder = DemandIntermittentGenLimitDay1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
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
                    "authorisedbyuser",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisedbyparticipantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandIntermittentGenLimitDay1Builder {
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedbyuser_array: arrow::array::builder::StringBuilder::new(),
            authorisedbyparticipantid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .tradingdate_array
            .append_value(row.tradingdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder.participantid_array.append_option(row.participantid());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.authorisedbyuser_array.append_option(row.authorisedbyuser());
        builder
            .authorisedbyparticipantid_array
            .append_option(row.authorisedbyparticipantid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedbyuser_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.authorisedbyparticipantid_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandIntermittentGenLimitDay1Builder {
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedbyuser_array: arrow::array::builder::StringBuilder,
    authorisedbyparticipantid_array: arrow::array::builder::StringBuilder,
}
pub struct DemandIntermittentGenScada1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandIntermittentGenScada1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandIntermittentGenScada1 {
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
pub struct DemandIntermittentGenScada1Mapping([usize; 6]);
/// # Summary
///
/// ## INTERMITTENT_GEN_SCADA
///
/// INTERMITTENT_GEN_SCADA provides the SCADA Availability for every intermittent generating unit, including Elements Available (wind turbines/solar inverters) and Local Limit
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Gen Scada
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * RUN_DATETIME
/// * SCADA_TYPE
#[derive(Debug, PartialEq, Eq)]
pub struct DemandIntermittentGenScada1Row<'data> {
    /// Date Time of the dispatch interval (interval ending)
    pub run_datetime: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// SCADA snapshot for intermittent generating unit at start of interval for a specified SCADA signal type. ELAV = Total Elements Available (# turbines for wind farms, # inverters for solar farms); LOCL = Local Limit (MW).
    pub scada_type: core::ops::Range<usize>,
    /// SCADA value snapshot for intermittent generating unit at start of interval for a specified SCADA signal type.
    pub scada_value: Option<rust_decimal::Decimal>,
    /// SCADA quality snapshot for intermittent generating unit at start of interval for a specified SCADA signal type.
    pub scada_quality: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandIntermittentGenScada1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn scada_type(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.scada_type.clone())
    }
    pub fn scada_quality(&self) -> Option<&str> {
        if self.scada_quality.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.scada_quality.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DemandIntermittentGenScada1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "INTERMITTENT_GEN_SCADA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandIntermittentGenScada1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "DUID",
        "SCADA_TYPE",
        "SCADA_VALUE",
        "SCADA_QUALITY",
        "LASTCHANGED",
    ];
    type Row<'row> = DemandIntermittentGenScada1Row<'row>;
    type FieldMapping = DemandIntermittentGenScada1Mapping;
    type PrimaryKey = DemandIntermittentGenScada1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandIntermittentGenScada1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            scada_type: row.get_range("scada_type", field_mapping.0[2])?,
            scada_value: row
                .get_opt_custom_parsed_at_idx(
                    "scada_value",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            scada_quality: row.get_opt_range("scada_quality", field_mapping.0[4])?,
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
        Ok(DemandIntermittentGenScada1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandIntermittentGenScada1PrimaryKey {
        DemandIntermittentGenScada1PrimaryKey {
            duid: row.duid().to_string(),
            run_datetime: row.run_datetime,
            scada_type: row.scada_type().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("demand_intermittent_gen_scada_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandIntermittentGenScada1Row {
            run_datetime: row.run_datetime.clone(),
            duid: row.duid.clone(),
            scada_type: row.scada_type.clone(),
            scada_value: row.scada_value.clone(),
            scada_quality: row.scada_quality.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandIntermittentGenScada1PrimaryKey {
    pub duid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub scada_type: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for DemandIntermittentGenScada1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentGenScada1Row<'data> {
    type Row<'other> = DemandIntermittentGenScada1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.run_datetime == row.run_datetime
            && self.scada_type() == row.scada_type()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DemandIntermittentGenScada1Row<'data> {
    type PrimaryKey = DemandIntermittentGenScada1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.run_datetime == key.run_datetime
            && self.scada_type() == key.scada_type
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandIntermittentGenScada1PrimaryKey {
    type Row<'other> = DemandIntermittentGenScada1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.run_datetime == row.run_datetime
            && self.scada_type == row.scada_type()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandIntermittentGenScada1PrimaryKey {
    type PrimaryKey = DemandIntermittentGenScada1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.run_datetime == key.run_datetime
            && self.scada_type == key.scada_type
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandIntermittentGenScada1 {
    type Builder = DemandIntermittentGenScada1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "scada_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "scada_value",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "scada_quality",
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
        DemandIntermittentGenScada1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            scada_type_array: arrow::array::builder::StringBuilder::new(),
            scada_value_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            scada_quality_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.scada_type_array.append_value(row.scada_type());
        builder
            .scada_value_array
            .append_option({
                row.scada_value
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder.scada_quality_array.append_option(row.scada_quality());
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.scada_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.scada_value_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.scada_quality_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandIntermittentGenScada1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    scada_type_array: arrow::array::builder::StringBuilder,
    scada_value_array: arrow::array::builder::Decimal128Builder,
    scada_quality_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DemandMtpasaIntermittentAvail2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandMtpasaIntermittentAvail2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandMtpasaIntermittentAvail2 {
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
pub struct DemandMtpasaIntermittentAvail2Mapping([usize; 7]);
/// # Summary
///
/// ## MTPASA_INTERMITTENT_AVAIL
///
/// A submission of expected plant availability for intermittent generators for use in MTPASA intermittent generation forecasts
///
/// * Data Set Name: Demand
/// * File Name: Mtpasa Intermittent Avail
/// * Data Version: 2
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DemandMtpasaIntermittentAvail2Row<'data> {
    /// Trading Day for which this cluster availability submission applies
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: core::ops::Range<usize>,
    /// Date and Time when this cluster availability submission was loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE. Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable
    pub elements_unavailable: Option<rust_decimal::Decimal>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are available for this TRADINGDATE. Value between 0 and the registered Number of Cluster Elements. Value = 0 means no elements available
    pub elements_available: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandMtpasaIntermittentAvail2Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn clusterid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.clusterid.clone())
    }
}
impl mmsdm_core::GetTable for DemandMtpasaIntermittentAvail2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "MTPASA_INTERMITTENT_AVAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandMtpasaIntermittentAvail2Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRADINGDATE",
        "DUID",
        "OFFERDATETIME",
        "CLUSTERID",
        "LASTCHANGED",
        "ELEMENTS_UNAVAILABLE",
        "ELEMENTS_AVAILABLE",
    ];
    type Row<'row> = DemandMtpasaIntermittentAvail2Row<'row>;
    type FieldMapping = DemandMtpasaIntermittentAvail2Mapping;
    type PrimaryKey = DemandMtpasaIntermittentAvail2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandMtpasaIntermittentAvail2Row {
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            clusterid: row.get_range("clusterid", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            elements_unavailable: row
                .get_opt_custom_parsed_at_idx(
                    "elements_unavailable",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            elements_available: row
                .get_opt_custom_parsed_at_idx(
                    "elements_available",
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
        Ok(DemandMtpasaIntermittentAvail2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandMtpasaIntermittentAvail2PrimaryKey {
        DemandMtpasaIntermittentAvail2PrimaryKey {
            clusterid: row.clusterid().to_string(),
            duid: row.duid().to_string(),
            offerdatetime: row.offerdatetime,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "demand_mtpasa_intermittent_avail_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandMtpasaIntermittentAvail2Row {
            tradingdate: row.tradingdate.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            clusterid: row.clusterid.clone(),
            lastchanged: row.lastchanged.clone(),
            elements_unavailable: row.elements_unavailable.clone(),
            elements_available: row.elements_available.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandMtpasaIntermittentAvail2PrimaryKey {
    pub clusterid: alloc::string::String,
    pub duid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandMtpasaIntermittentAvail2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandMtpasaIntermittentAvail2Row<'data> {
    type Row<'other> = DemandMtpasaIntermittentAvail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.clusterid() == row.clusterid() && self.duid() == row.duid()
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DemandMtpasaIntermittentAvail2Row<'data> {
    type PrimaryKey = DemandMtpasaIntermittentAvail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid() == key.clusterid && self.duid() == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandMtpasaIntermittentAvail2PrimaryKey {
    type Row<'other> = DemandMtpasaIntermittentAvail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.clusterid == row.clusterid() && self.duid == row.duid()
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandMtpasaIntermittentAvail2PrimaryKey {
    type PrimaryKey = DemandMtpasaIntermittentAvail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandMtpasaIntermittentAvail2 {
    type Builder = DemandMtpasaIntermittentAvail2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "clusterid",
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
                arrow::datatypes::Field::new(
                    "elements_unavailable",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "elements_available",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandMtpasaIntermittentAvail2Builder {
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            clusterid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            elements_unavailable_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            elements_available_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .tradingdate_array
            .append_value(row.tradingdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder.clusterid_array.append_value(row.clusterid());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .elements_unavailable_array
            .append_option({
                row.elements_unavailable
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .elements_available_array
            .append_option({
                row.elements_available
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
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clusterid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.elements_unavailable_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.elements_available_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandMtpasaIntermittentAvail2Builder {
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    clusterid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    elements_unavailable_array: arrow::array::builder::Decimal128Builder,
    elements_available_array: arrow::array::builder::Decimal128Builder,
}
pub struct DemandMtpasaIntermittentLimit1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandMtpasaIntermittentLimit1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandMtpasaIntermittentLimit1 {
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
pub struct DemandMtpasaIntermittentLimit1Mapping([usize; 7]);
/// # Summary
///
/// ## MTPASA_INTERMITTENT_LIMIT
///
/// A submission of expected maximum availability for intermittent generators for use in MTPASA intermittent generationforecasts
///
/// * Data Set Name: Demand
/// * File Name: Mtpasa Intermittent Limit
/// * Data Version: 1
///
/// # Description
/// The RESDEMANDTRK and PERDEMAND tables have a parent/child relationship, and define forecast regional demands since market start. RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date. PERDEMAND defines the numerical forecast values for each trading interval of a the trading day for that region. A complete trading day forecast for one region consists of one RESDEMANDTRK record and 48 PERDEMAND records.SourcePERDEMAND updates whenever AEMO issues a new or revised forecast. ST PASA forecasts update seven days at a time. Predispatch updates one date.Volume1296000 rows per yearNoteIn the context of a mandatory restrictions event the forecast schedule (MW) of restrictions are reported through the RESDEMANDTRK and PERDEMAND tables using the new field PerDemand.MR_Schedule. The relationship between fields and mandatory restriction terms for the 50% probability of exceedence forecast are:·UnRestricted Profile  = ResDemand + MR_Schedule·Restricted Profile  = ResDemand
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DemandMtpasaIntermittentLimit1Row<'data> {
    /// Trading Day for which this unit availability submission applies
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: core::ops::Range<usize>,
    /// Date time file processed
    pub offerdatetime: chrono::NaiveDateTime,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Maximum imposed MW limit. Value between 0 and the registered DUID Maximum Capacity.Value = -1 means no limit applies.
    pub uppermwlimit: Option<i64>,
    /// User entering the unit availability submission
    pub authorisedbyuser: core::ops::Range<usize>,
    /// Participant entering the unit availability submission
    pub authorisedbyparticipantid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandMtpasaIntermittentLimit1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn authorisedbyuser(&self) -> Option<&str> {
        if self.authorisedbyuser.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedbyuser.clone(),
                ),
            )
        }
    }
    pub fn authorisedbyparticipantid(&self) -> Option<&str> {
        if self.authorisedbyparticipantid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedbyparticipantid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DemandMtpasaIntermittentLimit1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "MTPASA_INTERMITTENT_LIMIT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandMtpasaIntermittentLimit1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRADINGDATE",
        "DUID",
        "OFFERDATETIME",
        "LASTCHANGED",
        "UPPERMWLIMIT",
        "AUTHORISEDBYUSER",
        "AUTHORISEDBYPARTICIPANTID",
    ];
    type Row<'row> = DemandMtpasaIntermittentLimit1Row<'row>;
    type FieldMapping = DemandMtpasaIntermittentLimit1Mapping;
    type PrimaryKey = DemandMtpasaIntermittentLimit1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandMtpasaIntermittentLimit1Row {
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            uppermwlimit: row.get_opt_parsed_at_idx("uppermwlimit", field_mapping.0[4])?,
            authorisedbyuser: row.get_opt_range("authorisedbyuser", field_mapping.0[5])?,
            authorisedbyparticipantid: row
                .get_opt_range("authorisedbyparticipantid", field_mapping.0[6])?,
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
        Ok(DemandMtpasaIntermittentLimit1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandMtpasaIntermittentLimit1PrimaryKey {
        DemandMtpasaIntermittentLimit1PrimaryKey {
            duid: row.duid().to_string(),
            offerdatetime: row.offerdatetime,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "demand_mtpasa_intermittent_limit_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandMtpasaIntermittentLimit1Row {
            tradingdate: row.tradingdate.clone(),
            duid: row.duid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            lastchanged: row.lastchanged.clone(),
            uppermwlimit: row.uppermwlimit.clone(),
            authorisedbyuser: row.authorisedbyuser.clone(),
            authorisedbyparticipantid: row.authorisedbyparticipantid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandMtpasaIntermittentLimit1PrimaryKey {
    pub duid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DemandMtpasaIntermittentLimit1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandMtpasaIntermittentLimit1Row<'data> {
    type Row<'other> = DemandMtpasaIntermittentLimit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DemandMtpasaIntermittentLimit1Row<'data> {
    type PrimaryKey = DemandMtpasaIntermittentLimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandMtpasaIntermittentLimit1PrimaryKey {
    type Row<'other> = DemandMtpasaIntermittentLimit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandMtpasaIntermittentLimit1PrimaryKey {
    type PrimaryKey = DemandMtpasaIntermittentLimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandMtpasaIntermittentLimit1 {
    type Builder = DemandMtpasaIntermittentLimit1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
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
                arrow::datatypes::Field::new(
                    "uppermwlimit",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisedbyuser",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisedbyparticipantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandMtpasaIntermittentLimit1Builder {
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            uppermwlimit_array: arrow::array::builder::Int64Builder::new(),
            authorisedbyuser_array: arrow::array::builder::StringBuilder::new(),
            authorisedbyparticipantid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .tradingdate_array
            .append_value(row.tradingdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .offerdatetime_array
            .append_value(row.offerdatetime.and_utc().timestamp_millis());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.uppermwlimit_array.append_option(row.uppermwlimit);
        builder.authorisedbyuser_array.append_option(row.authorisedbyuser());
        builder
            .authorisedbyparticipantid_array
            .append_option(row.authorisedbyparticipantid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.uppermwlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedbyuser_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.authorisedbyparticipantid_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandMtpasaIntermittentLimit1Builder {
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    uppermwlimit_array: arrow::array::builder::Int64Builder,
    authorisedbyuser_array: arrow::array::builder::StringBuilder,
    authorisedbyparticipantid_array: arrow::array::builder::StringBuilder,
}
pub struct DemandPeriod1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DemandPeriod1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandPeriod1 {
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
pub struct DemandPeriod1Mapping([usize; 11]);
/// # Summary
///
/// ## PERDEMAND
///
/// PERDEMAND sets out the regional demands and MR schedule data for each half-hour period. PERDEMAND is a child table to RESDEMANDTRK.
///
/// * Data Set Name: Demand
/// * File Name: Period
/// * Data Version: 1
///
/// # Description
/// The RESDEMANDTRK and PERDEMAND tables have a parent/child relationship, and define forecast regional demands since market start. RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date. PERDEMAND defines the numerical forecast values for each trading interval of a the trading day for that region. A complete trading day forecast for one region consists of one RESDEMANDTRK record and 48 PERDEMAND records.SourcePERDEMAND updates whenever AEMO issues a new or revised forecast. ST PASA forecasts update seven days at a time. Predispatch updates one date.Volume1296000 rows per yearNoteIn the context of a mandatory restrictions event the forecast schedule (MW) of restrictions are reported through the RESDEMANDTRK and PERDEMAND tables using the new field PerDemand.MR_Schedule. The relationship between fields and mandatory restriction terms for the 50% probability of exceedence forecast are:·UnRestricted Profile  = ResDemand + MR_Schedule·Restricted Profile  = ResDemand
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * OFFERDATE
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct DemandPeriod1Row<'data> {
    /// Market date the forecast is made for. First date of the 7 days.
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Market date of forecast up to 7 days ahead.
    pub settlementdate: chrono::NaiveDateTime,
    /// Differentiates this region from all other regions
    pub regionid: core::ops::Range<usize>,
    /// Date record issued
    pub offerdate: chrono::NaiveDateTime,
    /// Half hourly trading intervals from 04:30.
    pub periodid: rust_decimal::Decimal,
    /// The version of the RESDEMAND file for this date
    pub versionno: rust_decimal::Decimal,
    /// Base Demand forecast for period
    pub resdemand: Option<rust_decimal::Decimal>,
    /// Demand at 90% probability of exceedance
    pub demand90probability: Option<rust_decimal::Decimal>,
    /// Demand level for a 10% probability of exceedance
    pub demand10probability: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// MR_Schedule = Unrestricted Demand - POE
    pub mr_schedule: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandPeriod1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for DemandPeriod1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "PERIOD";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandPeriod1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "SETTLEMENTDATE",
        "REGIONID",
        "OFFERDATE",
        "PERIODID",
        "VERSIONNO",
        "RESDEMAND",
        "DEMAND90PROBABILITY",
        "DEMAND10PROBABILITY",
        "LASTCHANGED",
        "MR_SCHEDULE",
    ];
    type Row<'row> = DemandPeriod1Row<'row>;
    type FieldMapping = DemandPeriod1Mapping;
    type PrimaryKey = DemandPeriod1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandPeriod1Row {
            effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resdemand: row
                .get_opt_custom_parsed_at_idx(
                    "resdemand",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand90probability: row
                .get_opt_custom_parsed_at_idx(
                    "demand90probability",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand10probability: row
                .get_opt_custom_parsed_at_idx(
                    "demand10probability",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            mr_schedule: row
                .get_opt_custom_parsed_at_idx(
                    "mr_schedule",
                    field_mapping.0[10],
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
        Ok(DemandPeriod1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandPeriod1PrimaryKey {
        DemandPeriod1PrimaryKey {
            offerdate: row.offerdate,
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("demand_period_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandPeriod1Row {
            effectivedate: row.effectivedate.clone(),
            settlementdate: row.settlementdate.clone(),
            regionid: row.regionid.clone(),
            offerdate: row.offerdate.clone(),
            periodid: row.periodid.clone(),
            versionno: row.versionno.clone(),
            resdemand: row.resdemand.clone(),
            demand90probability: row.demand90probability.clone(),
            demand10probability: row.demand10probability.clone(),
            lastchanged: row.lastchanged.clone(),
            mr_schedule: row.mr_schedule.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandPeriod1PrimaryKey {
    pub offerdate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for DemandPeriod1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandPeriod1Row<'data> {
    type Row<'other> = DemandPeriod1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.offerdate == row.offerdate && self.periodid == row.periodid
            && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DemandPeriod1Row<'data> {
    type PrimaryKey = DemandPeriod1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandPeriod1PrimaryKey {
    type Row<'other> = DemandPeriod1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.offerdate == row.offerdate && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandPeriod1PrimaryKey {
    type PrimaryKey = DemandPeriod1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandPeriod1 {
    type Builder = DemandPeriod1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
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
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "resdemand",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand90probability",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand10probability",
                    arrow::datatypes::DataType::Decimal128(10, 0),
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
                    "mr_schedule",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DemandPeriod1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            resdemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            demand90probability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            demand10probability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            mr_schedule_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_option(
                row.effectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder.offerdate_array.append_value(row.offerdate.and_utc().timestamp_millis());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .resdemand_array
            .append_option({
                row.resdemand
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .demand90probability_array
            .append_option({
                row.demand90probability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .demand10probability_array
            .append_option({
                row.demand10probability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .mr_schedule_array
            .append_option({
                row.mr_schedule
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
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resdemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand90probability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand10probability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mr_schedule_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandPeriod1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    resdemand_array: arrow::array::builder::Decimal128Builder,
    demand90probability_array: arrow::array::builder::Decimal128Builder,
    demand10probability_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    mr_schedule_array: arrow::array::builder::Decimal128Builder,
}
pub struct DemandTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&DemandTrk1Row<'_>) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DemandTrk1 {
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
pub struct DemandTrk1Mapping([usize; 8]);
/// # Summary
///
/// ## RESDEMANDTRK
///
/// RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date.RESDEMANDTRK and PERDEMAND have a parent/child relationship, and are for defined forecast regional demands since market start. RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date. PERDEMAND defines the numerical forecast values for each trading interval of a the trading day for that region. A complete trading day forecast for one region consists of one RESDEMANDTRK record and 48 PERDEMAND records.
///
/// * Data Set Name: Demand
/// * File Name: Trk
/// * Data Version: 1
///
/// # Description
/// RESDEMANDTRK data is public, so is available to all participants.SourceRESDEMANDTRK updates are ad hoc.Volume27000 rows per year.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * OFFERDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct DemandTrk1Row<'data> {
    /// Trading Date of the regional forecast
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: core::ops::Range<usize>,
    /// Date the forecast was created
    pub offerdate: chrono::NaiveDateTime,
    /// Version of this forecast with respect to the Effectivedate and Offerdate
    pub versionno: rust_decimal::Decimal,
    /// Tracking purposes only
    pub filename: core::ops::Range<usize>,
    /// Date forecast authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Identifier of authorising user
    pub authorisedby: core::ops::Range<usize>,
    /// Date and time the record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DemandTrk1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
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
impl mmsdm_core::GetTable for DemandTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DEMAND";
    const TABLE_NAME: &'static str = "TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DemandTrk1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "REGIONID",
        "OFFERDATE",
        "VERSIONNO",
        "FILENAME",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = DemandTrk1Row<'row>;
    type FieldMapping = DemandTrk1Mapping;
    type PrimaryKey = DemandTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DemandTrk1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            filename: row.get_opt_range("filename", field_mapping.0[4])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
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
        Ok(DemandTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DemandTrk1PrimaryKey {
        DemandTrk1PrimaryKey {
            effectivedate: row.effectivedate,
            offerdate: row.offerdate,
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("demand_trk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DemandTrk1Row {
            effectivedate: row.effectivedate.clone(),
            regionid: row.regionid.clone(),
            offerdate: row.offerdate.clone(),
            versionno: row.versionno.clone(),
            filename: row.filename.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DemandTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub offerdate: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for DemandTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DemandTrk1Row<'data> {
    type Row<'other> = DemandTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.offerdate == row.offerdate
            && self.regionid() == row.regionid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DemandTrk1Row<'data> {
    type PrimaryKey = DemandTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.offerdate == key.offerdate
            && self.regionid() == key.regionid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for DemandTrk1PrimaryKey {
    type Row<'other> = DemandTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.offerdate == row.offerdate
            && self.regionid == row.regionid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DemandTrk1PrimaryKey {
    type PrimaryKey = DemandTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.offerdate == key.offerdate
            && self.regionid == key.regionid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DemandTrk1 {
    type Builder = DemandTrk1Builder;
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
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
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
                    "filename",
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
                    "authorisedby",
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
        DemandTrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            filename_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder.offerdate_array.append_value(row.offerdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.filename_array.append_option(row.filename());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
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
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DemandTrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    filename_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct RooftopActual2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &RooftopActual2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl RooftopActual2 {
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
pub struct RooftopActual2Mapping([usize; 6]);
/// # Summary
///
/// ## ROOFTOP_PV_ACTUAL
///
/// Estimate of regional Rooftop Solar actual generation for each half-hour interval in a day
///
/// * Data Set Name: Rooftop
/// * File Name: Actual
/// * Data Version: 2
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * TYPE
#[derive(Debug, PartialEq, Eq)]
pub struct RooftopActual2Row<'data> {
    /// The forecast half-hour interval (time ending)
    pub interval_datetime: chrono::NaiveDateTime,
    /// One of DAILY, MEASUREMENT or SATELLITE. DAILY- best quality estimated actuals, available day after. MEASUREMENT- best quality estimated actuals on day, delayed by 1 half hour. SATELLITE- estimated actuals using satellite imagery, delayed by 1 half hour.
    pub r#type: core::ops::Range<usize>,
    /// Region identifier
    pub regionid: core::ops::Range<usize>,
    /// Estimated generation in MW at the interval end
    pub power: Option<rust_decimal::Decimal>,
    /// Quality indicator. Represents the quality of the estimate.
    pub qi: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> RooftopActual2Row<'data> {
    pub fn r#type(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.r#type.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for RooftopActual2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "ROOFTOP";
    const TABLE_NAME: &'static str = "ACTUAL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = RooftopActual2Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERVAL_DATETIME",
        "TYPE",
        "REGIONID",
        "POWER",
        "QI",
        "LASTCHANGED",
    ];
    type Row<'row> = RooftopActual2Row<'row>;
    type FieldMapping = RooftopActual2Mapping;
    type PrimaryKey = RooftopActual2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(RooftopActual2Row {
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            r#type: row.get_range("type", field_mapping.0[1])?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            power: row
                .get_opt_custom_parsed_at_idx(
                    "power",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            qi: row
                .get_opt_custom_parsed_at_idx(
                    "qi",
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
        Ok(RooftopActual2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> RooftopActual2PrimaryKey {
        RooftopActual2PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            r#type: row.r#type().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("rooftop_actual_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        RooftopActual2Row {
            interval_datetime: row.interval_datetime.clone(),
            r#type: row.r#type.clone(),
            regionid: row.regionid.clone(),
            power: row.power.clone(),
            qi: row.qi.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RooftopActual2PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub r#type: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for RooftopActual2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for RooftopActual2Row<'data> {
    type Row<'other> = RooftopActual2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.r#type() == row.r#type()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for RooftopActual2Row<'data> {
    type PrimaryKey = RooftopActual2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.r#type() == key.r#type
    }
}
impl<'data> mmsdm_core::CompareWithRow for RooftopActual2PrimaryKey {
    type Row<'other> = RooftopActual2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.r#type == row.r#type()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for RooftopActual2PrimaryKey {
    type PrimaryKey = RooftopActual2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.r#type == key.r#type
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for RooftopActual2 {
    type Builder = RooftopActual2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "r#type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "power",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "qi",
                    arrow::datatypes::DataType::Decimal128(2, 1),
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
        RooftopActual2Builder {
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            r#type_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            power_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            qi_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 1)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.r#type_array.append_value(row.r#type());
        builder.regionid_array.append_value(row.regionid());
        builder
            .power_array
            .append_option({
                row.power
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .qi_array
            .append_option({
                row.qi
                    .map(|mut val| {
                        val.rescale(1);
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
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.r#type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.power_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.qi_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct RooftopActual2Builder {
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    r#type_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    power_array: arrow::array::builder::Decimal128Builder,
    qi_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct RooftopForecast1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &RooftopForecast1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl RooftopForecast1 {
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
pub struct RooftopForecast1Mapping([usize; 8]);
/// # Summary
///
/// ## ROOFTOP_PV_FORECAST
///
/// Regional forecasts of Rooftop Solar generation across the half-hour intervals over 8 days
///
/// * Data Set Name: Rooftop
/// * File Name: Forecast
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
/// * INTERVAL_DATETIME
/// * REGIONID
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct RooftopForecast1Row<'data> {
    /// Date time this forecast was produced
    pub version_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: core::ops::Range<usize>,
    /// The forecast half-hour interval (time ending)
    pub interval_datetime: chrono::NaiveDateTime,
    /// The average forecast value in MW at the interval end
    pub powermean: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance forecast value in MW at the interval end
    pub powerpoe50: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance forecast value in MW at the interval end
    pub powerpoelow: Option<rust_decimal::Decimal>,
    /// 10% probability of exceedance forecast value in MW at the interval end
    pub powerpoehigh: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> RooftopForecast1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for RooftopForecast1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "ROOFTOP";
    const TABLE_NAME: &'static str = "FORECAST";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = RooftopForecast1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "VERSION_DATETIME",
        "REGIONID",
        "INTERVAL_DATETIME",
        "POWERMEAN",
        "POWERPOE50",
        "POWERPOELOW",
        "POWERPOEHIGH",
        "LASTCHANGED",
    ];
    type Row<'row> = RooftopForecast1Row<'row>;
    type FieldMapping = RooftopForecast1Mapping;
    type PrimaryKey = RooftopForecast1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(RooftopForecast1Row {
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            powermean: row
                .get_opt_custom_parsed_at_idx(
                    "powermean",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            powerpoe50: row
                .get_opt_custom_parsed_at_idx(
                    "powerpoe50",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            powerpoelow: row
                .get_opt_custom_parsed_at_idx(
                    "powerpoelow",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            powerpoehigh: row
                .get_opt_custom_parsed_at_idx(
                    "powerpoehigh",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
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
        Ok(RooftopForecast1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> RooftopForecast1PrimaryKey {
        RooftopForecast1PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("rooftop_forecast_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        RooftopForecast1Row {
            version_datetime: row.version_datetime.clone(),
            regionid: row.regionid.clone(),
            interval_datetime: row.interval_datetime.clone(),
            powermean: row.powermean.clone(),
            powerpoe50: row.powerpoe50.clone(),
            powerpoelow: row.powerpoelow.clone(),
            powerpoehigh: row.powerpoehigh.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RooftopForecast1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for RooftopForecast1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for RooftopForecast1Row<'data> {
    type Row<'other> = RooftopForecast1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid()
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for RooftopForecast1Row<'data> {
    type PrimaryKey = RooftopForecast1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for RooftopForecast1PrimaryKey {
    type Row<'other> = RooftopForecast1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid()
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for RooftopForecast1PrimaryKey {
    type PrimaryKey = RooftopForecast1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for RooftopForecast1 {
    type Builder = RooftopForecast1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "version_datetime",
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
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "powermean",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "powerpoe50",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "powerpoelow",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "powerpoehigh",
                    arrow::datatypes::DataType::Decimal128(12, 3),
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
        RooftopForecast1Builder {
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            powermean_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            powerpoe50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            powerpoelow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            powerpoehigh_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder
            .powermean_array
            .append_option({
                row.powermean
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .powerpoe50_array
            .append_option({
                row.powerpoe50
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .powerpoelow_array
            .append_option({
                row.powerpoelow
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .powerpoehigh_array
            .append_option({
                row.powerpoehigh
                    .map(|mut val| {
                        val.rescale(3);
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
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powermean_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powerpoe50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powerpoelow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.powerpoehigh_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct RooftopForecast1Builder {
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    powermean_array: arrow::array::builder::Decimal128Builder,
    powerpoe50_array: arrow::array::builder::Decimal128Builder,
    powerpoelow_array: arrow::array::builder::Decimal128Builder,
    powerpoehigh_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
