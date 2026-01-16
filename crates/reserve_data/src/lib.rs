#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct MtpasaReservelimit1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaReservelimit1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaReservelimit1 {
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
pub struct MtpasaReservelimit1Mapping([usize; 6]);
/// # Summary
///
/// ## MTPASA_RESERVELIMIT
///
/// MT PASA input table defining a MT PASA Reserve Requirement within a single set. An MT PASA Reserve Requirement can span more than one region.
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit
/// * Data Version: 1
///
/// # Description
/// SourceMTPASA_RESERVELIMIT is updated on an ad hoc basis when a new Reserve Requirement is published.Volume~20 rows per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * RESERVELIMITID
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaReservelimit1Row<'data> {
    /// Trade date when the set of reserve requirements become effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Timestamp when the set of reserve requirements become effective
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve Requirement identifier
    pub reservelimitid: core::ops::Range<usize>,
    /// Description of this Reserve Requirement
    pub description: core::ops::Range<usize>,
    /// Right hand side value for this Reserve requirement
    pub rhs: Option<rust_decimal::Decimal>,
    /// Timestamp the record was last modified.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaReservelimit1Row<'data> {
    pub fn reservelimitid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.reservelimitid.clone(),
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
}
impl mmsdm_core::GetTable for MtpasaReservelimit1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "RESERVELIMIT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaReservelimit1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "RESERVELIMITID",
        "DESCRIPTION",
        "RHS",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaReservelimit1Row<'row>;
    type FieldMapping = MtpasaReservelimit1Mapping;
    type PrimaryKey = MtpasaReservelimit1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaReservelimit1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reservelimitid: row.get_range("reservelimitid", field_mapping.0[2])?,
            description: row.get_opt_range("description", field_mapping.0[3])?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
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
        Ok(MtpasaReservelimit1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaReservelimit1PrimaryKey {
        MtpasaReservelimit1PrimaryKey {
            effectivedate: row.effectivedate,
            reservelimitid: row.reservelimitid().to_string(),
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_reservelimit_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaReservelimit1Row {
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            reservelimitid: row.reservelimitid.clone(),
            description: row.description.clone(),
            rhs: row.rhs.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaReservelimit1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub reservelimitid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaReservelimit1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaReservelimit1Row<'data> {
    type Row<'other> = MtpasaReservelimit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.reservelimitid() == row.reservelimitid()
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimit1Row<'data> {
    type PrimaryKey = MtpasaReservelimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.reservelimitid() == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaReservelimit1PrimaryKey {
    type Row<'other> = MtpasaReservelimit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.reservelimitid == row.reservelimitid()
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimit1PrimaryKey {
    type PrimaryKey = MtpasaReservelimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.reservelimitid == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaReservelimit1 {
    type Builder = MtpasaReservelimit1Builder;
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
                    "version_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "reservelimitid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int64),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rhs",
                    arrow::datatypes::DataType::Decimal128(16, 6),
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
        MtpasaReservelimit1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            reservelimitid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            description_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int64Type,
            >::new(),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder.reservelimitid_array.append_value(row.reservelimitid());
        builder.description_array.append_option(row.description());
        builder
            .rhs_array
            .append_option({
                row.rhs
                    .map(|mut val| {
                        val.rescale(6);
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
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reservelimitid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaReservelimit1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    reservelimitid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    description_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int64Type,
    >,
    rhs_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaReservelimitRegion1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaReservelimitRegion1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaReservelimitRegion1 {
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
pub struct MtpasaReservelimitRegion1Mapping([usize; 6]);
/// # Summary
///
/// ## MTPASA_RESERVELIMIT_REGION
///
/// MT PASA input table to define the regions that are part of a single MT PASA Reserve Requirement
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit Region
/// * Data Version: 1
///
/// # Description
/// SourceMTPASA_RESERVELIMIT_REGION is updated on an ad hoc basis when a new Reserve Requirement is published.Volume~50 rows per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * RESERVELIMITID
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaReservelimitRegion1Row<'data> {
    /// Trade date when the set of reserve requirements become effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Timestamp when the set of reserve requirements become effective
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve requirement identifier
    pub reservelimitid: core::ops::Range<usize>,
    /// Region ID - identifier of a NEM region included in this requirement
    pub regionid: core::ops::Range<usize>,
    /// Coefficient for the region in this reserve requirement
    pub coef: Option<rust_decimal::Decimal>,
    /// Timestamp the record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaReservelimitRegion1Row<'data> {
    pub fn reservelimitid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.reservelimitid.clone(),
        )
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for MtpasaReservelimitRegion1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "RESERVELIMIT_REGION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaReservelimitRegion1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "RESERVELIMITID",
        "REGIONID",
        "COEF",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaReservelimitRegion1Row<'row>;
    type FieldMapping = MtpasaReservelimitRegion1Mapping;
    type PrimaryKey = MtpasaReservelimitRegion1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaReservelimitRegion1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reservelimitid: row.get_range("reservelimitid", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            coef: row
                .get_opt_custom_parsed_at_idx(
                    "coef",
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
        Ok(MtpasaReservelimitRegion1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaReservelimitRegion1PrimaryKey {
        MtpasaReservelimitRegion1PrimaryKey {
            effectivedate: row.effectivedate,
            regionid: row.regionid().to_string(),
            reservelimitid: row.reservelimitid().to_string(),
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_reservelimit_region_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaReservelimitRegion1Row {
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            reservelimitid: row.reservelimitid.clone(),
            regionid: row.regionid.clone(),
            coef: row.coef.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaReservelimitRegion1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub reservelimitid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaReservelimitRegion1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaReservelimitRegion1Row<'data> {
    type Row<'other> = MtpasaReservelimitRegion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid() == row.regionid()
            && self.reservelimitid() == row.reservelimitid()
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitRegion1Row<'data> {
    type PrimaryKey = MtpasaReservelimitRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid() == key.regionid
            && self.reservelimitid() == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaReservelimitRegion1PrimaryKey {
    type Row<'other> = MtpasaReservelimitRegion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid()
            && self.reservelimitid == row.reservelimitid()
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitRegion1PrimaryKey {
    type PrimaryKey = MtpasaReservelimitRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.reservelimitid == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaReservelimitRegion1 {
    type Builder = MtpasaReservelimitRegion1Builder;
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
                    "version_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "reservelimitid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "coef",
                    arrow::datatypes::DataType::Decimal128(16, 6),
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
        MtpasaReservelimitRegion1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            reservelimitid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            regionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            coef_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder.reservelimitid_array.append_value(row.reservelimitid());
        builder.regionid_array.append_value(row.regionid());
        builder
            .coef_array
            .append_option({
                row.coef
                    .map(|mut val| {
                        val.rescale(6);
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
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reservelimitid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.coef_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaReservelimitRegion1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    reservelimitid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    regionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    coef_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaReservelimitSet1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaReservelimitSet1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaReservelimitSet1 {
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
pub struct MtpasaReservelimitSet1Mapping([usize; 7]);
/// # Summary
///
/// ## MTPASA_RESERVELIMIT_SET
///
/// MT PASA input table defining a set of MT PASA Reserve Requirements. Note only one set can be active on a given date.
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit Set
/// * Data Version: 1
///
/// # Description
/// SourceMTPASA_RESERVELIMIT_SET is updated on an ad hoc basis when a new Reserve Requirement is published.Volume~2 rows per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaReservelimitSet1Row<'data> {
    /// Trade date when the set of reserve requirements become effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Timestamp when the set of reserve requirements become effective
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA LRC Reserve Requirement Set Identifier
    pub reservelimit_set_id: core::ops::Range<usize>,
    /// Description of this set of Reserve Requirements
    pub description: core::ops::Range<usize>,
    /// Date the requirement set was authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising this requirement set
    pub authorisedby: core::ops::Range<usize>,
    /// Timestamp the record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaReservelimitSet1Row<'data> {
    pub fn reservelimit_set_id(&self) -> Option<&str> {
        if self.reservelimit_set_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reservelimit_set_id.clone(),
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
impl mmsdm_core::GetTable for MtpasaReservelimitSet1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "RESERVELIMIT_SET";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaReservelimitSet1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "RESERVELIMIT_SET_ID",
        "DESCRIPTION",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaReservelimitSet1Row<'row>;
    type FieldMapping = MtpasaReservelimitSet1Mapping;
    type PrimaryKey = MtpasaReservelimitSet1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaReservelimitSet1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reservelimit_set_id: row
                .get_opt_range("reservelimit_set_id", field_mapping.0[2])?,
            description: row.get_opt_range("description", field_mapping.0[3])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[5])?,
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
        Ok(MtpasaReservelimitSet1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaReservelimitSet1PrimaryKey {
        MtpasaReservelimitSet1PrimaryKey {
            effectivedate: row.effectivedate,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_reservelimit_set_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaReservelimitSet1Row {
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            reservelimit_set_id: row.reservelimit_set_id.clone(),
            description: row.description.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaReservelimitSet1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaReservelimitSet1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaReservelimitSet1Row<'data> {
    type Row<'other> = MtpasaReservelimitSet1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitSet1Row<'data> {
    type PrimaryKey = MtpasaReservelimitSet1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaReservelimitSet1PrimaryKey {
    type Row<'other> = MtpasaReservelimitSet1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitSet1PrimaryKey {
    type PrimaryKey = MtpasaReservelimitSet1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaReservelimitSet1 {
    type Builder = MtpasaReservelimitSet1Builder;
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
                    "version_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "reservelimit_set_id",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int64),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
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
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
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
        MtpasaReservelimitSet1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            reservelimit_set_id_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            description_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int64Type,
            >::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder.reservelimit_set_id_array.append_option(row.reservelimit_set_id());
        builder.description_array.append_option(row.description());
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
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reservelimit_set_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
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
pub struct MtpasaReservelimitSet1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    reservelimit_set_id_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    description_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int64Type,
    >,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
