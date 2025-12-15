#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct MtpasaCaseresult1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaCaseresult1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaCaseresult1 {
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
pub struct MtpasaCaseresult1Mapping([usize; 4]);
/// # Summary
///
/// ## MTPASA_CASERESULT
///
/// MTPASA solution header table
///
/// * Data Set Name: Mtpasa
/// * File Name: Caseresult
/// * Data Version: 1
///
/// # Description
/// MTPASA_CASERESULT is public data.Holds one Record for entire solution
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
/// * RUN_NO
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaCaseresult1Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Version of PLEXOS used
    pub plexos_version: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaCaseresult1Row<'data> {
    pub fn plexos_version(&self) -> Option<&str> {
        if self.plexos_version.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.plexos_version.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MtpasaCaseresult1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "CASERESULT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaCaseresult1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUN_NO",
        "PLEXOS_VERSION",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaCaseresult1Row<'row>;
    type FieldMapping = MtpasaCaseresult1Mapping;
    type PrimaryKey = MtpasaCaseresult1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaCaseresult1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            plexos_version: row.get_opt_range("plexos_version", field_mapping.0[2])?,
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
        Ok(MtpasaCaseresult1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaCaseresult1PrimaryKey {
        MtpasaCaseresult1PrimaryKey {
            run_datetime: row.run_datetime,
            run_no: row.run_no,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_caseresult_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaCaseresult1Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            plexos_version: row.plexos_version.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaCaseresult1PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
}
impl mmsdm_core::PrimaryKey for MtpasaCaseresult1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaCaseresult1Row<'data> {
    type Row<'other> = MtpasaCaseresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime && self.run_no == row.run_no
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaCaseresult1Row<'data> {
    type PrimaryKey = MtpasaCaseresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.run_no == key.run_no
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaCaseresult1PrimaryKey {
    type Row<'other> = MtpasaCaseresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime && self.run_no == row.run_no
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaCaseresult1PrimaryKey {
    type PrimaryKey = MtpasaCaseresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.run_no == key.run_no
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaCaseresult1 {
    type Builder = MtpasaCaseresult1Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "plexos_version",
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
        MtpasaCaseresult1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            plexos_version_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.plexos_version_array.append_option(row.plexos_version());
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
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.plexos_version_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaCaseresult1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    plexos_version_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaConstraintresult1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaConstraintresult1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaConstraintresult1 {
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
pub struct MtpasaConstraintresult1Mapping([usize; 15]);
/// # Summary
///
/// ## MTPASA_CONSTRAINTRESULT
///
/// Constraint results for Binding or Violating Constraints
///
/// * Data Set Name: Mtpasa
/// * File Name: Constraintresult
/// * Data Version: 1
///
/// # Description
/// MTPASA_CONSTRAINTRESULT is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DAY
/// * DEMAND_POE_TYPE
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaConstraintresult1Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: core::ops::Range<usize>,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: core::ops::Range<usize>,
    /// Day this result is for
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the constraint. Only binding or violating constraints are reported
    pub constraintid: core::ops::Range<usize>,
    /// The effective date of the constraint used
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// The version of the constraint used
    pub versionno: Option<rust_decimal::Decimal>,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// Proportion of a constraint binding, across iterations and reference years
    pub probabilityofbinding: Option<rust_decimal::Decimal>,
    /// Proportion of a constraint violating, across iterations and reference years
    pub probabilityofviolation: Option<rust_decimal::Decimal>,
    /// The 90th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation90: Option<rust_decimal::Decimal>,
    /// The 50th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation50: Option<rust_decimal::Decimal>,
    /// The 10th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation10: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaConstraintresult1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn demand_poe_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.demand_poe_type.clone(),
        )
    }
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for MtpasaConstraintresult1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "CONSTRAINTRESULT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaConstraintresult1Mapping([
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
        18,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUN_NO",
        "RUNTYPE",
        "DEMAND_POE_TYPE",
        "DAY",
        "CONSTRAINTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PERIODID",
        "PROBABILITYOFBINDING",
        "PROBABILITYOFVIOLATION",
        "CONSTRAINTVIOLATION90",
        "CONSTRAINTVIOLATION50",
        "CONSTRAINTVIOLATION10",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaConstraintresult1Row<'row>;
    type FieldMapping = MtpasaConstraintresult1Mapping;
    type PrimaryKey = MtpasaConstraintresult1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaConstraintresult1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            runtype: row.get_range("runtype", field_mapping.0[2])?,
            demand_poe_type: row.get_range("demand_poe_type", field_mapping.0[3])?,
            day: row
                .get_custom_parsed_at_idx(
                    "day",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[5])?,
            effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_opt_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            probabilityofbinding: row
                .get_opt_custom_parsed_at_idx(
                    "probabilityofbinding",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            probabilityofviolation: row
                .get_opt_custom_parsed_at_idx(
                    "probabilityofviolation",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraintviolation90: row
                .get_opt_custom_parsed_at_idx(
                    "constraintviolation90",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraintviolation50: row
                .get_opt_custom_parsed_at_idx(
                    "constraintviolation50",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraintviolation10: row
                .get_opt_custom_parsed_at_idx(
                    "constraintviolation10",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[14],
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
        Ok(MtpasaConstraintresult1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaConstraintresult1PrimaryKey {
        MtpasaConstraintresult1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            day: row.day,
            demand_poe_type: row.demand_poe_type().to_string(),
            run_datetime: row.run_datetime,
            run_no: row.run_no,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_constraintresult_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaConstraintresult1Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            runtype: row.runtype.clone(),
            demand_poe_type: row.demand_poe_type.clone(),
            day: row.day.clone(),
            constraintid: row.constraintid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            probabilityofbinding: row.probabilityofbinding.clone(),
            probabilityofviolation: row.probabilityofviolation.clone(),
            constraintviolation90: row.constraintviolation90.clone(),
            constraintviolation50: row.constraintviolation50.clone(),
            constraintviolation10: row.constraintviolation10.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaConstraintresult1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaConstraintresult1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaConstraintresult1Row<'data> {
    type Row<'other> = MtpasaConstraintresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid() && self.day == row.day
            && self.demand_poe_type() == row.demand_poe_type()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintresult1Row<'data> {
    type PrimaryKey = MtpasaConstraintresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid && self.day == key.day
            && self.demand_poe_type() == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaConstraintresult1PrimaryKey {
    type Row<'other> = MtpasaConstraintresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid() && self.day == row.day
            && self.demand_poe_type == row.demand_poe_type()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintresult1PrimaryKey {
    type PrimaryKey = MtpasaConstraintresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.day == key.day
            && self.demand_poe_type == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaConstraintresult1 {
    type Builder = MtpasaConstraintresult1Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand_poe_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "probabilityofbinding",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "probabilityofviolation",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constraintviolation90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constraintviolation50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constraintviolation10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        MtpasaConstraintresult1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            demand_poe_type_array: arrow::array::builder::StringBuilder::new(),
            day_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            probabilityofbinding_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            probabilityofviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            constraintviolation90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            constraintviolation50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            constraintviolation10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.runtype_array.append_value(row.runtype());
        builder.demand_poe_type_array.append_value(row.demand_poe_type());
        builder.day_array.append_value(row.day.and_utc().timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
        builder
            .effectivedate_array
            .append_option(
                row.effectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
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
            .periodid_array
            .append_option({
                row.periodid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .probabilityofbinding_array
            .append_option({
                row.probabilityofbinding
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .probabilityofviolation_array
            .append_option({
                row.probabilityofviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .constraintviolation90_array
            .append_option({
                row.constraintviolation90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .constraintviolation50_array
            .append_option({
                row.constraintviolation50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .constraintviolation10_array
            .append_option({
                row.constraintviolation10
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_poe_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.probabilityofbinding_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.probabilityofviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintviolation90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintviolation50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintviolation10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaConstraintresult1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    demand_poe_type_array: arrow::array::builder::StringBuilder,
    day_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    probabilityofbinding_array: arrow::array::builder::Decimal128Builder,
    probabilityofviolation_array: arrow::array::builder::Decimal128Builder,
    constraintviolation90_array: arrow::array::builder::Decimal128Builder,
    constraintviolation50_array: arrow::array::builder::Decimal128Builder,
    constraintviolation10_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaConstraintsummary1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaConstraintsummary1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaConstraintsummary1 {
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
pub struct MtpasaConstraintsummary1Mapping([usize; 11]);
/// # Summary
///
/// ## MTPASA_CONSTRAINTSUMMARY
///
/// Constraint Summary results over aggregation periods
///
/// * Data Set Name: Mtpasa
/// * File Name: Constraintsummary
/// * Data Version: 1
///
/// # Description
/// MTPASA_CONSTRAINTSUMMARY is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * CONSTRAINTID
/// * DAY
/// * DEMAND_POE_TYPE
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaConstraintsummary1Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: core::ops::Range<usize>,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: core::ops::Range<usize>,
    /// Day this result is for
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the constraint. Only binding or violating constraints are reported
    pub constraintid: core::ops::Range<usize>,
    /// The effective date of the constraint used
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// The version of the constraintID
    pub versionno: Option<rust_decimal::Decimal>,
    /// Period data is aggregated over. Values are PEAK, SHOULDER, OFFPEAK. PEAK = 14:00-19:59, SHOULDER = 07:00-13:59 and 20:00-21:59, OFFPEAK = 22:00-06:59
    pub aggregation_period: core::ops::Range<usize>,
    /// Constraint hours binding or violating for period, averaged across iterations and reference years
    pub constrainthoursbinding: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaConstraintsummary1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn demand_poe_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.demand_poe_type.clone(),
        )
    }
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
    pub fn aggregation_period(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.aggregation_period.clone(),
        )
    }
}
impl mmsdm_core::GetTable for MtpasaConstraintsummary1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "CONSTRAINTSUMMARY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaConstraintsummary1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUN_NO",
        "RUNTYPE",
        "DEMAND_POE_TYPE",
        "DAY",
        "CONSTRAINTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AGGREGATION_PERIOD",
        "CONSTRAINTHOURSBINDING",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaConstraintsummary1Row<'row>;
    type FieldMapping = MtpasaConstraintsummary1Mapping;
    type PrimaryKey = MtpasaConstraintsummary1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaConstraintsummary1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            runtype: row.get_range("runtype", field_mapping.0[2])?,
            demand_poe_type: row.get_range("demand_poe_type", field_mapping.0[3])?,
            day: row
                .get_custom_parsed_at_idx(
                    "day",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[5])?,
            effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregation_period: row.get_range("aggregation_period", field_mapping.0[8])?,
            constrainthoursbinding: row
                .get_opt_custom_parsed_at_idx(
                    "constrainthoursbinding",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
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
        Ok(MtpasaConstraintsummary1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaConstraintsummary1PrimaryKey {
        MtpasaConstraintsummary1PrimaryKey {
            aggregation_period: row.aggregation_period().to_string(),
            constraintid: row.constraintid().to_string(),
            day: row.day,
            demand_poe_type: row.demand_poe_type().to_string(),
            run_datetime: row.run_datetime,
            run_no: row.run_no,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_constraintsummary_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaConstraintsummary1Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            runtype: row.runtype.clone(),
            demand_poe_type: row.demand_poe_type.clone(),
            day: row.day.clone(),
            constraintid: row.constraintid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            aggregation_period: row.aggregation_period.clone(),
            constrainthoursbinding: row.constrainthoursbinding.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaConstraintsummary1PrimaryKey {
    pub aggregation_period: alloc::string::String,
    pub constraintid: alloc::string::String,
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaConstraintsummary1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaConstraintsummary1Row<'data> {
    type Row<'other> = MtpasaConstraintsummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.aggregation_period() == row.aggregation_period()
            && self.constraintid() == row.constraintid() && self.day == row.day
            && self.demand_poe_type() == row.demand_poe_type()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintsummary1Row<'data> {
    type PrimaryKey = MtpasaConstraintsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period() == key.aggregation_period
            && self.constraintid() == key.constraintid && self.day == key.day
            && self.demand_poe_type() == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaConstraintsummary1PrimaryKey {
    type Row<'other> = MtpasaConstraintsummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.aggregation_period == row.aggregation_period()
            && self.constraintid == row.constraintid() && self.day == row.day
            && self.demand_poe_type == row.demand_poe_type()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintsummary1PrimaryKey {
    type PrimaryKey = MtpasaConstraintsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.constraintid == key.constraintid && self.day == key.day
            && self.demand_poe_type == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaConstraintsummary1 {
    type Builder = MtpasaConstraintsummary1Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand_poe_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregation_period",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constrainthoursbinding",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        MtpasaConstraintsummary1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            demand_poe_type_array: arrow::array::builder::StringBuilder::new(),
            day_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            aggregation_period_array: arrow::array::builder::StringBuilder::new(),
            constrainthoursbinding_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.runtype_array.append_value(row.runtype());
        builder.demand_poe_type_array.append_value(row.demand_poe_type());
        builder.day_array.append_value(row.day.and_utc().timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
        builder
            .effectivedate_array
            .append_option(
                row.effectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .versionno_array
            .append_option({
                row.versionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.aggregation_period_array.append_value(row.aggregation_period());
        builder
            .constrainthoursbinding_array
            .append_option({
                row.constrainthoursbinding
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_poe_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregation_period_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constrainthoursbinding_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaConstraintsummary1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    demand_poe_type_array: arrow::array::builder::StringBuilder,
    day_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    aggregation_period_array: arrow::array::builder::StringBuilder,
    constrainthoursbinding_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaDuidavailability3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaDuidavailability3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaDuidavailability3 {
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
pub struct MtpasaDuidavailability3Mapping([usize; 10]);
/// # Summary
///
/// ## MTPASA_DUIDAVAILABILITY
///
/// Offered PASA Availability of the scheduled generator DUID for each day over the Medium Term PASA period. The data in this table is input data to the MT PASA process it is not part of the MTPASA solution. The availability does not reflect any energy limitations in the MT PASA offers
///
/// * Data Set Name: Mtpasa
/// * File Name: Duidavailability
/// * Data Version: 3
///
/// # Description
/// MTPASA_INTERCONNECTORRESULT is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DUID
/// * PUBLISH_DATETIME
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaDuidavailability3Row<'data> {
    /// Date Time the report was published.
    pub publish_datetime: chrono::NaiveDateTime,
    /// Date on which the PASA availability of DUID applies.
    pub day: chrono::NaiveDateTime,
    /// NEM Region.
    pub regionid: core::ops::Range<usize>,
    /// NEM DUID.
    pub duid: core::ops::Range<usize>,
    /// Offered PASA Availability of Scheduled generator DUID for the day.
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Date Time of the latest offer used for DUID for this date.
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Status of a reported capacity value (e.g. 1 for Yes, 0 for No)
    pub carryoverstatus: Option<rust_decimal::Decimal>,
    /// The unit state value
    pub pasaunitstate: core::ops::Range<usize>,
    /// The recall time value
    pub pasarecalltime: Option<i64>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaDuidavailability3Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn pasaunitstate(&self) -> Option<&str> {
        if self.pasaunitstate.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.pasaunitstate.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MtpasaDuidavailability3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "DUIDAVAILABILITY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaDuidavailability3Mapping([
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
        "PUBLISH_DATETIME",
        "DAY",
        "REGIONID",
        "DUID",
        "PASAAVAILABILITY",
        "LATEST_OFFER_DATETIME",
        "LASTCHANGED",
        "CARRYOVERSTATUS",
        "PASAUNITSTATE",
        "PASARECALLTIME",
    ];
    type Row<'row> = MtpasaDuidavailability3Row<'row>;
    type FieldMapping = MtpasaDuidavailability3Mapping;
    type PrimaryKey = MtpasaDuidavailability3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaDuidavailability3Row {
            publish_datetime: row
                .get_custom_parsed_at_idx(
                    "publish_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            day: row
                .get_custom_parsed_at_idx(
                    "day",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            duid: row.get_range("duid", field_mapping.0[3])?,
            pasaavailability: row
                .get_opt_custom_parsed_at_idx(
                    "pasaavailability",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            latest_offer_datetime: row
                .get_opt_custom_parsed_at_idx(
                    "latest_offer_datetime",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            carryoverstatus: row
                .get_opt_custom_parsed_at_idx(
                    "carryoverstatus",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pasaunitstate: row.get_opt_range("pasaunitstate", field_mapping.0[8])?,
            pasarecalltime: row
                .get_opt_parsed_at_idx("pasarecalltime", field_mapping.0[9])?,
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
        Ok(MtpasaDuidavailability3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaDuidavailability3PrimaryKey {
        MtpasaDuidavailability3PrimaryKey {
            day: row.day,
            duid: row.duid().to_string(),
            publish_datetime: row.publish_datetime,
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_duidavailability_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaDuidavailability3Row {
            publish_datetime: row.publish_datetime.clone(),
            day: row.day.clone(),
            regionid: row.regionid.clone(),
            duid: row.duid.clone(),
            pasaavailability: row.pasaavailability.clone(),
            latest_offer_datetime: row.latest_offer_datetime.clone(),
            lastchanged: row.lastchanged.clone(),
            carryoverstatus: row.carryoverstatus.clone(),
            pasaunitstate: row.pasaunitstate.clone(),
            pasarecalltime: row.pasarecalltime.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaDuidavailability3PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub duid: alloc::string::String,
    pub publish_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaDuidavailability3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaDuidavailability3Row<'data> {
    type Row<'other> = MtpasaDuidavailability3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.duid() == row.duid()
            && self.publish_datetime == row.publish_datetime
            && self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaDuidavailability3Row<'data> {
    type PrimaryKey = MtpasaDuidavailability3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.duid() == key.duid
            && self.publish_datetime == key.publish_datetime
            && self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaDuidavailability3PrimaryKey {
    type Row<'other> = MtpasaDuidavailability3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.duid == row.duid()
            && self.publish_datetime == row.publish_datetime
            && self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaDuidavailability3PrimaryKey {
    type PrimaryKey = MtpasaDuidavailability3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.duid == key.duid
            && self.publish_datetime == key.publish_datetime
            && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaDuidavailability3 {
    type Builder = MtpasaDuidavailability3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "publish_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day",
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
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "pasaavailability",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "latest_offer_datetime",
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
                    "carryoverstatus",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pasaunitstate",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pasarecalltime",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MtpasaDuidavailability3Builder {
            publish_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            day_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            pasaavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            latest_offer_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            carryoverstatus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            pasaunitstate_array: arrow::array::builder::StringBuilder::new(),
            pasarecalltime_array: arrow::array::builder::Int64Builder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .publish_datetime_array
            .append_value(row.publish_datetime.and_utc().timestamp_millis());
        builder.day_array.append_value(row.day.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder.duid_array.append_value(row.duid());
        builder
            .pasaavailability_array
            .append_option({
                row.pasaavailability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .latest_offer_datetime_array
            .append_option(
                row.latest_offer_datetime.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .carryoverstatus_array
            .append_option({
                row.carryoverstatus
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.pasaunitstate_array.append_option(row.pasaunitstate());
        builder.pasarecalltime_array.append_option(row.pasarecalltime);
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.publish_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pasaavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.latest_offer_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.carryoverstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pasaunitstate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pasarecalltime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaDuidavailability3Builder {
    publish_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    day_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    pasaavailability_array: arrow::array::builder::Decimal128Builder,
    latest_offer_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    carryoverstatus_array: arrow::array::builder::Decimal128Builder,
    pasaunitstate_array: arrow::array::builder::StringBuilder,
    pasarecalltime_array: arrow::array::builder::Int64Builder,
}
pub struct MtpasaInterconnectorresult1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaInterconnectorresult1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaInterconnectorresult1 {
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
pub struct MtpasaInterconnectorresult1Mapping([usize; 15]);
/// # Summary
///
/// ## MTPASA_INTERCONNECTORRESULT
///
/// Interconnector results for interval of max demand per day
///
/// * Data Set Name: Mtpasa
/// * File Name: Interconnectorresult
/// * Data Version: 1
///
/// # Description
/// MTPASA_INTERCONNECTORRESULT is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DEMAND_POE_TYPE
/// * INTERCONNECTORID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaInterconnectorresult1Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: core::ops::Range<usize>,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: core::ops::Range<usize>,
    /// Day this result is for
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// The 90th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow10: Option<rust_decimal::Decimal>,
    /// Proportion of iterations and reference years with interconnector constrained when exporting
    pub probabilityofbindingexport: Option<rust_decimal::Decimal>,
    /// Proportion of iterations and reference years with interconnector constrained when importing
    pub probabilityofbindingimport: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit, averaged across iterations and reference years
    pub calculatedexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit, averaged across iterations and reference years
    pub calculatedimportlimit: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaInterconnectorresult1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn demand_poe_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.demand_poe_type.clone(),
        )
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for MtpasaInterconnectorresult1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "INTERCONNECTORRESULT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaInterconnectorresult1Mapping([
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
        18,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUN_NO",
        "RUNTYPE",
        "DEMAND_POE_TYPE",
        "DAY",
        "INTERCONNECTORID",
        "PERIODID",
        "FLOW90",
        "FLOW50",
        "FLOW10",
        "PROBABILITYOFBINDINGEXPORT",
        "PROBABILITYOFBINDINGIMPORT",
        "CALCULATEDEXPORTLIMIT",
        "CALCULATEDIMPORTLIMIT",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaInterconnectorresult1Row<'row>;
    type FieldMapping = MtpasaInterconnectorresult1Mapping;
    type PrimaryKey = MtpasaInterconnectorresult1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaInterconnectorresult1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            runtype: row.get_range("runtype", field_mapping.0[2])?,
            demand_poe_type: row.get_range("demand_poe_type", field_mapping.0[3])?,
            day: row
                .get_custom_parsed_at_idx(
                    "day",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[5])?,
            periodid: row
                .get_opt_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            flow90: row
                .get_opt_custom_parsed_at_idx(
                    "flow90",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            flow50: row
                .get_opt_custom_parsed_at_idx(
                    "flow50",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            flow10: row
                .get_opt_custom_parsed_at_idx(
                    "flow10",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            probabilityofbindingexport: row
                .get_opt_custom_parsed_at_idx(
                    "probabilityofbindingexport",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            probabilityofbindingimport: row
                .get_opt_custom_parsed_at_idx(
                    "probabilityofbindingimport",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            calculatedexportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "calculatedexportlimit",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            calculatedimportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "calculatedimportlimit",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[14],
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
        Ok(MtpasaInterconnectorresult1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaInterconnectorresult1PrimaryKey {
        MtpasaInterconnectorresult1PrimaryKey {
            day: row.day,
            demand_poe_type: row.demand_poe_type().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            run_datetime: row.run_datetime,
            run_no: row.run_no,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_interconnectorresult_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaInterconnectorresult1Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            runtype: row.runtype.clone(),
            demand_poe_type: row.demand_poe_type.clone(),
            day: row.day.clone(),
            interconnectorid: row.interconnectorid.clone(),
            periodid: row.periodid.clone(),
            flow90: row.flow90.clone(),
            flow50: row.flow50.clone(),
            flow10: row.flow10.clone(),
            probabilityofbindingexport: row.probabilityofbindingexport.clone(),
            probabilityofbindingimport: row.probabilityofbindingimport.clone(),
            calculatedexportlimit: row.calculatedexportlimit.clone(),
            calculatedimportlimit: row.calculatedimportlimit.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaInterconnectorresult1PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaInterconnectorresult1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaInterconnectorresult1Row<'data> {
    type Row<'other> = MtpasaInterconnectorresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.demand_poe_type() == row.demand_poe_type()
            && self.interconnectorid() == row.interconnectorid()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaInterconnectorresult1Row<'data> {
    type PrimaryKey = MtpasaInterconnectorresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type() == key.demand_poe_type
            && self.interconnectorid() == key.interconnectorid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaInterconnectorresult1PrimaryKey {
    type Row<'other> = MtpasaInterconnectorresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.demand_poe_type == row.demand_poe_type()
            && self.interconnectorid == row.interconnectorid()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaInterconnectorresult1PrimaryKey {
    type PrimaryKey = MtpasaInterconnectorresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type == key.demand_poe_type
            && self.interconnectorid == key.interconnectorid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaInterconnectorresult1 {
    type Builder = MtpasaInterconnectorresult1Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand_poe_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day",
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
                    true,
                ),
                arrow::datatypes::Field::new(
                    "flow90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "flow50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "flow10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "probabilityofbindingexport",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "probabilityofbindingimport",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calculatedexportlimit",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calculatedimportlimit",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        MtpasaInterconnectorresult1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            demand_poe_type_array: arrow::array::builder::StringBuilder::new(),
            day_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            flow90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            flow50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            flow10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            probabilityofbindingexport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            probabilityofbindingimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            calculatedexportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            calculatedimportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.runtype_array.append_value(row.runtype());
        builder.demand_poe_type_array.append_value(row.demand_poe_type());
        builder.day_array.append_value(row.day.and_utc().timestamp_millis());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .periodid_array
            .append_option({
                row.periodid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .flow90_array
            .append_option({
                row.flow90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .flow50_array
            .append_option({
                row.flow50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .flow10_array
            .append_option({
                row.flow10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .probabilityofbindingexport_array
            .append_option({
                row.probabilityofbindingexport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .probabilityofbindingimport_array
            .append_option({
                row.probabilityofbindingimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .calculatedexportlimit_array
            .append_option({
                row.calculatedexportlimit
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .calculatedimportlimit_array
            .append_option({
                row.calculatedimportlimit
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_poe_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.flow90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.flow50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.flow10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.probabilityofbindingexport_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.probabilityofbindingimport_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calculatedexportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calculatedimportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaInterconnectorresult1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    demand_poe_type_array: arrow::array::builder::StringBuilder,
    day_array: arrow::array::builder::TimestampMillisecondBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    flow90_array: arrow::array::builder::Decimal128Builder,
    flow50_array: arrow::array::builder::Decimal128Builder,
    flow10_array: arrow::array::builder::Decimal128Builder,
    probabilityofbindingexport_array: arrow::array::builder::Decimal128Builder,
    probabilityofbindingimport_array: arrow::array::builder::Decimal128Builder,
    calculatedexportlimit_array: arrow::array::builder::Decimal128Builder,
    calculatedimportlimit_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaLolpresult1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaLolpresult1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaLolpresult1 {
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
pub struct MtpasaLolpresult1Mapping([usize; 12]);
/// # Summary
///
/// ## MTPASA_LOLPRESULT
///
/// Results for Loss of Load Probability (LOLP) run per day
///
/// * Data Set Name: Mtpasa
/// * File Name: Lolpresult
/// * Data Version: 1
///
/// # Description
/// MTPASA_LOLPRESULT is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DAY
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaLolpresult1Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always LOLP
    pub runtype: core::ops::Range<usize>,
    /// Day this result is for
    pub day: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: core::ops::Range<usize>,
    /// The half hourly interval period with the highest LOLP, or highest region demand if LOLP = 0 for all intervals (1..48)
    pub worst_interval_periodid: Option<rust_decimal::Decimal>,
    /// The Abstract Operational Demand for the worst interval in this region (MW)
    pub worst_interval_demand: Option<rust_decimal::Decimal>,
    /// The half hourly aggregate intermittent generation for the worst interval in this region (MW)
    pub worst_interval_intgen: Option<rust_decimal::Decimal>,
    /// The half hourly aggregate demand side participation for the worst interval period in this region (MW)
    pub worst_interval_dsp: Option<rust_decimal::Decimal>,
    /// Loss of Load Probability for the worst interval in this region
    pub lossofloadprobability: Option<rust_decimal::Decimal>,
    /// Loss of Load Magnitude for the worst interval in this region. Values are LOW, MEDIUM, HIGH
    pub lossofloadmagnitude: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaLolpresult1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn lossofloadmagnitude(&self) -> Option<&str> {
        if self.lossofloadmagnitude.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.lossofloadmagnitude.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MtpasaLolpresult1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "LOLPRESULT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaLolpresult1Mapping([
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
        "RUN_DATETIME",
        "RUN_NO",
        "RUNTYPE",
        "DAY",
        "REGIONID",
        "WORST_INTERVAL_PERIODID",
        "WORST_INTERVAL_DEMAND",
        "WORST_INTERVAL_INTGEN",
        "WORST_INTERVAL_DSP",
        "LOSSOFLOADPROBABILITY",
        "LOSSOFLOADMAGNITUDE",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaLolpresult1Row<'row>;
    type FieldMapping = MtpasaLolpresult1Mapping;
    type PrimaryKey = MtpasaLolpresult1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaLolpresult1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            runtype: row.get_range("runtype", field_mapping.0[2])?,
            day: row
                .get_custom_parsed_at_idx(
                    "day",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[4])?,
            worst_interval_periodid: row
                .get_opt_custom_parsed_at_idx(
                    "worst_interval_periodid",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            worst_interval_demand: row
                .get_opt_custom_parsed_at_idx(
                    "worst_interval_demand",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            worst_interval_intgen: row
                .get_opt_custom_parsed_at_idx(
                    "worst_interval_intgen",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            worst_interval_dsp: row
                .get_opt_custom_parsed_at_idx(
                    "worst_interval_dsp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lossofloadprobability: row
                .get_opt_custom_parsed_at_idx(
                    "lossofloadprobability",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lossofloadmagnitude: row
                .get_opt_range("lossofloadmagnitude", field_mapping.0[10])?,
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
        Ok(MtpasaLolpresult1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaLolpresult1PrimaryKey {
        MtpasaLolpresult1PrimaryKey {
            day: row.day,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            run_no: row.run_no,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_lolpresult_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaLolpresult1Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            runtype: row.runtype.clone(),
            day: row.day.clone(),
            regionid: row.regionid.clone(),
            worst_interval_periodid: row.worst_interval_periodid.clone(),
            worst_interval_demand: row.worst_interval_demand.clone(),
            worst_interval_intgen: row.worst_interval_intgen.clone(),
            worst_interval_dsp: row.worst_interval_dsp.clone(),
            lossofloadprobability: row.lossofloadprobability.clone(),
            lossofloadmagnitude: row.lossofloadmagnitude.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaLolpresult1PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaLolpresult1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaLolpresult1Row<'data> {
    type Row<'other> = MtpasaLolpresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.regionid() == row.regionid()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaLolpresult1Row<'data> {
    type PrimaryKey = MtpasaLolpresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.regionid() == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaLolpresult1PrimaryKey {
    type Row<'other> = MtpasaLolpresult1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.regionid == row.regionid()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaLolpresult1PrimaryKey {
    type PrimaryKey = MtpasaLolpresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaLolpresult1 {
    type Builder = MtpasaLolpresult1Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day",
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
                    "worst_interval_periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "worst_interval_demand",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "worst_interval_intgen",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "worst_interval_dsp",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lossofloadprobability",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lossofloadmagnitude",
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
        MtpasaLolpresult1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            day_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            worst_interval_periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            worst_interval_demand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            worst_interval_intgen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            worst_interval_dsp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lossofloadprobability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            lossofloadmagnitude_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.runtype_array.append_value(row.runtype());
        builder.day_array.append_value(row.day.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .worst_interval_periodid_array
            .append_option({
                row.worst_interval_periodid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .worst_interval_demand_array
            .append_option({
                row.worst_interval_demand
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .worst_interval_intgen_array
            .append_option({
                row.worst_interval_intgen
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .worst_interval_dsp_array
            .append_option({
                row.worst_interval_dsp
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lossofloadprobability_array
            .append_option({
                row.lossofloadprobability
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder.lossofloadmagnitude_array.append_option(row.lossofloadmagnitude());
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
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.worst_interval_periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.worst_interval_demand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.worst_interval_intgen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.worst_interval_dsp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lossofloadprobability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lossofloadmagnitude_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaLolpresult1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    day_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    worst_interval_periodid_array: arrow::array::builder::Decimal128Builder,
    worst_interval_demand_array: arrow::array::builder::Decimal128Builder,
    worst_interval_intgen_array: arrow::array::builder::Decimal128Builder,
    worst_interval_dsp_array: arrow::array::builder::Decimal128Builder,
    lossofloadprobability_array: arrow::array::builder::Decimal128Builder,
    lossofloadmagnitude_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaRegionavailtrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaRegionavailtrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaRegionavailtrk1 {
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
pub struct MtpasaRegionavailtrk1Mapping([usize; 4]);
/// # Summary
///
/// ## MTPASA_REGIONAVAIL_TRK
///
/// The tracking table to assist in versioning of the region-aggregate offered PASA Availability data published to the MTPASA_REGIONAVAILABILITY table.
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionavailtrk
/// * Data Version: 1
///
/// # Description
/// MTPASA_REGIONAVAILABILITY is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PUBLISH_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaRegionavailtrk1Row<'data> {
    /// Date Time the report was published.
    pub publish_datetime: chrono::NaiveDateTime,
    /// First date of the report inclusive.
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last date of the report inclusive.
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Date Time of the latest offer used in the report.
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> MtpasaRegionavailtrk1Row<'data> {}
impl mmsdm_core::GetTable for MtpasaRegionavailtrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "REGIONAVAILTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaRegionavailtrk1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PUBLISH_DATETIME",
        "STARTDATE",
        "ENDDATE",
        "LATEST_OFFER_DATETIME",
    ];
    type Row<'row> = MtpasaRegionavailtrk1Row<'row>;
    type FieldMapping = MtpasaRegionavailtrk1Mapping;
    type PrimaryKey = MtpasaRegionavailtrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaRegionavailtrk1Row {
            publish_datetime: row
                .get_custom_parsed_at_idx(
                    "publish_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            latest_offer_datetime: row
                .get_opt_custom_parsed_at_idx(
                    "latest_offer_datetime",
                    field_mapping.0[3],
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
        Ok(MtpasaRegionavailtrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaRegionavailtrk1PrimaryKey {
        MtpasaRegionavailtrk1PrimaryKey {
            publish_datetime: row.publish_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_regionavailtrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaRegionavailtrk1Row {
            publish_datetime: row.publish_datetime.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            latest_offer_datetime: row.latest_offer_datetime.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaRegionavailtrk1PrimaryKey {
    pub publish_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionavailtrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionavailtrk1Row<'data> {
    type Row<'other> = MtpasaRegionavailtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.publish_datetime == row.publish_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailtrk1Row<'data> {
    type PrimaryKey = MtpasaRegionavailtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.publish_datetime == key.publish_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionavailtrk1PrimaryKey {
    type Row<'other> = MtpasaRegionavailtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.publish_datetime == row.publish_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailtrk1PrimaryKey {
    type PrimaryKey = MtpasaRegionavailtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.publish_datetime == key.publish_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionavailtrk1 {
    type Builder = MtpasaRegionavailtrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "publish_datetime",
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
                    "latest_offer_datetime",
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
        MtpasaRegionavailtrk1Builder {
            publish_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            latest_offer_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .publish_datetime_array
            .append_value(row.publish_datetime.and_utc().timestamp_millis());
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .latest_offer_datetime_array
            .append_option(
                row.latest_offer_datetime.map(|val| val.and_utc().timestamp_millis()),
            );
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.publish_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.latest_offer_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaRegionavailtrk1Builder {
    publish_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    latest_offer_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaRegionavailability4 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaRegionavailability4Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaRegionavailability4 {
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
pub struct MtpasaRegionavailability4Mapping([usize; 18]);
/// # Summary
///
/// ## MTPASA_REGIONAVAILABILITY
///
/// Stores the Region-aggregate offered PASA Availability of scheduled generators for each day over the Medium Term PASA period. The data in this table is an aggregate of input data to the MT PASA process it is not part of the MTPASA solution. The aggregate availability does not reflect any energy limitations in the MT PASA offers.
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionavailability
/// * Data Version: 4
///
/// # Description
/// MTPASA_REGIONAVAILABILITY is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DAY
/// * PUBLISH_DATETIME
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaRegionavailability4Row<'data> {
    /// Date Time the report was published.
    pub publish_datetime: chrono::NaiveDateTime,
    /// Date on which the aggregation applies.
    pub day: chrono::NaiveDateTime,
    /// NEM Region.
    pub regionid: core::ops::Range<usize>,
    /// Aggregate of the offered PASA Availability for all Scheduled generators in this region.
    pub pasaavailability_scheduled: Option<rust_decimal::Decimal>,
    /// Date Time of the latest offer used in the aggregation for this region and date.
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
    /// Region energy unconstrained MW capacity
    pub energyunconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Region energy constrained MW capacity
    pub energyconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW)
    pub nonscheduledgeneration: Option<rust_decimal::Decimal>,
    /// 10% probability demand (ex non-scheduled demand)
    pub demand10: Option<rust_decimal::Decimal>,
    /// 50% probability demand (ex non-scheduled demand)
    pub demand50: Option<rust_decimal::Decimal>,
    /// Total weekly operational as generated consumption (POE 10)
    pub energyreqdemand10: Option<rust_decimal::Decimal>,
    /// Total weekly operational as generated consumption (POE 50)
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Minimum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P10 traces (MW).
    pub demand10min: Option<rust_decimal::Decimal>,
    /// Maximum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P10 traces (MW).
    pub demand10max: Option<rust_decimal::Decimal>,
    /// Minimum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P50 traces (MW).
    pub demand50min: Option<rust_decimal::Decimal>,
    /// Maximum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P50 traces (MW).
    pub demand50max: Option<rust_decimal::Decimal>,
    /// Split of the CARRYOVER component of aggregate capacity vs the currently reported capacity.
    pub carryovercapacity: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaRegionavailability4Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for MtpasaRegionavailability4 {
    const VERSION: i32 = 4;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "REGIONAVAILABILITY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaRegionavailability4Mapping([
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
        18,
        19,
        20,
        21,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PUBLISH_DATETIME",
        "DAY",
        "REGIONID",
        "PASAAVAILABILITY_SCHEDULED",
        "LATEST_OFFER_DATETIME",
        "ENERGYUNCONSTRAINEDCAPACITY",
        "ENERGYCONSTRAINEDCAPACITY",
        "NONSCHEDULEDGENERATION",
        "DEMAND10",
        "DEMAND50",
        "ENERGYREQDEMAND10",
        "ENERGYREQDEMAND50",
        "LASTCHANGED",
        "DEMAND10MIN",
        "DEMAND10MAX",
        "DEMAND50MIN",
        "DEMAND50MAX",
        "CARRYOVERCAPACITY",
    ];
    type Row<'row> = MtpasaRegionavailability4Row<'row>;
    type FieldMapping = MtpasaRegionavailability4Mapping;
    type PrimaryKey = MtpasaRegionavailability4PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaRegionavailability4Row {
            publish_datetime: row
                .get_custom_parsed_at_idx(
                    "publish_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            day: row
                .get_custom_parsed_at_idx(
                    "day",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            pasaavailability_scheduled: row
                .get_opt_custom_parsed_at_idx(
                    "pasaavailability_scheduled",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            latest_offer_datetime: row
                .get_opt_custom_parsed_at_idx(
                    "latest_offer_datetime",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            energyunconstrainedcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "energyunconstrainedcapacity",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyconstrainedcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "energyconstrainedcapacity",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            nonscheduledgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "nonscheduledgeneration",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand10: row
                .get_opt_custom_parsed_at_idx(
                    "demand10",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50: row
                .get_opt_custom_parsed_at_idx(
                    "demand50",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyreqdemand10: row
                .get_opt_custom_parsed_at_idx(
                    "energyreqdemand10",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyreqdemand50: row
                .get_opt_custom_parsed_at_idx(
                    "energyreqdemand50",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            demand10min: row
                .get_opt_custom_parsed_at_idx(
                    "demand10min",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand10max: row
                .get_opt_custom_parsed_at_idx(
                    "demand10max",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50min: row
                .get_opt_custom_parsed_at_idx(
                    "demand50min",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50max: row
                .get_opt_custom_parsed_at_idx(
                    "demand50max",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            carryovercapacity: row
                .get_opt_custom_parsed_at_idx(
                    "carryovercapacity",
                    field_mapping.0[17],
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
        Ok(MtpasaRegionavailability4Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaRegionavailability4PrimaryKey {
        MtpasaRegionavailability4PrimaryKey {
            day: row.day,
            publish_datetime: row.publish_datetime,
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_regionavailability_v4_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaRegionavailability4Row {
            publish_datetime: row.publish_datetime.clone(),
            day: row.day.clone(),
            regionid: row.regionid.clone(),
            pasaavailability_scheduled: row.pasaavailability_scheduled.clone(),
            latest_offer_datetime: row.latest_offer_datetime.clone(),
            energyunconstrainedcapacity: row.energyunconstrainedcapacity.clone(),
            energyconstrainedcapacity: row.energyconstrainedcapacity.clone(),
            nonscheduledgeneration: row.nonscheduledgeneration.clone(),
            demand10: row.demand10.clone(),
            demand50: row.demand50.clone(),
            energyreqdemand10: row.energyreqdemand10.clone(),
            energyreqdemand50: row.energyreqdemand50.clone(),
            lastchanged: row.lastchanged.clone(),
            demand10min: row.demand10min.clone(),
            demand10max: row.demand10max.clone(),
            demand50min: row.demand50min.clone(),
            demand50max: row.demand50max.clone(),
            carryovercapacity: row.carryovercapacity.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaRegionavailability4PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub publish_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionavailability4PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionavailability4Row<'data> {
    type Row<'other> = MtpasaRegionavailability4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.publish_datetime == row.publish_datetime
            && self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailability4Row<'data> {
    type PrimaryKey = MtpasaRegionavailability4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.publish_datetime == key.publish_datetime
            && self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionavailability4PrimaryKey {
    type Row<'other> = MtpasaRegionavailability4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.publish_datetime == row.publish_datetime
            && self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailability4PrimaryKey {
    type PrimaryKey = MtpasaRegionavailability4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.publish_datetime == key.publish_datetime
            && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionavailability4 {
    type Builder = MtpasaRegionavailability4Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "publish_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day",
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
                    "pasaavailability_scheduled",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "latest_offer_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyunconstrainedcapacity",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyconstrainedcapacity",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nonscheduledgeneration",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyreqdemand10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyreqdemand50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
                    "demand10min",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand10max",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50min",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50max",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "carryovercapacity",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MtpasaRegionavailability4Builder {
            publish_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            day_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            pasaavailability_scheduled_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            latest_offer_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            energyunconstrainedcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            energyconstrainedcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            nonscheduledgeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            energyreqdemand10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            energyreqdemand50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            demand10min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand10max_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand50min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand50max_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            carryovercapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .publish_datetime_array
            .append_value(row.publish_datetime.and_utc().timestamp_millis());
        builder.day_array.append_value(row.day.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .pasaavailability_scheduled_array
            .append_option({
                row.pasaavailability_scheduled
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .latest_offer_datetime_array
            .append_option(
                row.latest_offer_datetime.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .energyunconstrainedcapacity_array
            .append_option({
                row.energyunconstrainedcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .energyconstrainedcapacity_array
            .append_option({
                row.energyconstrainedcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .nonscheduledgeneration_array
            .append_option({
                row.nonscheduledgeneration
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand10_array
            .append_option({
                row.demand10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand50_array
            .append_option({
                row.demand50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .energyreqdemand10_array
            .append_option({
                row.energyreqdemand10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .energyreqdemand50_array
            .append_option({
                row.energyreqdemand50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .demand10min_array
            .append_option({
                row.demand10min
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand10max_array
            .append_option({
                row.demand10max
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand50min_array
            .append_option({
                row.demand50min
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand50max_array
            .append_option({
                row.demand50max
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .carryovercapacity_array
            .append_option({
                row.carryovercapacity
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
                    alloc::sync::Arc::new(builder.publish_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.pasaavailability_scheduled_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.latest_offer_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.energyunconstrainedcapacity_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.energyconstrainedcapacity_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nonscheduledgeneration_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energyreqdemand10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energyreqdemand50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand10min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand10max_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50max_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.carryovercapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaRegionavailability4Builder {
    publish_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    day_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    pasaavailability_scheduled_array: arrow::array::builder::Decimal128Builder,
    latest_offer_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    energyunconstrainedcapacity_array: arrow::array::builder::Decimal128Builder,
    energyconstrainedcapacity_array: arrow::array::builder::Decimal128Builder,
    nonscheduledgeneration_array: arrow::array::builder::Decimal128Builder,
    demand10_array: arrow::array::builder::Decimal128Builder,
    demand50_array: arrow::array::builder::Decimal128Builder,
    energyreqdemand10_array: arrow::array::builder::Decimal128Builder,
    energyreqdemand50_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    demand10min_array: arrow::array::builder::Decimal128Builder,
    demand10max_array: arrow::array::builder::Decimal128Builder,
    demand50min_array: arrow::array::builder::Decimal128Builder,
    demand50max_array: arrow::array::builder::Decimal128Builder,
    carryovercapacity_array: arrow::array::builder::Decimal128Builder,
}
pub struct MtpasaRegioniteration1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaRegioniteration1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaRegioniteration1 {
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
pub struct MtpasaRegioniteration1Mapping([usize; 11]);
/// # Summary
///
/// ## MTPASA_REGIONITERATION
///
/// Region results for Unserved Energy (USE)
///
/// * Data Set Name: Mtpasa
/// * File Name: Regioniteration
/// * Data Version: 1
///
/// # Description
/// MTPASA_REGIONITERATION is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * DEMAND_POE_TYPE
/// * PERIOD_ENDING
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
/// * USE_ITERATION_ID
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaRegioniteration1Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: core::ops::Range<usize>,
    /// Demand POE type used. Value is POE10 or POE50
    pub demand_poe_type: core::ops::Range<usize>,
    /// Period data is aggregated over. Values are YEAR
    pub aggregation_period: core::ops::Range<usize>,
    /// Datetime of day at end of period (i.e. last day of year reported)
    pub period_ending: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: core::ops::Range<usize>,
    /// Iteration ID, only produced for iterations showing unserved energy>0
    pub use_iteration_id: i64,
    /// Number of half hours showing unserved energy over year, for iteration
    pub use_iteration_event_number: Option<rust_decimal::Decimal>,
    /// Average unserved energy event size for iteration over year (MW)
    pub use_iteration_event_average: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaRegioniteration1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn demand_poe_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.demand_poe_type.clone(),
        )
    }
    pub fn aggregation_period(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.aggregation_period.clone(),
        )
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for MtpasaRegioniteration1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "REGIONITERATION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaRegioniteration1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUN_NO",
        "RUNTYPE",
        "DEMAND_POE_TYPE",
        "AGGREGATION_PERIOD",
        "PERIOD_ENDING",
        "REGIONID",
        "USE_ITERATION_ID",
        "USE_ITERATION_EVENT_NUMBER",
        "USE_ITERATION_EVENT_AVERAGE",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaRegioniteration1Row<'row>;
    type FieldMapping = MtpasaRegioniteration1Mapping;
    type PrimaryKey = MtpasaRegioniteration1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaRegioniteration1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            runtype: row.get_range("runtype", field_mapping.0[2])?,
            demand_poe_type: row.get_range("demand_poe_type", field_mapping.0[3])?,
            aggregation_period: row.get_range("aggregation_period", field_mapping.0[4])?,
            period_ending: row
                .get_custom_parsed_at_idx(
                    "period_ending",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[6])?,
            use_iteration_id: row
                .get_parsed_at_idx("use_iteration_id", field_mapping.0[7])?,
            use_iteration_event_number: row
                .get_opt_custom_parsed_at_idx(
                    "use_iteration_event_number",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_iteration_event_average: row
                .get_opt_custom_parsed_at_idx(
                    "use_iteration_event_average",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
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
        Ok(MtpasaRegioniteration1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaRegioniteration1PrimaryKey {
        MtpasaRegioniteration1PrimaryKey {
            aggregation_period: row.aggregation_period().to_string(),
            demand_poe_type: row.demand_poe_type().to_string(),
            period_ending: row.period_ending,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            run_no: row.run_no,
            runtype: row.runtype().to_string(),
            use_iteration_id: row.use_iteration_id,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_regioniteration_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaRegioniteration1Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            runtype: row.runtype.clone(),
            demand_poe_type: row.demand_poe_type.clone(),
            aggregation_period: row.aggregation_period.clone(),
            period_ending: row.period_ending.clone(),
            regionid: row.regionid.clone(),
            use_iteration_id: row.use_iteration_id.clone(),
            use_iteration_event_number: row.use_iteration_event_number.clone(),
            use_iteration_event_average: row.use_iteration_event_average.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaRegioniteration1PrimaryKey {
    pub aggregation_period: alloc::string::String,
    pub demand_poe_type: alloc::string::String,
    pub period_ending: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: alloc::string::String,
    pub use_iteration_id: i64,
}
impl mmsdm_core::PrimaryKey for MtpasaRegioniteration1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegioniteration1Row<'data> {
    type Row<'other> = MtpasaRegioniteration1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.aggregation_period() == row.aggregation_period()
            && self.demand_poe_type() == row.demand_poe_type()
            && self.period_ending == row.period_ending
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.run_no == row.run_no && self.runtype() == row.runtype()
            && self.use_iteration_id == row.use_iteration_id
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaRegioniteration1Row<'data> {
    type PrimaryKey = MtpasaRegioniteration1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period() == key.aggregation_period
            && self.demand_poe_type() == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid() == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype() == key.runtype
            && self.use_iteration_id == key.use_iteration_id
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegioniteration1PrimaryKey {
    type Row<'other> = MtpasaRegioniteration1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.aggregation_period == row.aggregation_period()
            && self.demand_poe_type == row.demand_poe_type()
            && self.period_ending == row.period_ending && self.regionid == row.regionid()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype()
            && self.use_iteration_id == row.use_iteration_id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegioniteration1PrimaryKey {
    type PrimaryKey = MtpasaRegioniteration1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.demand_poe_type == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
            && self.use_iteration_id == key.use_iteration_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegioniteration1 {
    type Builder = MtpasaRegioniteration1Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand_poe_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "aggregation_period",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "period_ending",
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
                    "use_iteration_id",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "use_iteration_event_number",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_iteration_event_average",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        MtpasaRegioniteration1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            demand_poe_type_array: arrow::array::builder::StringBuilder::new(),
            aggregation_period_array: arrow::array::builder::StringBuilder::new(),
            period_ending_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            use_iteration_id_array: arrow::array::builder::Int64Builder::new(),
            use_iteration_event_number_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_iteration_event_average_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.runtype_array.append_value(row.runtype());
        builder.demand_poe_type_array.append_value(row.demand_poe_type());
        builder.aggregation_period_array.append_value(row.aggregation_period());
        builder
            .period_ending_array
            .append_value(row.period_ending.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder.use_iteration_id_array.append_value(row.use_iteration_id);
        builder
            .use_iteration_event_number_array
            .append_option({
                row.use_iteration_event_number
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_iteration_event_average_array
            .append_option({
                row.use_iteration_event_average
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_poe_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregation_period_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.period_ending_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_iteration_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.use_iteration_event_number_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.use_iteration_event_average_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaRegioniteration1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    demand_poe_type_array: arrow::array::builder::StringBuilder,
    aggregation_period_array: arrow::array::builder::StringBuilder,
    period_ending_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    use_iteration_id_array: arrow::array::builder::Int64Builder,
    use_iteration_event_number_array: arrow::array::builder::Decimal128Builder,
    use_iteration_event_average_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MtpasaRegionresult2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaRegionresult2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaRegionresult2 {
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
pub struct MtpasaRegionresult2Mapping([usize; 36]);
/// # Summary
///
/// ## MTPASA_REGIONRESULT
///
/// Region results for interval of max demand per day.
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionresult
/// * Data Version: 2
///
/// # Description
/// MTPASA_REGIONRESULT is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DEMAND_POE_TYPE
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaRegionresult2Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: core::ops::Range<usize>,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: core::ops::Range<usize>,
    /// Day this result is for
    pub day: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: core::ops::Range<usize>,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// Demand value from selected half hourly interval (MW)
    pub demand: Option<rust_decimal::Decimal>,
    /// The total installed capacity of all generation (MW)
    pub aggregateinstalledcapacity: Option<rust_decimal::Decimal>,
    /// Total number of iterations and reference years performed
    pub numberofiterations: Option<rust_decimal::Decimal>,
    /// Number of iterations and reference years with unserved energy>0
    pub use_numberofiterations: Option<rust_decimal::Decimal>,
    /// Maximum unserved energy, across iterations and reference years (MW)
    pub use_max: Option<rust_decimal::Decimal>,
    /// Upper quartile unserved energy, across iterations and reference years (MW)
    pub use_upperquartile: Option<rust_decimal::Decimal>,
    /// Median unserved energy, across iterations and reference years (MW)
    pub use_median: Option<rust_decimal::Decimal>,
    /// Lower quartile unserved energy, across iterations and reference years (MW)
    pub use_lowerquartile: Option<rust_decimal::Decimal>,
    /// Minimum unserved energy, across iterations and reference years (MW)
    pub use_min: Option<rust_decimal::Decimal>,
    /// Average unserved energy, across iterations and reference years (MW)
    pub use_average: Option<rust_decimal::Decimal>,
    /// Average unserved energy event size, across iterations and reference years (MW)
    pub use_event_average: Option<rust_decimal::Decimal>,
    /// The 90th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen10: Option<rust_decimal::Decimal>,
    /// The 90th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen10: Option<rust_decimal::Decimal>,
    /// The 90th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation10: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The 90% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen90: Option<rust_decimal::Decimal>,
    /// The 50% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen50: Option<rust_decimal::Decimal>,
    /// The 10% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen10: Option<rust_decimal::Decimal>,
    /// Minimum available capacity, across iterations and reference years (MW).
    pub totalavailablegenmin: Option<rust_decimal::Decimal>,
    /// The 10% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen10: Option<rust_decimal::Decimal>,
    /// The 50% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen50: Option<rust_decimal::Decimal>,
    /// The 90% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen90: Option<rust_decimal::Decimal>,
    /// Maximum available capacity, across iterations and reference years (MW).
    pub totalavailablegenmax: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaRegionresult2Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn demand_poe_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.demand_poe_type.clone(),
        )
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for MtpasaRegionresult2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "REGIONRESULT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaRegionresult2Mapping([
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
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        39,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUN_NO",
        "RUNTYPE",
        "DEMAND_POE_TYPE",
        "DAY",
        "REGIONID",
        "PERIODID",
        "DEMAND",
        "AGGREGATEINSTALLEDCAPACITY",
        "NUMBEROFITERATIONS",
        "USE_NUMBEROFITERATIONS",
        "USE_MAX",
        "USE_UPPERQUARTILE",
        "USE_MEDIAN",
        "USE_LOWERQUARTILE",
        "USE_MIN",
        "USE_AVERAGE",
        "USE_EVENT_AVERAGE",
        "TOTALSCHEDULEDGEN90",
        "TOTALSCHEDULEDGEN50",
        "TOTALSCHEDULEDGEN10",
        "TOTALINTERMITTENTGEN90",
        "TOTALINTERMITTENTGEN50",
        "TOTALINTERMITTENTGEN10",
        "DEMANDSIDEPARTICIPATION90",
        "DEMANDSIDEPARTICIPATION50",
        "DEMANDSIDEPARTICIPATION10",
        "LASTCHANGED",
        "TOTALSEMISCHEDULEGEN90",
        "TOTALSEMISCHEDULEGEN50",
        "TOTALSEMISCHEDULEGEN10",
        "TOTALAVAILABLEGENMIN",
        "TOTALAVAILABLEGEN10",
        "TOTALAVAILABLEGEN50",
        "TOTALAVAILABLEGEN90",
        "TOTALAVAILABLEGENMAX",
    ];
    type Row<'row> = MtpasaRegionresult2Row<'row>;
    type FieldMapping = MtpasaRegionresult2Mapping;
    type PrimaryKey = MtpasaRegionresult2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaRegionresult2Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            runtype: row.get_range("runtype", field_mapping.0[2])?,
            demand_poe_type: row.get_range("demand_poe_type", field_mapping.0[3])?,
            day: row
                .get_custom_parsed_at_idx(
                    "day",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[5])?,
            periodid: row
                .get_opt_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand: row
                .get_opt_custom_parsed_at_idx(
                    "demand",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregateinstalledcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "aggregateinstalledcapacity",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            numberofiterations: row
                .get_opt_custom_parsed_at_idx(
                    "numberofiterations",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_numberofiterations: row
                .get_opt_custom_parsed_at_idx(
                    "use_numberofiterations",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_max: row
                .get_opt_custom_parsed_at_idx(
                    "use_max",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_upperquartile: row
                .get_opt_custom_parsed_at_idx(
                    "use_upperquartile",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_median: row
                .get_opt_custom_parsed_at_idx(
                    "use_median",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_lowerquartile: row
                .get_opt_custom_parsed_at_idx(
                    "use_lowerquartile",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_min: row
                .get_opt_custom_parsed_at_idx(
                    "use_min",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_average: row
                .get_opt_custom_parsed_at_idx(
                    "use_average",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_event_average: row
                .get_opt_custom_parsed_at_idx(
                    "use_event_average",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalscheduledgen90: row
                .get_opt_custom_parsed_at_idx(
                    "totalscheduledgen90",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalscheduledgen50: row
                .get_opt_custom_parsed_at_idx(
                    "totalscheduledgen50",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalscheduledgen10: row
                .get_opt_custom_parsed_at_idx(
                    "totalscheduledgen10",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalintermittentgen90: row
                .get_opt_custom_parsed_at_idx(
                    "totalintermittentgen90",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalintermittentgen50: row
                .get_opt_custom_parsed_at_idx(
                    "totalintermittentgen50",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalintermittentgen10: row
                .get_opt_custom_parsed_at_idx(
                    "totalintermittentgen10",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demandsideparticipation90: row
                .get_opt_custom_parsed_at_idx(
                    "demandsideparticipation90",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demandsideparticipation50: row
                .get_opt_custom_parsed_at_idx(
                    "demandsideparticipation50",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demandsideparticipation10: row
                .get_opt_custom_parsed_at_idx(
                    "demandsideparticipation10",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[27],
                    mmsdm_core::mms_datetime::parse,
                )?,
            totalsemischedulegen90: row
                .get_opt_custom_parsed_at_idx(
                    "totalsemischedulegen90",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalsemischedulegen50: row
                .get_opt_custom_parsed_at_idx(
                    "totalsemischedulegen50",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalsemischedulegen10: row
                .get_opt_custom_parsed_at_idx(
                    "totalsemischedulegen10",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalavailablegenmin: row
                .get_opt_custom_parsed_at_idx(
                    "totalavailablegenmin",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalavailablegen10: row
                .get_opt_custom_parsed_at_idx(
                    "totalavailablegen10",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalavailablegen50: row
                .get_opt_custom_parsed_at_idx(
                    "totalavailablegen50",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalavailablegen90: row
                .get_opt_custom_parsed_at_idx(
                    "totalavailablegen90",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalavailablegenmax: row
                .get_opt_custom_parsed_at_idx(
                    "totalavailablegenmax",
                    field_mapping.0[35],
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
        Ok(MtpasaRegionresult2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaRegionresult2PrimaryKey {
        MtpasaRegionresult2PrimaryKey {
            day: row.day,
            demand_poe_type: row.demand_poe_type().to_string(),
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            run_no: row.run_no,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_regionresult_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaRegionresult2Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            runtype: row.runtype.clone(),
            demand_poe_type: row.demand_poe_type.clone(),
            day: row.day.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            demand: row.demand.clone(),
            aggregateinstalledcapacity: row.aggregateinstalledcapacity.clone(),
            numberofiterations: row.numberofiterations.clone(),
            use_numberofiterations: row.use_numberofiterations.clone(),
            use_max: row.use_max.clone(),
            use_upperquartile: row.use_upperquartile.clone(),
            use_median: row.use_median.clone(),
            use_lowerquartile: row.use_lowerquartile.clone(),
            use_min: row.use_min.clone(),
            use_average: row.use_average.clone(),
            use_event_average: row.use_event_average.clone(),
            totalscheduledgen90: row.totalscheduledgen90.clone(),
            totalscheduledgen50: row.totalscheduledgen50.clone(),
            totalscheduledgen10: row.totalscheduledgen10.clone(),
            totalintermittentgen90: row.totalintermittentgen90.clone(),
            totalintermittentgen50: row.totalintermittentgen50.clone(),
            totalintermittentgen10: row.totalintermittentgen10.clone(),
            demandsideparticipation90: row.demandsideparticipation90.clone(),
            demandsideparticipation50: row.demandsideparticipation50.clone(),
            demandsideparticipation10: row.demandsideparticipation10.clone(),
            lastchanged: row.lastchanged.clone(),
            totalsemischedulegen90: row.totalsemischedulegen90.clone(),
            totalsemischedulegen50: row.totalsemischedulegen50.clone(),
            totalsemischedulegen10: row.totalsemischedulegen10.clone(),
            totalavailablegenmin: row.totalavailablegenmin.clone(),
            totalavailablegen10: row.totalavailablegen10.clone(),
            totalavailablegen50: row.totalavailablegen50.clone(),
            totalavailablegen90: row.totalavailablegen90.clone(),
            totalavailablegenmax: row.totalavailablegenmax.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaRegionresult2PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: alloc::string::String,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionresult2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionresult2Row<'data> {
    type Row<'other> = MtpasaRegionresult2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.demand_poe_type() == row.demand_poe_type()
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.run_no == row.run_no && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaRegionresult2Row<'data> {
    type PrimaryKey = MtpasaRegionresult2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type() == key.demand_poe_type
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
            && self.run_no == key.run_no && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionresult2PrimaryKey {
    type Row<'other> = MtpasaRegionresult2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day == row.day && self.demand_poe_type == row.demand_poe_type()
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
            && self.run_no == row.run_no && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionresult2PrimaryKey {
    type PrimaryKey = MtpasaRegionresult2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type == key.demand_poe_type
            && self.regionid == key.regionid && self.run_datetime == key.run_datetime
            && self.run_no == key.run_no && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionresult2 {
    type Builder = MtpasaRegionresult2Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand_poe_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day",
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
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregateinstalledcapacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "numberofiterations",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_numberofiterations",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_max",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_upperquartile",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_median",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_lowerquartile",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_min",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_average",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_event_average",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalscheduledgen90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalscheduledgen50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalscheduledgen10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalintermittentgen90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalintermittentgen50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalintermittentgen10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demandsideparticipation90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demandsideparticipation50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demandsideparticipation10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
                    "totalsemischedulegen90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalsemischedulegen50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalsemischedulegen10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalavailablegenmin",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalavailablegen10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalavailablegen50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalavailablegen90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalavailablegenmax",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MtpasaRegionresult2Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            demand_poe_type_array: arrow::array::builder::StringBuilder::new(),
            day_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            demand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            aggregateinstalledcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            numberofiterations_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_numberofiterations_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_max_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_upperquartile_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_median_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_lowerquartile_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_average_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_event_average_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalscheduledgen90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalscheduledgen50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalscheduledgen10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalintermittentgen90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalintermittentgen50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalintermittentgen10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demandsideparticipation90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demandsideparticipation50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demandsideparticipation10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            totalsemischedulegen90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalsemischedulegen50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalsemischedulegen10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalavailablegenmin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalavailablegen10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalavailablegen50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalavailablegen90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalavailablegenmax_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.runtype_array.append_value(row.runtype());
        builder.demand_poe_type_array.append_value(row.demand_poe_type());
        builder.day_array.append_value(row.day.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .periodid_array
            .append_option({
                row.periodid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .demand_array
            .append_option({
                row.demand
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .aggregateinstalledcapacity_array
            .append_option({
                row.aggregateinstalledcapacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .numberofiterations_array
            .append_option({
                row.numberofiterations
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_numberofiterations_array
            .append_option({
                row.use_numberofiterations
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_max_array
            .append_option({
                row.use_max
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_upperquartile_array
            .append_option({
                row.use_upperquartile
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_median_array
            .append_option({
                row.use_median
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_lowerquartile_array
            .append_option({
                row.use_lowerquartile
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_min_array
            .append_option({
                row.use_min
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_average_array
            .append_option({
                row.use_average
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_event_average_array
            .append_option({
                row.use_event_average
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalscheduledgen90_array
            .append_option({
                row.totalscheduledgen90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalscheduledgen50_array
            .append_option({
                row.totalscheduledgen50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalscheduledgen10_array
            .append_option({
                row.totalscheduledgen10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalintermittentgen90_array
            .append_option({
                row.totalintermittentgen90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalintermittentgen50_array
            .append_option({
                row.totalintermittentgen50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalintermittentgen10_array
            .append_option({
                row.totalintermittentgen10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demandsideparticipation90_array
            .append_option({
                row.demandsideparticipation90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demandsideparticipation50_array
            .append_option({
                row.demandsideparticipation50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demandsideparticipation10_array
            .append_option({
                row.demandsideparticipation10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .totalsemischedulegen90_array
            .append_option({
                row.totalsemischedulegen90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalsemischedulegen50_array
            .append_option({
                row.totalsemischedulegen50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalsemischedulegen10_array
            .append_option({
                row.totalsemischedulegen10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalavailablegenmin_array
            .append_option({
                row.totalavailablegenmin
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalavailablegen10_array
            .append_option({
                row.totalavailablegen10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalavailablegen50_array
            .append_option({
                row.totalavailablegen50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalavailablegen90_array
            .append_option({
                row.totalavailablegen90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalavailablegenmax_array
            .append_option({
                row.totalavailablegenmax
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_poe_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.aggregateinstalledcapacity_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.numberofiterations_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_numberofiterations_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_max_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_upperquartile_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_median_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_lowerquartile_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_average_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_event_average_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalscheduledgen90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalscheduledgen50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalscheduledgen10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalintermittentgen90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalintermittentgen50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalintermittentgen10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.demandsideparticipation90_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.demandsideparticipation50_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.demandsideparticipation10_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalsemischedulegen90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalsemischedulegen50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalsemischedulegen10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalavailablegenmin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalavailablegen10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalavailablegen50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalavailablegen90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalavailablegenmax_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaRegionresult2Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    demand_poe_type_array: arrow::array::builder::StringBuilder,
    day_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    demand_array: arrow::array::builder::Decimal128Builder,
    aggregateinstalledcapacity_array: arrow::array::builder::Decimal128Builder,
    numberofiterations_array: arrow::array::builder::Decimal128Builder,
    use_numberofiterations_array: arrow::array::builder::Decimal128Builder,
    use_max_array: arrow::array::builder::Decimal128Builder,
    use_upperquartile_array: arrow::array::builder::Decimal128Builder,
    use_median_array: arrow::array::builder::Decimal128Builder,
    use_lowerquartile_array: arrow::array::builder::Decimal128Builder,
    use_min_array: arrow::array::builder::Decimal128Builder,
    use_average_array: arrow::array::builder::Decimal128Builder,
    use_event_average_array: arrow::array::builder::Decimal128Builder,
    totalscheduledgen90_array: arrow::array::builder::Decimal128Builder,
    totalscheduledgen50_array: arrow::array::builder::Decimal128Builder,
    totalscheduledgen10_array: arrow::array::builder::Decimal128Builder,
    totalintermittentgen90_array: arrow::array::builder::Decimal128Builder,
    totalintermittentgen50_array: arrow::array::builder::Decimal128Builder,
    totalintermittentgen10_array: arrow::array::builder::Decimal128Builder,
    demandsideparticipation90_array: arrow::array::builder::Decimal128Builder,
    demandsideparticipation50_array: arrow::array::builder::Decimal128Builder,
    demandsideparticipation10_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    totalsemischedulegen90_array: arrow::array::builder::Decimal128Builder,
    totalsemischedulegen50_array: arrow::array::builder::Decimal128Builder,
    totalsemischedulegen10_array: arrow::array::builder::Decimal128Builder,
    totalavailablegenmin_array: arrow::array::builder::Decimal128Builder,
    totalavailablegen10_array: arrow::array::builder::Decimal128Builder,
    totalavailablegen50_array: arrow::array::builder::Decimal128Builder,
    totalavailablegen90_array: arrow::array::builder::Decimal128Builder,
    totalavailablegenmax_array: arrow::array::builder::Decimal128Builder,
}
pub struct MtpasaRegionsummary1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MtpasaRegionsummary1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MtpasaRegionsummary1 {
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
pub struct MtpasaRegionsummary1Mapping([usize; 30]);
/// # Summary
///
/// ## MTPASA_REGIONSUMMARY
///
/// Region Results summary over aggregation periods.
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionsummary
/// * Data Version: 1
///
/// # Description
/// MTPASA_REGIONSUMMARY is public data.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * DEMAND_POE_TYPE
/// * PERIOD_ENDING
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct MtpasaRegionsummary1Row<'data> {
    /// Date processing of the run begins.
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: core::ops::Range<usize>,
    /// Demand POE type used. Value are POE10, POE50
    pub demand_poe_type: core::ops::Range<usize>,
    /// Period data is aggregated over. Values are YEAR, MONTH
    pub aggregation_period: core::ops::Range<usize>,
    /// Datetime of day at end of period (i.e. last day of month or year reported)
    pub period_ending: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: core::ops::Range<usize>,
    /// Native demand calculated from Operational As Generated trace supplied by Energy Forecasting
    pub nativedemand: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 10th percentile of iterations and reference years (MWh)
    pub use_percentile10: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 20th percentile of iterations and reference years (MWh)
    pub use_percentile20: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 30th percentile of iterations and reference years (MWh)
    pub use_percentile30: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 40th percentile of iterations and reference years (MWh)
    pub use_percentile40: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 50th percentile of iterations and reference years (MWh)
    pub use_percentile50: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 60th percentile of iterations and reference years (MWh)
    pub use_percentile60: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 70th percentile of iterations and reference years (MWh)
    pub use_percentile70: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 80th percentile of iterations and reference years (MWh)
    pub use_percentile80: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 90th percentile of iterations and reference years (MWh)
    pub use_percentile90: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 100th percentile of iterations and reference years (MWh)
    pub use_percentile100: Option<rust_decimal::Decimal>,
    /// Average period unserved energy across iterations and reference years (MWh)
    pub use_average: Option<rust_decimal::Decimal>,
    /// Total number of iterations and reference years performed
    pub numberofiterations: Option<rust_decimal::Decimal>,
    /// Number of iterations and reference years showing unserved energy
    pub use_numberofiterations: Option<rust_decimal::Decimal>,
    /// Maximum unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy>0 (MW)
    pub use_event_max: Option<rust_decimal::Decimal>,
    /// Upper quartile unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy>0 (MW)
    pub use_event_upperquartile: Option<rust_decimal::Decimal>,
    /// Median unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy>0 (MW)
    pub use_event_median: Option<rust_decimal::Decimal>,
    /// Lower quartile unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy>0 (MW)
    pub use_event_lowerquartile: Option<rust_decimal::Decimal>,
    /// Minimum unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy>0 (MW)
    pub use_event_min: Option<rust_decimal::Decimal>,
    /// Fixed Values of 0.696 for 50 POE and 0.304 for 10 POE.
    pub weight: Option<rust_decimal::Decimal>,
    /// Weighted average USE per region = (USE_AVERAGE_POE10/NATIVE_DEMAND_POE_10*WEIGHT_POE_10 + USE_AVERAGE_POE50/NATIVE_DEMAND_POE_50*WEIGHT_POE_50)*100
    pub use_weighted_avg: Option<rust_decimal::Decimal>,
    /// LRC Condition reported (Value=1) if USE_WEIGHTED_AVG >= 0.002% otherwise (Value=0)
    pub lrc: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MtpasaRegionsummary1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn demand_poe_type(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.demand_poe_type.clone(),
        )
    }
    pub fn aggregation_period(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.aggregation_period.clone(),
        )
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for MtpasaRegionsummary1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MTPASA";
    const TABLE_NAME: &'static str = "REGIONSUMMARY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MtpasaRegionsummary1Mapping([
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
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        33,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUN_NO",
        "RUNTYPE",
        "DEMAND_POE_TYPE",
        "AGGREGATION_PERIOD",
        "PERIOD_ENDING",
        "REGIONID",
        "NATIVEDEMAND",
        "USE_PERCENTILE10",
        "USE_PERCENTILE20",
        "USE_PERCENTILE30",
        "USE_PERCENTILE40",
        "USE_PERCENTILE50",
        "USE_PERCENTILE60",
        "USE_PERCENTILE70",
        "USE_PERCENTILE80",
        "USE_PERCENTILE90",
        "USE_PERCENTILE100",
        "USE_AVERAGE",
        "NUMBEROFITERATIONS",
        "USE_NUMBEROFITERATIONS",
        "USE_EVENT_MAX",
        "USE_EVENT_UPPERQUARTILE",
        "USE_EVENT_MEDIAN",
        "USE_EVENT_LOWERQUARTILE",
        "USE_EVENT_MIN",
        "WEIGHT",
        "USE_WEIGHTED_AVG",
        "LRC",
        "LASTCHANGED",
    ];
    type Row<'row> = MtpasaRegionsummary1Row<'row>;
    type FieldMapping = MtpasaRegionsummary1Mapping;
    type PrimaryKey = MtpasaRegionsummary1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MtpasaRegionsummary1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            run_no: row.get_parsed_at_idx("run_no", field_mapping.0[1])?,
            runtype: row.get_range("runtype", field_mapping.0[2])?,
            demand_poe_type: row.get_range("demand_poe_type", field_mapping.0[3])?,
            aggregation_period: row.get_range("aggregation_period", field_mapping.0[4])?,
            period_ending: row
                .get_custom_parsed_at_idx(
                    "period_ending",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[6])?,
            nativedemand: row
                .get_opt_custom_parsed_at_idx(
                    "nativedemand",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile10: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile10",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile20: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile20",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile30: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile30",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile40: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile40",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile50: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile50",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile60: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile60",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile70: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile70",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile80: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile80",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile90: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile90",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_percentile100: row
                .get_opt_custom_parsed_at_idx(
                    "use_percentile100",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_average: row
                .get_opt_custom_parsed_at_idx(
                    "use_average",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            numberofiterations: row
                .get_opt_custom_parsed_at_idx(
                    "numberofiterations",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_numberofiterations: row
                .get_opt_custom_parsed_at_idx(
                    "use_numberofiterations",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_event_max: row
                .get_opt_custom_parsed_at_idx(
                    "use_event_max",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_event_upperquartile: row
                .get_opt_custom_parsed_at_idx(
                    "use_event_upperquartile",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_event_median: row
                .get_opt_custom_parsed_at_idx(
                    "use_event_median",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_event_lowerquartile: row
                .get_opt_custom_parsed_at_idx(
                    "use_event_lowerquartile",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_event_min: row
                .get_opt_custom_parsed_at_idx(
                    "use_event_min",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            weight: row
                .get_opt_custom_parsed_at_idx(
                    "weight",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            use_weighted_avg: row
                .get_opt_custom_parsed_at_idx(
                    "use_weighted_avg",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lrc: row
                .get_opt_custom_parsed_at_idx(
                    "lrc",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[29],
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
        Ok(MtpasaRegionsummary1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MtpasaRegionsummary1PrimaryKey {
        MtpasaRegionsummary1PrimaryKey {
            aggregation_period: row.aggregation_period().to_string(),
            demand_poe_type: row.demand_poe_type().to_string(),
            period_ending: row.period_ending,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            run_no: row.run_no,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mtpasa_regionsummary_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MtpasaRegionsummary1Row {
            run_datetime: row.run_datetime.clone(),
            run_no: row.run_no.clone(),
            runtype: row.runtype.clone(),
            demand_poe_type: row.demand_poe_type.clone(),
            aggregation_period: row.aggregation_period.clone(),
            period_ending: row.period_ending.clone(),
            regionid: row.regionid.clone(),
            nativedemand: row.nativedemand.clone(),
            use_percentile10: row.use_percentile10.clone(),
            use_percentile20: row.use_percentile20.clone(),
            use_percentile30: row.use_percentile30.clone(),
            use_percentile40: row.use_percentile40.clone(),
            use_percentile50: row.use_percentile50.clone(),
            use_percentile60: row.use_percentile60.clone(),
            use_percentile70: row.use_percentile70.clone(),
            use_percentile80: row.use_percentile80.clone(),
            use_percentile90: row.use_percentile90.clone(),
            use_percentile100: row.use_percentile100.clone(),
            use_average: row.use_average.clone(),
            numberofiterations: row.numberofiterations.clone(),
            use_numberofiterations: row.use_numberofiterations.clone(),
            use_event_max: row.use_event_max.clone(),
            use_event_upperquartile: row.use_event_upperquartile.clone(),
            use_event_median: row.use_event_median.clone(),
            use_event_lowerquartile: row.use_event_lowerquartile.clone(),
            use_event_min: row.use_event_min.clone(),
            weight: row.weight.clone(),
            use_weighted_avg: row.use_weighted_avg.clone(),
            lrc: row.lrc.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MtpasaRegionsummary1PrimaryKey {
    pub aggregation_period: alloc::string::String,
    pub demand_poe_type: alloc::string::String,
    pub period_ending: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionsummary1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionsummary1Row<'data> {
    type Row<'other> = MtpasaRegionsummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.aggregation_period() == row.aggregation_period()
            && self.demand_poe_type() == row.demand_poe_type()
            && self.period_ending == row.period_ending
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.run_no == row.run_no && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MtpasaRegionsummary1Row<'data> {
    type PrimaryKey = MtpasaRegionsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period() == key.aggregation_period
            && self.demand_poe_type() == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid() == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for MtpasaRegionsummary1PrimaryKey {
    type Row<'other> = MtpasaRegionsummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.aggregation_period == row.aggregation_period()
            && self.demand_poe_type == row.demand_poe_type()
            && self.period_ending == row.period_ending && self.regionid == row.regionid()
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionsummary1PrimaryKey {
    type PrimaryKey = MtpasaRegionsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.demand_poe_type == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionsummary1 {
    type Builder = MtpasaRegionsummary1Builder;
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
                    "run_no",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand_poe_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "aggregation_period",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "period_ending",
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
                    "nativedemand",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile20",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile30",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile40",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile60",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile70",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile80",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_percentile100",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_average",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "numberofiterations",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_numberofiterations",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_event_max",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_event_upperquartile",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_event_median",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_event_lowerquartile",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_event_min",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "weight",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "use_weighted_avg",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lrc",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        MtpasaRegionsummary1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            run_no_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            demand_poe_type_array: arrow::array::builder::StringBuilder::new(),
            aggregation_period_array: arrow::array::builder::StringBuilder::new(),
            period_ending_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            nativedemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile20_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile30_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile40_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile60_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile70_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile80_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_percentile100_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_average_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            numberofiterations_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_numberofiterations_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_event_max_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_event_upperquartile_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_event_median_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_event_lowerquartile_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            use_event_min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            weight_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            use_weighted_avg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lrc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.run_no_array.append_value(row.run_no);
        builder.runtype_array.append_value(row.runtype());
        builder.demand_poe_type_array.append_value(row.demand_poe_type());
        builder.aggregation_period_array.append_value(row.aggregation_period());
        builder
            .period_ending_array
            .append_value(row.period_ending.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .nativedemand_array
            .append_option({
                row.nativedemand
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile10_array
            .append_option({
                row.use_percentile10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile20_array
            .append_option({
                row.use_percentile20
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile30_array
            .append_option({
                row.use_percentile30
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile40_array
            .append_option({
                row.use_percentile40
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile50_array
            .append_option({
                row.use_percentile50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile60_array
            .append_option({
                row.use_percentile60
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile70_array
            .append_option({
                row.use_percentile70
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile80_array
            .append_option({
                row.use_percentile80
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile90_array
            .append_option({
                row.use_percentile90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_percentile100_array
            .append_option({
                row.use_percentile100
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_average_array
            .append_option({
                row.use_average
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .numberofiterations_array
            .append_option({
                row.numberofiterations
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_numberofiterations_array
            .append_option({
                row.use_numberofiterations
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_event_max_array
            .append_option({
                row.use_event_max
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_event_upperquartile_array
            .append_option({
                row.use_event_upperquartile
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_event_median_array
            .append_option({
                row.use_event_median
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_event_lowerquartile_array
            .append_option({
                row.use_event_lowerquartile
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .use_event_min_array
            .append_option({
                row.use_event_min
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .weight_array
            .append_option({
                row.weight
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .use_weighted_avg_array
            .append_option({
                row.use_weighted_avg
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lrc_array
            .append_option({
                row.lrc
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.run_no_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_poe_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregation_period_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.period_ending_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nativedemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile20_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile30_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile40_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile60_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile70_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile80_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_percentile100_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_average_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.numberofiterations_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_numberofiterations_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_event_max_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_event_upperquartile_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_event_median_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_event_lowerquartile_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_event_min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.weight_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.use_weighted_avg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lrc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MtpasaRegionsummary1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    run_no_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    demand_poe_type_array: arrow::array::builder::StringBuilder,
    aggregation_period_array: arrow::array::builder::StringBuilder,
    period_ending_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    nativedemand_array: arrow::array::builder::Decimal128Builder,
    use_percentile10_array: arrow::array::builder::Decimal128Builder,
    use_percentile20_array: arrow::array::builder::Decimal128Builder,
    use_percentile30_array: arrow::array::builder::Decimal128Builder,
    use_percentile40_array: arrow::array::builder::Decimal128Builder,
    use_percentile50_array: arrow::array::builder::Decimal128Builder,
    use_percentile60_array: arrow::array::builder::Decimal128Builder,
    use_percentile70_array: arrow::array::builder::Decimal128Builder,
    use_percentile80_array: arrow::array::builder::Decimal128Builder,
    use_percentile90_array: arrow::array::builder::Decimal128Builder,
    use_percentile100_array: arrow::array::builder::Decimal128Builder,
    use_average_array: arrow::array::builder::Decimal128Builder,
    numberofiterations_array: arrow::array::builder::Decimal128Builder,
    use_numberofiterations_array: arrow::array::builder::Decimal128Builder,
    use_event_max_array: arrow::array::builder::Decimal128Builder,
    use_event_upperquartile_array: arrow::array::builder::Decimal128Builder,
    use_event_median_array: arrow::array::builder::Decimal128Builder,
    use_event_lowerquartile_array: arrow::array::builder::Decimal128Builder,
    use_event_min_array: arrow::array::builder::Decimal128Builder,
    weight_array: arrow::array::builder::Decimal128Builder,
    use_weighted_avg_array: arrow::array::builder::Decimal128Builder,
    lrc_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
