#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct MccCasesolution1;
pub struct MccCasesolution1Mapping([usize; 1]);
/// # Summary
///
/// ## MCC_CASESOLUTION
///  _Top level table for each MCC dispatch rerun process. Note there will be one record for each dispatch interval_
///
/// * Data Set Name: Mcc
/// * File Name: Casesolution
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MccCasesolution1Row<'data> {
    /// 5-minute Dispatch Run identifier
    pub run_datetime: chrono::NaiveDateTime,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> MccCasesolution1Row<'data> {}
impl mmsdm_core::GetTable for MccCasesolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MCC";
    const TABLE_NAME: &'static str = "CASESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MccCasesolution1Mapping([4]);
    const COLUMNS: &'static [&'static str] = &["RUN_DATETIME"];
    type Row<'row> = MccCasesolution1Row<'row>;
    type FieldMapping = MccCasesolution1Mapping;
    type PrimaryKey = MccCasesolution1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MccCasesolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: core::marker::PhantomData,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
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
        Ok(MccCasesolution1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let run_datetime = row
            .get_custom_parsed_at_idx(
                "run_datetime",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(run_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(run_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MccCasesolution1PrimaryKey {
        MccCasesolution1PrimaryKey {
            run_datetime: row.run_datetime,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.run_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.run_datetime).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "mcc_casesolution_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MccCasesolution1Row {
            run_datetime: row.run_datetime.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MccCasesolution1PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MccCasesolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MccCasesolution1Row<'data> {
    type Row<'other> = MccCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MccCasesolution1Row<'data> {
    type PrimaryKey = MccCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MccCasesolution1PrimaryKey {
    type Row<'other> = MccCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MccCasesolution1PrimaryKey {
    type PrimaryKey = MccCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MccCasesolution1 {
    type Builder = MccCasesolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MccCasesolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MccCasesolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct MccConstraintsolution1;
pub struct MccConstraintsolution1Mapping([usize; 4]);
/// # Summary
///
/// ## MCC_CONSTRAINTSOLUTION
///  _Constraint solution data from the MCC dispatch rerun process. Note only constraints with a non-zero marginal value are published._
///
/// * Data Set Name: Mcc
/// * File Name: Constraintsolution
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MccConstraintsolution1Row<'data> {
    /// 5-minute Dispatch Run identifier
    pub run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    /// Generic Constraint RHS Value for this MCC run
    pub rhs: Option<rust_decimal::Decimal>,
    /// Generic Constraint Marginal Value for this MCC run
    pub marginalvalue: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MccConstraintsolution1Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for MccConstraintsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MCC";
    const TABLE_NAME: &'static str = "CONSTRAINTSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MccConstraintsolution1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "CONSTRAINTID",
        "RHS",
        "MARGINALVALUE",
    ];
    type Row<'row> = MccConstraintsolution1Row<'row>;
    type FieldMapping = MccConstraintsolution1Mapping;
    type PrimaryKey = MccConstraintsolution1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MccConstraintsolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[1])?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
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
        Ok(MccConstraintsolution1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let run_datetime = row
            .get_custom_parsed_at_idx(
                "run_datetime",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(run_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(run_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MccConstraintsolution1PrimaryKey {
        MccConstraintsolution1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            run_datetime: row.run_datetime,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.run_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.run_datetime).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "mcc_constraintsolution_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MccConstraintsolution1Row {
            run_datetime: row.run_datetime.clone(),
            constraintid: row.constraintid.clone(),
            rhs: row.rhs.clone(),
            marginalvalue: row.marginalvalue.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MccConstraintsolution1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MccConstraintsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MccConstraintsolution1Row<'data> {
    type Row<'other> = MccConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MccConstraintsolution1Row<'data> {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MccConstraintsolution1PrimaryKey {
    type Row<'other> = MccConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid() && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MccConstraintsolution1PrimaryKey {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MccConstraintsolution1 {
    type Builder = MccConstraintsolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
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
                    "rhs",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MccConstraintsolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampSecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp());
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
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MccConstraintsolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampSecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
    rhs_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
}
