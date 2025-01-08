#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct DispatchocdConstraintrelaxation2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchocdConstraintrelaxation2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchocdConstraintrelaxation2 {
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
pub struct DispatchocdConstraintrelaxation2Mapping([usize; 6]);
/// # Summary
///
/// ## CONSTRAINTRELAXATION_OCD
///  _CONSTRAINTRELAXATION_OCD contains details of interconnector constraints and unit ancillary service constraints relaxed in the over-constrained dispatch (OCD) re-run for this interval (if there was one).<br>Note: INTERVENTION is not included in CONSTRAINTRELAXATION_OCD, since the relaxation of the same constraint is the same amount in both intervened and non-intervened cases._
///
/// * Data Set Name: Dispatchocd
/// * File Name: Constraintrelaxation
/// * Data Version: 2
///
/// # Description
///  Source The occurrences of Over-Constrained Dispatch (OCD) re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Over-constrained dispatch (OCD) re-run (if there was one). Volume Rows per day: ~2 Mb per month: &lt;1 The estimates on the number of rows are based on a 1% occurrence rate for OCD runs.
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchocdConstraintrelaxation2Row<'data> {
    /// End date and time of the dispatch interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no
    pub runno: rust_decimal::Decimal,
    /// Constraint identifier
    pub constraintid: core::ops::Range<usize>,
    /// Relaxed RHS used in attempt to avoid constraint violation
    pub rhs: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Version Number
    pub versionno: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchocdConstraintrelaxation2Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchocdConstraintrelaxation2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "DISPATCHOCD";
    const TABLE_NAME: &'static str = "CONSTRAINTRELAXATION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchocdConstraintrelaxation2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "CONSTRAINTID",
        "RHS",
        "LASTCHANGED",
        "VERSIONNO",
    ];
    type Row<'row> = DispatchocdConstraintrelaxation2Row<'row>;
    type FieldMapping = DispatchocdConstraintrelaxation2Mapping;
    type PrimaryKey = DispatchocdConstraintrelaxation2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchocdConstraintrelaxation2Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[2])?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[5],
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
        Ok(DispatchocdConstraintrelaxation2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchocdConstraintrelaxation2PrimaryKey {
        DispatchocdConstraintrelaxation2PrimaryKey {
            constraintid: row.constraintid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "dispatchocd_constraintrelaxation_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchocdConstraintrelaxation2Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            constraintid: row.constraintid.clone(),
            rhs: row.rhs.clone(),
            lastchanged: row.lastchanged.clone(),
            versionno: row.versionno.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchocdConstraintrelaxation2PrimaryKey {
    pub constraintid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for DispatchocdConstraintrelaxation2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchocdConstraintrelaxation2Row<'data> {
    type Row<'other> = DispatchocdConstraintrelaxation2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DispatchocdConstraintrelaxation2Row<'data> {
    type PrimaryKey = DispatchocdConstraintrelaxation2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchocdConstraintrelaxation2PrimaryKey {
    type Row<'other> = DispatchocdConstraintrelaxation2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchocdConstraintrelaxation2PrimaryKey {
    type PrimaryKey = DispatchocdConstraintrelaxation2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchocdConstraintrelaxation2 {
    type Builder = DispatchocdConstraintrelaxation2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
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
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchocdConstraintrelaxation2Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.constraintid_array.append_value(row.constraintid());
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
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchocdConstraintrelaxation2Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    constraintid_array: arrow::array::builder::StringBuilder,
    rhs_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchBlockedConstraints1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchBlockedConstraints1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchBlockedConstraints1 {
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
pub struct DispatchBlockedConstraints1Mapping([usize; 3]);
/// # Summary
///
/// ## DISPATCHBLOCKEDCONSTRAINT
///  _DISPATCH Blocked Constraints lists any constraints that were blocked in a dispatch run. If no constraints are blocked, there will be no rows for that dispatch run._
///
/// * Data Set Name: Dispatch
/// * File Name: Blocked Constraints
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchBlockedConstraints1Row<'data> {
    /// Dispatch Interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchBlockedConstraints1Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchBlockedConstraints1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "BLOCKED_CONSTRAINTS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchBlockedConstraints1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "CONSTRAINTID",
    ];
    type Row<'row> = DispatchBlockedConstraints1Row<'row>;
    type FieldMapping = DispatchBlockedConstraints1Mapping;
    type PrimaryKey = DispatchBlockedConstraints1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchBlockedConstraints1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[2])?,
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
        Ok(DispatchBlockedConstraints1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchBlockedConstraints1PrimaryKey {
        DispatchBlockedConstraints1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_blocked_constraints_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchBlockedConstraints1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            constraintid: row.constraintid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchBlockedConstraints1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchBlockedConstraints1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchBlockedConstraints1Row<'data> {
    type Row<'other> = DispatchBlockedConstraints1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchBlockedConstraints1Row<'data> {
    type PrimaryKey = DispatchBlockedConstraints1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchBlockedConstraints1PrimaryKey {
    type Row<'other> = DispatchBlockedConstraints1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchBlockedConstraints1PrimaryKey {
    type PrimaryKey = DispatchBlockedConstraints1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchBlockedConstraints1 {
    type Builder = DispatchBlockedConstraints1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchBlockedConstraints1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.constraintid_array.append_value(row.constraintid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchBlockedConstraints1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    constraintid_array: arrow::array::builder::StringBuilder,
}
pub struct DispatchCaseSolution2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchCaseSolution2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchCaseSolution2 {
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
pub struct DispatchCaseSolution2Mapping([usize; 24]);
/// # Summary
///
/// ## DISPATCHCASESOLUTION
///  _DISPATCHCASESOLUTION shows information relating to the complete dispatch run. The fields in DISPATCHCASESOLUTION provide an overview of the dispatch run results allowing immediate identification of conditions such as energy or FCAS deficiencies._
///
/// * Data Set Name: Dispatch
/// * File Name: Case Solution
/// * Data Version: 2
///
/// # Description
///  The DISPATCHCASESOLUTION data is public. Source DISPATCHCASESOLUTION updates every 5 minutes. Volume Approximately 288 records per day.
///
///
///
/// # Primary Key Columns
///
/// * RUNNO
/// * SETTLEMENTDATE
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchCaseSolution2Row<'data> {
    /// Date and time of the dispatch interval (e.g. five minute dispatch interval ending 28/09/2000 16:35)
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention flag - refer to package documentation for definition and practical query examples
    pub intervention: rust_decimal::Decimal,
    /// Overconstrained dispatch indicator: <br>* OCD = detecting over-constrained dispatch<br>* null = no special condition
    pub casesubtype: core::ops::Range<usize>,
    /// If non-zero indicated one of the following conditions:<br>* 1 = Supply Scarcity, Excess generation or constraint violations<br>* X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: core::ops::Range<usize>,
    /// Non-Physical Losses algorithm invoked occurred during this run
    pub nonphysicallosses: Option<rust_decimal::Decimal>,
    /// The Objective function from the LP
    pub totalobjective: Option<rust_decimal::Decimal>,
    /// Total Region Demand violations
    pub totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Total interconnector violations
    pub totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Total generic constraint violations
    pub totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Total ramp rate violations
    pub totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Total unit capacity violations
    pub totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Total of 5 minute ancillary service region violations
    pub total5minviolation: Option<rust_decimal::Decimal>,
    /// Total of Regulation ancillary service region violations
    pub totalregviolation: Option<rust_decimal::Decimal>,
    /// Total of 6 second ancillary service region violations
    pub total6secviolation: Option<rust_decimal::Decimal>,
    /// Total of 60 second ancillary service region violations
    pub total60secviolation: Option<rust_decimal::Decimal>,
    /// Total of ancillary service trader profile violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Total of fast start trader profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag indicating the SCADA status for FCAS Interconnector dead-band. "0" if SCADA Status or requesting Constraint not invoked. "1" if SCADA Status AND requesting Constraint is invoked
    pub switchruninitialstatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Solution – from PeriodSolution
    pub switchrunbeststatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Intervention Physical Solution - from PeriodSolution
    pub switchrunbeststatus_int: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchCaseSolution2Row<'data> {
    pub fn casesubtype(&self) -> Option<&str> {
        if self.casesubtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.casesubtype.clone(),
                ),
            )
        }
    }
    pub fn spdversion(&self) -> Option<&str> {
        if self.spdversion.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spdversion.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchCaseSolution2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "CASE_SOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchCaseSolution2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "INTERVENTION",
        "CASESUBTYPE",
        "SOLUTIONSTATUS",
        "SPDVERSION",
        "NONPHYSICALLOSSES",
        "TOTALOBJECTIVE",
        "TOTALAREAGENVIOLATION",
        "TOTALINTERCONNECTORVIOLATION",
        "TOTALGENERICVIOLATION",
        "TOTALRAMPRATEVIOLATION",
        "TOTALUNITMWCAPACITYVIOLATION",
        "TOTAL5MINVIOLATION",
        "TOTALREGVIOLATION",
        "TOTAL6SECVIOLATION",
        "TOTAL60SECVIOLATION",
        "TOTALASPROFILEVIOLATION",
        "TOTALFASTSTARTVIOLATION",
        "TOTALENERGYOFFERVIOLATION",
        "LASTCHANGED",
        "SWITCHRUNINITIALSTATUS",
        "SWITCHRUNBESTSTATUS",
        "SWITCHRUNBESTSTATUS_INT",
    ];
    type Row<'row> = DispatchCaseSolution2Row<'row>;
    type FieldMapping = DispatchCaseSolution2Mapping;
    type PrimaryKey = DispatchCaseSolution2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchCaseSolution2Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            casesubtype: row.get_opt_range("casesubtype", field_mapping.0[3])?,
            solutionstatus: row
                .get_opt_custom_parsed_at_idx(
                    "solutionstatus",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            spdversion: row.get_opt_range("spdversion", field_mapping.0[5])?,
            nonphysicallosses: row
                .get_opt_custom_parsed_at_idx(
                    "nonphysicallosses",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalobjective: row
                .get_opt_custom_parsed_at_idx(
                    "totalobjective",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalareagenviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalareagenviolation",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalinterconnectorviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalinterconnectorviolation",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalgenericviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalgenericviolation",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalramprateviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalramprateviolation",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalunitmwcapacityviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalunitmwcapacityviolation",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total5minviolation",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalregviolation",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total6secviolation",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total60secviolation",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalasprofileviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalasprofileviolation",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalfaststartviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalfaststartviolation",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalenergyofferviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalenergyofferviolation",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[20],
                    mmsdm_core::mms_datetime::parse,
                )?,
            switchruninitialstatus: row
                .get_opt_custom_parsed_at_idx(
                    "switchruninitialstatus",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            switchrunbeststatus: row
                .get_opt_custom_parsed_at_idx(
                    "switchrunbeststatus",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            switchrunbeststatus_int: row
                .get_opt_custom_parsed_at_idx(
                    "switchrunbeststatus_int",
                    field_mapping.0[23],
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
        Ok(DispatchCaseSolution2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchCaseSolution2PrimaryKey {
        DispatchCaseSolution2PrimaryKey {
            runno: row.runno,
            settlementdate: row.settlementdate,
            intervention: row.intervention,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_case_solution_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchCaseSolution2Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            intervention: row.intervention.clone(),
            casesubtype: row.casesubtype.clone(),
            solutionstatus: row.solutionstatus.clone(),
            spdversion: row.spdversion.clone(),
            nonphysicallosses: row.nonphysicallosses.clone(),
            totalobjective: row.totalobjective.clone(),
            totalareagenviolation: row.totalareagenviolation.clone(),
            totalinterconnectorviolation: row.totalinterconnectorviolation.clone(),
            totalgenericviolation: row.totalgenericviolation.clone(),
            totalramprateviolation: row.totalramprateviolation.clone(),
            totalunitmwcapacityviolation: row.totalunitmwcapacityviolation.clone(),
            total5minviolation: row.total5minviolation.clone(),
            totalregviolation: row.totalregviolation.clone(),
            total6secviolation: row.total6secviolation.clone(),
            total60secviolation: row.total60secviolation.clone(),
            totalasprofileviolation: row.totalasprofileviolation.clone(),
            totalfaststartviolation: row.totalfaststartviolation.clone(),
            totalenergyofferviolation: row.totalenergyofferviolation.clone(),
            lastchanged: row.lastchanged.clone(),
            switchruninitialstatus: row.switchruninitialstatus.clone(),
            switchrunbeststatus: row.switchrunbeststatus.clone(),
            switchrunbeststatus_int: row.switchrunbeststatus_int.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchCaseSolution2PrimaryKey {
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for DispatchCaseSolution2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchCaseSolution2Row<'data> {
    type Row<'other> = DispatchCaseSolution2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.runno == row.runno && self.settlementdate == row.settlementdate
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchCaseSolution2Row<'data> {
    type PrimaryKey = DispatchCaseSolution2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.runno == key.runno && self.settlementdate == key.settlementdate
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchCaseSolution2PrimaryKey {
    type Row<'other> = DispatchCaseSolution2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.runno == row.runno && self.settlementdate == row.settlementdate
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchCaseSolution2PrimaryKey {
    type PrimaryKey = DispatchCaseSolution2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.runno == key.runno && self.settlementdate == key.settlementdate
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchCaseSolution2 {
    type Builder = DispatchCaseSolution2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "casesubtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "solutionstatus",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spdversion",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nonphysicallosses",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalobjective",
                    arrow::datatypes::DataType::Decimal128(27, 10),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalareagenviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalinterconnectorviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalgenericviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalramprateviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalunitmwcapacityviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "total5minviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalregviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "total6secviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "total60secviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalasprofileviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalfaststartviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalenergyofferviolation",
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
                    "switchruninitialstatus",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "switchrunbeststatus",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "switchrunbeststatus_int",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchCaseSolution2Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            casesubtype_array: arrow::array::builder::StringBuilder::new(),
            solutionstatus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            spdversion_array: arrow::array::builder::StringBuilder::new(),
            nonphysicallosses_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            totalobjective_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 10)),
            totalareagenviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalinterconnectorviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalgenericviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalramprateviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalunitmwcapacityviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            total5minviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalregviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            total6secviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            total60secviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalasprofileviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalfaststartviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            switchruninitialstatus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            switchrunbeststatus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            switchrunbeststatus_int_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.casesubtype_array.append_option(row.casesubtype());
        builder
            .solutionstatus_array
            .append_option({
                row.solutionstatus
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.spdversion_array.append_option(row.spdversion());
        builder
            .nonphysicallosses_array
            .append_option({
                row.nonphysicallosses
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .totalobjective_array
            .append_option({
                row.totalobjective
                    .map(|mut val| {
                        val.rescale(10);
                        val.mantissa()
                    })
            });
        builder
            .totalareagenviolation_array
            .append_option({
                row.totalareagenviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalinterconnectorviolation_array
            .append_option({
                row.totalinterconnectorviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalgenericviolation_array
            .append_option({
                row.totalgenericviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalramprateviolation_array
            .append_option({
                row.totalramprateviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalunitmwcapacityviolation_array
            .append_option({
                row.totalunitmwcapacityviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .total5minviolation_array
            .append_option({
                row.total5minviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalregviolation_array
            .append_option({
                row.totalregviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .total6secviolation_array
            .append_option({
                row.total6secviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .total60secviolation_array
            .append_option({
                row.total60secviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalasprofileviolation_array
            .append_option({
                row.totalasprofileviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalfaststartviolation_array
            .append_option({
                row.totalfaststartviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalenergyofferviolation_array
            .append_option({
                row.totalenergyofferviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .switchruninitialstatus_array
            .append_option({
                row.switchruninitialstatus
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .switchrunbeststatus_array
            .append_option({
                row.switchrunbeststatus
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .switchrunbeststatus_int_array
            .append_option({
                row.switchrunbeststatus_int
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.casesubtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.solutionstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spdversion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nonphysicallosses_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalobjective_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalareagenviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.totalinterconnectorviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalgenericviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalramprateviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.totalunitmwcapacityviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.total5minviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalregviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.total6secviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.total60secviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalasprofileviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalfaststartviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.totalenergyofferviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.switchruninitialstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.switchrunbeststatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.switchrunbeststatus_int_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchCaseSolution2Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    casesubtype_array: arrow::array::builder::StringBuilder,
    solutionstatus_array: arrow::array::builder::Decimal128Builder,
    spdversion_array: arrow::array::builder::StringBuilder,
    nonphysicallosses_array: arrow::array::builder::Decimal128Builder,
    totalobjective_array: arrow::array::builder::Decimal128Builder,
    totalareagenviolation_array: arrow::array::builder::Decimal128Builder,
    totalinterconnectorviolation_array: arrow::array::builder::Decimal128Builder,
    totalgenericviolation_array: arrow::array::builder::Decimal128Builder,
    totalramprateviolation_array: arrow::array::builder::Decimal128Builder,
    totalunitmwcapacityviolation_array: arrow::array::builder::Decimal128Builder,
    total5minviolation_array: arrow::array::builder::Decimal128Builder,
    totalregviolation_array: arrow::array::builder::Decimal128Builder,
    total6secviolation_array: arrow::array::builder::Decimal128Builder,
    total60secviolation_array: arrow::array::builder::Decimal128Builder,
    totalasprofileviolation_array: arrow::array::builder::Decimal128Builder,
    totalfaststartviolation_array: arrow::array::builder::Decimal128Builder,
    totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    switchruninitialstatus_array: arrow::array::builder::Decimal128Builder,
    switchrunbeststatus_array: arrow::array::builder::Decimal128Builder,
    switchrunbeststatus_int_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchConstraint5 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchConstraint5Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchConstraint5 {
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
pub struct DispatchConstraint5Mapping([usize; 13]);
/// # Summary
///
/// ## DISPATCHCONSTRAINT
///  _DISPATCHCONSTRAINT sets out details of all binding and interregion constraints in each dispatch run. Note: invoked constraints can be established from GENCONSETINVOKE. Binding constraints show as marginal value &gt;$0. Interconnector constraints are listed so RHS (SCADA calculated limits) can be reported._
///
/// * Data Set Name: Dispatch
/// * File Name: Constraint
/// * Data Version: 5
///
/// # Description
///  DISPATCHCONSTRAINT is public data, and is available to all participants. Source DISPATCHCONSTRAINT updates every five minutes.
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchConstraint5Row<'data> {
    /// Market date starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    /// Manual Intervention flag, which, if set (1), causes predispatch to solve twice.
    pub intervention: rust_decimal::Decimal,
    /// Right hand Side value as used in dispatch.
    pub rhs: Option<rust_decimal::Decimal>,
    /// $ Value of binding constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation in MW
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    pub duid: core::ops::Range<usize>,
    /// Effective date of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    pub genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    pub genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    pub lhs: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchConstraint5Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
    pub fn duid(&self) -> Option<&str> {
        if self.duid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone()),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchConstraint5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "CONSTRAINT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchConstraint5Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "CONSTRAINTID",
        "DISPATCHINTERVAL",
        "INTERVENTION",
        "RHS",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "LASTCHANGED",
        "DUID",
        "GENCONID_EFFECTIVEDATE",
        "GENCONID_VERSIONNO",
        "LHS",
    ];
    type Row<'row> = DispatchConstraint5Row<'row>;
    type FieldMapping = DispatchConstraint5Mapping;
    type PrimaryKey = DispatchConstraint5PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchConstraint5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[2])?,
            dispatchinterval: row
                .get_parsed_at_idx("dispatchinterval", field_mapping.0[3])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violationdegree",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_opt_range("duid", field_mapping.0[9])?,
            genconid_effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "genconid_effectivedate",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            genconid_versionno: row
                .get_opt_custom_parsed_at_idx(
                    "genconid_versionno",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lhs: row
                .get_opt_custom_parsed_at_idx(
                    "lhs",
                    field_mapping.0[12],
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
        Ok(DispatchConstraint5Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchConstraint5PrimaryKey {
        DispatchConstraint5PrimaryKey {
            constraintid: row.constraintid().to_string(),
            dispatchinterval: row.dispatchinterval,
            intervention: row.intervention,
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_constraint_v5_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchConstraint5Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            constraintid: row.constraintid.clone(),
            dispatchinterval: row.dispatchinterval.clone(),
            intervention: row.intervention.clone(),
            rhs: row.rhs.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            lastchanged: row.lastchanged.clone(),
            duid: row.duid.clone(),
            genconid_effectivedate: row.genconid_effectivedate.clone(),
            genconid_versionno: row.genconid_versionno.clone(),
            lhs: row.lhs.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchConstraint5PrimaryKey {
    pub constraintid: alloc::string::String,
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchConstraint5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchConstraint5Row<'data> {
    type Row<'other> = DispatchConstraint5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchConstraint5Row<'data> {
    type PrimaryKey = DispatchConstraint5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchConstraint5PrimaryKey {
    type Row<'other> = DispatchConstraint5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchConstraint5PrimaryKey {
    type PrimaryKey = DispatchConstraint5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchConstraint5 {
    type Builder = DispatchConstraint5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "dispatchinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rhs",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violationdegree",
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
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "genconid_effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "genconid_versionno",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lhs",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchConstraint5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            genconid_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            genconid_versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            lhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.constraintid_array.append_value(row.constraintid());
        builder
            .dispatchinterval_array
            .append_value(row.dispatchinterval.start().and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .rhs_array
            .append_option({
                row.rhs
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginalvalue_array
            .append_option({
                row.marginalvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violationdegree_array
            .append_option({
                row.violationdegree
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.duid_array.append_option(row.duid());
        builder
            .genconid_effectivedate_array
            .append_option(
                row.genconid_effectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .genconid_versionno_array
            .append_option({
                row.genconid_versionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lhs_array
            .append_option({
                row.lhs
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchConstraint5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    constraintid_array: arrow::array::builder::StringBuilder,
    dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    rhs_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    genconid_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    genconid_versionno_array: arrow::array::builder::Decimal128Builder,
    lhs_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchInterconnectorres3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchInterconnectorres3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchInterconnectorres3 {
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
pub struct DispatchInterconnectorres3Mapping([usize; 22]);
/// # Summary
///
/// ## DISPATCHINTERCONNECTORRES
///  _DISPATCHINTERCONNECTORRES sets out MW flow and losses on each interconnector for each dispatch period, including fields for the Frequency Controlled Ancillary Services export and import limits and extra reporting of the generic constraints set the energy import and export limits._
///
/// * Data Set Name: Dispatch
/// * File Name: Interconnectorres
/// * Data Version: 3
///
/// # Description
///  DISPATCHINTERCONNECTORRES is public data, and is available to all participants. Source DISPATCHINTERCONNECTORRES updates every 5 minutes. Note MW losses can be negative depending on the flow. The definition of direction of flow for an interconnector is that positive flow starts from the FROMREGION in the INTERCONNECTOR table.
///
///
///
/// # Primary Key Columns
///
/// * DISPATCHINTERVAL
/// * INTERCONNECTORID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchInterconnectorres3Row<'data> {
    /// Market date starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    /// Intervention case or not
    pub intervention: rust_decimal::Decimal,
    /// Metered MW Flow from SCADA.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Target MW Flow for next 5 mins.
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Shadow price resulting from thermal or reserve sharing constraints on Interconnector import/export (0 unless binding) - NEMDE Solution InterconnectorSolution element "Price" attribute
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation on interconnector constraints
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last changed.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Calculated export limit applying to energy only.
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy only.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust prices between regions.
    pub marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    pub exportgenconid: core::ops::Range<usize>,
    /// Generic Constraint setting the import limit
    pub importgenconid: core::ops::Range<usize>,
    /// Calculated export limit applying to energy + FCAS.
    pub fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + FCAS.
    pub fcasimportlimit: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    pub local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    pub local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_import: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchInterconnectorres3Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn exportgenconid(&self) -> Option<&str> {
        if self.exportgenconid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.exportgenconid.clone(),
                ),
            )
        }
    }
    pub fn importgenconid(&self) -> Option<&str> {
        if self.importgenconid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.importgenconid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchInterconnectorres3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "INTERCONNECTORRES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchInterconnectorres3Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "INTERCONNECTORID",
        "DISPATCHINTERVAL",
        "INTERVENTION",
        "METEREDMWFLOW",
        "MWFLOW",
        "MWLOSSES",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "LASTCHANGED",
        "EXPORTLIMIT",
        "IMPORTLIMIT",
        "MARGINALLOSS",
        "EXPORTGENCONID",
        "IMPORTGENCONID",
        "FCASEXPORTLIMIT",
        "FCASIMPORTLIMIT",
        "LOCAL_PRICE_ADJUSTMENT_EXPORT",
        "LOCALLY_CONSTRAINED_EXPORT",
        "LOCAL_PRICE_ADJUSTMENT_IMPORT",
        "LOCALLY_CONSTRAINED_IMPORT",
    ];
    type Row<'row> = DispatchInterconnectorres3Row<'row>;
    type FieldMapping = DispatchInterconnectorres3Mapping;
    type PrimaryKey = DispatchInterconnectorres3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchInterconnectorres3Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            dispatchinterval: row
                .get_parsed_at_idx("dispatchinterval", field_mapping.0[3])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meteredmwflow: row
                .get_opt_custom_parsed_at_idx(
                    "meteredmwflow",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwlosses: row
                .get_opt_custom_parsed_at_idx(
                    "mwlosses",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violationdegree",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            exportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "exportlimit",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importlimit: row
                .get_opt_custom_parsed_at_idx(
                    "importlimit",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalloss: row
                .get_opt_custom_parsed_at_idx(
                    "marginalloss",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportgenconid: row.get_opt_range("exportgenconid", field_mapping.0[14])?,
            importgenconid: row.get_opt_range("importgenconid", field_mapping.0[15])?,
            fcasexportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "fcasexportlimit",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fcasimportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "fcasimportlimit",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            local_price_adjustment_export: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_export",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_export: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_export",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            local_price_adjustment_import: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_import",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_import: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_import",
                    field_mapping.0[21],
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
        Ok(DispatchInterconnectorres3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchInterconnectorres3PrimaryKey {
        DispatchInterconnectorres3PrimaryKey {
            dispatchinterval: row.dispatchinterval,
            interconnectorid: row.interconnectorid().to_string(),
            intervention: row.intervention,
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_interconnectorres_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchInterconnectorres3Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            dispatchinterval: row.dispatchinterval.clone(),
            intervention: row.intervention.clone(),
            meteredmwflow: row.meteredmwflow.clone(),
            mwflow: row.mwflow.clone(),
            mwlosses: row.mwlosses.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            lastchanged: row.lastchanged.clone(),
            exportlimit: row.exportlimit.clone(),
            importlimit: row.importlimit.clone(),
            marginalloss: row.marginalloss.clone(),
            exportgenconid: row.exportgenconid.clone(),
            importgenconid: row.importgenconid.clone(),
            fcasexportlimit: row.fcasexportlimit.clone(),
            fcasimportlimit: row.fcasimportlimit.clone(),
            local_price_adjustment_export: row.local_price_adjustment_export.clone(),
            locally_constrained_export: row.locally_constrained_export.clone(),
            local_price_adjustment_import: row.local_price_adjustment_import.clone(),
            locally_constrained_import: row.locally_constrained_import.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchInterconnectorres3PrimaryKey {
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    pub interconnectorid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchInterconnectorres3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchInterconnectorres3Row<'data> {
    type Row<'other> = DispatchInterconnectorres3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.interconnectorid() == row.interconnectorid()
            && self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchInterconnectorres3Row<'data> {
    type PrimaryKey = DispatchInterconnectorres3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.interconnectorid() == key.interconnectorid
            && self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchInterconnectorres3PrimaryKey {
    type Row<'other> = DispatchInterconnectorres3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.interconnectorid == row.interconnectorid()
            && self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchInterconnectorres3PrimaryKey {
    type PrimaryKey = DispatchInterconnectorres3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.interconnectorid == key.interconnectorid
            && self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchInterconnectorres3 {
    type Builder = DispatchInterconnectorres3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "dispatchinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meteredmwflow",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwlosses",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violationdegree",
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
                    "exportlimit",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importlimit",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalloss",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportgenconid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importgenconid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fcasexportlimit",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fcasimportlimit",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "local_price_adjustment_export",
                    arrow::datatypes::DataType::Decimal128(10, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "locally_constrained_export",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "local_price_adjustment_import",
                    arrow::datatypes::DataType::Decimal128(10, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "locally_constrained_import",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchInterconnectorres3Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            meteredmwflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwlosses_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            exportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            importlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalloss_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            exportgenconid_array: arrow::array::builder::StringBuilder::new(),
            importgenconid_array: arrow::array::builder::StringBuilder::new(),
            fcasexportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            fcasimportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            local_price_adjustment_export_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 2)),
            locally_constrained_export_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            local_price_adjustment_import_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 2)),
            locally_constrained_import_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .dispatchinterval_array
            .append_value(row.dispatchinterval.start().and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .meteredmwflow_array
            .append_option({
                row.meteredmwflow
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow_array
            .append_option({
                row.mwflow
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwlosses_array
            .append_option({
                row.mwlosses
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginalvalue_array
            .append_option({
                row.marginalvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violationdegree_array
            .append_option({
                row.violationdegree
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .exportlimit_array
            .append_option({
                row.exportlimit
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .importlimit_array
            .append_option({
                row.importlimit
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginalloss_array
            .append_option({
                row.marginalloss
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder.exportgenconid_array.append_option(row.exportgenconid());
        builder.importgenconid_array.append_option(row.importgenconid());
        builder
            .fcasexportlimit_array
            .append_option({
                row.fcasexportlimit
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .fcasimportlimit_array
            .append_option({
                row.fcasimportlimit
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .local_price_adjustment_export_array
            .append_option({
                row.local_price_adjustment_export
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .locally_constrained_export_array
            .append_option({
                row.locally_constrained_export
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .local_price_adjustment_import_array
            .append_option({
                row.local_price_adjustment_import
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .locally_constrained_import_array
            .append_option({
                row.locally_constrained_import
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meteredmwflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwlosses_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalloss_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportgenconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importgenconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fcasexportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fcasimportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.local_price_adjustment_export_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.locally_constrained_export_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.local_price_adjustment_import_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.locally_constrained_import_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchInterconnectorres3Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    meteredmwflow_array: arrow::array::builder::Decimal128Builder,
    mwflow_array: arrow::array::builder::Decimal128Builder,
    mwlosses_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    exportlimit_array: arrow::array::builder::Decimal128Builder,
    importlimit_array: arrow::array::builder::Decimal128Builder,
    marginalloss_array: arrow::array::builder::Decimal128Builder,
    exportgenconid_array: arrow::array::builder::StringBuilder,
    importgenconid_array: arrow::array::builder::StringBuilder,
    fcasexportlimit_array: arrow::array::builder::Decimal128Builder,
    fcasimportlimit_array: arrow::array::builder::Decimal128Builder,
    local_price_adjustment_export_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_export_array: arrow::array::builder::Decimal128Builder,
    local_price_adjustment_import_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_import_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchUnitSolution5 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchUnitSolution5Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchUnitSolution5 {
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
pub struct DispatchUnitSolution5Mapping([usize; 68]);
/// # Summary
///
/// ## DISPATCHLOAD
///  _DISPATCHLOAD set out the current SCADA MW and target MW for each dispatchable unit, including relevant Frequency Control Ancillary Services (FCAS) enabling targets for each five minutes and additional fields to handle the new Ancillary Services functionality. Fast Start Plant status is indicated by dispatch mode._
///
/// * Data Set Name: Dispatch
/// * File Name: Unit Solution
/// * Data Version: 5
///
/// # Description
///  DISPATCHLOAD data is confidential for the current day, showing own details for participant and becomes public after close of business yesterday, and is available to all participants. Source DISPATCHLOAD shows data for every 5 minutes for all units, even zero targets. Volume Expect 40-50,000 records per day. All units are repeated, even zero targets. Note ** A flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined using the low 3 bits as follows: Flag Name Bit Description Enabled 0 The unit is enabled to provide this ancillary service type. Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. Interpretation of the bit-flags as a number gives the following possibilities (i.e. other combinations are not possible): Numeric Value Bit (2,1,0) Meaning 0 000 Not stranded, not trapped, not enabled. 1 001 Not stranded, not trapped, is enabled. 3 011 Not stranded, is trapped, is enabled. 4 100 Is stranded, not trapped, not enabled. For example, testing for availability can be done by checking for odd (=available) or even (=unavailable) number (e.g.  mod(flag,2)  results in 0 for unavailable and 1 for available). *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values.
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchUnitSolution5Row<'data> {
    /// Market date and time starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Not used
    pub tradetype: Option<rust_decimal::Decimal>,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    /// Intervention flag if intervention run
    pub intervention: rust_decimal::Decimal,
    /// Connection point identifier for DUID
    pub connectionpointid: core::ops::Range<usize>,
    /// Dispatch mode for fast start plant (0 to 4).
    pub dispatchmode: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS<br>* 1 = on<br>* 0 = off
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period. Negative values when Bi-directional Unit start from importing power, otherwise positive.
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period. Negative values when Bi-directional Unit is importing power, otherwise positive.
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate used in dispatch (lesser of bid or telemetered rate).
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lesser of bid or telemetered rate).
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Lower 5 min reserve target
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec reserve target
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec reserve target
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min reserve target
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec reserve target
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec reserve target
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Not Used
    pub downepf: Option<rust_decimal::Decimal>,
    /// Not Used
    pub upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min
    pub marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds
    pub marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds
    pub marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation MW 5 min
    pub violation5mindegree: Option<rust_decimal::Decimal>,
    /// Violation MW 60 seconds
    pub violation60secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW 6 seconds
    pub violation6secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW energy
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// For Scheduled units, this is the MAXAVAIL bid availability For Semi-scheduled units, this is the lower of MAXAVAIL bid availability and UIGF
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag  - see
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag  - see
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag  - see
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag  - see
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  - see
    pub lowerregflags: Option<rust_decimal::Decimal>,
    /// RaiseReg availability - minimum of bid and telemetered value
    pub raiseregavailability: Option<rust_decimal::Decimal>,
    /// RaiseReg enablement max point - minimum of bid and telemetered value
    pub raiseregenablementmax: Option<rust_decimal::Decimal>,
    /// RaiseReg Enablement Min point - maximum of bid and telemetered value
    pub raiseregenablementmin: Option<rust_decimal::Decimal>,
    /// Lower Reg availability - minimum of bid and telemetered value
    pub lowerregavailability: Option<rust_decimal::Decimal>,
    /// Lower Reg enablement Max point - minimum of bid and telemetered value
    pub lowerregenablementmax: Option<rust_decimal::Decimal>,
    /// Lower Reg Enablement Min point - maximum of bid and telemetered value
    pub lowerregenablementmin: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    pub raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    pub raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    pub raise5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise reg availability
    pub raiseregactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 6sec availability
    pub lower6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 60sec availability
    pub lower60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 5min availability
    pub lower5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower reg availability
    pub lowerregactualavailability: Option<rust_decimal::Decimal>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
    /// Minutes for which the unit has been in the current DISPATCHMODE. From NEMDE TRADERSOLUTION element FSTARGETMODETIME attribute.
    pub dispatchmodetime: Option<rust_decimal::Decimal>,
    /// Mode specific to units within an aggregate. 0 - no monitoring, 1 - aggregate monitoring, 2 - individual monitoring due to constraint
    pub conformance_mode: Option<rust_decimal::Decimal>,
    /// For Semi-Scheduled units. Unconstrained Intermittent Generation Forecast value provided to NEMDE
    pub uigf: Option<rust_decimal::Decimal>,
    /// Dispatched Raise1Sec - TraderSolution element R1Target attribute
    pub raise1sec: Option<rust_decimal::Decimal>,
    /// TraderSolution element R1Flags attribute
    pub raise1secflags: Option<rust_decimal::Decimal>,
    /// Dispatched Lower1Sec - TraderSolution element L1Target attribute
    pub lower1sec: Option<rust_decimal::Decimal>,
    /// TraderSolution element L1Flags attribute
    pub lower1secflags: Option<rust_decimal::Decimal>,
    /// Trapezium adjusted Raise 1Sec Availability
    pub raise1secactualavailability: Option<rust_decimal::Decimal>,
    /// Trapezium adjusted Lower 1Sec Availability
    pub lower1secactualavailability: Option<rust_decimal::Decimal>,
    /// BDU only. The energy storage at the start of this dispatch interval (MWh)
    pub initial_energy_storage: Option<rust_decimal::Decimal>,
    /// BDU only. The projected energy storage based on cleared energy and regulation FCAS dispatch (MWh)
    pub energy_storage: Option<rust_decimal::Decimal>,
    /// BDU only. Load side availability (BidOfferPeriod.MAXAVAIL where DIRECTION = LOAD)
    pub min_availability: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchUnitSolution5Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn connectionpointid(&self) -> Option<&str> {
        if self.connectionpointid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.connectionpointid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchUnitSolution5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "UNIT_SOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchUnitSolution5Mapping([
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
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
        49,
        50,
        51,
        52,
        53,
        54,
        55,
        56,
        57,
        58,
        59,
        60,
        61,
        62,
        63,
        64,
        65,
        66,
        67,
        68,
        69,
        70,
        71,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "DUID",
        "TRADETYPE",
        "DISPATCHINTERVAL",
        "INTERVENTION",
        "CONNECTIONPOINTID",
        "DISPATCHMODE",
        "AGCSTATUS",
        "INITIALMW",
        "TOTALCLEARED",
        "RAMPDOWNRATE",
        "RAMPUPRATE",
        "LOWER5MIN",
        "LOWER60SEC",
        "LOWER6SEC",
        "RAISE5MIN",
        "RAISE60SEC",
        "RAISE6SEC",
        "DOWNEPF",
        "UPEPF",
        "MARGINAL5MINVALUE",
        "MARGINAL60SECVALUE",
        "MARGINAL6SECVALUE",
        "MARGINALVALUE",
        "VIOLATION5MINDEGREE",
        "VIOLATION60SECDEGREE",
        "VIOLATION6SECDEGREE",
        "VIOLATIONDEGREE",
        "LASTCHANGED",
        "LOWERREG",
        "RAISEREG",
        "AVAILABILITY",
        "RAISE6SECFLAGS",
        "RAISE60SECFLAGS",
        "RAISE5MINFLAGS",
        "RAISEREGFLAGS",
        "LOWER6SECFLAGS",
        "LOWER60SECFLAGS",
        "LOWER5MINFLAGS",
        "LOWERREGFLAGS",
        "RAISEREGAVAILABILITY",
        "RAISEREGENABLEMENTMAX",
        "RAISEREGENABLEMENTMIN",
        "LOWERREGAVAILABILITY",
        "LOWERREGENABLEMENTMAX",
        "LOWERREGENABLEMENTMIN",
        "RAISE6SECACTUALAVAILABILITY",
        "RAISE60SECACTUALAVAILABILITY",
        "RAISE5MINACTUALAVAILABILITY",
        "RAISEREGACTUALAVAILABILITY",
        "LOWER6SECACTUALAVAILABILITY",
        "LOWER60SECACTUALAVAILABILITY",
        "LOWER5MINACTUALAVAILABILITY",
        "LOWERREGACTUALAVAILABILITY",
        "SEMIDISPATCHCAP",
        "DISPATCHMODETIME",
        "CONFORMANCE_MODE",
        "UIGF",
        "RAISE1SEC",
        "RAISE1SECFLAGS",
        "LOWER1SEC",
        "LOWER1SECFLAGS",
        "RAISE1SECACTUALAVAILABILITY",
        "LOWER1SECACTUALAVAILABILITY",
        "INITIAL_ENERGY_STORAGE",
        "ENERGY_STORAGE",
        "MIN_AVAILABILITY",
    ];
    type Row<'row> = DispatchUnitSolution5Row<'row>;
    type FieldMapping = DispatchUnitSolution5Mapping;
    type PrimaryKey = DispatchUnitSolution5PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchUnitSolution5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[2])?,
            tradetype: row
                .get_opt_custom_parsed_at_idx(
                    "tradetype",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchinterval: row
                .get_parsed_at_idx("dispatchinterval", field_mapping.0[4])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[6])?,
            dispatchmode: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchmode",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            agcstatus: row
                .get_opt_custom_parsed_at_idx(
                    "agcstatus",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initialmw: row
                .get_opt_custom_parsed_at_idx(
                    "initialmw",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalcleared: row
                .get_opt_custom_parsed_at_idx(
                    "totalcleared",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampdownrate: row
                .get_opt_custom_parsed_at_idx(
                    "rampdownrate",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampuprate: row
                .get_opt_custom_parsed_at_idx(
                    "rampuprate",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5min: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            downepf: row
                .get_opt_custom_parsed_at_idx(
                    "downepf",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            upepf: row
                .get_opt_custom_parsed_at_idx(
                    "upepf",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginal5minvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginal5minvalue",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginal60secvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginal60secvalue",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginal6secvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginal6secvalue",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violation5mindegree: row
                .get_opt_custom_parsed_at_idx(
                    "violation5mindegree",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violation60secdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violation60secdegree",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violation6secdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violation6secdegree",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violationdegree",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[29],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lowerreg: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availability: row
                .get_opt_custom_parsed_at_idx(
                    "availability",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secflags",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secflags",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minflags",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregflags: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregflags",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secflags",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secflags",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minflags",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregflags: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregflags",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregavailability",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregenablementmax: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregenablementmax",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregenablementmin: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregenablementmin",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregavailability",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregenablementmax: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregenablementmax",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregenablementmin: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregenablementmin",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secactualavailability",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secactualavailability",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minactualavailability",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregactualavailability",
                    field_mapping.0[50],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secactualavailability",
                    field_mapping.0[51],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secactualavailability",
                    field_mapping.0[52],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minactualavailability",
                    field_mapping.0[53],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregactualavailability",
                    field_mapping.0[54],
                    mmsdm_core::mms_decimal::parse,
                )?,
            semidispatchcap: row
                .get_opt_custom_parsed_at_idx(
                    "semidispatchcap",
                    field_mapping.0[55],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchmodetime: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchmodetime",
                    field_mapping.0[56],
                    mmsdm_core::mms_decimal::parse,
                )?,
            conformance_mode: row
                .get_opt_custom_parsed_at_idx(
                    "conformance_mode",
                    field_mapping.0[57],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uigf: row
                .get_opt_custom_parsed_at_idx(
                    "uigf",
                    field_mapping.0[58],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise1sec",
                    field_mapping.0[59],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secflags",
                    field_mapping.0[60],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower1sec",
                    field_mapping.0[61],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secflags",
                    field_mapping.0[62],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secactualavailability",
                    field_mapping.0[63],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secactualavailability",
                    field_mapping.0[64],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initial_energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "initial_energy_storage",
                    field_mapping.0[65],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "energy_storage",
                    field_mapping.0[66],
                    mmsdm_core::mms_decimal::parse,
                )?,
            min_availability: row
                .get_opt_custom_parsed_at_idx(
                    "min_availability",
                    field_mapping.0[67],
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
        Ok(DispatchUnitSolution5Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchUnitSolution5PrimaryKey {
        DispatchUnitSolution5PrimaryKey {
            duid: row.duid().to_string(),
            intervention: row.intervention,
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_unit_solution_v5_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchUnitSolution5Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            duid: row.duid.clone(),
            tradetype: row.tradetype.clone(),
            dispatchinterval: row.dispatchinterval.clone(),
            intervention: row.intervention.clone(),
            connectionpointid: row.connectionpointid.clone(),
            dispatchmode: row.dispatchmode.clone(),
            agcstatus: row.agcstatus.clone(),
            initialmw: row.initialmw.clone(),
            totalcleared: row.totalcleared.clone(),
            rampdownrate: row.rampdownrate.clone(),
            rampuprate: row.rampuprate.clone(),
            lower5min: row.lower5min.clone(),
            lower60sec: row.lower60sec.clone(),
            lower6sec: row.lower6sec.clone(),
            raise5min: row.raise5min.clone(),
            raise60sec: row.raise60sec.clone(),
            raise6sec: row.raise6sec.clone(),
            downepf: row.downepf.clone(),
            upepf: row.upepf.clone(),
            marginal5minvalue: row.marginal5minvalue.clone(),
            marginal60secvalue: row.marginal60secvalue.clone(),
            marginal6secvalue: row.marginal6secvalue.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violation5mindegree: row.violation5mindegree.clone(),
            violation60secdegree: row.violation60secdegree.clone(),
            violation6secdegree: row.violation6secdegree.clone(),
            violationdegree: row.violationdegree.clone(),
            lastchanged: row.lastchanged.clone(),
            lowerreg: row.lowerreg.clone(),
            raisereg: row.raisereg.clone(),
            availability: row.availability.clone(),
            raise6secflags: row.raise6secflags.clone(),
            raise60secflags: row.raise60secflags.clone(),
            raise5minflags: row.raise5minflags.clone(),
            raiseregflags: row.raiseregflags.clone(),
            lower6secflags: row.lower6secflags.clone(),
            lower60secflags: row.lower60secflags.clone(),
            lower5minflags: row.lower5minflags.clone(),
            lowerregflags: row.lowerregflags.clone(),
            raiseregavailability: row.raiseregavailability.clone(),
            raiseregenablementmax: row.raiseregenablementmax.clone(),
            raiseregenablementmin: row.raiseregenablementmin.clone(),
            lowerregavailability: row.lowerregavailability.clone(),
            lowerregenablementmax: row.lowerregenablementmax.clone(),
            lowerregenablementmin: row.lowerregenablementmin.clone(),
            raise6secactualavailability: row.raise6secactualavailability.clone(),
            raise60secactualavailability: row.raise60secactualavailability.clone(),
            raise5minactualavailability: row.raise5minactualavailability.clone(),
            raiseregactualavailability: row.raiseregactualavailability.clone(),
            lower6secactualavailability: row.lower6secactualavailability.clone(),
            lower60secactualavailability: row.lower60secactualavailability.clone(),
            lower5minactualavailability: row.lower5minactualavailability.clone(),
            lowerregactualavailability: row.lowerregactualavailability.clone(),
            semidispatchcap: row.semidispatchcap.clone(),
            dispatchmodetime: row.dispatchmodetime.clone(),
            conformance_mode: row.conformance_mode.clone(),
            uigf: row.uigf.clone(),
            raise1sec: row.raise1sec.clone(),
            raise1secflags: row.raise1secflags.clone(),
            lower1sec: row.lower1sec.clone(),
            lower1secflags: row.lower1secflags.clone(),
            raise1secactualavailability: row.raise1secactualavailability.clone(),
            lower1secactualavailability: row.lower1secactualavailability.clone(),
            initial_energy_storage: row.initial_energy_storage.clone(),
            energy_storage: row.energy_storage.clone(),
            min_availability: row.min_availability.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchUnitSolution5PrimaryKey {
    pub duid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchUnitSolution5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchUnitSolution5Row<'data> {
    type Row<'other> = DispatchUnitSolution5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.intervention == row.intervention
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchUnitSolution5Row<'data> {
    type PrimaryKey = DispatchUnitSolution5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchUnitSolution5PrimaryKey {
    type Row<'other> = DispatchUnitSolution5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.intervention == row.intervention
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchUnitSolution5PrimaryKey {
    type PrimaryKey = DispatchUnitSolution5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchUnitSolution5 {
    type Builder = DispatchUnitSolution5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tradetype",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchmode",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "agcstatus",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "initialmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalcleared",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rampdownrate",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rampuprate",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5min",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5min",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "downepf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "upepf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginal5minvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginal60secvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginal6secvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violation5mindegree",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violation60secdegree",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violation6secdegree",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violationdegree",
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
                    "lowerreg",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereg",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availability",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregavailability",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregenablementmax",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregenablementmin",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregavailability",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregenablementmax",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregenablementmin",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "semidispatchcap",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchmodetime",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "conformance_mode",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "uigf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "initial_energy_storage",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energy_storage",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "min_availability",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchUnitSolution5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            tradetype_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            dispatchmode_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            agcstatus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            initialmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalcleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rampdownrate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rampuprate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            downepf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            upepf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginal5minvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginal60secvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginal6secvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violation5mindegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violation60secdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violation6secdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lowerreg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raisereg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise60secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise5minflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raiseregflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower6secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower60secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower5minflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lowerregflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raiseregavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregenablementmax_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregenablementmin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregenablementmax_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregenablementmin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            raise60secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            raise5minactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            raiseregactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower6secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower60secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower5minactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lowerregactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            semidispatchcap_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            dispatchmodetime_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            conformance_mode_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower1sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise1secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower1secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            initial_energy_storage_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            energy_storage_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            min_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.duid_array.append_value(row.duid());
        builder
            .tradetype_array
            .append_option({
                row.tradetype
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .dispatchinterval_array
            .append_value(row.dispatchinterval.start().and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_option(row.connectionpointid());
        builder
            .dispatchmode_array
            .append_option({
                row.dispatchmode
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .agcstatus_array
            .append_option({
                row.agcstatus
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .initialmw_array
            .append_option({
                row.initialmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .totalcleared_array
            .append_option({
                row.totalcleared
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rampdownrate_array
            .append_option({
                row.rampdownrate
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rampuprate_array
            .append_option({
                row.rampuprate
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5min_array
            .append_option({
                row.lower5min
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60sec_array
            .append_option({
                row.lower60sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6sec_array
            .append_option({
                row.lower6sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5min_array
            .append_option({
                row.raise5min
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60sec_array
            .append_option({
                row.raise60sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6sec_array
            .append_option({
                row.raise6sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .downepf_array
            .append_option({
                row.downepf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .upepf_array
            .append_option({
                row.upepf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginal5minvalue_array
            .append_option({
                row.marginal5minvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginal60secvalue_array
            .append_option({
                row.marginal60secvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginal6secvalue_array
            .append_option({
                row.marginal6secvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginalvalue_array
            .append_option({
                row.marginalvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violation5mindegree_array
            .append_option({
                row.violation5mindegree
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violation60secdegree_array
            .append_option({
                row.violation60secdegree
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violation6secdegree_array
            .append_option({
                row.violation6secdegree
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violationdegree_array
            .append_option({
                row.violationdegree
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .lowerreg_array
            .append_option({
                row.lowerreg
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raisereg_array
            .append_option({
                row.raisereg
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .availability_array
            .append_option({
                row.availability
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secflags_array
            .append_option({
                row.raise6secflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise60secflags_array
            .append_option({
                row.raise60secflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise5minflags_array
            .append_option({
                row.raise5minflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raiseregflags_array
            .append_option({
                row.raiseregflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower6secflags_array
            .append_option({
                row.lower6secflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower60secflags_array
            .append_option({
                row.lower60secflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower5minflags_array
            .append_option({
                row.lower5minflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lowerregflags_array
            .append_option({
                row.lowerregflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raiseregavailability_array
            .append_option({
                row.raiseregavailability
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raiseregenablementmax_array
            .append_option({
                row.raiseregenablementmax
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raiseregenablementmin_array
            .append_option({
                row.raiseregenablementmin
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregavailability_array
            .append_option({
                row.lowerregavailability
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregenablementmax_array
            .append_option({
                row.lowerregenablementmax
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregenablementmin_array
            .append_option({
                row.lowerregenablementmin
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secactualavailability_array
            .append_option({
                row.raise6secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .raise60secactualavailability_array
            .append_option({
                row.raise60secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .raise5minactualavailability_array
            .append_option({
                row.raise5minactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .raiseregactualavailability_array
            .append_option({
                row.raiseregactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower6secactualavailability_array
            .append_option({
                row.lower6secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower60secactualavailability_array
            .append_option({
                row.lower60secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower5minactualavailability_array
            .append_option({
                row.lower5minactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lowerregactualavailability_array
            .append_option({
                row.lowerregactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .semidispatchcap_array
            .append_option({
                row.semidispatchcap
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .dispatchmodetime_array
            .append_option({
                row.dispatchmodetime
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .conformance_mode_array
            .append_option({
                row.conformance_mode
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .uigf_array
            .append_option({
                row.uigf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise1sec_array
            .append_option({
                row.raise1sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise1secflags_array
            .append_option({
                row.raise1secflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower1sec_array
            .append_option({
                row.lower1sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower1secflags_array
            .append_option({
                row.lower1secflags
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise1secactualavailability_array
            .append_option({
                row.raise1secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower1secactualavailability_array
            .append_option({
                row.lower1secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .initial_energy_storage_array
            .append_option({
                row.initial_energy_storage
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .energy_storage_array
            .append_option({
                row.energy_storage
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .min_availability_array
            .append_option({
                row.min_availability
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tradetype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchmode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.agcstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initialmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalcleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rampdownrate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rampuprate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.downepf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.upepf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginal5minvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginal60secvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginal6secvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violation5mindegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violation60secdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violation6secdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregenablementmax_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregenablementmin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregenablementmax_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregenablementmin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise6secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise60secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise5minactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raiseregactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower6secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower60secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower5minactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lowerregactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.semidispatchcap_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchmodetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.conformance_mode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise1secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower1secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initial_energy_storage_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energy_storage_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.min_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchUnitSolution5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    tradetype_array: arrow::array::builder::Decimal128Builder,
    dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    dispatchmode_array: arrow::array::builder::Decimal128Builder,
    agcstatus_array: arrow::array::builder::Decimal128Builder,
    initialmw_array: arrow::array::builder::Decimal128Builder,
    totalcleared_array: arrow::array::builder::Decimal128Builder,
    rampdownrate_array: arrow::array::builder::Decimal128Builder,
    rampuprate_array: arrow::array::builder::Decimal128Builder,
    lower5min_array: arrow::array::builder::Decimal128Builder,
    lower60sec_array: arrow::array::builder::Decimal128Builder,
    lower6sec_array: arrow::array::builder::Decimal128Builder,
    raise5min_array: arrow::array::builder::Decimal128Builder,
    raise60sec_array: arrow::array::builder::Decimal128Builder,
    raise6sec_array: arrow::array::builder::Decimal128Builder,
    downepf_array: arrow::array::builder::Decimal128Builder,
    upepf_array: arrow::array::builder::Decimal128Builder,
    marginal5minvalue_array: arrow::array::builder::Decimal128Builder,
    marginal60secvalue_array: arrow::array::builder::Decimal128Builder,
    marginal6secvalue_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violation5mindegree_array: arrow::array::builder::Decimal128Builder,
    violation60secdegree_array: arrow::array::builder::Decimal128Builder,
    violation6secdegree_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    lowerreg_array: arrow::array::builder::Decimal128Builder,
    raisereg_array: arrow::array::builder::Decimal128Builder,
    availability_array: arrow::array::builder::Decimal128Builder,
    raise6secflags_array: arrow::array::builder::Decimal128Builder,
    raise60secflags_array: arrow::array::builder::Decimal128Builder,
    raise5minflags_array: arrow::array::builder::Decimal128Builder,
    raiseregflags_array: arrow::array::builder::Decimal128Builder,
    lower6secflags_array: arrow::array::builder::Decimal128Builder,
    lower60secflags_array: arrow::array::builder::Decimal128Builder,
    lower5minflags_array: arrow::array::builder::Decimal128Builder,
    lowerregflags_array: arrow::array::builder::Decimal128Builder,
    raiseregavailability_array: arrow::array::builder::Decimal128Builder,
    raiseregenablementmax_array: arrow::array::builder::Decimal128Builder,
    raiseregenablementmin_array: arrow::array::builder::Decimal128Builder,
    lowerregavailability_array: arrow::array::builder::Decimal128Builder,
    lowerregenablementmax_array: arrow::array::builder::Decimal128Builder,
    lowerregenablementmin_array: arrow::array::builder::Decimal128Builder,
    raise6secactualavailability_array: arrow::array::builder::Decimal128Builder,
    raise60secactualavailability_array: arrow::array::builder::Decimal128Builder,
    raise5minactualavailability_array: arrow::array::builder::Decimal128Builder,
    raiseregactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower6secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower60secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower5minactualavailability_array: arrow::array::builder::Decimal128Builder,
    lowerregactualavailability_array: arrow::array::builder::Decimal128Builder,
    semidispatchcap_array: arrow::array::builder::Decimal128Builder,
    dispatchmodetime_array: arrow::array::builder::Decimal128Builder,
    conformance_mode_array: arrow::array::builder::Decimal128Builder,
    uigf_array: arrow::array::builder::Decimal128Builder,
    raise1sec_array: arrow::array::builder::Decimal128Builder,
    raise1secflags_array: arrow::array::builder::Decimal128Builder,
    lower1sec_array: arrow::array::builder::Decimal128Builder,
    lower1secflags_array: arrow::array::builder::Decimal128Builder,
    raise1secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower1secactualavailability_array: arrow::array::builder::Decimal128Builder,
    initial_energy_storage_array: arrow::array::builder::Decimal128Builder,
    energy_storage_array: arrow::array::builder::Decimal128Builder,
    min_availability_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchOffertrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchOffertrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchOffertrk1 {
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
pub struct DispatchOffertrk1Mapping([usize; 6]);
/// # Summary
///
/// ## DISPATCHOFFERTRK
///  _DISPATCHOFFERTRK is the energy and ancillary service bid tracking table for the Dispatch process. The table identifies which bids from BIDDAYOFFER and BIDOFFERPERIOD were applied for a given unit and bid type for each dispatch interval._
///
/// * Data Set Name: Dispatch
/// * File Name: Offertrk
/// * Data Version: 1
///
/// # Description
///  DISPATCHOFFERTRK  data is confidential to each participant until the next trading day, when the data is public to all participants.  Source DISPATCHOFFERTRK updates every 5 minutes. Volume Approximately 250,000 records per day.
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchOffertrk1Row<'data> {
    /// Date and time of the dispatch interval (e.g. five minute dispatch interval ending 28/09/2000 16:35)
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Bid type Identifier - the ancillary service to which the bid applies
    pub bidtype: core::ops::Range<usize>,
    /// Settlement date of bid applied
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    pub bidofferdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchOffertrk1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for DispatchOffertrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "OFFERTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchOffertrk1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "DUID",
        "BIDTYPE",
        "BIDSETTLEMENTDATE",
        "BIDOFFERDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = DispatchOffertrk1Row<'row>;
    type FieldMapping = DispatchOffertrk1Mapping;
    type PrimaryKey = DispatchOffertrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchOffertrk1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            bidtype: row.get_range("bidtype", field_mapping.0[2])?,
            bidsettlementdate: row
                .get_opt_custom_parsed_at_idx(
                    "bidsettlementdate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            bidofferdate: row
                .get_opt_custom_parsed_at_idx(
                    "bidofferdate",
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
        Ok(DispatchOffertrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchOffertrk1PrimaryKey {
        DispatchOffertrk1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            duid: row.duid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_offertrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchOffertrk1Row {
            settlementdate: row.settlementdate.clone(),
            duid: row.duid.clone(),
            bidtype: row.bidtype.clone(),
            bidsettlementdate: row.bidsettlementdate.clone(),
            bidofferdate: row.bidofferdate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchOffertrk1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub duid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchOffertrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchOffertrk1Row<'data> {
    type Row<'other> = DispatchOffertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.duid() == row.duid()
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchOffertrk1Row<'data> {
    type PrimaryKey = DispatchOffertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.duid() == key.duid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchOffertrk1PrimaryKey {
    type Row<'other> = DispatchOffertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.duid == row.duid()
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchOffertrk1PrimaryKey {
    type PrimaryKey = DispatchOffertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.duid == key.duid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchOffertrk1 {
    type Builder = DispatchOffertrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bidsettlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bidofferdate",
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
        DispatchOffertrk1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            bidofferdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .bidsettlementdate_array
            .append_option(
                row.bidsettlementdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .bidofferdate_array
            .append_option(row.bidofferdate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidsettlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidofferdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchOffertrk1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    bidofferdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DispatchPrice5 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchPrice5Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchPrice5 {
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
pub struct DispatchPrice5Mapping([usize; 66]);
/// # Summary
///
/// ## DISPATCHPRICE
///  _DISPATCHPRICE records 5 minute dispatch prices for energy and FCAS, including whether an intervention has occurred, or price override (e.g. for Administered Price Cap). DISPATCHPRICE updates when price adjustments occur, in which case the new price is written to the RRP field, and the old price to the ROP field as an audit trail._
///
/// * Data Set Name: Dispatch
/// * File Name: Price
/// * Data Version: 5
///
/// # Description
///  Source DISPATCHPRICE updates every 5 minutes. Note APCFLAG is a 5-bit Region-based field indicating that the original Dispatch Price (ROP) calculated by the Dispatch Algorithm for a region has undergone modification by one of more of the following processes: Bit Value Description 5 16 Price Scaling via Inter-regional Loss Factor (IRLF) 4 8 Price manually overwritten 3 4 MPC or MPF binding (ROP was outside of MPC/MPF) 2 2 VoLL Override applied 1 1 APC or APF binding (ROP was outside of APC/APF) Where: · MPC = Market Price Cap · MPF = Market Price Floor · APC = Administered Price Cap · APF = Administered Price Floor xxxAPCFLAGs are each a 5-bit Region-based field indicating FCAS price post-processing (where "ROP" is the original NEMDE Solver price): Bit Cum Value Description 5 16 Not applicable 4 8 Price manually overwritten 3 4 MPC ($VoLL) or MPF ($zero) binding (xxFCAS ROP was outside of MPC/MPF) 2 2 Not applicable 1 1 APC or APF binding (ROP was outside of APC/APF)
///
///
///
/// # Primary Key Columns
///
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchPrice5Row<'data> {
    /// Market date and time starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Dispatch interval identifier 001 to 288 in format YYYYMMDDPPP
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    /// Manual intervention flag
    pub intervention: rust_decimal::Decimal,
    /// Regional Reference Price for this dispatch period. RRP is the price used to settle the market
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price - no longer used
    pub eep: Option<rust_decimal::Decimal>,
    /// Regional Override Price, being the original price prior to any price scaling, price capping or VoLL override being applied. The APC flag allows the determination of whether capping, scaling or override occurred
    pub rop: Option<rust_decimal::Decimal>,
    /// APC Active flag (see note)
    pub apcflag: Option<rust_decimal::Decimal>,
    /// Market suspended flag
    pub marketsuspendedflag: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raiseregapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lowerregapcflag: Option<rust_decimal::Decimal>,
    /// Status of regional prices for this dispatch interval "NOT FIRM" or "FIRM"
    pub price_status: core::ops::Range<usize>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    /// Communicates the current OCD status for this dispatch interval.  Values of: 'NOT_OCD', 'OCD_UNRESOLVED', 'OCD_RESOLVED'.
    pub ocd_status: core::ops::Range<usize>,
    /// Communicates the current MII status for this dispatch interval.  Values of: 'NOT_MII', 'MII_SUBJECT_TO_REVIEW', 'MII_PRICE_REJECTED', 'MII_PRICE_ACCEPTED'.
    pub mii_status: core::ops::Range<usize>,
    /// Regional Raise 1Sec Price - R1Price attribute after capping/flooring
    pub raise1secrrp: Option<rust_decimal::Decimal>,
    /// Raise1Sec Regional Original Price - uncapped/unfloored and unscaled
    pub raise1secrop: Option<rust_decimal::Decimal>,
    /// BitFlag field for Price adjustments - "1" = Voll_Override; "4" = Floor_VoLL; "8" = Manual_Override; "16" = Price_Scaled
    pub raise1secapcflag: Option<rust_decimal::Decimal>,
    /// Regional Lower 1Sec Price - RegionSolution element L1Price attribute
    pub lower1secrrp: Option<rust_decimal::Decimal>,
    /// Lower1Sec Regional Original Price - uncapped/unfloored and unscaled
    pub lower1secrop: Option<rust_decimal::Decimal>,
    /// BitFlag field for Price adjustments - "1" = Voll_Override; "4" = Floor_VoLL; "8" = Manual_Override; "16" = Price_Scaled
    pub lower1secapcflag: Option<rust_decimal::Decimal>,
    /// Price before AP capping or scaling - for Rolling Sum Price monitoring
    pub pre_ap_raise1_price: Option<rust_decimal::Decimal>,
    /// Price before AP capping or scaling - for Rolling Sum Price monitoring
    pub pre_ap_lower1_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise1_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower1_price: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchPrice5Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn price_status(&self) -> Option<&str> {
        if self.price_status.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.price_status.clone(),
                ),
            )
        }
    }
    pub fn ocd_status(&self) -> Option<&str> {
        if self.ocd_status.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.ocd_status.clone(),
                ),
            )
        }
    }
    pub fn mii_status(&self) -> Option<&str> {
        if self.mii_status.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.mii_status.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchPrice5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "PRICE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchPrice5Mapping([
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
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
        49,
        50,
        51,
        52,
        53,
        54,
        55,
        56,
        57,
        58,
        59,
        60,
        61,
        62,
        63,
        64,
        65,
        66,
        67,
        68,
        69,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "REGIONID",
        "DISPATCHINTERVAL",
        "INTERVENTION",
        "RRP",
        "EEP",
        "ROP",
        "APCFLAG",
        "MARKETSUSPENDEDFLAG",
        "LASTCHANGED",
        "RAISE6SECRRP",
        "RAISE6SECROP",
        "RAISE6SECAPCFLAG",
        "RAISE60SECRRP",
        "RAISE60SECROP",
        "RAISE60SECAPCFLAG",
        "RAISE5MINRRP",
        "RAISE5MINROP",
        "RAISE5MINAPCFLAG",
        "RAISEREGRRP",
        "RAISEREGROP",
        "RAISEREGAPCFLAG",
        "LOWER6SECRRP",
        "LOWER6SECROP",
        "LOWER6SECAPCFLAG",
        "LOWER60SECRRP",
        "LOWER60SECROP",
        "LOWER60SECAPCFLAG",
        "LOWER5MINRRP",
        "LOWER5MINROP",
        "LOWER5MINAPCFLAG",
        "LOWERREGRRP",
        "LOWERREGROP",
        "LOWERREGAPCFLAG",
        "PRICE_STATUS",
        "PRE_AP_ENERGY_PRICE",
        "PRE_AP_RAISE6_PRICE",
        "PRE_AP_RAISE60_PRICE",
        "PRE_AP_RAISE5MIN_PRICE",
        "PRE_AP_RAISEREG_PRICE",
        "PRE_AP_LOWER6_PRICE",
        "PRE_AP_LOWER60_PRICE",
        "PRE_AP_LOWER5MIN_PRICE",
        "PRE_AP_LOWERREG_PRICE",
        "CUMUL_PRE_AP_ENERGY_PRICE",
        "CUMUL_PRE_AP_RAISE6_PRICE",
        "CUMUL_PRE_AP_RAISE60_PRICE",
        "CUMUL_PRE_AP_RAISE5MIN_PRICE",
        "CUMUL_PRE_AP_RAISEREG_PRICE",
        "CUMUL_PRE_AP_LOWER6_PRICE",
        "CUMUL_PRE_AP_LOWER60_PRICE",
        "CUMUL_PRE_AP_LOWER5MIN_PRICE",
        "CUMUL_PRE_AP_LOWERREG_PRICE",
        "OCD_STATUS",
        "MII_STATUS",
        "RAISE1SECRRP",
        "RAISE1SECROP",
        "RAISE1SECAPCFLAG",
        "LOWER1SECRRP",
        "LOWER1SECROP",
        "LOWER1SECAPCFLAG",
        "PRE_AP_RAISE1_PRICE",
        "PRE_AP_LOWER1_PRICE",
        "CUMUL_PRE_AP_RAISE1_PRICE",
        "CUMUL_PRE_AP_LOWER1_PRICE",
    ];
    type Row<'row> = DispatchPrice5Row<'row>;
    type FieldMapping = DispatchPrice5Mapping;
    type PrimaryKey = DispatchPrice5PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchPrice5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            dispatchinterval: row
                .get_parsed_at_idx("dispatchinterval", field_mapping.0[3])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep: row
                .get_opt_custom_parsed_at_idx(
                    "eep",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rop: row
                .get_opt_custom_parsed_at_idx(
                    "rop",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apcflag: row
                .get_opt_custom_parsed_at_idx(
                    "apcflag",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marketsuspendedflag: row
                .get_opt_custom_parsed_at_idx(
                    "marketsuspendedflag",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            raise6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secrrp",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secrop",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secapcflag",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secrrp",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secrop",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secapcflag",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minrrp",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minrop",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minapcflag",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregrrp",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregrop: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregrop",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregapcflag",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secrrp",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secrop",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secapcflag",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secrrp",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secrop",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secapcflag",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minrrp",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minrop",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minapcflag",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregrrp",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregrop: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregrop",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregapcflag",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            price_status: row.get_opt_range("price_status", field_mapping.0[35])?,
            pre_ap_energy_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_energy_price",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_raise6_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_raise6_price",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_raise60_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_raise60_price",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_raise5min_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_raise5min_price",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_raisereg_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_raisereg_price",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_lower6_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_lower6_price",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_lower60_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_lower60_price",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_lower5min_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_lower5min_price",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_lowerreg_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_lowerreg_price",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_energy_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_energy_price",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_raise6_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_raise6_price",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_raise60_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_raise60_price",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_raise5min_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_raise5min_price",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_raisereg_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_raisereg_price",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_lower6_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_lower6_price",
                    field_mapping.0[50],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_lower60_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_lower60_price",
                    field_mapping.0[51],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_lower5min_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_lower5min_price",
                    field_mapping.0[52],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_lowerreg_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_lowerreg_price",
                    field_mapping.0[53],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ocd_status: row.get_opt_range("ocd_status", field_mapping.0[54])?,
            mii_status: row.get_opt_range("mii_status", field_mapping.0[55])?,
            raise1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secrrp",
                    field_mapping.0[56],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secrop",
                    field_mapping.0[57],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secapcflag",
                    field_mapping.0[58],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secrrp",
                    field_mapping.0[59],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secrop",
                    field_mapping.0[60],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secapcflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secapcflag",
                    field_mapping.0[61],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_raise1_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_raise1_price",
                    field_mapping.0[62],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pre_ap_lower1_price: row
                .get_opt_custom_parsed_at_idx(
                    "pre_ap_lower1_price",
                    field_mapping.0[63],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_raise1_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_raise1_price",
                    field_mapping.0[64],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_pre_ap_lower1_price: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_pre_ap_lower1_price",
                    field_mapping.0[65],
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
        Ok(DispatchPrice5Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchPrice5PrimaryKey {
        DispatchPrice5PrimaryKey {
            dispatchinterval: row.dispatchinterval,
            intervention: row.intervention,
            regionid: row.regionid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_price_v5_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchPrice5Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            regionid: row.regionid.clone(),
            dispatchinterval: row.dispatchinterval.clone(),
            intervention: row.intervention.clone(),
            rrp: row.rrp.clone(),
            eep: row.eep.clone(),
            rop: row.rop.clone(),
            apcflag: row.apcflag.clone(),
            marketsuspendedflag: row.marketsuspendedflag.clone(),
            lastchanged: row.lastchanged.clone(),
            raise6secrrp: row.raise6secrrp.clone(),
            raise6secrop: row.raise6secrop.clone(),
            raise6secapcflag: row.raise6secapcflag.clone(),
            raise60secrrp: row.raise60secrrp.clone(),
            raise60secrop: row.raise60secrop.clone(),
            raise60secapcflag: row.raise60secapcflag.clone(),
            raise5minrrp: row.raise5minrrp.clone(),
            raise5minrop: row.raise5minrop.clone(),
            raise5minapcflag: row.raise5minapcflag.clone(),
            raiseregrrp: row.raiseregrrp.clone(),
            raiseregrop: row.raiseregrop.clone(),
            raiseregapcflag: row.raiseregapcflag.clone(),
            lower6secrrp: row.lower6secrrp.clone(),
            lower6secrop: row.lower6secrop.clone(),
            lower6secapcflag: row.lower6secapcflag.clone(),
            lower60secrrp: row.lower60secrrp.clone(),
            lower60secrop: row.lower60secrop.clone(),
            lower60secapcflag: row.lower60secapcflag.clone(),
            lower5minrrp: row.lower5minrrp.clone(),
            lower5minrop: row.lower5minrop.clone(),
            lower5minapcflag: row.lower5minapcflag.clone(),
            lowerregrrp: row.lowerregrrp.clone(),
            lowerregrop: row.lowerregrop.clone(),
            lowerregapcflag: row.lowerregapcflag.clone(),
            price_status: row.price_status.clone(),
            pre_ap_energy_price: row.pre_ap_energy_price.clone(),
            pre_ap_raise6_price: row.pre_ap_raise6_price.clone(),
            pre_ap_raise60_price: row.pre_ap_raise60_price.clone(),
            pre_ap_raise5min_price: row.pre_ap_raise5min_price.clone(),
            pre_ap_raisereg_price: row.pre_ap_raisereg_price.clone(),
            pre_ap_lower6_price: row.pre_ap_lower6_price.clone(),
            pre_ap_lower60_price: row.pre_ap_lower60_price.clone(),
            pre_ap_lower5min_price: row.pre_ap_lower5min_price.clone(),
            pre_ap_lowerreg_price: row.pre_ap_lowerreg_price.clone(),
            cumul_pre_ap_energy_price: row.cumul_pre_ap_energy_price.clone(),
            cumul_pre_ap_raise6_price: row.cumul_pre_ap_raise6_price.clone(),
            cumul_pre_ap_raise60_price: row.cumul_pre_ap_raise60_price.clone(),
            cumul_pre_ap_raise5min_price: row.cumul_pre_ap_raise5min_price.clone(),
            cumul_pre_ap_raisereg_price: row.cumul_pre_ap_raisereg_price.clone(),
            cumul_pre_ap_lower6_price: row.cumul_pre_ap_lower6_price.clone(),
            cumul_pre_ap_lower60_price: row.cumul_pre_ap_lower60_price.clone(),
            cumul_pre_ap_lower5min_price: row.cumul_pre_ap_lower5min_price.clone(),
            cumul_pre_ap_lowerreg_price: row.cumul_pre_ap_lowerreg_price.clone(),
            ocd_status: row.ocd_status.clone(),
            mii_status: row.mii_status.clone(),
            raise1secrrp: row.raise1secrrp.clone(),
            raise1secrop: row.raise1secrop.clone(),
            raise1secapcflag: row.raise1secapcflag.clone(),
            lower1secrrp: row.lower1secrrp.clone(),
            lower1secrop: row.lower1secrop.clone(),
            lower1secapcflag: row.lower1secapcflag.clone(),
            pre_ap_raise1_price: row.pre_ap_raise1_price.clone(),
            pre_ap_lower1_price: row.pre_ap_lower1_price.clone(),
            cumul_pre_ap_raise1_price: row.cumul_pre_ap_raise1_price.clone(),
            cumul_pre_ap_lower1_price: row.cumul_pre_ap_lower1_price.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchPrice5PrimaryKey {
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    pub intervention: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchPrice5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchPrice5Row<'data> {
    type Row<'other> = DispatchPrice5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention && self.regionid() == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchPrice5Row<'data> {
    type PrimaryKey = DispatchPrice5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention && self.regionid() == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchPrice5PrimaryKey {
    type Row<'other> = DispatchPrice5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention && self.regionid == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchPrice5PrimaryKey {
    type PrimaryKey = DispatchPrice5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchPrice5 {
    type Builder = DispatchPrice5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "dispatchinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marketsuspendedflag",
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
                arrow::datatypes::Field::new(
                    "raise6secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "price_status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_energy_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_raise6_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_raise60_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_raise5min_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_raisereg_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_lower6_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_lower60_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_lower5min_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_lowerreg_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_energy_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_raise6_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_raise60_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_raise5min_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_raisereg_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_lower6_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_lower60_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_lower5min_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_lowerreg_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ocd_status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mii_status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secrop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secapcflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_raise1_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pre_ap_lower1_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_raise1_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_pre_ap_lower1_price",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchPrice5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            apcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            marketsuspendedflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            raise6secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise60secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise5minrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raiseregrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower6secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower60secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower5minrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lowerregrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            price_status_array: arrow::array::builder::StringBuilder::new(),
            pre_ap_energy_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_raise6_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_raise60_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_raise5min_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_raisereg_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_lower6_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_lower60_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_lower5min_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_lowerreg_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_energy_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_raise6_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_raise60_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_raise5min_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_raisereg_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_lower6_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_lower60_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_lower5min_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_lowerreg_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ocd_status_array: arrow::array::builder::StringBuilder::new(),
            mii_status_array: arrow::array::builder::StringBuilder::new(),
            raise1secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1secapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower1secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1secapcflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            pre_ap_raise1_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            pre_ap_lower1_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_raise1_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_pre_ap_lower1_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .dispatchinterval_array
            .append_value(row.dispatchinterval.start().and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep_array
            .append_option({
                row.eep
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rop_array
            .append_option({
                row.rop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .apcflag_array
            .append_option({
                row.apcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .marketsuspendedflag_array
            .append_option({
                row.marketsuspendedflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .raise6secrrp_array
            .append_option({
                row.raise6secrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secrop_array
            .append_option({
                row.raise6secrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secapcflag_array
            .append_option({
                row.raise6secapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise60secrrp_array
            .append_option({
                row.raise60secrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secrop_array
            .append_option({
                row.raise60secrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secapcflag_array
            .append_option({
                row.raise60secapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise5minrrp_array
            .append_option({
                row.raise5minrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minrop_array
            .append_option({
                row.raise5minrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minapcflag_array
            .append_option({
                row.raise5minapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raiseregrrp_array
            .append_option({
                row.raiseregrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raiseregrop_array
            .append_option({
                row.raiseregrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raiseregapcflag_array
            .append_option({
                row.raiseregapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower6secrrp_array
            .append_option({
                row.lower6secrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secrop_array
            .append_option({
                row.lower6secrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secapcflag_array
            .append_option({
                row.lower6secapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower60secrrp_array
            .append_option({
                row.lower60secrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secrop_array
            .append_option({
                row.lower60secrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secapcflag_array
            .append_option({
                row.lower60secapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower5minrrp_array
            .append_option({
                row.lower5minrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minrop_array
            .append_option({
                row.lower5minrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minapcflag_array
            .append_option({
                row.lower5minapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lowerregrrp_array
            .append_option({
                row.lowerregrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregrop_array
            .append_option({
                row.lowerregrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregapcflag_array
            .append_option({
                row.lowerregapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.price_status_array.append_option(row.price_status());
        builder
            .pre_ap_energy_price_array
            .append_option({
                row.pre_ap_energy_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_raise6_price_array
            .append_option({
                row.pre_ap_raise6_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_raise60_price_array
            .append_option({
                row.pre_ap_raise60_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_raise5min_price_array
            .append_option({
                row.pre_ap_raise5min_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_raisereg_price_array
            .append_option({
                row.pre_ap_raisereg_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_lower6_price_array
            .append_option({
                row.pre_ap_lower6_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_lower60_price_array
            .append_option({
                row.pre_ap_lower60_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_lower5min_price_array
            .append_option({
                row.pre_ap_lower5min_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_lowerreg_price_array
            .append_option({
                row.pre_ap_lowerreg_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_energy_price_array
            .append_option({
                row.cumul_pre_ap_energy_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_raise6_price_array
            .append_option({
                row.cumul_pre_ap_raise6_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_raise60_price_array
            .append_option({
                row.cumul_pre_ap_raise60_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_raise5min_price_array
            .append_option({
                row.cumul_pre_ap_raise5min_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_raisereg_price_array
            .append_option({
                row.cumul_pre_ap_raisereg_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_lower6_price_array
            .append_option({
                row.cumul_pre_ap_lower6_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_lower60_price_array
            .append_option({
                row.cumul_pre_ap_lower60_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_lower5min_price_array
            .append_option({
                row.cumul_pre_ap_lower5min_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_lowerreg_price_array
            .append_option({
                row.cumul_pre_ap_lowerreg_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder.ocd_status_array.append_option(row.ocd_status());
        builder.mii_status_array.append_option(row.mii_status());
        builder
            .raise1secrrp_array
            .append_option({
                row.raise1secrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise1secrop_array
            .append_option({
                row.raise1secrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise1secapcflag_array
            .append_option({
                row.raise1secapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower1secrrp_array
            .append_option({
                row.lower1secrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower1secrop_array
            .append_option({
                row.lower1secrop
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower1secapcflag_array
            .append_option({
                row.lower1secapcflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_raise1_price_array
            .append_option({
                row.pre_ap_raise1_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .pre_ap_lower1_price_array
            .append_option({
                row.pre_ap_lower1_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_raise1_price_array
            .append_option({
                row.cumul_pre_ap_raise1_price
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_pre_ap_lower1_price_array
            .append_option({
                row.cumul_pre_ap_lower1_price
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketsuspendedflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.price_status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_energy_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_raise6_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_raise60_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_raise5min_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_raisereg_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_lower6_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_lower60_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_lower5min_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_lowerreg_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_energy_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_raise6_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_raise60_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_raise5min_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_raisereg_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_lower6_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_lower60_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_lower5min_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_lowerreg_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ocd_status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mii_status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1secapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secapcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_raise1_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pre_ap_lower1_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_raise1_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_pre_ap_lower1_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchPrice5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    eep_array: arrow::array::builder::Decimal128Builder,
    rop_array: arrow::array::builder::Decimal128Builder,
    apcflag_array: arrow::array::builder::Decimal128Builder,
    marketsuspendedflag_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    raise6secrrp_array: arrow::array::builder::Decimal128Builder,
    raise6secrop_array: arrow::array::builder::Decimal128Builder,
    raise6secapcflag_array: arrow::array::builder::Decimal128Builder,
    raise60secrrp_array: arrow::array::builder::Decimal128Builder,
    raise60secrop_array: arrow::array::builder::Decimal128Builder,
    raise60secapcflag_array: arrow::array::builder::Decimal128Builder,
    raise5minrrp_array: arrow::array::builder::Decimal128Builder,
    raise5minrop_array: arrow::array::builder::Decimal128Builder,
    raise5minapcflag_array: arrow::array::builder::Decimal128Builder,
    raiseregrrp_array: arrow::array::builder::Decimal128Builder,
    raiseregrop_array: arrow::array::builder::Decimal128Builder,
    raiseregapcflag_array: arrow::array::builder::Decimal128Builder,
    lower6secrrp_array: arrow::array::builder::Decimal128Builder,
    lower6secrop_array: arrow::array::builder::Decimal128Builder,
    lower6secapcflag_array: arrow::array::builder::Decimal128Builder,
    lower60secrrp_array: arrow::array::builder::Decimal128Builder,
    lower60secrop_array: arrow::array::builder::Decimal128Builder,
    lower60secapcflag_array: arrow::array::builder::Decimal128Builder,
    lower5minrrp_array: arrow::array::builder::Decimal128Builder,
    lower5minrop_array: arrow::array::builder::Decimal128Builder,
    lower5minapcflag_array: arrow::array::builder::Decimal128Builder,
    lowerregrrp_array: arrow::array::builder::Decimal128Builder,
    lowerregrop_array: arrow::array::builder::Decimal128Builder,
    lowerregapcflag_array: arrow::array::builder::Decimal128Builder,
    price_status_array: arrow::array::builder::StringBuilder,
    pre_ap_energy_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_raise6_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_raise60_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_raise5min_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_raisereg_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_lower6_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_lower60_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_lower5min_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_lowerreg_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_energy_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_raise6_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_raise60_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_raise5min_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_raisereg_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_lower6_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_lower60_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_lower5min_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_lowerreg_price_array: arrow::array::builder::Decimal128Builder,
    ocd_status_array: arrow::array::builder::StringBuilder,
    mii_status_array: arrow::array::builder::StringBuilder,
    raise1secrrp_array: arrow::array::builder::Decimal128Builder,
    raise1secrop_array: arrow::array::builder::Decimal128Builder,
    raise1secapcflag_array: arrow::array::builder::Decimal128Builder,
    lower1secrrp_array: arrow::array::builder::Decimal128Builder,
    lower1secrop_array: arrow::array::builder::Decimal128Builder,
    lower1secapcflag_array: arrow::array::builder::Decimal128Builder,
    pre_ap_raise1_price_array: arrow::array::builder::Decimal128Builder,
    pre_ap_lower1_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_raise1_price_array: arrow::array::builder::Decimal128Builder,
    cumul_pre_ap_lower1_price_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchRegionsum8 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchRegionsum8Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchRegionsum8 {
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
pub struct DispatchRegionsum8Mapping([usize; 125]);
/// # Summary
///
/// ## DISPATCHREGIONSUM
///  _DISPATCHREGIONSUM sets out the 5-minute solution for each dispatch run for each region, including the Frequency Control Ancillary Services (FCAS) services provided. Additional fields are for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations._
///
/// * Data Set Name: Dispatch
/// * File Name: Regionsum
/// * Data Version: 8
///
/// # Description
///  DISPATCHREGIONSUM is public data, and is available to all participants. Source DISPATCHREGIONSUM updates every 5 minutes. Note For details of calculations about load calculations, refer to Chapter 3 of the "Statement of Opportunities" *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values. From 16 February 2006, the old reserve values are no longer populated (i.e. are null), being LORSurplus and LRCSurplus. For more details on the changes to Reporting of Reserve Condition Data, refer to AEMO Communication 2042. For the best available indicator of reserve condition in each of the regions of the NEM for each trading interval, refer to the latest run of the Pre-Dispatch PASA (see table PDPASA_REGIONSOLUTION).
///
///
///
/// # Primary Key Columns
///
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchRegionsum8Row<'data> {
    /// Market date and time starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    /// Manual Intervention flag
    pub intervention: rust_decimal::Decimal,
    /// Demand (less loads)
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    pub availableload: Option<rust_decimal::Decimal>,
    /// 5 minute forecast adjust
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Dispatched Generation
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Dispatched Load (add to total demand to get inherent region demand).
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// MW quantity of excess
    pub excessgeneration: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    pub lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    pub lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    pub lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 5 min
    pub lower5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    pub lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 5 min
    pub lower5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    pub lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 5 min
    pub lower5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    pub lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    pub lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    pub lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 60 sec
    pub lower60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    pub lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 60 sec
    pub lower60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    pub lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 60 sec
    pub lower60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    pub lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    pub lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    pub lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 6 sec
    pub lower6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    pub lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 6 sec
    pub lower6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    pub lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 6 sec
    pub lower6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW dispatch
    pub raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise price of lower 5 min
    pub raise5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    pub raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 5 min
    pub raise5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    pub raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 5 min
    pub raise5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    pub raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    pub raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 60 sec local dispatch
    pub raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 60 sec
    pub raise60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    pub raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 60 sec
    pub raise60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec total requirement
    pub raise60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 60 sec
    pub raise60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW dispatch
    pub raise6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW imported
    pub raise6secimport: Option<rust_decimal::Decimal>,
    /// Raise 6 sec local dispatch
    pub raise6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 6 sec
    pub raise6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec local requirement
    pub raise6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 6 sec
    pub raise6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec total requirement
    pub raise6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 6 sec
    pub raise6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Calculated dispatch error
    pub aggegatedispatcherror: Option<rust_decimal::Decimal>,
    /// Calculated dispatch error
    pub aggregatedispatcherror: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of initial generation and import for region
    pub initialsupply: Option<rust_decimal::Decimal>,
    /// Sum of cleared generation and import for region
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation MW imported
    pub lowerregimport: Option<rust_decimal::Decimal>,
    /// Lower Regulation local dispatch
    pub lowerreglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation local requirement
    pub lowerreglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation total requirement
    pub lowerregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation MW imported
    pub raiseregimport: Option<rust_decimal::Decimal>,
    /// Raise Regulation local dispatch
    pub raisereglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation local requirement
    pub raisereglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation total requirement
    pub raiseregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min local requirement
    pub raise5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg local requirement
    pub raisereglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 sec local requirement
    pub raise60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 sec local requirement
    pub raise6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min local requirement
    pub lower5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg local requirement
    pub lowerreglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 sec local requirement
    pub lower60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 sec local requirement
    pub lower6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min requirement
    pub raise5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg requirement
    pub raiseregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 seconds requirement
    pub raise60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 seconds requirement
    pub raise6secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min requirement
    pub lower5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg requirement
    pub lowerregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 seconds requirement
    pub lower60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 seconds requirement
    pub lower6secviolation: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    pub raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    pub raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    pub raise5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise reg availability
    pub raiseregactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 6sec availability
    pub lower6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 60sec availability
    pub lower60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 5min availability
    pub lower5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower reg availability
    pub lowerregactualavailability: Option<rust_decimal::Decimal>,
    /// Not in use after 17 Feb 2006. Total short term generation capacity reserve used in assessing lack of reserve condition
    pub lorsurplus: Option<rust_decimal::Decimal>,
    /// Not in use after 17 Feb 2006. Total short term generation capacity reserve above the stated low reserve condition requirement
    pub lrcsurplus: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW
    pub semischedule_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced
    pub semischedule_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is solar
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is wind
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is solar
    pub ss_solar_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is wind
    pub ss_wind_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is solar
    pub ss_solar_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is wind
    pub ss_wind_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated MW value at start of interval for Wholesale Demand Response (WDR) units
    pub wdr_initialmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated available MW for Wholesale Demand Response (WDR) units
    pub wdr_available: Option<rust_decimal::Decimal>,
    /// Regional aggregated dispatched MW for Wholesale Demand Response (WDR) units
    pub wdr_dispatched: Option<rust_decimal::Decimal>,
    /// For Semi-Scheduled units. Aggregate Energy Availability from Solar units in that region
    pub ss_solar_availability: Option<rust_decimal::Decimal>,
    /// For Semi-Scheduled units. Aggregate Energy Availability from Wind units in that region
    pub ss_wind_availability: Option<rust_decimal::Decimal>,
    /// Total Raise1Sec Dispatched in Region - RegionSolution element R1Dispatch attribute
    pub raise1seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Total Lower1Sec Dispatched in Region - RegionSolution element L1Dispatch attribute
    pub lower1seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Trapezium adjusted Raise1Sec availability (summated from UnitSolution)
    pub raise1secactualavailability: Option<rust_decimal::Decimal>,
    /// Trapezium adjusted Lower1Sec availability (summated from UnitSolution)
    pub lower1secactualavailability: Option<rust_decimal::Decimal>,
    /// Regional aggregated energy storage where the DUID type is BDU (MWh)
    pub bdu_energy_storage: Option<rust_decimal::Decimal>,
    /// Total available load side BDU summated for region (MW)
    pub bdu_min_avail: Option<rust_decimal::Decimal>,
    /// Total available generation side BDU summated for region (MW)
    pub bdu_max_avail: Option<rust_decimal::Decimal>,
    /// Regional aggregated cleared MW where the DUID type is BDU. Net of export (Generation)
    pub bdu_clearedmw_gen: Option<rust_decimal::Decimal>,
    /// Regional aggregated cleared MW where the DUID type is BDU. Net of import (Load)
    pub bdu_clearedmw_load: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchRegionsum8Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchRegionsum8 {
    const VERSION: i32 = 8;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "REGIONSUM";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchRegionsum8Mapping([
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
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
        49,
        50,
        51,
        52,
        53,
        54,
        55,
        56,
        57,
        58,
        59,
        60,
        61,
        62,
        63,
        64,
        65,
        66,
        67,
        68,
        69,
        70,
        71,
        72,
        73,
        74,
        75,
        76,
        77,
        78,
        79,
        80,
        81,
        82,
        83,
        84,
        85,
        86,
        87,
        88,
        89,
        90,
        91,
        92,
        93,
        94,
        95,
        96,
        97,
        98,
        99,
        100,
        101,
        102,
        103,
        104,
        105,
        106,
        107,
        108,
        109,
        110,
        111,
        112,
        113,
        114,
        115,
        116,
        117,
        118,
        119,
        120,
        121,
        122,
        123,
        124,
        125,
        126,
        127,
        128,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "REGIONID",
        "DISPATCHINTERVAL",
        "INTERVENTION",
        "TOTALDEMAND",
        "AVAILABLEGENERATION",
        "AVAILABLELOAD",
        "DEMANDFORECAST",
        "DISPATCHABLEGENERATION",
        "DISPATCHABLELOAD",
        "NETINTERCHANGE",
        "EXCESSGENERATION",
        "LOWER5MINDISPATCH",
        "LOWER5MINIMPORT",
        "LOWER5MINLOCALDISPATCH",
        "LOWER5MINLOCALPRICE",
        "LOWER5MINLOCALREQ",
        "LOWER5MINPRICE",
        "LOWER5MINREQ",
        "LOWER5MINSUPPLYPRICE",
        "LOWER60SECDISPATCH",
        "LOWER60SECIMPORT",
        "LOWER60SECLOCALDISPATCH",
        "LOWER60SECLOCALPRICE",
        "LOWER60SECLOCALREQ",
        "LOWER60SECPRICE",
        "LOWER60SECREQ",
        "LOWER60SECSUPPLYPRICE",
        "LOWER6SECDISPATCH",
        "LOWER6SECIMPORT",
        "LOWER6SECLOCALDISPATCH",
        "LOWER6SECLOCALPRICE",
        "LOWER6SECLOCALREQ",
        "LOWER6SECPRICE",
        "LOWER6SECREQ",
        "LOWER6SECSUPPLYPRICE",
        "RAISE5MINDISPATCH",
        "RAISE5MINIMPORT",
        "RAISE5MINLOCALDISPATCH",
        "RAISE5MINLOCALPRICE",
        "RAISE5MINLOCALREQ",
        "RAISE5MINPRICE",
        "RAISE5MINREQ",
        "RAISE5MINSUPPLYPRICE",
        "RAISE60SECDISPATCH",
        "RAISE60SECIMPORT",
        "RAISE60SECLOCALDISPATCH",
        "RAISE60SECLOCALPRICE",
        "RAISE60SECLOCALREQ",
        "RAISE60SECPRICE",
        "RAISE60SECREQ",
        "RAISE60SECSUPPLYPRICE",
        "RAISE6SECDISPATCH",
        "RAISE6SECIMPORT",
        "RAISE6SECLOCALDISPATCH",
        "RAISE6SECLOCALPRICE",
        "RAISE6SECLOCALREQ",
        "RAISE6SECPRICE",
        "RAISE6SECREQ",
        "RAISE6SECSUPPLYPRICE",
        "AGGEGATEDISPATCHERROR",
        "AGGREGATEDISPATCHERROR",
        "LASTCHANGED",
        "INITIALSUPPLY",
        "CLEAREDSUPPLY",
        "LOWERREGIMPORT",
        "LOWERREGLOCALDISPATCH",
        "LOWERREGLOCALREQ",
        "LOWERREGREQ",
        "RAISEREGIMPORT",
        "RAISEREGLOCALDISPATCH",
        "RAISEREGLOCALREQ",
        "RAISEREGREQ",
        "RAISE5MINLOCALVIOLATION",
        "RAISEREGLOCALVIOLATION",
        "RAISE60SECLOCALVIOLATION",
        "RAISE6SECLOCALVIOLATION",
        "LOWER5MINLOCALVIOLATION",
        "LOWERREGLOCALVIOLATION",
        "LOWER60SECLOCALVIOLATION",
        "LOWER6SECLOCALVIOLATION",
        "RAISE5MINVIOLATION",
        "RAISEREGVIOLATION",
        "RAISE60SECVIOLATION",
        "RAISE6SECVIOLATION",
        "LOWER5MINVIOLATION",
        "LOWERREGVIOLATION",
        "LOWER60SECVIOLATION",
        "LOWER6SECVIOLATION",
        "RAISE6SECACTUALAVAILABILITY",
        "RAISE60SECACTUALAVAILABILITY",
        "RAISE5MINACTUALAVAILABILITY",
        "RAISEREGACTUALAVAILABILITY",
        "LOWER6SECACTUALAVAILABILITY",
        "LOWER60SECACTUALAVAILABILITY",
        "LOWER5MINACTUALAVAILABILITY",
        "LOWERREGACTUALAVAILABILITY",
        "LORSURPLUS",
        "LRCSURPLUS",
        "TOTALINTERMITTENTGENERATION",
        "DEMAND_AND_NONSCHEDGEN",
        "UIGF",
        "SEMISCHEDULE_CLEAREDMW",
        "SEMISCHEDULE_COMPLIANCEMW",
        "SS_SOLAR_UIGF",
        "SS_WIND_UIGF",
        "SS_SOLAR_CLEAREDMW",
        "SS_WIND_CLEAREDMW",
        "SS_SOLAR_COMPLIANCEMW",
        "SS_WIND_COMPLIANCEMW",
        "WDR_INITIALMW",
        "WDR_AVAILABLE",
        "WDR_DISPATCHED",
        "SS_SOLAR_AVAILABILITY",
        "SS_WIND_AVAILABILITY",
        "RAISE1SECLOCALDISPATCH",
        "LOWER1SECLOCALDISPATCH",
        "RAISE1SECACTUALAVAILABILITY",
        "LOWER1SECACTUALAVAILABILITY",
        "BDU_ENERGY_STORAGE",
        "BDU_MIN_AVAIL",
        "BDU_MAX_AVAIL",
        "BDU_CLEAREDMW_GEN",
        "BDU_CLEAREDMW_LOAD",
    ];
    type Row<'row> = DispatchRegionsum8Row<'row>;
    type FieldMapping = DispatchRegionsum8Mapping;
    type PrimaryKey = DispatchRegionsum8PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchRegionsum8Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            dispatchinterval: row
                .get_parsed_at_idx("dispatchinterval", field_mapping.0[3])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totaldemand: row
                .get_opt_custom_parsed_at_idx(
                    "totaldemand",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availablegeneration: row
                .get_opt_custom_parsed_at_idx(
                    "availablegeneration",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availableload: row
                .get_opt_custom_parsed_at_idx(
                    "availableload",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demandforecast: row
                .get_opt_custom_parsed_at_idx(
                    "demandforecast",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchablegeneration: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchablegeneration",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchableload: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchableload",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netinterchange: row
                .get_opt_custom_parsed_at_idx(
                    "netinterchange",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            excessgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "excessgeneration",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5mindispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower5mindispatch",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minimport",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocaldispatch",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalprice",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalreq",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minprice",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minreq",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minsupplyprice",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secdispatch",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secimport",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocaldispatch",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalprice",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalreq",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secprice",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secreq",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secsupplyprice",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secdispatch",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secimport",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocaldispatch",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalprice",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalreq",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secprice",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secreq",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secsupplyprice",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5mindispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise5mindispatch",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minimport",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocaldispatch",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalprice",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalreq",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minprice",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minreq",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minsupplyprice",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secdispatch",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secimport",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocaldispatch",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalprice",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalreq",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secprice",
                    field_mapping.0[50],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secreq",
                    field_mapping.0[51],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secsupplyprice",
                    field_mapping.0[52],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secdispatch",
                    field_mapping.0[53],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secimport",
                    field_mapping.0[54],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocaldispatch",
                    field_mapping.0[55],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalprice",
                    field_mapping.0[56],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalreq",
                    field_mapping.0[57],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secprice",
                    field_mapping.0[58],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secreq",
                    field_mapping.0[59],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secsupplyprice",
                    field_mapping.0[60],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggegatedispatcherror: row
                .get_opt_custom_parsed_at_idx(
                    "aggegatedispatcherror",
                    field_mapping.0[61],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregatedispatcherror: row
                .get_opt_custom_parsed_at_idx(
                    "aggregatedispatcherror",
                    field_mapping.0[62],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[63],
                    mmsdm_core::mms_datetime::parse,
                )?,
            initialsupply: row
                .get_opt_custom_parsed_at_idx(
                    "initialsupply",
                    field_mapping.0[64],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedsupply: row
                .get_opt_custom_parsed_at_idx(
                    "clearedsupply",
                    field_mapping.0[65],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregimport: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregimport",
                    field_mapping.0[66],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocaldispatch",
                    field_mapping.0[67],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocalreq",
                    field_mapping.0[68],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregreq: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregreq",
                    field_mapping.0[69],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregimport: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregimport",
                    field_mapping.0[70],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocaldispatch",
                    field_mapping.0[71],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalreq",
                    field_mapping.0[72],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregreq: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregreq",
                    field_mapping.0[73],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalviolation",
                    field_mapping.0[74],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalviolation",
                    field_mapping.0[75],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalviolation",
                    field_mapping.0[76],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalviolation",
                    field_mapping.0[77],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalviolation",
                    field_mapping.0[78],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocalviolation",
                    field_mapping.0[79],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalviolation",
                    field_mapping.0[80],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalviolation",
                    field_mapping.0[81],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minviolation",
                    field_mapping.0[82],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregviolation",
                    field_mapping.0[83],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secviolation",
                    field_mapping.0[84],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secviolation",
                    field_mapping.0[85],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minviolation",
                    field_mapping.0[86],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregviolation",
                    field_mapping.0[87],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secviolation",
                    field_mapping.0[88],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secviolation",
                    field_mapping.0[89],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secactualavailability",
                    field_mapping.0[90],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secactualavailability",
                    field_mapping.0[91],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minactualavailability",
                    field_mapping.0[92],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregactualavailability",
                    field_mapping.0[93],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secactualavailability",
                    field_mapping.0[94],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secactualavailability",
                    field_mapping.0[95],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minactualavailability",
                    field_mapping.0[96],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregactualavailability",
                    field_mapping.0[97],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lorsurplus: row
                .get_opt_custom_parsed_at_idx(
                    "lorsurplus",
                    field_mapping.0[98],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lrcsurplus: row
                .get_opt_custom_parsed_at_idx(
                    "lrcsurplus",
                    field_mapping.0[99],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalintermittentgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "totalintermittentgeneration",
                    field_mapping.0[100],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand_and_nonschedgen: row
                .get_opt_custom_parsed_at_idx(
                    "demand_and_nonschedgen",
                    field_mapping.0[101],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uigf: row
                .get_opt_custom_parsed_at_idx(
                    "uigf",
                    field_mapping.0[102],
                    mmsdm_core::mms_decimal::parse,
                )?,
            semischedule_clearedmw: row
                .get_opt_custom_parsed_at_idx(
                    "semischedule_clearedmw",
                    field_mapping.0[103],
                    mmsdm_core::mms_decimal::parse,
                )?,
            semischedule_compliancemw: row
                .get_opt_custom_parsed_at_idx(
                    "semischedule_compliancemw",
                    field_mapping.0[104],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_uigf",
                    field_mapping.0[105],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_uigf",
                    field_mapping.0[106],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_clearedmw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_clearedmw",
                    field_mapping.0[107],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_clearedmw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_clearedmw",
                    field_mapping.0[108],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_compliancemw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_compliancemw",
                    field_mapping.0[109],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_compliancemw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_compliancemw",
                    field_mapping.0[110],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_initialmw: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_initialmw",
                    field_mapping.0[111],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_available: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_available",
                    field_mapping.0[112],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_dispatched: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_dispatched",
                    field_mapping.0[113],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_availability: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_availability",
                    field_mapping.0[114],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_availability: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_availability",
                    field_mapping.0[115],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise1seclocaldispatch",
                    field_mapping.0[116],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower1seclocaldispatch",
                    field_mapping.0[117],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secactualavailability",
                    field_mapping.0[118],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secactualavailability",
                    field_mapping.0[119],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_energy_storage",
                    field_mapping.0[120],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_min_avail: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_min_avail",
                    field_mapping.0[121],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_max_avail: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_max_avail",
                    field_mapping.0[122],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_clearedmw_gen: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_clearedmw_gen",
                    field_mapping.0[123],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_clearedmw_load: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_clearedmw_load",
                    field_mapping.0[124],
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
        Ok(DispatchRegionsum8Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchRegionsum8PrimaryKey {
        DispatchRegionsum8PrimaryKey {
            dispatchinterval: row.dispatchinterval,
            intervention: row.intervention,
            regionid: row.regionid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_regionsum_v8_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchRegionsum8Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            regionid: row.regionid.clone(),
            dispatchinterval: row.dispatchinterval.clone(),
            intervention: row.intervention.clone(),
            totaldemand: row.totaldemand.clone(),
            availablegeneration: row.availablegeneration.clone(),
            availableload: row.availableload.clone(),
            demandforecast: row.demandforecast.clone(),
            dispatchablegeneration: row.dispatchablegeneration.clone(),
            dispatchableload: row.dispatchableload.clone(),
            netinterchange: row.netinterchange.clone(),
            excessgeneration: row.excessgeneration.clone(),
            lower5mindispatch: row.lower5mindispatch.clone(),
            lower5minimport: row.lower5minimport.clone(),
            lower5minlocaldispatch: row.lower5minlocaldispatch.clone(),
            lower5minlocalprice: row.lower5minlocalprice.clone(),
            lower5minlocalreq: row.lower5minlocalreq.clone(),
            lower5minprice: row.lower5minprice.clone(),
            lower5minreq: row.lower5minreq.clone(),
            lower5minsupplyprice: row.lower5minsupplyprice.clone(),
            lower60secdispatch: row.lower60secdispatch.clone(),
            lower60secimport: row.lower60secimport.clone(),
            lower60seclocaldispatch: row.lower60seclocaldispatch.clone(),
            lower60seclocalprice: row.lower60seclocalprice.clone(),
            lower60seclocalreq: row.lower60seclocalreq.clone(),
            lower60secprice: row.lower60secprice.clone(),
            lower60secreq: row.lower60secreq.clone(),
            lower60secsupplyprice: row.lower60secsupplyprice.clone(),
            lower6secdispatch: row.lower6secdispatch.clone(),
            lower6secimport: row.lower6secimport.clone(),
            lower6seclocaldispatch: row.lower6seclocaldispatch.clone(),
            lower6seclocalprice: row.lower6seclocalprice.clone(),
            lower6seclocalreq: row.lower6seclocalreq.clone(),
            lower6secprice: row.lower6secprice.clone(),
            lower6secreq: row.lower6secreq.clone(),
            lower6secsupplyprice: row.lower6secsupplyprice.clone(),
            raise5mindispatch: row.raise5mindispatch.clone(),
            raise5minimport: row.raise5minimport.clone(),
            raise5minlocaldispatch: row.raise5minlocaldispatch.clone(),
            raise5minlocalprice: row.raise5minlocalprice.clone(),
            raise5minlocalreq: row.raise5minlocalreq.clone(),
            raise5minprice: row.raise5minprice.clone(),
            raise5minreq: row.raise5minreq.clone(),
            raise5minsupplyprice: row.raise5minsupplyprice.clone(),
            raise60secdispatch: row.raise60secdispatch.clone(),
            raise60secimport: row.raise60secimport.clone(),
            raise60seclocaldispatch: row.raise60seclocaldispatch.clone(),
            raise60seclocalprice: row.raise60seclocalprice.clone(),
            raise60seclocalreq: row.raise60seclocalreq.clone(),
            raise60secprice: row.raise60secprice.clone(),
            raise60secreq: row.raise60secreq.clone(),
            raise60secsupplyprice: row.raise60secsupplyprice.clone(),
            raise6secdispatch: row.raise6secdispatch.clone(),
            raise6secimport: row.raise6secimport.clone(),
            raise6seclocaldispatch: row.raise6seclocaldispatch.clone(),
            raise6seclocalprice: row.raise6seclocalprice.clone(),
            raise6seclocalreq: row.raise6seclocalreq.clone(),
            raise6secprice: row.raise6secprice.clone(),
            raise6secreq: row.raise6secreq.clone(),
            raise6secsupplyprice: row.raise6secsupplyprice.clone(),
            aggegatedispatcherror: row.aggegatedispatcherror.clone(),
            aggregatedispatcherror: row.aggregatedispatcherror.clone(),
            lastchanged: row.lastchanged.clone(),
            initialsupply: row.initialsupply.clone(),
            clearedsupply: row.clearedsupply.clone(),
            lowerregimport: row.lowerregimport.clone(),
            lowerreglocaldispatch: row.lowerreglocaldispatch.clone(),
            lowerreglocalreq: row.lowerreglocalreq.clone(),
            lowerregreq: row.lowerregreq.clone(),
            raiseregimport: row.raiseregimport.clone(),
            raisereglocaldispatch: row.raisereglocaldispatch.clone(),
            raisereglocalreq: row.raisereglocalreq.clone(),
            raiseregreq: row.raiseregreq.clone(),
            raise5minlocalviolation: row.raise5minlocalviolation.clone(),
            raisereglocalviolation: row.raisereglocalviolation.clone(),
            raise60seclocalviolation: row.raise60seclocalviolation.clone(),
            raise6seclocalviolation: row.raise6seclocalviolation.clone(),
            lower5minlocalviolation: row.lower5minlocalviolation.clone(),
            lowerreglocalviolation: row.lowerreglocalviolation.clone(),
            lower60seclocalviolation: row.lower60seclocalviolation.clone(),
            lower6seclocalviolation: row.lower6seclocalviolation.clone(),
            raise5minviolation: row.raise5minviolation.clone(),
            raiseregviolation: row.raiseregviolation.clone(),
            raise60secviolation: row.raise60secviolation.clone(),
            raise6secviolation: row.raise6secviolation.clone(),
            lower5minviolation: row.lower5minviolation.clone(),
            lowerregviolation: row.lowerregviolation.clone(),
            lower60secviolation: row.lower60secviolation.clone(),
            lower6secviolation: row.lower6secviolation.clone(),
            raise6secactualavailability: row.raise6secactualavailability.clone(),
            raise60secactualavailability: row.raise60secactualavailability.clone(),
            raise5minactualavailability: row.raise5minactualavailability.clone(),
            raiseregactualavailability: row.raiseregactualavailability.clone(),
            lower6secactualavailability: row.lower6secactualavailability.clone(),
            lower60secactualavailability: row.lower60secactualavailability.clone(),
            lower5minactualavailability: row.lower5minactualavailability.clone(),
            lowerregactualavailability: row.lowerregactualavailability.clone(),
            lorsurplus: row.lorsurplus.clone(),
            lrcsurplus: row.lrcsurplus.clone(),
            totalintermittentgeneration: row.totalintermittentgeneration.clone(),
            demand_and_nonschedgen: row.demand_and_nonschedgen.clone(),
            uigf: row.uigf.clone(),
            semischedule_clearedmw: row.semischedule_clearedmw.clone(),
            semischedule_compliancemw: row.semischedule_compliancemw.clone(),
            ss_solar_uigf: row.ss_solar_uigf.clone(),
            ss_wind_uigf: row.ss_wind_uigf.clone(),
            ss_solar_clearedmw: row.ss_solar_clearedmw.clone(),
            ss_wind_clearedmw: row.ss_wind_clearedmw.clone(),
            ss_solar_compliancemw: row.ss_solar_compliancemw.clone(),
            ss_wind_compliancemw: row.ss_wind_compliancemw.clone(),
            wdr_initialmw: row.wdr_initialmw.clone(),
            wdr_available: row.wdr_available.clone(),
            wdr_dispatched: row.wdr_dispatched.clone(),
            ss_solar_availability: row.ss_solar_availability.clone(),
            ss_wind_availability: row.ss_wind_availability.clone(),
            raise1seclocaldispatch: row.raise1seclocaldispatch.clone(),
            lower1seclocaldispatch: row.lower1seclocaldispatch.clone(),
            raise1secactualavailability: row.raise1secactualavailability.clone(),
            lower1secactualavailability: row.lower1secactualavailability.clone(),
            bdu_energy_storage: row.bdu_energy_storage.clone(),
            bdu_min_avail: row.bdu_min_avail.clone(),
            bdu_max_avail: row.bdu_max_avail.clone(),
            bdu_clearedmw_gen: row.bdu_clearedmw_gen.clone(),
            bdu_clearedmw_load: row.bdu_clearedmw_load.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchRegionsum8PrimaryKey {
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    pub intervention: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchRegionsum8PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchRegionsum8Row<'data> {
    type Row<'other> = DispatchRegionsum8Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention && self.regionid() == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchRegionsum8Row<'data> {
    type PrimaryKey = DispatchRegionsum8PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention && self.regionid() == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchRegionsum8PrimaryKey {
    type Row<'other> = DispatchRegionsum8Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention && self.regionid == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchRegionsum8PrimaryKey {
    type PrimaryKey = DispatchRegionsum8PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchRegionsum8 {
    type Builder = DispatchRegionsum8Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "dispatchinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "totaldemand",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availablegeneration",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availableload",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demandforecast",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchablegeneration",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchableload",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "netinterchange",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "excessgeneration",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5mindispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minlocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minlocalprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minlocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minsupplyprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secdispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60seclocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60seclocalprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secsupplyprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secdispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6seclocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6seclocalprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secsupplyprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5mindispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minlocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minlocalprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minlocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minsupplyprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secdispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60seclocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60seclocalprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secsupplyprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secdispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6seclocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6seclocalprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secsupplyprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggegatedispatcherror",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregatedispatcherror",
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
                    "initialsupply",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearedsupply",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerreglocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerreglocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregimport",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereglocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereglocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minlocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereglocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60seclocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6seclocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minlocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerreglocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60seclocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6seclocalviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secviolation",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lorsurplus",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lrcsurplus",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalintermittentgeneration",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand_and_nonschedgen",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "uigf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "semischedule_clearedmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "semischedule_compliancemw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_uigf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_uigf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_clearedmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_clearedmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_compliancemw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_compliancemw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdr_initialmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdr_available",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdr_dispatched",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_availability",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_availability",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1seclocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1seclocaldispatch",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secactualavailability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bdu_energy_storage",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bdu_min_avail",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bdu_max_avail",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bdu_clearedmw_gen",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bdu_clearedmw_load",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchRegionsum8Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            totaldemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            availablegeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            availableload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            demandforecast_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            dispatchablegeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            dispatchableload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            netinterchange_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            excessgeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5mindispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minlocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minlocalprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minlocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minsupplyprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60seclocalprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secsupplyprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6seclocalprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secsupplyprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5mindispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minlocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minlocalprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minlocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minsupplyprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60seclocalprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secsupplyprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6seclocalprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secsupplyprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            aggegatedispatcherror_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            aggregatedispatcherror_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            initialsupply_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            clearedsupply_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerreglocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerreglocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raisereglocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raisereglocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minlocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raisereglocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60seclocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6seclocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minlocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerreglocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60seclocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6seclocalviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            raise60secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            raise5minactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            raiseregactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower6secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower60secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower5minactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lowerregactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lorsurplus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lrcsurplus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            totalintermittentgeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            demand_and_nonschedgen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            semischedule_clearedmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            semischedule_compliancemw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_solar_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_wind_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_solar_clearedmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_wind_clearedmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_solar_compliancemw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_wind_compliancemw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            wdr_initialmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            wdr_available_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            wdr_dispatched_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_solar_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ss_wind_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lower1secactualavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            bdu_energy_storage_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            bdu_min_avail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            bdu_max_avail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            bdu_clearedmw_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            bdu_clearedmw_load_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .dispatchinterval_array
            .append_value(row.dispatchinterval.start().and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .totaldemand_array
            .append_option({
                row.totaldemand
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .availablegeneration_array
            .append_option({
                row.availablegeneration
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .availableload_array
            .append_option({
                row.availableload
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .demandforecast_array
            .append_option({
                row.demandforecast
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .dispatchablegeneration_array
            .append_option({
                row.dispatchablegeneration
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .dispatchableload_array
            .append_option({
                row.dispatchableload
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .netinterchange_array
            .append_option({
                row.netinterchange
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .excessgeneration_array
            .append_option({
                row.excessgeneration
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5mindispatch_array
            .append_option({
                row.lower5mindispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minimport_array
            .append_option({
                row.lower5minimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minlocaldispatch_array
            .append_option({
                row.lower5minlocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minlocalprice_array
            .append_option({
                row.lower5minlocalprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minlocalreq_array
            .append_option({
                row.lower5minlocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minprice_array
            .append_option({
                row.lower5minprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minreq_array
            .append_option({
                row.lower5minreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minsupplyprice_array
            .append_option({
                row.lower5minsupplyprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secdispatch_array
            .append_option({
                row.lower60secdispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secimport_array
            .append_option({
                row.lower60secimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60seclocaldispatch_array
            .append_option({
                row.lower60seclocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60seclocalprice_array
            .append_option({
                row.lower60seclocalprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60seclocalreq_array
            .append_option({
                row.lower60seclocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secprice_array
            .append_option({
                row.lower60secprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secreq_array
            .append_option({
                row.lower60secreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secsupplyprice_array
            .append_option({
                row.lower60secsupplyprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secdispatch_array
            .append_option({
                row.lower6secdispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secimport_array
            .append_option({
                row.lower6secimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6seclocaldispatch_array
            .append_option({
                row.lower6seclocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6seclocalprice_array
            .append_option({
                row.lower6seclocalprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6seclocalreq_array
            .append_option({
                row.lower6seclocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secprice_array
            .append_option({
                row.lower6secprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secreq_array
            .append_option({
                row.lower6secreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secsupplyprice_array
            .append_option({
                row.lower6secsupplyprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5mindispatch_array
            .append_option({
                row.raise5mindispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minimport_array
            .append_option({
                row.raise5minimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minlocaldispatch_array
            .append_option({
                row.raise5minlocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minlocalprice_array
            .append_option({
                row.raise5minlocalprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minlocalreq_array
            .append_option({
                row.raise5minlocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minprice_array
            .append_option({
                row.raise5minprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minreq_array
            .append_option({
                row.raise5minreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minsupplyprice_array
            .append_option({
                row.raise5minsupplyprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secdispatch_array
            .append_option({
                row.raise60secdispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secimport_array
            .append_option({
                row.raise60secimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60seclocaldispatch_array
            .append_option({
                row.raise60seclocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60seclocalprice_array
            .append_option({
                row.raise60seclocalprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60seclocalreq_array
            .append_option({
                row.raise60seclocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secprice_array
            .append_option({
                row.raise60secprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secreq_array
            .append_option({
                row.raise60secreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secsupplyprice_array
            .append_option({
                row.raise60secsupplyprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secdispatch_array
            .append_option({
                row.raise6secdispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secimport_array
            .append_option({
                row.raise6secimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6seclocaldispatch_array
            .append_option({
                row.raise6seclocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6seclocalprice_array
            .append_option({
                row.raise6seclocalprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6seclocalreq_array
            .append_option({
                row.raise6seclocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secprice_array
            .append_option({
                row.raise6secprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secreq_array
            .append_option({
                row.raise6secreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secsupplyprice_array
            .append_option({
                row.raise6secsupplyprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .aggegatedispatcherror_array
            .append_option({
                row.aggegatedispatcherror
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .aggregatedispatcherror_array
            .append_option({
                row.aggregatedispatcherror
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .initialsupply_array
            .append_option({
                row.initialsupply
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .clearedsupply_array
            .append_option({
                row.clearedsupply
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregimport_array
            .append_option({
                row.lowerregimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerreglocaldispatch_array
            .append_option({
                row.lowerreglocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerreglocalreq_array
            .append_option({
                row.lowerreglocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregreq_array
            .append_option({
                row.lowerregreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raiseregimport_array
            .append_option({
                row.raiseregimport
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raisereglocaldispatch_array
            .append_option({
                row.raisereglocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raisereglocalreq_array
            .append_option({
                row.raisereglocalreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raiseregreq_array
            .append_option({
                row.raiseregreq
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minlocalviolation_array
            .append_option({
                row.raise5minlocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raisereglocalviolation_array
            .append_option({
                row.raisereglocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60seclocalviolation_array
            .append_option({
                row.raise60seclocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6seclocalviolation_array
            .append_option({
                row.raise6seclocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minlocalviolation_array
            .append_option({
                row.lower5minlocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerreglocalviolation_array
            .append_option({
                row.lowerreglocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60seclocalviolation_array
            .append_option({
                row.lower60seclocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6seclocalviolation_array
            .append_option({
                row.lower6seclocalviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise5minviolation_array
            .append_option({
                row.raise5minviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raiseregviolation_array
            .append_option({
                row.raiseregviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise60secviolation_array
            .append_option({
                row.raise60secviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secviolation_array
            .append_option({
                row.raise6secviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower5minviolation_array
            .append_option({
                row.lower5minviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lowerregviolation_array
            .append_option({
                row.lowerregviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower60secviolation_array
            .append_option({
                row.lower60secviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower6secviolation_array
            .append_option({
                row.lower6secviolation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise6secactualavailability_array
            .append_option({
                row.raise6secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .raise60secactualavailability_array
            .append_option({
                row.raise60secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .raise5minactualavailability_array
            .append_option({
                row.raise5minactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .raiseregactualavailability_array
            .append_option({
                row.raiseregactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower6secactualavailability_array
            .append_option({
                row.lower6secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower60secactualavailability_array
            .append_option({
                row.lower60secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower5minactualavailability_array
            .append_option({
                row.lower5minactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lowerregactualavailability_array
            .append_option({
                row.lowerregactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lorsurplus_array
            .append_option({
                row.lorsurplus
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lrcsurplus_array
            .append_option({
                row.lrcsurplus
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .totalintermittentgeneration_array
            .append_option({
                row.totalintermittentgeneration
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .demand_and_nonschedgen_array
            .append_option({
                row.demand_and_nonschedgen
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .uigf_array
            .append_option({
                row.uigf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .semischedule_clearedmw_array
            .append_option({
                row.semischedule_clearedmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .semischedule_compliancemw_array
            .append_option({
                row.semischedule_compliancemw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_uigf_array
            .append_option({
                row.ss_solar_uigf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_uigf_array
            .append_option({
                row.ss_wind_uigf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_clearedmw_array
            .append_option({
                row.ss_solar_clearedmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_clearedmw_array
            .append_option({
                row.ss_wind_clearedmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_compliancemw_array
            .append_option({
                row.ss_solar_compliancemw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_compliancemw_array
            .append_option({
                row.ss_wind_compliancemw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .wdr_initialmw_array
            .append_option({
                row.wdr_initialmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .wdr_available_array
            .append_option({
                row.wdr_available
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .wdr_dispatched_array
            .append_option({
                row.wdr_dispatched
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_availability_array
            .append_option({
                row.ss_solar_availability
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_availability_array
            .append_option({
                row.ss_wind_availability
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise1seclocaldispatch_array
            .append_option({
                row.raise1seclocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lower1seclocaldispatch_array
            .append_option({
                row.lower1seclocaldispatch
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .raise1secactualavailability_array
            .append_option({
                row.raise1secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lower1secactualavailability_array
            .append_option({
                row.lower1secactualavailability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .bdu_energy_storage_array
            .append_option({
                row.bdu_energy_storage
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .bdu_min_avail_array
            .append_option({
                row.bdu_min_avail
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .bdu_max_avail_array
            .append_option({
                row.bdu_max_avail
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .bdu_clearedmw_gen_array
            .append_option({
                row.bdu_clearedmw_gen
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .bdu_clearedmw_load_array
            .append_option({
                row.bdu_clearedmw_load
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totaldemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availablegeneration_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availableload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demandforecast_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchablegeneration_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchableload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.netinterchange_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.excessgeneration_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5mindispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minlocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minlocalprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minlocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minsupplyprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60seclocalprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secsupplyprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6seclocalprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secsupplyprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5mindispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minlocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minlocalprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minlocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minsupplyprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60seclocalprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secsupplyprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6seclocalprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secsupplyprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggegatedispatcherror_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregatedispatcherror_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initialsupply_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedsupply_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreglocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreglocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereglocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereglocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minlocalviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereglocalviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise60seclocalviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6seclocalviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minlocalviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreglocalviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower60seclocalviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6seclocalviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise6secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise60secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise5minactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raiseregactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower6secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower60secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower5minactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lowerregactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorsurplus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lrcsurplus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.totalintermittentgeneration_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_and_nonschedgen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.semischedule_clearedmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.semischedule_compliancemw_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_clearedmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_clearedmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_compliancemw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_compliancemw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdr_initialmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdr_available_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdr_dispatched_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.raise1secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lower1secactualavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bdu_energy_storage_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bdu_min_avail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bdu_max_avail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bdu_clearedmw_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bdu_clearedmw_load_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchRegionsum8Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    totaldemand_array: arrow::array::builder::Decimal128Builder,
    availablegeneration_array: arrow::array::builder::Decimal128Builder,
    availableload_array: arrow::array::builder::Decimal128Builder,
    demandforecast_array: arrow::array::builder::Decimal128Builder,
    dispatchablegeneration_array: arrow::array::builder::Decimal128Builder,
    dispatchableload_array: arrow::array::builder::Decimal128Builder,
    netinterchange_array: arrow::array::builder::Decimal128Builder,
    excessgeneration_array: arrow::array::builder::Decimal128Builder,
    lower5mindispatch_array: arrow::array::builder::Decimal128Builder,
    lower5minimport_array: arrow::array::builder::Decimal128Builder,
    lower5minlocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower5minlocalprice_array: arrow::array::builder::Decimal128Builder,
    lower5minlocalreq_array: arrow::array::builder::Decimal128Builder,
    lower5minprice_array: arrow::array::builder::Decimal128Builder,
    lower5minreq_array: arrow::array::builder::Decimal128Builder,
    lower5minsupplyprice_array: arrow::array::builder::Decimal128Builder,
    lower60secdispatch_array: arrow::array::builder::Decimal128Builder,
    lower60secimport_array: arrow::array::builder::Decimal128Builder,
    lower60seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower60seclocalprice_array: arrow::array::builder::Decimal128Builder,
    lower60seclocalreq_array: arrow::array::builder::Decimal128Builder,
    lower60secprice_array: arrow::array::builder::Decimal128Builder,
    lower60secreq_array: arrow::array::builder::Decimal128Builder,
    lower60secsupplyprice_array: arrow::array::builder::Decimal128Builder,
    lower6secdispatch_array: arrow::array::builder::Decimal128Builder,
    lower6secimport_array: arrow::array::builder::Decimal128Builder,
    lower6seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower6seclocalprice_array: arrow::array::builder::Decimal128Builder,
    lower6seclocalreq_array: arrow::array::builder::Decimal128Builder,
    lower6secprice_array: arrow::array::builder::Decimal128Builder,
    lower6secreq_array: arrow::array::builder::Decimal128Builder,
    lower6secsupplyprice_array: arrow::array::builder::Decimal128Builder,
    raise5mindispatch_array: arrow::array::builder::Decimal128Builder,
    raise5minimport_array: arrow::array::builder::Decimal128Builder,
    raise5minlocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raise5minlocalprice_array: arrow::array::builder::Decimal128Builder,
    raise5minlocalreq_array: arrow::array::builder::Decimal128Builder,
    raise5minprice_array: arrow::array::builder::Decimal128Builder,
    raise5minreq_array: arrow::array::builder::Decimal128Builder,
    raise5minsupplyprice_array: arrow::array::builder::Decimal128Builder,
    raise60secdispatch_array: arrow::array::builder::Decimal128Builder,
    raise60secimport_array: arrow::array::builder::Decimal128Builder,
    raise60seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raise60seclocalprice_array: arrow::array::builder::Decimal128Builder,
    raise60seclocalreq_array: arrow::array::builder::Decimal128Builder,
    raise60secprice_array: arrow::array::builder::Decimal128Builder,
    raise60secreq_array: arrow::array::builder::Decimal128Builder,
    raise60secsupplyprice_array: arrow::array::builder::Decimal128Builder,
    raise6secdispatch_array: arrow::array::builder::Decimal128Builder,
    raise6secimport_array: arrow::array::builder::Decimal128Builder,
    raise6seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raise6seclocalprice_array: arrow::array::builder::Decimal128Builder,
    raise6seclocalreq_array: arrow::array::builder::Decimal128Builder,
    raise6secprice_array: arrow::array::builder::Decimal128Builder,
    raise6secreq_array: arrow::array::builder::Decimal128Builder,
    raise6secsupplyprice_array: arrow::array::builder::Decimal128Builder,
    aggegatedispatcherror_array: arrow::array::builder::Decimal128Builder,
    aggregatedispatcherror_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    initialsupply_array: arrow::array::builder::Decimal128Builder,
    clearedsupply_array: arrow::array::builder::Decimal128Builder,
    lowerregimport_array: arrow::array::builder::Decimal128Builder,
    lowerreglocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lowerreglocalreq_array: arrow::array::builder::Decimal128Builder,
    lowerregreq_array: arrow::array::builder::Decimal128Builder,
    raiseregimport_array: arrow::array::builder::Decimal128Builder,
    raisereglocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raisereglocalreq_array: arrow::array::builder::Decimal128Builder,
    raiseregreq_array: arrow::array::builder::Decimal128Builder,
    raise5minlocalviolation_array: arrow::array::builder::Decimal128Builder,
    raisereglocalviolation_array: arrow::array::builder::Decimal128Builder,
    raise60seclocalviolation_array: arrow::array::builder::Decimal128Builder,
    raise6seclocalviolation_array: arrow::array::builder::Decimal128Builder,
    lower5minlocalviolation_array: arrow::array::builder::Decimal128Builder,
    lowerreglocalviolation_array: arrow::array::builder::Decimal128Builder,
    lower60seclocalviolation_array: arrow::array::builder::Decimal128Builder,
    lower6seclocalviolation_array: arrow::array::builder::Decimal128Builder,
    raise5minviolation_array: arrow::array::builder::Decimal128Builder,
    raiseregviolation_array: arrow::array::builder::Decimal128Builder,
    raise60secviolation_array: arrow::array::builder::Decimal128Builder,
    raise6secviolation_array: arrow::array::builder::Decimal128Builder,
    lower5minviolation_array: arrow::array::builder::Decimal128Builder,
    lowerregviolation_array: arrow::array::builder::Decimal128Builder,
    lower60secviolation_array: arrow::array::builder::Decimal128Builder,
    lower6secviolation_array: arrow::array::builder::Decimal128Builder,
    raise6secactualavailability_array: arrow::array::builder::Decimal128Builder,
    raise60secactualavailability_array: arrow::array::builder::Decimal128Builder,
    raise5minactualavailability_array: arrow::array::builder::Decimal128Builder,
    raiseregactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower6secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower60secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower5minactualavailability_array: arrow::array::builder::Decimal128Builder,
    lowerregactualavailability_array: arrow::array::builder::Decimal128Builder,
    lorsurplus_array: arrow::array::builder::Decimal128Builder,
    lrcsurplus_array: arrow::array::builder::Decimal128Builder,
    totalintermittentgeneration_array: arrow::array::builder::Decimal128Builder,
    demand_and_nonschedgen_array: arrow::array::builder::Decimal128Builder,
    uigf_array: arrow::array::builder::Decimal128Builder,
    semischedule_clearedmw_array: arrow::array::builder::Decimal128Builder,
    semischedule_compliancemw_array: arrow::array::builder::Decimal128Builder,
    ss_solar_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_wind_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_solar_clearedmw_array: arrow::array::builder::Decimal128Builder,
    ss_wind_clearedmw_array: arrow::array::builder::Decimal128Builder,
    ss_solar_compliancemw_array: arrow::array::builder::Decimal128Builder,
    ss_wind_compliancemw_array: arrow::array::builder::Decimal128Builder,
    wdr_initialmw_array: arrow::array::builder::Decimal128Builder,
    wdr_available_array: arrow::array::builder::Decimal128Builder,
    wdr_dispatched_array: arrow::array::builder::Decimal128Builder,
    ss_solar_availability_array: arrow::array::builder::Decimal128Builder,
    ss_wind_availability_array: arrow::array::builder::Decimal128Builder,
    raise1seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower1seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raise1secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower1secactualavailability_array: arrow::array::builder::Decimal128Builder,
    bdu_energy_storage_array: arrow::array::builder::Decimal128Builder,
    bdu_min_avail_array: arrow::array::builder::Decimal128Builder,
    bdu_max_avail_array: arrow::array::builder::Decimal128Builder,
    bdu_clearedmw_gen_array: arrow::array::builder::Decimal128Builder,
    bdu_clearedmw_load_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchocdConstraintFcasOcd1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchocdConstraintFcasOcd1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchocdConstraintFcasOcd1 {
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
pub struct DispatchocdConstraintFcasOcd1Mapping([usize; 9]);
/// # Summary
///
/// ## DISPATCH_CONSTRAINT_FCAS_OCD
///  _FCAS constraint solution from OCD re-run._
///
/// * Data Set Name: Dispatchocd
/// * File Name: Constraint Fcas Ocd
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchocdConstraintFcasOcd1Row<'data> {
    /// Dispatch interval that the prices were loaded to
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: i64,
    /// Intervention 0/1
    pub intervention: i64,
    /// ConstraintID/GenconID
    pub constraintid: core::ops::Range<usize>,
    /// VersionNo
    pub versionno: i64,
    /// The datetime that the record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// RHS from OCD re-run
    pub rhs: Option<rust_decimal::Decimal>,
    /// marginalvalue from OCD re-run
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// The violation degree of this constraint in the solution result
    pub violationdegree: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchocdConstraintFcasOcd1Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchocdConstraintFcasOcd1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCHOCD";
    const TABLE_NAME: &'static str = "CONSTRAINT_FCAS_OCD";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchocdConstraintFcasOcd1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "INTERVENTION",
        "CONSTRAINTID",
        "VERSIONNO",
        "LASTCHANGED",
        "RHS",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
    ];
    type Row<'row> = DispatchocdConstraintFcasOcd1Row<'row>;
    type FieldMapping = DispatchocdConstraintFcasOcd1Mapping;
    type PrimaryKey = DispatchocdConstraintFcasOcd1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchocdConstraintFcasOcd1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row.get_parsed_at_idx("runno", field_mapping.0[1])?,
            intervention: row.get_parsed_at_idx("intervention", field_mapping.0[2])?,
            constraintid: row.get_range("constraintid", field_mapping.0[3])?,
            versionno: row.get_parsed_at_idx("versionno", field_mapping.0[4])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violationdegree",
                    field_mapping.0[8],
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
        Ok(DispatchocdConstraintFcasOcd1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchocdConstraintFcasOcd1PrimaryKey {
        DispatchocdConstraintFcasOcd1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            intervention: row.intervention,
            runno: row.runno,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "dispatchocd_constraint_fcas_ocd_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchocdConstraintFcasOcd1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            intervention: row.intervention.clone(),
            constraintid: row.constraintid.clone(),
            versionno: row.versionno.clone(),
            lastchanged: row.lastchanged.clone(),
            rhs: row.rhs.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchocdConstraintFcasOcd1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub intervention: i64,
    pub runno: i64,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for DispatchocdConstraintFcasOcd1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchocdConstraintFcasOcd1Row<'data> {
    type Row<'other> = DispatchocdConstraintFcasOcd1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DispatchocdConstraintFcasOcd1Row<'data> {
    type PrimaryKey = DispatchocdConstraintFcasOcd1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchocdConstraintFcasOcd1PrimaryKey {
    type Row<'other> = DispatchocdConstraintFcasOcd1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid() && self.intervention == row.intervention
            && self.runno == row.runno && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchocdConstraintFcasOcd1PrimaryKey {
    type PrimaryKey = DispatchocdConstraintFcasOcd1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchocdConstraintFcasOcd1 {
    type Builder = DispatchocdConstraintFcasOcd1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Int64,
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
                    "rhs",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violationdegree",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchocdConstraintFcasOcd1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Int64Builder::new(),
            intervention_array: arrow::array::builder::Int64Builder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Int64Builder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.runno_array.append_value(row.runno);
        builder.intervention_array.append_value(row.intervention);
        builder.constraintid_array.append_value(row.constraintid());
        builder.versionno_array.append_value(row.versionno);
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .rhs_array
            .append_option({
                row.rhs
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marginalvalue_array
            .append_option({
                row.marginalvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violationdegree_array
            .append_option({
                row.violationdegree
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchocdConstraintFcasOcd1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Int64Builder,
    intervention_array: arrow::array::builder::Int64Builder,
    constraintid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Int64Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    rhs_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchFcasReq2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchFcasReq2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchFcasReq2 {
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
pub struct DispatchFcasReq2Mapping([usize; 16]);
/// # Summary
///
/// ## DISPATCH_FCAS_REQ
///  _DISPATCH_FCAS_REQ shows Dispatch Constraint tracking for Regional FCAS recovery._
///
/// * Data Set Name: Dispatch
/// * File Name: Fcas Req
/// * Data Version: 2
///
/// # Description
///  DISPATCH_FCAS_REQ is public data and is available to all participants. Source DISPATCH_FCAS_REQ updates with each dispatch run (5 minutes). Volume Approximately 10,000 rows per day
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * GENCONID
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchFcasReq2Row<'data> {
    /// Settlement date and time of Dispatch Interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention Flag
    pub intervention: rust_decimal::Decimal,
    /// Generic Constraint ID - Join to table GenConData
    pub genconid: core::ops::Range<usize>,
    /// &nbsp;
    pub regionid: core::ops::Range<usize>,
    /// DUID offered type
    pub bidtype: core::ops::Range<usize>,
    /// Generic Constraint EffectiveDate - Join to table GenConData
    pub genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    pub genconversionno: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Date record is changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The base cost of the constraint for this service, before the regulation/contingency split
    pub base_cost: Option<rust_decimal::Decimal>,
    /// The adjusted cost of the constraint for this service, before the regulation/contingency split
    pub adjusted_cost: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CMPF, based on dispatched data
    pub estimated_cmpf: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CRMPF, based on dispatched data
    pub estimated_crmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CMPF based recovery
    pub recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CRMPF based recovery
    pub recovery_factor_crmpf: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchFcasReq2Row<'data> {
    pub fn genconid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconid.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for DispatchFcasReq2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "FCAS_REQ";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchFcasReq2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "INTERVENTION",
        "GENCONID",
        "REGIONID",
        "BIDTYPE",
        "GENCONEFFECTIVEDATE",
        "GENCONVERSIONNO",
        "MARGINALVALUE",
        "LASTCHANGED",
        "BASE_COST",
        "ADJUSTED_COST",
        "ESTIMATED_CMPF",
        "ESTIMATED_CRMPF",
        "RECOVERY_FACTOR_CMPF",
        "RECOVERY_FACTOR_CRMPF",
    ];
    type Row<'row> = DispatchFcasReq2Row<'row>;
    type FieldMapping = DispatchFcasReq2Mapping;
    type PrimaryKey = DispatchFcasReq2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchFcasReq2Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconid: row.get_range("genconid", field_mapping.0[3])?,
            regionid: row.get_range("regionid", field_mapping.0[4])?,
            bidtype: row.get_range("bidtype", field_mapping.0[5])?,
            genconeffectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "genconeffectivedate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            genconversionno: row
                .get_opt_custom_parsed_at_idx(
                    "genconversionno",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            base_cost: row
                .get_opt_custom_parsed_at_idx(
                    "base_cost",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            adjusted_cost: row
                .get_opt_custom_parsed_at_idx(
                    "adjusted_cost",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            estimated_cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "estimated_cmpf",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            estimated_crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "estimated_crmpf",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_cmpf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_crmpf",
                    field_mapping.0[15],
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
        Ok(DispatchFcasReq2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchFcasReq2PrimaryKey {
        DispatchFcasReq2PrimaryKey {
            bidtype: row.bidtype().to_string(),
            genconid: row.genconid().to_string(),
            intervention: row.intervention,
            regionid: row.regionid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_fcas_req_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchFcasReq2Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            intervention: row.intervention.clone(),
            genconid: row.genconid.clone(),
            regionid: row.regionid.clone(),
            bidtype: row.bidtype.clone(),
            genconeffectivedate: row.genconeffectivedate.clone(),
            genconversionno: row.genconversionno.clone(),
            marginalvalue: row.marginalvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            base_cost: row.base_cost.clone(),
            adjusted_cost: row.adjusted_cost.clone(),
            estimated_cmpf: row.estimated_cmpf.clone(),
            estimated_crmpf: row.estimated_crmpf.clone(),
            recovery_factor_cmpf: row.recovery_factor_cmpf.clone(),
            recovery_factor_crmpf: row.recovery_factor_crmpf.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchFcasReq2PrimaryKey {
    pub bidtype: alloc::string::String,
    pub genconid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchFcasReq2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchFcasReq2Row<'data> {
    type Row<'other> = DispatchFcasReq2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.genconid() == row.genconid()
            && self.intervention == row.intervention && self.regionid() == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchFcasReq2Row<'data> {
    type PrimaryKey = DispatchFcasReq2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.genconid() == key.genconid
            && self.intervention == key.intervention && self.regionid() == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchFcasReq2PrimaryKey {
    type Row<'other> = DispatchFcasReq2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.genconid == row.genconid()
            && self.intervention == row.intervention && self.regionid == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchFcasReq2PrimaryKey {
    type PrimaryKey = DispatchFcasReq2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.genconid == key.genconid
            && self.intervention == key.intervention && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchFcasReq2 {
    type Builder = DispatchFcasReq2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconeffectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "genconversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
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
                arrow::datatypes::Field::new(
                    "base_cost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "adjusted_cost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "estimated_cmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "estimated_crmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_factor_cmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_factor_crmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchFcasReq2Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            genconid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            genconeffectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            genconversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            base_cost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            adjusted_cost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            estimated_cmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            estimated_crmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_factor_cmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_factor_crmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconid_array.append_value(row.genconid());
        builder.regionid_array.append_value(row.regionid());
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .genconeffectivedate_array
            .append_option(
                row.genconeffectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .genconversionno_array
            .append_option({
                row.genconversionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .marginalvalue_array
            .append_option({
                row.marginalvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .base_cost_array
            .append_option({
                row.base_cost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .adjusted_cost_array
            .append_option({
                row.adjusted_cost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .estimated_cmpf_array
            .append_option({
                row.estimated_cmpf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .estimated_crmpf_array
            .append_option({
                row.estimated_crmpf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_factor_cmpf_array
            .append_option({
                row.recovery_factor_cmpf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_factor_crmpf_array
            .append_option({
                row.recovery_factor_crmpf
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconeffectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.base_cost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.adjusted_cost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.estimated_cmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.estimated_crmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recovery_factor_cmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recovery_factor_crmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchFcasReq2Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    genconid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    genconeffectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    genconversionno_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    base_cost_array: arrow::array::builder::Decimal128Builder,
    adjusted_cost_array: arrow::array::builder::Decimal128Builder,
    estimated_cmpf_array: arrow::array::builder::Decimal128Builder,
    estimated_crmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_cmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_crmpf_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchInterconnection1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchInterconnection1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchInterconnection1 {
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
pub struct DispatchInterconnection1Mapping([usize; 12]);
/// # Summary
///
/// ## DISPATCH_INTERCONNECTION
///  _Inter-regional flow information common to or aggregated for regulated (i.e. not MNSP) Interconnectors spanning the From-Region and To-Region - NB only the physical run is calculated'_
///
/// * Data Set Name: Dispatch
/// * File Name: Interconnection
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * FROM_REGIONID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
/// * TO_REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchInterconnection1Row<'data> {
    /// Market date starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention case or not
    pub intervention: rust_decimal::Decimal,
    /// Nominated RegionID from which the energy flows
    pub from_regionid: core::ops::Range<usize>,
    /// Nominated RegionID to which the energy flows
    pub to_regionid: core::ops::Range<usize>,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP
    pub dispatchinterval: mmsdm_core::DispatchPeriod,
    /// Inter-Regional Loss Factor. Calculated based on the MWFLOW and the nominal From and To Region losses.
    pub irlf: Option<rust_decimal::Decimal>,
    /// Summed MW flow of the parallel regulated Interconnectors
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Summed Metered MW flow of the parallel regulated Interconnectors
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal From Region
    pub from_region_mw_losses: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal To Region
    pub to_region_mw_losses: Option<rust_decimal::Decimal>,
    /// The datetime that the record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchInterconnection1Row<'data> {
    pub fn from_regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.from_regionid.clone())
    }
    pub fn to_regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.to_regionid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchInterconnection1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "INTERCONNECTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchInterconnection1Mapping([
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
        "SETTLEMENTDATE",
        "RUNNO",
        "INTERVENTION",
        "FROM_REGIONID",
        "TO_REGIONID",
        "DISPATCHINTERVAL",
        "IRLF",
        "MWFLOW",
        "METEREDMWFLOW",
        "FROM_REGION_MW_LOSSES",
        "TO_REGION_MW_LOSSES",
        "LASTCHANGED",
    ];
    type Row<'row> = DispatchInterconnection1Row<'row>;
    type FieldMapping = DispatchInterconnection1Mapping;
    type PrimaryKey = DispatchInterconnection1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchInterconnection1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            from_regionid: row.get_range("from_regionid", field_mapping.0[3])?,
            to_regionid: row.get_range("to_regionid", field_mapping.0[4])?,
            dispatchinterval: row
                .get_parsed_at_idx("dispatchinterval", field_mapping.0[5])?,
            irlf: row
                .get_opt_custom_parsed_at_idx(
                    "irlf",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meteredmwflow: row
                .get_opt_custom_parsed_at_idx(
                    "meteredmwflow",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            from_region_mw_losses: row
                .get_opt_custom_parsed_at_idx(
                    "from_region_mw_losses",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            to_region_mw_losses: row
                .get_opt_custom_parsed_at_idx(
                    "to_region_mw_losses",
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
        Ok(DispatchInterconnection1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchInterconnection1PrimaryKey {
        DispatchInterconnection1PrimaryKey {
            from_regionid: row.from_regionid().to_string(),
            intervention: row.intervention,
            runno: row.runno,
            settlementdate: row.settlementdate,
            to_regionid: row.to_regionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_interconnection_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchInterconnection1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            intervention: row.intervention.clone(),
            from_regionid: row.from_regionid.clone(),
            to_regionid: row.to_regionid.clone(),
            dispatchinterval: row.dispatchinterval.clone(),
            irlf: row.irlf.clone(),
            mwflow: row.mwflow.clone(),
            meteredmwflow: row.meteredmwflow.clone(),
            from_region_mw_losses: row.from_region_mw_losses.clone(),
            to_region_mw_losses: row.to_region_mw_losses.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchInterconnection1PrimaryKey {
    pub from_regionid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub to_regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for DispatchInterconnection1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchInterconnection1Row<'data> {
    type Row<'other> = DispatchInterconnection1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.from_regionid() == row.from_regionid()
            && self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.to_regionid() == row.to_regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchInterconnection1Row<'data> {
    type PrimaryKey = DispatchInterconnection1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.from_regionid() == key.from_regionid
            && self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.to_regionid() == key.to_regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchInterconnection1PrimaryKey {
    type Row<'other> = DispatchInterconnection1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.from_regionid == row.from_regionid()
            && self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.to_regionid == row.to_regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchInterconnection1PrimaryKey {
    type PrimaryKey = DispatchInterconnection1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.from_regionid == key.from_regionid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
            && self.to_regionid == key.to_regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchInterconnection1 {
    type Builder = DispatchInterconnection1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "from_regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "to_regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "dispatchinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "irlf",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meteredmwflow",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "from_region_mw_losses",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "to_region_mw_losses",
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
        DispatchInterconnection1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            from_regionid_array: arrow::array::builder::StringBuilder::new(),
            to_regionid_array: arrow::array::builder::StringBuilder::new(),
            dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            irlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            meteredmwflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            from_region_mw_losses_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            to_region_mw_losses_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.from_regionid_array.append_value(row.from_regionid());
        builder.to_regionid_array.append_value(row.to_regionid());
        builder
            .dispatchinterval_array
            .append_value(row.dispatchinterval.start().and_utc().timestamp_millis());
        builder
            .irlf_array
            .append_option({
                row.irlf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow_array
            .append_option({
                row.mwflow
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .meteredmwflow_array
            .append_option({
                row.meteredmwflow
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .from_region_mw_losses_array
            .append_option({
                row.from_region_mw_losses
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .to_region_mw_losses_array
            .append_option({
                row.to_region_mw_losses
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.from_regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.to_regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.irlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meteredmwflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.from_region_mw_losses_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.to_region_mw_losses_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchInterconnection1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    from_regionid_array: arrow::array::builder::StringBuilder,
    to_regionid_array: arrow::array::builder::StringBuilder,
    dispatchinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
    irlf_array: arrow::array::builder::Decimal128Builder,
    mwflow_array: arrow::array::builder::Decimal128Builder,
    meteredmwflow_array: arrow::array::builder::Decimal128Builder,
    from_region_mw_losses_array: arrow::array::builder::Decimal128Builder,
    to_region_mw_losses_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DispatchLocalPrice1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchLocalPrice1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchLocalPrice1 {
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
pub struct DispatchLocalPrice1Mapping([usize; 4]);
/// # Summary
///
/// ## DISPATCH_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period. Note that from 2014 Mid year release only records with non-zero Local_Price_Adjustment values are issued_
///
/// * Data Set Name: Dispatch
/// * File Name: Local Price
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchLocalPrice1Row<'data> {
    /// Market date time starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchLocalPrice1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchLocalPrice1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "LOCAL_PRICE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchLocalPrice1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "DUID",
        "LOCAL_PRICE_ADJUSTMENT",
        "LOCALLY_CONSTRAINED",
    ];
    type Row<'row> = DispatchLocalPrice1Row<'row>;
    type FieldMapping = DispatchLocalPrice1Mapping;
    type PrimaryKey = DispatchLocalPrice1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchLocalPrice1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            local_price_adjustment: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained",
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
        Ok(DispatchLocalPrice1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchLocalPrice1PrimaryKey {
        DispatchLocalPrice1PrimaryKey {
            duid: row.duid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_local_price_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchLocalPrice1Row {
            settlementdate: row.settlementdate.clone(),
            duid: row.duid.clone(),
            local_price_adjustment: row.local_price_adjustment.clone(),
            locally_constrained: row.locally_constrained.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchLocalPrice1PrimaryKey {
    pub duid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchLocalPrice1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchLocalPrice1Row<'data> {
    type Row<'other> = DispatchLocalPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchLocalPrice1Row<'data> {
    type PrimaryKey = DispatchLocalPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchLocalPrice1PrimaryKey {
    type Row<'other> = DispatchLocalPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchLocalPrice1PrimaryKey {
    type PrimaryKey = DispatchLocalPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchLocalPrice1 {
    type Builder = DispatchLocalPrice1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "local_price_adjustment",
                    arrow::datatypes::DataType::Decimal128(10, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "locally_constrained",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchLocalPrice1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            local_price_adjustment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 2)),
            locally_constrained_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .local_price_adjustment_array
            .append_option({
                row.local_price_adjustment
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .locally_constrained_array
            .append_option({
                row.locally_constrained
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.local_price_adjustment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.locally_constrained_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchLocalPrice1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    local_price_adjustment_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchMnspbidtrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchMnspbidtrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchMnspbidtrk1 {
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
pub struct DispatchMnspbidtrk1Mapping([usize; 8]);
/// # Summary
///
/// ## DISPATCH_MNSPBIDTRK
///  _DISPATCH_MNSPBIDTRK shows the MNSP bid tracking, including the bid version used in each dispatch run for each MNSP Interconnector Link. The table identifies which bids from MNSP_DAYOFFER and MNSP_BIDOFFERPERIOD were applied._
///
/// * Data Set Name: Dispatch
/// * File Name: Mnspbidtrk
/// * Data Version: 1
///
/// # Description
///  DISPATCH_MNSPBIDTRK shows own details for participant as they occur, with all details until close of business yesterday being available to all participants after end of day. Source DISPATCH_MNSPBIDTRK potentially updates every 5 minutes. Volume 220, 000 per year
///
///
///
/// # Primary Key Columns
///
/// * LINKID
/// * PARTICIPANTID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchMnspbidtrk1Row<'data> {
    /// Market date starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Participant that owns unit during effective record period
    pub participantid: core::ops::Range<usize>,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: core::ops::Range<usize>,
    /// Offer date for bid
    pub offersettlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    pub offereffectivedate: Option<chrono::NaiveDateTime>,
    /// VersionNo of the bid/offer used
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Record creation timestamp
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchMnspbidtrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn linkid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.linkid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchMnspbidtrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "MNSPBIDTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchMnspbidtrk1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "PARTICIPANTID",
        "LINKID",
        "OFFERSETTLEMENTDATE",
        "OFFEREFFECTIVEDATE",
        "OFFERVERSIONNO",
        "LASTCHANGED",
    ];
    type Row<'row> = DispatchMnspbidtrk1Row<'row>;
    type FieldMapping = DispatchMnspbidtrk1Mapping;
    type PrimaryKey = DispatchMnspbidtrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchMnspbidtrk1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            linkid: row.get_range("linkid", field_mapping.0[3])?,
            offersettlementdate: row
                .get_opt_custom_parsed_at_idx(
                    "offersettlementdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offereffectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "offereffectivedate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerversionno: row
                .get_opt_custom_parsed_at_idx(
                    "offerversionno",
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
        Ok(DispatchMnspbidtrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchMnspbidtrk1PrimaryKey {
        DispatchMnspbidtrk1PrimaryKey {
            linkid: row.linkid().to_string(),
            participantid: row.participantid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_mnspbidtrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchMnspbidtrk1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            participantid: row.participantid.clone(),
            linkid: row.linkid.clone(),
            offersettlementdate: row.offersettlementdate.clone(),
            offereffectivedate: row.offereffectivedate.clone(),
            offerversionno: row.offerversionno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchMnspbidtrk1PrimaryKey {
    pub linkid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchMnspbidtrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchMnspbidtrk1Row<'data> {
    type Row<'other> = DispatchMnspbidtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid() == row.linkid() && self.participantid() == row.participantid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchMnspbidtrk1Row<'data> {
    type PrimaryKey = DispatchMnspbidtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid() == key.linkid && self.participantid() == key.participantid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchMnspbidtrk1PrimaryKey {
    type Row<'other> = DispatchMnspbidtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid == row.linkid() && self.participantid == row.participantid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchMnspbidtrk1PrimaryKey {
    type PrimaryKey = DispatchMnspbidtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid && self.participantid == key.participantid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchMnspbidtrk1 {
    type Builder = DispatchMnspbidtrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "linkid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offersettlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offereffectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerversionno",
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
        DispatchMnspbidtrk1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            linkid_array: arrow::array::builder::StringBuilder::new(),
            offersettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offereffectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.linkid_array.append_value(row.linkid());
        builder
            .offersettlementdate_array
            .append_option(
                row.offersettlementdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .offereffectivedate_array
            .append_option(
                row.offereffectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .offerversionno_array
            .append_option({
                row.offerversionno
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.linkid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offersettlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offereffectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchMnspbidtrk1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    linkid_array: arrow::array::builder::StringBuilder,
    offersettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offereffectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerversionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DispatchMrScheduleTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchMrScheduleTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchMrScheduleTrk1 {
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
pub struct DispatchMrScheduleTrk1Mapping([usize; 5]);
/// # Summary
///
/// ## DISPATCH_MR_SCHEDULE_TRK
///  _DISPATCH_MR_SCHEDULE_TRK records the Mandatory Restrictions Acceptance Schedule applied to this dispatch interval for this region.<br>DISPATCH_MR_SCHEDULE_TRK is populated by the Dispatch process and records the MR Offer Stack applied in each dispatch interval. DISPATCH_MR_SCHEDULE_TRK is used by Settlements to calculate payments according to the correct MR offer stack._
///
/// * Data Set Name: Dispatch
/// * File Name: Mr Schedule Trk
/// * Data Version: 1
///
/// # Description
///  DISPATCH_MR_SCHEDULE_TRK  data is public to all participants. Source DISPATCH_MR_SCHEDULE_TRK updates are ad hoc. Volume 2 rows per year.
///
///
///
/// # Primary Key Columns
///
/// * REGIONID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchMrScheduleTrk1Row<'data> {
    /// Date Time of the Dispatch Interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Unique RegionID; Key reference to MR_Event_Schedule
    pub regionid: core::ops::Range<usize>,
    /// Mandatory Restriction date; Key reference to MR_Event_Schedule table
    pub mr_date: Option<chrono::NaiveDateTime>,
    /// Date Time the MR  acceptance stack was created; Key reference to MR_Event_Schedule table
    pub version_datetime: Option<chrono::NaiveDateTime>,
    /// Date and  time the record was last inserted/modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchMrScheduleTrk1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchMrScheduleTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "MR_SCHEDULE_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchMrScheduleTrk1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "REGIONID",
        "MR_DATE",
        "VERSION_DATETIME",
        "LASTCHANGED",
    ];
    type Row<'row> = DispatchMrScheduleTrk1Row<'row>;
    type FieldMapping = DispatchMrScheduleTrk1Mapping;
    type PrimaryKey = DispatchMrScheduleTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchMrScheduleTrk1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            mr_date: row
                .get_opt_custom_parsed_at_idx(
                    "mr_date",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            version_datetime: row
                .get_opt_custom_parsed_at_idx(
                    "version_datetime",
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
        Ok(DispatchMrScheduleTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchMrScheduleTrk1PrimaryKey {
        DispatchMrScheduleTrk1PrimaryKey {
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_mr_schedule_trk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchMrScheduleTrk1Row {
            settlementdate: row.settlementdate.clone(),
            regionid: row.regionid.clone(),
            mr_date: row.mr_date.clone(),
            version_datetime: row.version_datetime.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchMrScheduleTrk1PrimaryKey {
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchMrScheduleTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchMrScheduleTrk1Row<'data> {
    type Row<'other> = DispatchMrScheduleTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid() == row.regionid() && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchMrScheduleTrk1Row<'data> {
    type PrimaryKey = DispatchMrScheduleTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid() == key.regionid && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchMrScheduleTrk1PrimaryKey {
    type Row<'other> = DispatchMrScheduleTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid == row.regionid() && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchMrScheduleTrk1PrimaryKey {
    type PrimaryKey = DispatchMrScheduleTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchMrScheduleTrk1 {
    type Builder = DispatchMrScheduleTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
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
                    "mr_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "version_datetime",
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
        DispatchMrScheduleTrk1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            mr_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .mr_date_array
            .append_option(row.mr_date.map(|val| val.and_utc().timestamp_millis()));
        builder
            .version_datetime_array
            .append_option(
                row.version_datetime.map(|val| val.and_utc().timestamp_millis()),
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mr_date_array.finish())
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
pub struct DispatchMrScheduleTrk1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    mr_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct PriceloadPriceRevision1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PriceloadPriceRevision1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PriceloadPriceRevision1 {
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
pub struct PriceloadPriceRevision1Mapping([usize; 9]);
/// # Summary
///
/// ## DISPATCH_PRICE_REVISION
///  _An audit trail of price changes on the DISPATCHPRICE table (i.e. for 5 minute dispatch prices for energy and FCAS)._
///
/// * Data Set Name: Priceload
/// * File Name: Price Revision
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct PriceloadPriceRevision1Row<'data> {
    /// Market date and time starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Manual intervention flag; always 0
    pub intervention: rust_decimal::Decimal,
    /// Affected Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Affected Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// Version No of price revision for this settlement date
    pub versionno: i64,
    /// New RRP in DISPATCHPRICE table
    pub rrp_new: Option<rust_decimal::Decimal>,
    /// Old RRP from DISPATCHPRICE table
    pub rrp_old: Option<rust_decimal::Decimal>,
    /// The datetime the record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PriceloadPriceRevision1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for PriceloadPriceRevision1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PRICELOAD";
    const TABLE_NAME: &'static str = "PRICE_REVISION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PriceloadPriceRevision1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "INTERVENTION",
        "REGIONID",
        "BIDTYPE",
        "VERSIONNO",
        "RRP_NEW",
        "RRP_OLD",
        "LASTCHANGED",
    ];
    type Row<'row> = PriceloadPriceRevision1Row<'row>;
    type FieldMapping = PriceloadPriceRevision1Mapping;
    type PrimaryKey = PriceloadPriceRevision1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PriceloadPriceRevision1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            bidtype: row.get_range("bidtype", field_mapping.0[4])?,
            versionno: row.get_parsed_at_idx("versionno", field_mapping.0[5])?,
            rrp_new: row
                .get_opt_custom_parsed_at_idx(
                    "rrp_new",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp_old: row
                .get_opt_custom_parsed_at_idx(
                    "rrp_old",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
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
        Ok(PriceloadPriceRevision1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PriceloadPriceRevision1PrimaryKey {
        PriceloadPriceRevision1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            intervention: row.intervention,
            regionid: row.regionid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("priceload_price_revision_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PriceloadPriceRevision1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            intervention: row.intervention.clone(),
            regionid: row.regionid.clone(),
            bidtype: row.bidtype.clone(),
            versionno: row.versionno.clone(),
            rrp_new: row.rrp_new.clone(),
            rrp_old: row.rrp_old.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriceloadPriceRevision1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for PriceloadPriceRevision1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PriceloadPriceRevision1Row<'data> {
    type Row<'other> = PriceloadPriceRevision1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.intervention == row.intervention
            && self.regionid() == row.regionid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PriceloadPriceRevision1Row<'data> {
    type PrimaryKey = PriceloadPriceRevision1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.intervention == key.intervention
            && self.regionid() == key.regionid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PriceloadPriceRevision1PrimaryKey {
    type Row<'other> = PriceloadPriceRevision1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.intervention == row.intervention
            && self.regionid == row.regionid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PriceloadPriceRevision1PrimaryKey {
    type PrimaryKey = PriceloadPriceRevision1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.intervention == key.intervention
            && self.regionid == key.regionid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PriceloadPriceRevision1 {
    type Builder = PriceloadPriceRevision1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rrp_new",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp_old",
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
        PriceloadPriceRevision1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Int64Builder::new(),
            rrp_new_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp_old_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder.bidtype_array.append_value(row.bidtype());
        builder.versionno_array.append_value(row.versionno);
        builder
            .rrp_new_array
            .append_option({
                row.rrp_new
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp_old_array
            .append_option({
                row.rrp_old
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_new_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_old_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PriceloadPriceRevision1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Int64Builder,
    rrp_new_array: arrow::array::builder::Decimal128Builder,
    rrp_old_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DispatchUnitConformance2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchUnitConformance2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchUnitConformance2 {
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
pub struct DispatchUnitConformance2Mapping([usize; 21]);
/// # Summary
///
/// ## DISPATCH_UNIT_CONFORMANCE
///  _DISPATCH_UNIT_CONFORMANCE details the conformance of a scheduled units operation with respect to a cleared target on dispatch interval basis.<br>Data is confidential_
///
/// * Data Set Name: Dispatch
/// * File Name: Unit Conformance
/// * Data Version: 2
///
/// # Description
///  DISPATCH_UNIT_CONFORMANCE data is confidential. Source DISPATCH_UNIT_CONFORMANCE shows data for every 5 minutes for all scheduled units Volume Rows per day: 288 per scheduled unit
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchUnitConformance2Row<'data> {
    /// Dispatch Interval that the conformance data applies to
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier, or Aggregate Dispatch Group identifier
    pub duid: core::ops::Range<usize>,
    /// Dispatch Target - MW
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Unit output measured at the conclusion of the dispatch interval - MW (MWB)
    pub actualmw: Option<rust_decimal::Decimal>,
    /// Rate of change in direction of error MW per minute
    pub roc: Option<rust_decimal::Decimal>,
    /// Offered unit capacity - MW (MWO)
    pub availability: Option<rust_decimal::Decimal>,
    /// Lower Regulation FCAS enabled - MW (FCL)
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation FCAS enabled - MW (FCR)
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Calculated small trigger error limit in MW
    pub striglm: Option<rust_decimal::Decimal>,
    /// Calculated large trigger error limit in MW
    pub ltriglm: Option<rust_decimal::Decimal>,
    /// Calculated actual error
    pub mwerror: Option<rust_decimal::Decimal>,
    /// Max of mwerror while that unit was not in a normal state
    pub max_mwerror: Option<rust_decimal::Decimal>,
    /// Large trigger error count. Reset when mwerror changes sign
    pub lecount: Option<i64>,
    /// Small trigger error count.  Reset when mwerror changes sign
    pub secount: Option<i64>,
    /// Unit conformance status.<br>NORMAL<br>OFF-TARGET<br>NOT-RESPONDING<br>NC-PENDING<br>NON-CONFORMING<br>SUSPENDED
    pub status: core::ops::Range<usize>,
    /// Participant action required in response to current STATUS
    pub participant_status_action: core::ops::Range<usize>,
    /// conformance operating mode<br>MANUAL<br>AUTO
    pub operating_mode: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Aggregate Dispatch Group to which this dispatch unit belongs
    pub adg_id: core::ops::Range<usize>,
    /// Boolean representation flagging if the Target is capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
    /// For an individual unit in an aggregate dispatch group (where DUID &lt;&gt; ADG_ID), Mode specific to that unit. 0 - no monitoring, 1 - aggregate monitoring, 2 - individual monitoring due to constraint. For the aggregate dispatch group (where DUID = ADG_ID), 0 - no aggregate monitoring, 1 - aggregate monitoring
    pub conformance_mode: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchUnitConformance2Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn status(&self) -> Option<&str> {
        if self.status.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.status.clone(),
                ),
            )
        }
    }
    pub fn participant_status_action(&self) -> Option<&str> {
        if self.participant_status_action.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.participant_status_action.clone(),
                ),
            )
        }
    }
    pub fn operating_mode(&self) -> Option<&str> {
        if self.operating_mode.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.operating_mode.clone(),
                ),
            )
        }
    }
    pub fn adg_id(&self) -> Option<&str> {
        if self.adg_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.adg_id.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchUnitConformance2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "UNIT_CONFORMANCE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchUnitConformance2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERVAL_DATETIME",
        "DUID",
        "TOTALCLEARED",
        "ACTUALMW",
        "ROC",
        "AVAILABILITY",
        "LOWERREG",
        "RAISEREG",
        "STRIGLM",
        "LTRIGLM",
        "MWERROR",
        "MAX_MWERROR",
        "LECOUNT",
        "SECOUNT",
        "STATUS",
        "PARTICIPANT_STATUS_ACTION",
        "OPERATING_MODE",
        "LASTCHANGED",
        "ADG_ID",
        "SEMIDISPATCHCAP",
        "CONFORMANCE_MODE",
    ];
    type Row<'row> = DispatchUnitConformance2Row<'row>;
    type FieldMapping = DispatchUnitConformance2Mapping;
    type PrimaryKey = DispatchUnitConformance2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchUnitConformance2Row {
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            totalcleared: row
                .get_opt_custom_parsed_at_idx(
                    "totalcleared",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            actualmw: row
                .get_opt_custom_parsed_at_idx(
                    "actualmw",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            roc: row
                .get_opt_custom_parsed_at_idx(
                    "roc",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availability: row
                .get_opt_custom_parsed_at_idx(
                    "availability",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreg: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            striglm: row
                .get_opt_custom_parsed_at_idx(
                    "striglm",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ltriglm: row
                .get_opt_custom_parsed_at_idx(
                    "ltriglm",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwerror: row
                .get_opt_custom_parsed_at_idx(
                    "mwerror",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            max_mwerror: row
                .get_opt_custom_parsed_at_idx(
                    "max_mwerror",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lecount: row.get_opt_parsed_at_idx("lecount", field_mapping.0[12])?,
            secount: row.get_opt_parsed_at_idx("secount", field_mapping.0[13])?,
            status: row.get_opt_range("status", field_mapping.0[14])?,
            participant_status_action: row
                .get_opt_range("participant_status_action", field_mapping.0[15])?,
            operating_mode: row.get_opt_range("operating_mode", field_mapping.0[16])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[17],
                    mmsdm_core::mms_datetime::parse,
                )?,
            adg_id: row.get_opt_range("adg_id", field_mapping.0[18])?,
            semidispatchcap: row
                .get_opt_custom_parsed_at_idx(
                    "semidispatchcap",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            conformance_mode: row
                .get_opt_custom_parsed_at_idx(
                    "conformance_mode",
                    field_mapping.0[20],
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
        Ok(DispatchUnitConformance2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchUnitConformance2PrimaryKey {
        DispatchUnitConformance2PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_unit_conformance_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchUnitConformance2Row {
            interval_datetime: row.interval_datetime.clone(),
            duid: row.duid.clone(),
            totalcleared: row.totalcleared.clone(),
            actualmw: row.actualmw.clone(),
            roc: row.roc.clone(),
            availability: row.availability.clone(),
            lowerreg: row.lowerreg.clone(),
            raisereg: row.raisereg.clone(),
            striglm: row.striglm.clone(),
            ltriglm: row.ltriglm.clone(),
            mwerror: row.mwerror.clone(),
            max_mwerror: row.max_mwerror.clone(),
            lecount: row.lecount.clone(),
            secount: row.secount.clone(),
            status: row.status.clone(),
            participant_status_action: row.participant_status_action.clone(),
            operating_mode: row.operating_mode.clone(),
            lastchanged: row.lastchanged.clone(),
            adg_id: row.adg_id.clone(),
            semidispatchcap: row.semidispatchcap.clone(),
            conformance_mode: row.conformance_mode.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchUnitConformance2PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchUnitConformance2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchUnitConformance2Row<'data> {
    type Row<'other> = DispatchUnitConformance2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchUnitConformance2Row<'data> {
    type PrimaryKey = DispatchUnitConformance2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchUnitConformance2PrimaryKey {
    type Row<'other> = DispatchUnitConformance2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchUnitConformance2PrimaryKey {
    type PrimaryKey = DispatchUnitConformance2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchUnitConformance2 {
    type Builder = DispatchUnitConformance2Builder;
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
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "totalcleared",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "actualmw",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "roc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availability",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerreg",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereg",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "striglm",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ltriglm",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwerror",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "max_mwerror",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lecount",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "secount",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participant_status_action",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operating_mode",
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
                    "adg_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "semidispatchcap",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "conformance_mode",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchUnitConformance2Builder {
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            totalcleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            actualmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            roc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lowerreg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            raisereg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            striglm_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ltriglm_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            mwerror_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            max_mwerror_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lecount_array: arrow::array::builder::Int64Builder::new(),
            secount_array: arrow::array::builder::Int64Builder::new(),
            status_array: arrow::array::builder::StringBuilder::new(),
            participant_status_action_array: arrow::array::builder::StringBuilder::new(),
            operating_mode_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            adg_id_array: arrow::array::builder::StringBuilder::new(),
            semidispatchcap_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            conformance_mode_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .totalcleared_array
            .append_option({
                row.totalcleared
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .actualmw_array
            .append_option({
                row.actualmw
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .roc_array
            .append_option({
                row.roc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .availability_array
            .append_option({
                row.availability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lowerreg_array
            .append_option({
                row.lowerreg
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .raisereg_array
            .append_option({
                row.raisereg
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .striglm_array
            .append_option({
                row.striglm
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ltriglm_array
            .append_option({
                row.ltriglm
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .mwerror_array
            .append_option({
                row.mwerror
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .max_mwerror_array
            .append_option({
                row.max_mwerror
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.lecount_array.append_option(row.lecount);
        builder.secount_array.append_option(row.secount);
        builder.status_array.append_option(row.status());
        builder
            .participant_status_action_array
            .append_option(row.participant_status_action());
        builder.operating_mode_array.append_option(row.operating_mode());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.adg_id_array.append_option(row.adg_id());
        builder
            .semidispatchcap_array
            .append_option({
                row.semidispatchcap
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .conformance_mode_array
            .append_option({
                row.conformance_mode
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
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalcleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.actualmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.roc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.striglm_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ltriglm_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwerror_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.max_mwerror_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lecount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.secount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.participant_status_action_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.operating_mode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.adg_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.semidispatchcap_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.conformance_mode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchUnitConformance2Builder {
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    totalcleared_array: arrow::array::builder::Decimal128Builder,
    actualmw_array: arrow::array::builder::Decimal128Builder,
    roc_array: arrow::array::builder::Decimal128Builder,
    availability_array: arrow::array::builder::Decimal128Builder,
    lowerreg_array: arrow::array::builder::Decimal128Builder,
    raisereg_array: arrow::array::builder::Decimal128Builder,
    striglm_array: arrow::array::builder::Decimal128Builder,
    ltriglm_array: arrow::array::builder::Decimal128Builder,
    mwerror_array: arrow::array::builder::Decimal128Builder,
    max_mwerror_array: arrow::array::builder::Decimal128Builder,
    lecount_array: arrow::array::builder::Int64Builder,
    secount_array: arrow::array::builder::Int64Builder,
    status_array: arrow::array::builder::StringBuilder,
    participant_status_action_array: arrow::array::builder::StringBuilder,
    operating_mode_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    adg_id_array: arrow::array::builder::StringBuilder,
    semidispatchcap_array: arrow::array::builder::Decimal128Builder,
    conformance_mode_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchUnitScada1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchUnitScada1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchUnitScada1 {
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
pub struct DispatchUnitScada1Mapping([usize; 3]);
/// # Summary
///
/// ## DISPATCH_UNIT_SCADA
///  _Dispatchable unit MW from SCADA at the start of the dispatch interval. The table includes all scheduled and semi-scheduled (and non-scheduled units where SCADA is available)_
///
/// * Data Set Name: Dispatch
/// * File Name: Unit Scada
/// * Data Version: 1
///
/// # Description
///  DISPATCH_UNIT_SCADA data  is public data, and is available to all participants. Source DISPATCH_UNIT_SCADA shows data for every 5 minutes for all scheduled units Volume Rows per day: 288 per each scheduled, semi-scheduled (and non-scheduled unit where SCADA is available)
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchUnitScada1Row<'data> {
    /// Date Time of the Dispatch Interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Instantaneous MW reading from SCADA at the start of the Dispatch interval
    pub scadavalue: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchUnitScada1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for DispatchUnitScada1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "UNIT_SCADA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchUnitScada1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &["SETTLEMENTDATE", "DUID", "SCADAVALUE"];
    type Row<'row> = DispatchUnitScada1Row<'row>;
    type FieldMapping = DispatchUnitScada1Mapping;
    type PrimaryKey = DispatchUnitScada1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchUnitScada1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            scadavalue: row
                .get_opt_custom_parsed_at_idx(
                    "scadavalue",
                    field_mapping.0[2],
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
        Ok(DispatchUnitScada1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchUnitScada1PrimaryKey {
        DispatchUnitScada1PrimaryKey {
            duid: row.duid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_unit_scada_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchUnitScada1Row {
            settlementdate: row.settlementdate.clone(),
            duid: row.duid.clone(),
            scadavalue: row.scadavalue.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchUnitScada1PrimaryKey {
    pub duid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchUnitScada1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchUnitScada1Row<'data> {
    type Row<'other> = DispatchUnitScada1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchUnitScada1Row<'data> {
    type PrimaryKey = DispatchUnitScada1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchUnitScada1PrimaryKey {
    type Row<'other> = DispatchUnitScada1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchUnitScada1PrimaryKey {
    type PrimaryKey = DispatchUnitScada1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchUnitScada1 {
    type Builder = DispatchUnitScada1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "scadavalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchUnitScada1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            scadavalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .scadavalue_array
            .append_option({
                row.scadavalue
                    .map(|mut val| {
                        val.rescale(6);
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.scadavalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchUnitScada1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    scadavalue_array: arrow::array::builder::Decimal128Builder,
}
pub struct DispatchIntermittentForecastTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchIntermittentForecastTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchIntermittentForecastTrk1 {
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
pub struct DispatchIntermittentForecastTrk1Mapping([usize; 5]);
/// # Summary
///
/// ## INTERMITTENT_FORECAST_TRK
///  _Uniquely tracks which Intermittent Generation forecast was used for the DUID in which Dispatch run_
///
/// * Data Set Name: Dispatch
/// * File Name: Intermittent Forecast Trk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchIntermittentForecastTrk1Row<'data> {
    /// DateTime of the Dispatch run (dispatch interval ending)
    pub settlementdate: chrono::NaiveDateTime,
    /// Tracks to INTERMITTENT_DS_RUN.DUID
    pub duid: core::ops::Range<usize>,
    /// Tracks to INTERMITTENT_DS_RUN.ORIGIN, except when the forecast used is either SCADA or FCST or Last Target
    pub origin: core::ops::Range<usize>,
    /// Tracks to INTERMITTENT_DS_RUN.FORECAST_PRIORITY, except for -1 which denotes SCADA or FCST, and 0 which denotes Last Target
    pub forecast_priority: Option<rust_decimal::Decimal>,
    /// Tracks to INTERMITTENT_DS_RUN.OFFERDATETIME
    pub offerdatetime: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchIntermittentForecastTrk1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn origin(&self) -> Option<&str> {
        if self.origin.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.origin.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchIntermittentForecastTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "INTERMITTENT_FORECAST_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchIntermittentForecastTrk1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "DUID",
        "ORIGIN",
        "FORECAST_PRIORITY",
        "OFFERDATETIME",
    ];
    type Row<'row> = DispatchIntermittentForecastTrk1Row<'row>;
    type FieldMapping = DispatchIntermittentForecastTrk1Mapping;
    type PrimaryKey = DispatchIntermittentForecastTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchIntermittentForecastTrk1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            origin: row.get_opt_range("origin", field_mapping.0[2])?,
            forecast_priority: row
                .get_opt_custom_parsed_at_idx(
                    "forecast_priority",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            offerdatetime: row
                .get_opt_custom_parsed_at_idx(
                    "offerdatetime",
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
        Ok(DispatchIntermittentForecastTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchIntermittentForecastTrk1PrimaryKey {
        DispatchIntermittentForecastTrk1PrimaryKey {
            duid: row.duid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "dispatch_intermittent_forecast_trk_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchIntermittentForecastTrk1Row {
            settlementdate: row.settlementdate.clone(),
            duid: row.duid.clone(),
            origin: row.origin.clone(),
            forecast_priority: row.forecast_priority.clone(),
            offerdatetime: row.offerdatetime.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchIntermittentForecastTrk1PrimaryKey {
    pub duid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchIntermittentForecastTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchIntermittentForecastTrk1Row<'data> {
    type Row<'other> = DispatchIntermittentForecastTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for DispatchIntermittentForecastTrk1Row<'data> {
    type PrimaryKey = DispatchIntermittentForecastTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchIntermittentForecastTrk1PrimaryKey {
    type Row<'other> = DispatchIntermittentForecastTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchIntermittentForecastTrk1PrimaryKey {
    type PrimaryKey = DispatchIntermittentForecastTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchIntermittentForecastTrk1 {
    type Builder = DispatchIntermittentForecastTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "origin",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "forecast_priority",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
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
        DispatchIntermittentForecastTrk1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            origin_array: arrow::array::builder::StringBuilder::new(),
            forecast_priority_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.origin_array.append_option(row.origin());
        builder
            .forecast_priority_array
            .append_option({
                row.forecast_priority
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .offerdatetime_array
            .append_option(
                row.offerdatetime.map(|val| val.and_utc().timestamp_millis()),
            );
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.origin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.forecast_priority_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchIntermittentForecastTrk1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    origin_array: arrow::array::builder::StringBuilder,
    forecast_priority_array: arrow::array::builder::Decimal128Builder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DispatchNegativeResidue1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchNegativeResidue1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchNegativeResidue1 {
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
pub struct DispatchNegativeResidue1Mapping([usize; 15]);
/// # Summary
///
/// ## NEGATIVE_RESIDUE
///  _Shows the inputs provided to the Negative Residue Constraints in the Dispatch horizon_
///
/// * Data Set Name: Dispatch
/// * File Name: Negative Residue
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * DIRECTIONAL_INTERCONNECTORID
/// * NRM_DATETIME
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchNegativeResidue1Row<'data> {
    /// Dispatch Interval
    pub settlementdate: chrono::NaiveDateTime,
    /// The time that residue information is processed
    pub nrm_datetime: chrono::NaiveDateTime,
    /// Negative residue related direction interconnector id
    pub directional_interconnectorid: core::ops::Range<usize>,
    /// Is 1 if negative residue process is on, else is 0
    pub nrm_activated_flag: Option<rust_decimal::Decimal>,
    /// Negative residue triggering amount
    pub cumul_negresidue_amount: Option<rust_decimal::Decimal>,
    /// Previous trading interval cumulative negative residue amount
    pub cumul_negresidue_prev_ti: Option<rust_decimal::Decimal>,
    /// Current trading interval negative residue amount
    pub negresidue_current_ti: Option<rust_decimal::Decimal>,
    /// The cumulative negative residue for the next trading interval (PD)
    pub negresidue_pd_next_ti: Option<rust_decimal::Decimal>,
    /// SubjectToReview, Indeterminate, Accepted or Rejected
    pub price_revision: core::ops::Range<usize>,
    /// Predispatch sequence number
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// The starting DI when NRM event is active
    pub event_activated_di: Option<chrono::NaiveDateTime>,
    /// The finishing DI when NRM event stops being active.
    pub event_deactivated_di: Option<chrono::NaiveDateTime>,
    /// Count of the number of DIs not binding by this constraint
    pub di_notbinding_count: Option<rust_decimal::Decimal>,
    /// Count of the number of DIs violated by this constraint
    pub di_violated_count: Option<rust_decimal::Decimal>,
    /// 1 if constraint is blocked, else 0
    pub nrmconstraint_blocked_flag: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchNegativeResidue1Row<'data> {
    pub fn directional_interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.directional_interconnectorid.clone(),
        )
    }
    pub fn price_revision(&self) -> Option<&str> {
        if self.price_revision.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.price_revision.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for DispatchNegativeResidue1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCH";
    const TABLE_NAME: &'static str = "NEGATIVE_RESIDUE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchNegativeResidue1Mapping([
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
        "SETTLEMENTDATE",
        "NRM_DATETIME",
        "DIRECTIONAL_INTERCONNECTORID",
        "NRM_ACTIVATED_FLAG",
        "CUMUL_NEGRESIDUE_AMOUNT",
        "CUMUL_NEGRESIDUE_PREV_TI",
        "NEGRESIDUE_CURRENT_TI",
        "NEGRESIDUE_PD_NEXT_TI",
        "PRICE_REVISION",
        "PREDISPATCHSEQNO",
        "EVENT_ACTIVATED_DI",
        "EVENT_DEACTIVATED_DI",
        "DI_NOTBINDING_COUNT",
        "DI_VIOLATED_COUNT",
        "NRMCONSTRAINT_BLOCKED_FLAG",
    ];
    type Row<'row> = DispatchNegativeResidue1Row<'row>;
    type FieldMapping = DispatchNegativeResidue1Mapping;
    type PrimaryKey = DispatchNegativeResidue1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchNegativeResidue1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            nrm_datetime: row
                .get_custom_parsed_at_idx(
                    "nrm_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            directional_interconnectorid: row
                .get_range("directional_interconnectorid", field_mapping.0[2])?,
            nrm_activated_flag: row
                .get_opt_custom_parsed_at_idx(
                    "nrm_activated_flag",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_negresidue_amount: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_negresidue_amount",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cumul_negresidue_prev_ti: row
                .get_opt_custom_parsed_at_idx(
                    "cumul_negresidue_prev_ti",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            negresidue_current_ti: row
                .get_opt_custom_parsed_at_idx(
                    "negresidue_current_ti",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            negresidue_pd_next_ti: row
                .get_opt_custom_parsed_at_idx(
                    "negresidue_pd_next_ti",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            price_revision: row.get_opt_range("price_revision", field_mapping.0[8])?,
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[9])?,
            event_activated_di: row
                .get_opt_custom_parsed_at_idx(
                    "event_activated_di",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            event_deactivated_di: row
                .get_opt_custom_parsed_at_idx(
                    "event_deactivated_di",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            di_notbinding_count: row
                .get_opt_custom_parsed_at_idx(
                    "di_notbinding_count",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            di_violated_count: row
                .get_opt_custom_parsed_at_idx(
                    "di_violated_count",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            nrmconstraint_blocked_flag: row
                .get_opt_custom_parsed_at_idx(
                    "nrmconstraint_blocked_flag",
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
        Ok(DispatchNegativeResidue1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchNegativeResidue1PrimaryKey {
        DispatchNegativeResidue1PrimaryKey {
            directional_interconnectorid: row.directional_interconnectorid().to_string(),
            nrm_datetime: row.nrm_datetime,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatch_negative_residue_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchNegativeResidue1Row {
            settlementdate: row.settlementdate.clone(),
            nrm_datetime: row.nrm_datetime.clone(),
            directional_interconnectorid: row.directional_interconnectorid.clone(),
            nrm_activated_flag: row.nrm_activated_flag.clone(),
            cumul_negresidue_amount: row.cumul_negresidue_amount.clone(),
            cumul_negresidue_prev_ti: row.cumul_negresidue_prev_ti.clone(),
            negresidue_current_ti: row.negresidue_current_ti.clone(),
            negresidue_pd_next_ti: row.negresidue_pd_next_ti.clone(),
            price_revision: row.price_revision.clone(),
            predispatchseqno: row.predispatchseqno.clone(),
            event_activated_di: row.event_activated_di.clone(),
            event_deactivated_di: row.event_deactivated_di.clone(),
            di_notbinding_count: row.di_notbinding_count.clone(),
            di_violated_count: row.di_violated_count.clone(),
            nrmconstraint_blocked_flag: row.nrmconstraint_blocked_flag.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchNegativeResidue1PrimaryKey {
    pub directional_interconnectorid: alloc::string::String,
    pub nrm_datetime: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchNegativeResidue1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchNegativeResidue1Row<'data> {
    type Row<'other> = DispatchNegativeResidue1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.directional_interconnectorid() == row.directional_interconnectorid()
            && self.nrm_datetime == row.nrm_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchNegativeResidue1Row<'data> {
    type PrimaryKey = DispatchNegativeResidue1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.directional_interconnectorid() == key.directional_interconnectorid
            && self.nrm_datetime == key.nrm_datetime
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchNegativeResidue1PrimaryKey {
    type Row<'other> = DispatchNegativeResidue1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.directional_interconnectorid == row.directional_interconnectorid()
            && self.nrm_datetime == row.nrm_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchNegativeResidue1PrimaryKey {
    type PrimaryKey = DispatchNegativeResidue1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.directional_interconnectorid == key.directional_interconnectorid
            && self.nrm_datetime == key.nrm_datetime
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchNegativeResidue1 {
    type Builder = DispatchNegativeResidue1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "nrm_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "directional_interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "nrm_activated_flag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_negresidue_amount",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cumul_negresidue_prev_ti",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "negresidue_current_ti",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "negresidue_pd_next_ti",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "price_revision",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "event_activated_di",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "event_deactivated_di",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "di_notbinding_count",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "di_violated_count",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nrmconstraint_blocked_flag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchNegativeResidue1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            nrm_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            directional_interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            nrm_activated_flag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            cumul_negresidue_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            cumul_negresidue_prev_ti_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            negresidue_current_ti_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            negresidue_pd_next_ti_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            price_revision_array: arrow::array::builder::StringBuilder::new(),
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            event_activated_di_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            event_deactivated_di_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            di_notbinding_count_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            di_violated_count_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            nrmconstraint_blocked_flag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .nrm_datetime_array
            .append_value(row.nrm_datetime.and_utc().timestamp_millis());
        builder
            .directional_interconnectorid_array
            .append_value(row.directional_interconnectorid());
        builder
            .nrm_activated_flag_array
            .append_option({
                row.nrm_activated_flag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .cumul_negresidue_amount_array
            .append_option({
                row.cumul_negresidue_amount
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cumul_negresidue_prev_ti_array
            .append_option({
                row.cumul_negresidue_prev_ti
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .negresidue_current_ti_array
            .append_option({
                row.negresidue_current_ti
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .negresidue_pd_next_ti_array
            .append_option({
                row.negresidue_pd_next_ti
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder.price_revision_array.append_option(row.price_revision());
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().and_utc().timestamp_millis());
        builder
            .event_activated_di_array
            .append_option(
                row.event_activated_di.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .event_deactivated_di_array
            .append_option(
                row.event_deactivated_di.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .di_notbinding_count_array
            .append_option({
                row.di_notbinding_count
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .di_violated_count_array
            .append_option({
                row.di_violated_count
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .nrmconstraint_blocked_flag_array
            .append_option({
                row.nrmconstraint_blocked_flag
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nrm_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.directional_interconnectorid_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nrm_activated_flag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cumul_negresidue_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.cumul_negresidue_prev_ti_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.negresidue_current_ti_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.negresidue_pd_next_ti_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.price_revision_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.event_activated_di_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.event_deactivated_di_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.di_notbinding_count_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.di_violated_count_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.nrmconstraint_blocked_flag_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchNegativeResidue1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    nrm_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    directional_interconnectorid_array: arrow::array::builder::StringBuilder,
    nrm_activated_flag_array: arrow::array::builder::Decimal128Builder,
    cumul_negresidue_amount_array: arrow::array::builder::Decimal128Builder,
    cumul_negresidue_prev_ti_array: arrow::array::builder::Decimal128Builder,
    negresidue_current_ti_array: arrow::array::builder::Decimal128Builder,
    negresidue_pd_next_ti_array: arrow::array::builder::Decimal128Builder,
    price_revision_array: arrow::array::builder::StringBuilder,
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    event_activated_di_array: arrow::array::builder::TimestampMillisecondBuilder,
    event_deactivated_di_array: arrow::array::builder::TimestampMillisecondBuilder,
    di_notbinding_count_array: arrow::array::builder::Decimal128Builder,
    di_violated_count_array: arrow::array::builder::Decimal128Builder,
    nrmconstraint_blocked_flag_array: arrow::array::builder::Decimal128Builder,
}
