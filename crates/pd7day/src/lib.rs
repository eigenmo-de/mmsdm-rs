#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct Pd7dayCasesolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &Pd7dayCasesolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl Pd7dayCasesolution1 {
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
pub struct Pd7dayCasesolution1Mapping([usize; 3]);
/// # Summary
///
/// ## PD7DAY_CASESOLUTION
///
/// PD7DAY case solution table
///
/// * Data Set Name: Pd7day
/// * File Name: Casesolution
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
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct Pd7dayCasesolution1Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run.
    pub intervention: rust_decimal::Decimal,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> Pd7dayCasesolution1Row<'data> {}
impl mmsdm_core::GetTable for Pd7dayCasesolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PD7DAY";
    const TABLE_NAME: &'static str = "CASESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = Pd7dayCasesolution1Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVENTION",
        "LASTCHANGED",
    ];
    type Row<'row> = Pd7dayCasesolution1Row<'row>;
    type FieldMapping = Pd7dayCasesolution1Mapping;
    type PrimaryKey = Pd7dayCasesolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(Pd7dayCasesolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
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
        Ok(Pd7dayCasesolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> Pd7dayCasesolution1PrimaryKey {
        Pd7dayCasesolution1PrimaryKey {
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("pd7day_casesolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        Pd7dayCasesolution1Row {
            run_datetime: row.run_datetime.clone(),
            intervention: row.intervention.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pd7dayCasesolution1PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for Pd7dayCasesolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayCasesolution1Row<'data> {
    type Row<'other> = Pd7dayCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for Pd7dayCasesolution1Row<'data> {
    type PrimaryKey = Pd7dayCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayCasesolution1PrimaryKey {
    type Row<'other> = Pd7dayCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for Pd7dayCasesolution1PrimaryKey {
    type PrimaryKey = Pd7dayCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for Pd7dayCasesolution1 {
    type Builder = Pd7dayCasesolution1Builder;
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
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
        Pd7dayCasesolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
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
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct Pd7dayCasesolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct Pd7dayConstraintsolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &Pd7dayConstraintsolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl Pd7dayConstraintsolution1 {
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
pub struct Pd7dayConstraintsolution1Mapping([usize; 9]);
/// # Summary
///
/// ## PD7DAY_CONSTRAINTSOLUTION
///
/// PD7DAY constraint solution
///
/// * Data Set Name: Pd7day
/// * File Name: Constraintsolution
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
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * INTERVENTION
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct Pd7dayConstraintsolution1Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run.
    pub intervention: rust_decimal::Decimal,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    /// Right Hand Side value in the capacity evaluation in MW
    pub rhs: Option<rust_decimal::Decimal>,
    /// Marginal cost of constraint (>0 if binding) in $/MW
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Amount of Violation (>0 if violating) in MW
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values in MW
    pub lhs: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> Pd7dayConstraintsolution1Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for Pd7dayConstraintsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PD7DAY";
    const TABLE_NAME: &'static str = "CONSTRAINTSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = Pd7dayConstraintsolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVENTION",
        "INTERVAL_DATETIME",
        "CONSTRAINTID",
        "RHS",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "LHS",
        "LASTCHANGED",
    ];
    type Row<'row> = Pd7dayConstraintsolution1Row<'row>;
    type FieldMapping = Pd7dayConstraintsolution1Mapping;
    type PrimaryKey = Pd7dayConstraintsolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(Pd7dayConstraintsolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[3])?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violationdegree",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lhs: row
                .get_opt_custom_parsed_at_idx(
                    "lhs",
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
        Ok(Pd7dayConstraintsolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> Pd7dayConstraintsolution1PrimaryKey {
        Pd7dayConstraintsolution1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            interval_datetime: row.interval_datetime,
            intervention: row.intervention,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("pd7day_constraintsolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        Pd7dayConstraintsolution1Row {
            run_datetime: row.run_datetime.clone(),
            intervention: row.intervention.clone(),
            interval_datetime: row.interval_datetime.clone(),
            constraintid: row.constraintid.clone(),
            rhs: row.rhs.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            lhs: row.lhs.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pd7dayConstraintsolution1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for Pd7dayConstraintsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayConstraintsolution1Row<'data> {
    type Row<'other> = Pd7dayConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.intervention == row.intervention
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for Pd7dayConstraintsolution1Row<'data> {
    type PrimaryKey = Pd7dayConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.intervention == key.intervention
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayConstraintsolution1PrimaryKey {
    type Row<'other> = Pd7dayConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.intervention == row.intervention
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for Pd7dayConstraintsolution1PrimaryKey {
    type PrimaryKey = Pd7dayConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.intervention == key.intervention
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for Pd7dayConstraintsolution1 {
    type Builder = Pd7dayConstraintsolution1Builder;
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
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
                    "constraintid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
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
                    "lhs",
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
        Pd7dayConstraintsolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
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
            .lhs_array
            .append_option({
                row.lhs
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct Pd7dayConstraintsolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    rhs_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lhs_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct Pd7dayInterconnectorsolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &Pd7dayInterconnectorsolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl Pd7dayInterconnectorsolution1 {
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
pub struct Pd7dayInterconnectorsolution1Mapping([usize; 21]);
/// # Summary
///
/// ## PD7DAY_INTERCONNECTORSOLUTION
///
/// PD7DAY intereconnector solution
///
/// * Data Set Name: Pd7day
/// * File Name: Interconnectorsolution
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
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * INTERVENTION
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct Pd7dayInterconnectorsolution1Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run.
    pub intervention: rust_decimal::Decimal,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// SCADA MW Flow measured at Run start. For periods subsequent to the first period of a PD7DAY run, this value represents the cleared target for the previous period of that PD7DAY run.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Cleared Interconnector loading level (MW)
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Interconnector Losses at cleared flow
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Marginal cost of Interconnector standing data limits (if binding)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation of Interconnector standing data limits
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor at the cleared flow
    pub marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    pub exportconstraintid: core::ops::Range<usize>,
    /// Generic Constraint setting the import limit
    pub importconstraintid: core::ops::Range<usize>,
    /// Calculated export limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasimportlimit: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor >= 0)
    pub local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor >= 0)
    pub local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_import: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> Pd7dayInterconnectorsolution1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn exportconstraintid(&self) -> Option<&str> {
        if self.exportconstraintid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.exportconstraintid.clone(),
                ),
            )
        }
    }
    pub fn importconstraintid(&self) -> Option<&str> {
        if self.importconstraintid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.importconstraintid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for Pd7dayInterconnectorsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PD7DAY";
    const TABLE_NAME: &'static str = "INTERCONNECTORSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = Pd7dayInterconnectorsolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVENTION",
        "INTERVAL_DATETIME",
        "INTERCONNECTORID",
        "METEREDMWFLOW",
        "MWFLOW",
        "MWLOSSES",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "EXPORTLIMIT",
        "IMPORTLIMIT",
        "MARGINALLOSS",
        "EXPORTCONSTRAINTID",
        "IMPORTCONSTRAINTID",
        "FCASEXPORTLIMIT",
        "FCASIMPORTLIMIT",
        "LOCAL_PRICE_ADJUSTMENT_EXPORT",
        "LOCALLY_CONSTRAINED_EXPORT",
        "LOCAL_PRICE_ADJUSTMENT_IMPORT",
        "LOCALLY_CONSTRAINED_IMPORT",
        "LASTCHANGED",
    ];
    type Row<'row> = Pd7dayInterconnectorsolution1Row<'row>;
    type FieldMapping = Pd7dayInterconnectorsolution1Mapping;
    type PrimaryKey = Pd7dayInterconnectorsolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(Pd7dayInterconnectorsolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[3])?,
            meteredmwflow: row
                .get_opt_custom_parsed_at_idx(
                    "meteredmwflow",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwlosses: row
                .get_opt_custom_parsed_at_idx(
                    "mwlosses",
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
            exportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "exportlimit",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importlimit: row
                .get_opt_custom_parsed_at_idx(
                    "importlimit",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalloss: row
                .get_opt_custom_parsed_at_idx(
                    "marginalloss",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportconstraintid: row
                .get_opt_range("exportconstraintid", field_mapping.0[12])?,
            importconstraintid: row
                .get_opt_range("importconstraintid", field_mapping.0[13])?,
            fcasexportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "fcasexportlimit",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fcasimportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "fcasimportlimit",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            local_price_adjustment_export: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_export",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_export: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_export",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            local_price_adjustment_import: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_import",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_import: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_import",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[20],
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
        Ok(Pd7dayInterconnectorsolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> Pd7dayInterconnectorsolution1PrimaryKey {
        Pd7dayInterconnectorsolution1PrimaryKey {
            interconnectorid: row.interconnectorid().to_string(),
            interval_datetime: row.interval_datetime,
            intervention: row.intervention,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("pd7day_interconnectorsolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        Pd7dayInterconnectorsolution1Row {
            run_datetime: row.run_datetime.clone(),
            intervention: row.intervention.clone(),
            interval_datetime: row.interval_datetime.clone(),
            interconnectorid: row.interconnectorid.clone(),
            meteredmwflow: row.meteredmwflow.clone(),
            mwflow: row.mwflow.clone(),
            mwlosses: row.mwlosses.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            exportlimit: row.exportlimit.clone(),
            importlimit: row.importlimit.clone(),
            marginalloss: row.marginalloss.clone(),
            exportconstraintid: row.exportconstraintid.clone(),
            importconstraintid: row.importconstraintid.clone(),
            fcasexportlimit: row.fcasexportlimit.clone(),
            fcasimportlimit: row.fcasimportlimit.clone(),
            local_price_adjustment_export: row.local_price_adjustment_export.clone(),
            locally_constrained_export: row.locally_constrained_export.clone(),
            local_price_adjustment_import: row.local_price_adjustment_import.clone(),
            locally_constrained_import: row.locally_constrained_import.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pd7dayInterconnectorsolution1PrimaryKey {
    pub interconnectorid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for Pd7dayInterconnectorsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayInterconnectorsolution1Row<'data> {
    type Row<'other> = Pd7dayInterconnectorsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.intervention == row.intervention
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for Pd7dayInterconnectorsolution1Row<'data> {
    type PrimaryKey = Pd7dayInterconnectorsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.intervention == key.intervention
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayInterconnectorsolution1PrimaryKey {
    type Row<'other> = Pd7dayInterconnectorsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.intervention == row.intervention
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for Pd7dayInterconnectorsolution1PrimaryKey {
    type PrimaryKey = Pd7dayInterconnectorsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.intervention == key.intervention
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for Pd7dayInterconnectorsolution1 {
    type Builder = Pd7dayInterconnectorsolution1Builder;
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
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
                    "interconnectorid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
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
                    "exportconstraintid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importconstraintid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
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
        Pd7dayInterconnectorsolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
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
            exportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            importlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalloss_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            exportconstraintid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            importconstraintid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
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
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.interconnectorid_array.append_value(row.interconnectorid());
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
        builder.exportconstraintid_array.append_option(row.exportconstraintid());
        builder.importconstraintid_array.append_option(row.importconstraintid());
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
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
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
                    alloc::sync::Arc::new(builder.exportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalloss_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportconstraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importconstraintid_array.finish())
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct Pd7dayInterconnectorsolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interconnectorid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    meteredmwflow_array: arrow::array::builder::Decimal128Builder,
    mwflow_array: arrow::array::builder::Decimal128Builder,
    mwlosses_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    exportlimit_array: arrow::array::builder::Decimal128Builder,
    importlimit_array: arrow::array::builder::Decimal128Builder,
    marginalloss_array: arrow::array::builder::Decimal128Builder,
    exportconstraintid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    importconstraintid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    fcasexportlimit_array: arrow::array::builder::Decimal128Builder,
    fcasimportlimit_array: arrow::array::builder::Decimal128Builder,
    local_price_adjustment_export_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_export_array: arrow::array::builder::Decimal128Builder,
    local_price_adjustment_import_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_import_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct Pd7dayMarketSummary2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &Pd7dayMarketSummary2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl Pd7dayMarketSummary2 {
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
pub struct Pd7dayMarketSummary2Mapping([usize; 3]);
/// # Summary
///
/// ## PD7DAY_MARKET_SUMMARY
///
/// PD7DAY market summary showing calculated gas fuel forecasts
///
/// * Data Set Name: Pd7day
/// * File Name: Market Summary
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
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct Pd7dayMarketSummary2Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// The total gas consumption in TJ
    pub gpg_fuel_forecast_tj: Option<rust_decimal::Decimal>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> Pd7dayMarketSummary2Row<'data> {}
impl mmsdm_core::GetTable for Pd7dayMarketSummary2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "PD7DAY";
    const TABLE_NAME: &'static str = "MARKET_SUMMARY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = Pd7dayMarketSummary2Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "GPG_FUEL_FORECAST_TJ",
    ];
    type Row<'row> = Pd7dayMarketSummary2Row<'row>;
    type FieldMapping = Pd7dayMarketSummary2Mapping;
    type PrimaryKey = Pd7dayMarketSummary2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(Pd7dayMarketSummary2Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            gpg_fuel_forecast_tj: row
                .get_opt_custom_parsed_at_idx(
                    "gpg_fuel_forecast_tj",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
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
        Ok(Pd7dayMarketSummary2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> Pd7dayMarketSummary2PrimaryKey {
        Pd7dayMarketSummary2PrimaryKey {
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("pd7day_market_summary_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        Pd7dayMarketSummary2Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            gpg_fuel_forecast_tj: row.gpg_fuel_forecast_tj.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pd7dayMarketSummary2PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for Pd7dayMarketSummary2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayMarketSummary2Row<'data> {
    type Row<'other> = Pd7dayMarketSummary2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for Pd7dayMarketSummary2Row<'data> {
    type PrimaryKey = Pd7dayMarketSummary2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayMarketSummary2PrimaryKey {
    type Row<'other> = Pd7dayMarketSummary2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for Pd7dayMarketSummary2PrimaryKey {
    type PrimaryKey = Pd7dayMarketSummary2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for Pd7dayMarketSummary2 {
    type Builder = Pd7dayMarketSummary2Builder;
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
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "gpg_fuel_forecast_tj",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        Pd7dayMarketSummary2Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            gpg_fuel_forecast_tj_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder
            .gpg_fuel_forecast_tj_array
            .append_option({
                row.gpg_fuel_forecast_tj
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gpg_fuel_forecast_tj_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct Pd7dayMarketSummary2Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    gpg_fuel_forecast_tj_array: arrow::array::builder::Decimal128Builder,
}
pub struct Pd7dayPricesolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &Pd7dayPricesolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl Pd7dayPricesolution1 {
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
pub struct Pd7dayPricesolution1Mapping([usize; 18]);
/// # Summary
///
/// ## PD7DAY_PRICESOLUTION
///
/// PD7DAY price solution
///
/// * Data Set Name: Pd7day
/// * File Name: Pricesolution
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
/// * INTERVENTION
/// * REGIONID
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct Pd7dayPricesolution1Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run.
    pub intervention: rust_decimal::Decimal,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Region Reference Price (Energy)
    pub rrp: Option<rust_decimal::Decimal>,
    /// Regional Lower 1Sec Price - RegionSolution element L1Price attribute
    pub lower1secrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower6Sec)
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower60Sec)
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower5Min)
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (LowerReg)
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// Regional Raise 1Sec Price - R1Price attribute after capping/flooring
    pub raise1secrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise6Sec)
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise60Sec)
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise5Min)
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Region Reference Price (RaiseReg)
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Energy Storage for BDU at the start of the interval(MWh) - Region Aggregated
    pub bdu_initial_energy_storage: Option<rust_decimal::Decimal>,
    /// Energy storage for Daily Energy Constrained Scheduled Generating Units at the start of the interval(MWh) - Region Aggregated
    pub decgen_initial_energy_storage: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> Pd7dayPricesolution1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for Pd7dayPricesolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PD7DAY";
    const TABLE_NAME: &'static str = "PRICESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = Pd7dayPricesolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVENTION",
        "INTERVAL_DATETIME",
        "REGIONID",
        "RRP",
        "LOWER1SECRRP",
        "LOWER6SECRRP",
        "LOWER60SECRRP",
        "LOWER5MINRRP",
        "LOWERREGRRP",
        "RAISE1SECRRP",
        "RAISE6SECRRP",
        "RAISE60SECRRP",
        "RAISE5MINRRP",
        "RAISEREGRRP",
        "LASTCHANGED",
        "BDU_INITIAL_ENERGY_STORAGE",
        "DECGEN_INITIAL_ENERGY_STORAGE",
    ];
    type Row<'row> = Pd7dayPricesolution1Row<'row>;
    type FieldMapping = Pd7dayPricesolution1Mapping;
    type PrimaryKey = Pd7dayPricesolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(Pd7dayPricesolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secrrp",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secrrp",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secrrp",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minrrp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregrrp",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secrrp",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secrrp",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secrrp",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minrrp",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregrrp",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[15],
                    mmsdm_core::mms_datetime::parse,
                )?,
            bdu_initial_energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_initial_energy_storage",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            decgen_initial_energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "decgen_initial_energy_storage",
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
        Ok(Pd7dayPricesolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> Pd7dayPricesolution1PrimaryKey {
        Pd7dayPricesolution1PrimaryKey {
            interval_datetime: row.interval_datetime,
            intervention: row.intervention,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("pd7day_pricesolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        Pd7dayPricesolution1Row {
            run_datetime: row.run_datetime.clone(),
            intervention: row.intervention.clone(),
            interval_datetime: row.interval_datetime.clone(),
            regionid: row.regionid.clone(),
            rrp: row.rrp.clone(),
            lower1secrrp: row.lower1secrrp.clone(),
            lower6secrrp: row.lower6secrrp.clone(),
            lower60secrrp: row.lower60secrrp.clone(),
            lower5minrrp: row.lower5minrrp.clone(),
            lowerregrrp: row.lowerregrrp.clone(),
            raise1secrrp: row.raise1secrrp.clone(),
            raise6secrrp: row.raise6secrrp.clone(),
            raise60secrrp: row.raise60secrrp.clone(),
            raise5minrrp: row.raise5minrrp.clone(),
            raiseregrrp: row.raiseregrrp.clone(),
            lastchanged: row.lastchanged.clone(),
            bdu_initial_energy_storage: row.bdu_initial_energy_storage.clone(),
            decgen_initial_energy_storage: row.decgen_initial_energy_storage.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pd7dayPricesolution1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for Pd7dayPricesolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayPricesolution1Row<'data> {
    type Row<'other> = Pd7dayPricesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.intervention == row.intervention && self.regionid() == row.regionid()
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for Pd7dayPricesolution1Row<'data> {
    type PrimaryKey = Pd7dayPricesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.intervention == key.intervention && self.regionid() == key.regionid
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for Pd7dayPricesolution1PrimaryKey {
    type Row<'other> = Pd7dayPricesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.intervention == row.intervention && self.regionid == row.regionid()
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for Pd7dayPricesolution1PrimaryKey {
    type PrimaryKey = Pd7dayPricesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.intervention == key.intervention && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for Pd7dayPricesolution1 {
    type Builder = Pd7dayPricesolution1Builder;
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
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
                    "regionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregrrp",
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
                    "bdu_initial_energy_storage",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "decgen_initial_energy_storage",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        Pd7dayPricesolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            bdu_initial_energy_storage_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            decgen_initial_energy_storage_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
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
            .lower1secrrp_array
            .append_option({
                row.lower1secrrp
                    .map(|mut val| {
                        val.rescale(5);
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
            .lower60secrrp_array
            .append_option({
                row.lower60secrrp
                    .map(|mut val| {
                        val.rescale(5);
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
            .lowerregrrp_array
            .append_option({
                row.lowerregrrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
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
            .raise6secrrp_array
            .append_option({
                row.raise6secrrp
                    .map(|mut val| {
                        val.rescale(5);
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
            .raise5minrrp_array
            .append_option({
                row.raise5minrrp
                    .map(|mut val| {
                        val.rescale(5);
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
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .bdu_initial_energy_storage_array
            .append_option({
                row.bdu_initial_energy_storage
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .decgen_initial_energy_storage_array
            .append_option({
                row.decgen_initial_energy_storage
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.bdu_initial_energy_storage_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.decgen_initial_energy_storage_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct Pd7dayPricesolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    rrp_array: arrow::array::builder::Decimal128Builder,
    lower1secrrp_array: arrow::array::builder::Decimal128Builder,
    lower6secrrp_array: arrow::array::builder::Decimal128Builder,
    lower60secrrp_array: arrow::array::builder::Decimal128Builder,
    lower5minrrp_array: arrow::array::builder::Decimal128Builder,
    lowerregrrp_array: arrow::array::builder::Decimal128Builder,
    raise1secrrp_array: arrow::array::builder::Decimal128Builder,
    raise6secrrp_array: arrow::array::builder::Decimal128Builder,
    raise60secrrp_array: arrow::array::builder::Decimal128Builder,
    raise5minrrp_array: arrow::array::builder::Decimal128Builder,
    raiseregrrp_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    bdu_initial_energy_storage_array: arrow::array::builder::Decimal128Builder,
    decgen_initial_energy_storage_array: arrow::array::builder::Decimal128Builder,
}
