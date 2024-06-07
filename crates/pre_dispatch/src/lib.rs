#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct PredispatchBlockedConstraints1;
pub struct PredispatchBlockedConstraints1Mapping([usize; 2]);
/// # Summary
///
/// ## PREDISPATCHBLOCKEDCONSTRAINT
///  _PREDISPATCH Blocked Constraints lists any constraints that were blocked in a Predispatch run. If no constraints are blocked, there will be no rows for that predispatch run._
///
/// * Data Set Name: Predispatch
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
/// * PREDISPATCHSEQNO
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchBlockedConstraints1Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchBlockedConstraints1Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for PredispatchBlockedConstraints1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "BLOCKED_CONSTRAINTS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchBlockedConstraints1Mapping([
        4,
        5,
    ]);
    const COLUMNS: &'static [&'static str] = &["PREDISPATCHSEQNO", "CONSTRAINTID"];
    type Row<'row> = PredispatchBlockedConstraints1Row<'row>;
    type FieldMapping = PredispatchBlockedConstraints1Mapping;
    type PrimaryKey = PredispatchBlockedConstraints1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchBlockedConstraints1Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            constraintid: row.get_range("constraintid", field_mapping.0[1])?,
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
        Ok(PredispatchBlockedConstraints1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let predispatchseqno: mmsdm_core::TradingPeriod = row
            .get_parsed_at_idx("predispatchseqno", 4)?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(predispatchseqno).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchBlockedConstraints1PrimaryKey {
        PredispatchBlockedConstraints1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            predispatchseqno: row.predispatchseqno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.predispatchseqno).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_blocked_constraints_v1_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchBlockedConstraints1Row {
            predispatchseqno: row.predispatchseqno.clone(),
            constraintid: row.constraintid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchBlockedConstraints1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub predispatchseqno: mmsdm_core::TradingPeriod,
}
impl mmsdm_core::PrimaryKey for PredispatchBlockedConstraints1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchBlockedConstraints1Row<'data> {
    type Row<'other> = PredispatchBlockedConstraints1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for PredispatchBlockedConstraints1Row<'data> {
    type PrimaryKey = PredispatchBlockedConstraints1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.predispatchseqno == key.predispatchseqno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchBlockedConstraints1PrimaryKey {
    type Row<'other> = PredispatchBlockedConstraints1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchBlockedConstraints1PrimaryKey {
    type PrimaryKey = PredispatchBlockedConstraints1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.predispatchseqno == key.predispatchseqno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchBlockedConstraints1 {
    type Builder = PredispatchBlockedConstraints1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        PredispatchBlockedConstraints1Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchBlockedConstraints1Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
}
pub struct PredispatchCaseSolution1;
pub struct PredispatchCaseSolution1Mapping([usize; 20]);
/// # Summary
///
/// ## PREDISPATCHCASESOLUTION
///  _PREDISPATCHCASESOLUTION provides information relating to the complete predispatch run. The fields provide an overview of the dispatch run results allowing immediate identification of conditions such as energy or FCAS deficiencies._
///
/// * Data Set Name: Predispatch
/// * File Name: Case Solution
/// * Data Version: 1
///
/// # Description
///  PREDISPATCHCASESOLUTION data is public, so is available to all participants. Source PREDISPATCHCASESOLUTION updates every half-hour. Volume Approximately 48 records per day.
///
///
///
/// # Primary Key Columns
///
/// * PREDISPATCHSEQNO
/// * RUNNO
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchCaseSolution1Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// Predispatch run no, normally 1.
    pub runno: rust_decimal::Decimal,
    /// If non-zero indicated one of the following conditions: 1 = Supply Scarcity, Excess generation or constraint violations, -X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: core::ops::Range<usize>,
    /// Non-Physical Losses algorithm invoked during this run
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
    /// Total of Energy Constrained unit offer violations.
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Pre-Dispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    pub intervention: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchCaseSolution1Row<'data> {
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
impl mmsdm_core::GetTable for PredispatchCaseSolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "CASE_SOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchCaseSolution1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PREDISPATCHSEQNO",
        "RUNNO",
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
        "TOTALENERGYCONSTRVIOLATION",
        "TOTALENERGYOFFERVIOLATION",
        "LASTCHANGED",
        "INTERVENTION",
    ];
    type Row<'row> = PredispatchCaseSolution1Row<'row>;
    type FieldMapping = PredispatchCaseSolution1Mapping;
    type PrimaryKey = PredispatchCaseSolution1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchCaseSolution1Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            solutionstatus: row
                .get_opt_custom_parsed_at_idx(
                    "solutionstatus",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            spdversion: row.get_opt_range("spdversion", field_mapping.0[3])?,
            nonphysicallosses: row
                .get_opt_custom_parsed_at_idx(
                    "nonphysicallosses",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalobjective: row
                .get_opt_custom_parsed_at_idx(
                    "totalobjective",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalareagenviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalareagenviolation",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalinterconnectorviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalinterconnectorviolation",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalgenericviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalgenericviolation",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalramprateviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalramprateviolation",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalunitmwcapacityviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalunitmwcapacityviolation",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total5minviolation",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalregviolation",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total6secviolation",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total60secviolation",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalasprofileviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalasprofileviolation",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalenergyconstrviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalenergyconstrviolation",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalenergyofferviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalenergyofferviolation",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[18],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[19],
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
        Ok(PredispatchCaseSolution1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let predispatchseqno: mmsdm_core::TradingPeriod = row
            .get_parsed_at_idx("predispatchseqno", 4)?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(predispatchseqno).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchCaseSolution1PrimaryKey {
        PredispatchCaseSolution1PrimaryKey {
            predispatchseqno: row.predispatchseqno,
            runno: row.runno,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.predispatchseqno).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_case_solution_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchCaseSolution1Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
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
            totalenergyconstrviolation: row.totalenergyconstrviolation.clone(),
            totalenergyofferviolation: row.totalenergyofferviolation.clone(),
            lastchanged: row.lastchanged.clone(),
            intervention: row.intervention.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchCaseSolution1PrimaryKey {
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    pub runno: rust_decimal::Decimal,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchCaseSolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchCaseSolution1Row<'data> {
    type Row<'other> = PredispatchCaseSolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.predispatchseqno == row.predispatchseqno && self.runno == row.runno
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchCaseSolution1Row<'data> {
    type PrimaryKey = PredispatchCaseSolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.predispatchseqno == key.predispatchseqno && self.runno == key.runno
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchCaseSolution1PrimaryKey {
    type Row<'other> = PredispatchCaseSolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.predispatchseqno == row.predispatchseqno && self.runno == row.runno
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchCaseSolution1PrimaryKey {
    type PrimaryKey = PredispatchCaseSolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.predispatchseqno == key.predispatchseqno && self.runno == key.runno
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchCaseSolution1 {
    type Builder = PredispatchCaseSolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
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
                    "totalenergyconstrviolation",
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        PredispatchCaseSolution1Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
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
            totalenergyconstrviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
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
            .totalenergyconstrviolation_array
            .append_option({
                row.totalenergyconstrviolation
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
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
                    alloc::sync::Arc::new(
                        builder.totalenergyconstrviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.totalenergyofferviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchCaseSolution1Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
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
    totalenergyconstrviolation_array: arrow::array::builder::Decimal128Builder,
    totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
}
pub struct PredispatchConstraintSolution5;
pub struct PredispatchConstraintSolution5Mapping([usize; 14]);
/// # Summary
///
/// ## PREDISPATCHCONSTRAINT
///  _PREDISPATCHCONSTRAINT sets out constraints that are binding in each predispatch run and interconnector constraints (whether binding or not). Only binding and interconnector constraints are reported. Binding contracts have marginal value greater than $0. Interconnector constraints are listed so RHS values can be reported for ST PASA.<br>Constraint solutions only report fixed loading /MR constraints on the next day._
///
/// * Data Set Name: Predispatch
/// * File Name: Constraint Solution
/// * Data Version: 5
///
/// # Description
///  PREDISPATCHCONSTRAINT data is confidential on the day of creation, and public to all participants after the end of the market day. Source PREDISPATCHCONSTRAINT updates with every thirty-minute predispatch run. Note The PERIODID columns in tables PREDISPATCHCONSTRAINT and PREDISPATCH_FCAS_REQ have no consistent relationship with the other PERIODID values in the other tables in the PRE-DISPATCH package (such as PREDISPATCHPRICE). AEMO and many Participants appreciate the data model is inconsistent, but the cost of changing existing systems has been judged as being unjustifiable. An additional field DATETIME was added to allow joins between these data sets.
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchConstraintSolution5Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Generic constraint identifier
    pub constraintid: core::ops::Range<usize>,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    pub periodid: mmsdm_core::TradingPeriod,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
    /// RHS value used.
    pub rhs: Option<rust_decimal::Decimal>,
    /// Marginal value of violated constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of constraint violation
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    pub datetime: chrono::NaiveDateTime,
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
impl<'data> PredispatchConstraintSolution5Row<'data> {
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
impl mmsdm_core::GetTable for PredispatchConstraintSolution5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "CONSTRAINT_SOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchConstraintSolution5Mapping([
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
        "PREDISPATCHSEQNO",
        "RUNNO",
        "CONSTRAINTID",
        "PERIODID",
        "INTERVENTION",
        "RHS",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "LASTCHANGED",
        "DATETIME",
        "DUID",
        "GENCONID_EFFECTIVEDATE",
        "GENCONID_VERSIONNO",
        "LHS",
    ];
    type Row<'row> = PredispatchConstraintSolution5Row<'row>;
    type FieldMapping = PredispatchConstraintSolution5Mapping;
    type PrimaryKey = PredispatchConstraintSolution5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchConstraintSolution5Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[2])?,
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[3])?,
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
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_opt_range("duid", field_mapping.0[10])?,
            genconid_effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "genconid_effectivedate",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            genconid_versionno: row
                .get_opt_custom_parsed_at_idx(
                    "genconid_versionno",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lhs: row
                .get_opt_custom_parsed_at_idx(
                    "lhs",
                    field_mapping.0[13],
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
        Ok(PredispatchConstraintSolution5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 13, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchConstraintSolution5PrimaryKey {
        PredispatchConstraintSolution5PrimaryKey {
            constraintid: row.constraintid().to_string(),
            datetime: row.datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_constraint_solution_v5_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchConstraintSolution5Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            constraintid: row.constraintid.clone(),
            periodid: row.periodid.clone(),
            intervention: row.intervention.clone(),
            rhs: row.rhs.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            lastchanged: row.lastchanged.clone(),
            datetime: row.datetime.clone(),
            duid: row.duid.clone(),
            genconid_effectivedate: row.genconid_effectivedate.clone(),
            genconid_versionno: row.genconid_versionno.clone(),
            lhs: row.lhs.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchConstraintSolution5PrimaryKey {
    pub constraintid: alloc::string::String,
    pub datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchConstraintSolution5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchConstraintSolution5Row<'data> {
    type Row<'other> = PredispatchConstraintSolution5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid() && self.datetime == row.datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for PredispatchConstraintSolution5Row<'data> {
    type PrimaryKey = PredispatchConstraintSolution5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid && self.datetime == key.datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchConstraintSolution5PrimaryKey {
    type Row<'other> = PredispatchConstraintSolution5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid() && self.datetime == row.datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchConstraintSolution5PrimaryKey {
    type PrimaryKey = PredispatchConstraintSolution5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.datetime == key.datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchConstraintSolution5 {
    type Builder = PredispatchConstraintSolution5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
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
                    "datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
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
        PredispatchConstraintSolution5Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            violationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.constraintid_array.append_value(row.constraintid());
        builder.periodid_array.append_value(row.periodid.start().timestamp_millis());
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
        builder.duid_array.append_option(row.duid());
        builder
            .genconid_effectivedate_array
            .append_option(row.genconid_effectivedate.map(|val| val.timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
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
                    alloc::sync::Arc::new(builder.datetime_array.finish())
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
pub struct PredispatchConstraintSolution5Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    constraintid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    rhs_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    genconid_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    genconid_versionno_array: arrow::array::builder::Decimal128Builder,
    lhs_array: arrow::array::builder::Decimal128Builder,
}
pub struct PredispatchInterconnectorSoln3;
pub struct PredispatchInterconnectorSoln3Mapping([usize; 23]);
/// # Summary
///
/// ## PREDISPATCHINTERCONNECTORRES
///  _PREDISPATCHINTERCONNECTORRES records Interconnector flows and losses for the periods calculated in each predispatch run. Only binding and interconnector constraints are reported.<br>Some fields are for the Frequency Controlled Ancillary Services export and import limits and extra reporting of the generic constraint setting the energy import and export limits._
///
/// * Data Set Name: Predispatch
/// * File Name: Interconnector Soln
/// * Data Version: 3
///
/// # Description
///  Source PREDISPATCHINTERCONNECTORRES updates with every thirty-minute predispatch run. Note MW losses can be negative depending on the flow. The definition of direction of flow for an interconnector is that positive flow starts from the FROMREGION in INTERCONNECTOR.
///
///
///
/// # Primary Key Columns
///
/// * DATETIME
/// * INTERCONNECTORID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchInterconnectorSoln3Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: core::ops::Range<usize>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
    /// Metered MW Flow from EMS. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Flow
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// $ Marginal value of interconnector constraint from SPD
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation of interconnector constraint in MW
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last changed.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    pub datetime: chrono::NaiveDateTime,
    /// Calculated export limit.
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust bids between reports.
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
impl<'data> PredispatchInterconnectorSoln3Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
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
impl mmsdm_core::GetTable for PredispatchInterconnectorSoln3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "INTERCONNECTOR_SOLN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchInterconnectorSoln3Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PREDISPATCHSEQNO",
        "RUNNO",
        "INTERCONNECTORID",
        "PERIODID",
        "INTERVENTION",
        "METEREDMWFLOW",
        "MWFLOW",
        "MWLOSSES",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "LASTCHANGED",
        "DATETIME",
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
    type Row<'row> = PredispatchInterconnectorSoln3Row<'row>;
    type FieldMapping = PredispatchInterconnectorSoln3Mapping;
    type PrimaryKey = PredispatchInterconnectorSoln3PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchInterconnectorSoln3Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            periodid: row.get_opt_range("periodid", field_mapping.0[3])?,
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
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            exportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "exportlimit",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importlimit: row
                .get_opt_custom_parsed_at_idx(
                    "importlimit",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalloss: row
                .get_opt_custom_parsed_at_idx(
                    "marginalloss",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportgenconid: row.get_opt_range("exportgenconid", field_mapping.0[15])?,
            importgenconid: row.get_opt_range("importgenconid", field_mapping.0[16])?,
            fcasexportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "fcasexportlimit",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fcasimportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "fcasimportlimit",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            local_price_adjustment_export: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_export",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_export: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_export",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            local_price_adjustment_import: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_import",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_import: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_import",
                    field_mapping.0[22],
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
        Ok(PredispatchInterconnectorSoln3Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 15, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchInterconnectorSoln3PrimaryKey {
        PredispatchInterconnectorSoln3PrimaryKey {
            datetime: row.datetime,
            interconnectorid: row.interconnectorid().to_string(),
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_interconnector_soln_v3_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchInterconnectorSoln3Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            periodid: row.periodid.clone(),
            intervention: row.intervention.clone(),
            meteredmwflow: row.meteredmwflow.clone(),
            mwflow: row.mwflow.clone(),
            mwlosses: row.mwlosses.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            lastchanged: row.lastchanged.clone(),
            datetime: row.datetime.clone(),
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
pub struct PredispatchInterconnectorSoln3PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub interconnectorid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchInterconnectorSoln3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchInterconnectorSoln3Row<'data> {
    type Row<'other> = PredispatchInterconnectorSoln3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime
            && self.interconnectorid() == row.interconnectorid()
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for PredispatchInterconnectorSoln3Row<'data> {
    type PrimaryKey = PredispatchInterconnectorSoln3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid() == key.interconnectorid
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchInterconnectorSoln3PrimaryKey {
    type Row<'other> = PredispatchInterconnectorSoln3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.interconnectorid == row.interconnectorid()
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchInterconnectorSoln3PrimaryKey {
    type PrimaryKey = PredispatchInterconnectorSoln3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid == key.interconnectorid
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchInterconnectorSoln3 {
    type Builder = PredispatchInterconnectorSoln3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
                    "datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
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
        PredispatchInterconnectorSoln3Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
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
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.periodid_array.append_option(row.periodid());
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
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
                    alloc::sync::Arc::new(builder.datetime_array.finish())
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
pub struct PredispatchInterconnectorSoln3Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    meteredmwflow_array: arrow::array::builder::Decimal128Builder,
    mwflow_array: arrow::array::builder::Decimal128Builder,
    mwlosses_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
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
pub struct PredispatchInterconnectrSens1;
pub struct PredispatchInterconnectrSens1Mapping([usize; 51]);
/// # Summary
///
/// ## PREDISPATCHINTERSENSITIVITIES
///  _PREDISPATCHINTERSENSITIVITIES sets out the sensitivity flows for each interconnector by period._
///
/// * Data Set Name: Predispatch
/// * File Name: Interconnectr Sens
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * DATETIME
/// * INTERCONNECTORID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchInterconnectrSens1Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: core::ops::Range<usize>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
    /// Period date and time
    pub datetime: chrono::NaiveDateTime,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 1
    pub mwflow1: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 2
    pub mwflow2: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 3
    pub mwflow3: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 4
    pub mwflow4: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 5
    pub mwflow5: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 6
    pub mwflow6: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 7
    pub mwflow7: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 8
    pub mwflow8: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 9
    pub mwflow9: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 10
    pub mwflow10: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 11
    pub mwflow11: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 12
    pub mwflow12: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 13
    pub mwflow13: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 14
    pub mwflow14: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 15
    pub mwflow15: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 16
    pub mwflow16: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 17
    pub mwflow17: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 18
    pub mwflow18: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 19
    pub mwflow19: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 20
    pub mwflow20: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 21
    pub mwflow21: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 22
    pub mwflow22: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 23
    pub mwflow23: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 24
    pub mwflow24: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 25
    pub mwflow25: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 26
    pub mwflow26: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 27
    pub mwflow27: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 28
    pub mwflow28: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 29
    pub mwflow29: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 30
    pub mwflow30: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 31
    pub mwflow31: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 32
    pub mwflow32: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 33
    pub mwflow33: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 34
    pub mwflow34: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 35
    pub mwflow35: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 36
    pub mwflow36: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 37
    pub mwflow37: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 38
    pub mwflow38: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 39
    pub mwflow39: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 40
    pub mwflow40: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 41
    pub mwflow41: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 42
    pub mwflow42: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 43
    pub mwflow43: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchInterconnectrSens1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for PredispatchInterconnectrSens1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "INTERCONNECTR_SENS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchInterconnectrSens1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PREDISPATCHSEQNO",
        "RUNNO",
        "INTERCONNECTORID",
        "PERIODID",
        "INTERVENTION",
        "DATETIME",
        "INTERVENTION_ACTIVE",
        "MWFLOW1",
        "MWFLOW2",
        "MWFLOW3",
        "MWFLOW4",
        "MWFLOW5",
        "MWFLOW6",
        "MWFLOW7",
        "MWFLOW8",
        "MWFLOW9",
        "MWFLOW10",
        "MWFLOW11",
        "MWFLOW12",
        "MWFLOW13",
        "MWFLOW14",
        "MWFLOW15",
        "MWFLOW16",
        "MWFLOW17",
        "MWFLOW18",
        "MWFLOW19",
        "MWFLOW20",
        "MWFLOW21",
        "MWFLOW22",
        "MWFLOW23",
        "MWFLOW24",
        "MWFLOW25",
        "MWFLOW26",
        "MWFLOW27",
        "MWFLOW28",
        "MWFLOW29",
        "MWFLOW30",
        "MWFLOW31",
        "MWFLOW32",
        "MWFLOW33",
        "MWFLOW34",
        "MWFLOW35",
        "MWFLOW36",
        "MWFLOW37",
        "MWFLOW38",
        "MWFLOW39",
        "MWFLOW40",
        "MWFLOW41",
        "MWFLOW42",
        "MWFLOW43",
        "LASTCHANGED",
    ];
    type Row<'row> = PredispatchInterconnectrSens1Row<'row>;
    type FieldMapping = PredispatchInterconnectrSens1Mapping;
    type PrimaryKey = PredispatchInterconnectrSens1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchInterconnectrSens1Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            periodid: row.get_opt_range("periodid", field_mapping.0[3])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention_active: row
                .get_opt_custom_parsed_at_idx(
                    "intervention_active",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow1: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow1",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow2: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow2",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow3: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow3",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow4: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow4",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow5: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow5",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow6: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow6",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow7: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow7",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow8: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow8",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow9: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow9",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow10: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow10",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow11: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow11",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow12: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow12",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow13: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow13",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow14: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow14",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow15: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow15",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow16: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow16",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow17: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow17",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow18: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow18",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow19: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow19",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow20: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow20",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow21: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow21",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow22: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow22",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow23: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow23",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow24: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow24",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow25: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow25",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow26: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow26",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow27: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow27",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow28: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow28",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow29: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow29",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow30: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow30",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow31: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow31",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow32: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow32",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow33: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow33",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow34: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow34",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow35: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow35",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow36: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow36",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow37: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow37",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow38: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow38",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow39: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow39",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow40: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow40",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow41: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow41",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow42: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow42",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow43: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow43",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[50],
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
        Ok(PredispatchInterconnectrSens1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 9, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchInterconnectrSens1PrimaryKey {
        PredispatchInterconnectrSens1PrimaryKey {
            datetime: row.datetime,
            interconnectorid: row.interconnectorid().to_string(),
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_interconnectr_sens_v1_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchInterconnectrSens1Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            periodid: row.periodid.clone(),
            intervention: row.intervention.clone(),
            datetime: row.datetime.clone(),
            intervention_active: row.intervention_active.clone(),
            mwflow1: row.mwflow1.clone(),
            mwflow2: row.mwflow2.clone(),
            mwflow3: row.mwflow3.clone(),
            mwflow4: row.mwflow4.clone(),
            mwflow5: row.mwflow5.clone(),
            mwflow6: row.mwflow6.clone(),
            mwflow7: row.mwflow7.clone(),
            mwflow8: row.mwflow8.clone(),
            mwflow9: row.mwflow9.clone(),
            mwflow10: row.mwflow10.clone(),
            mwflow11: row.mwflow11.clone(),
            mwflow12: row.mwflow12.clone(),
            mwflow13: row.mwflow13.clone(),
            mwflow14: row.mwflow14.clone(),
            mwflow15: row.mwflow15.clone(),
            mwflow16: row.mwflow16.clone(),
            mwflow17: row.mwflow17.clone(),
            mwflow18: row.mwflow18.clone(),
            mwflow19: row.mwflow19.clone(),
            mwflow20: row.mwflow20.clone(),
            mwflow21: row.mwflow21.clone(),
            mwflow22: row.mwflow22.clone(),
            mwflow23: row.mwflow23.clone(),
            mwflow24: row.mwflow24.clone(),
            mwflow25: row.mwflow25.clone(),
            mwflow26: row.mwflow26.clone(),
            mwflow27: row.mwflow27.clone(),
            mwflow28: row.mwflow28.clone(),
            mwflow29: row.mwflow29.clone(),
            mwflow30: row.mwflow30.clone(),
            mwflow31: row.mwflow31.clone(),
            mwflow32: row.mwflow32.clone(),
            mwflow33: row.mwflow33.clone(),
            mwflow34: row.mwflow34.clone(),
            mwflow35: row.mwflow35.clone(),
            mwflow36: row.mwflow36.clone(),
            mwflow37: row.mwflow37.clone(),
            mwflow38: row.mwflow38.clone(),
            mwflow39: row.mwflow39.clone(),
            mwflow40: row.mwflow40.clone(),
            mwflow41: row.mwflow41.clone(),
            mwflow42: row.mwflow42.clone(),
            mwflow43: row.mwflow43.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchInterconnectrSens1PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub interconnectorid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchInterconnectrSens1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchInterconnectrSens1Row<'data> {
    type Row<'other> = PredispatchInterconnectrSens1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime
            && self.interconnectorid() == row.interconnectorid()
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for PredispatchInterconnectrSens1Row<'data> {
    type PrimaryKey = PredispatchInterconnectrSens1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid() == key.interconnectorid
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchInterconnectrSens1PrimaryKey {
    type Row<'other> = PredispatchInterconnectrSens1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.interconnectorid == row.interconnectorid()
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchInterconnectrSens1PrimaryKey {
    type PrimaryKey = PredispatchInterconnectrSens1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid == key.interconnectorid
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchInterconnectrSens1 {
    type Builder = PredispatchInterconnectrSens1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention_active",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow1",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow2",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow3",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow4",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow5",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow6",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow7",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow8",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow9",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow10",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow11",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow12",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow13",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow14",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow15",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow16",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow17",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow18",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow19",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow20",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow21",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow22",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow23",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow24",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow25",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow26",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow27",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow28",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow29",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow30",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow31",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow32",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow33",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow34",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow35",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow36",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow37",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow38",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow39",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow40",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow41",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow42",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mwflow43",
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
        PredispatchInterconnectrSens1Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_active_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            mwflow1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow11_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow12_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow13_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow14_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow15_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow16_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow17_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow18_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow19_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow20_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow21_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow22_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow23_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow24_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow25_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow26_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow27_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow28_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow29_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow30_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow31_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow32_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow33_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow34_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow35_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow36_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow37_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow38_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow39_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow40_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow41_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow42_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mwflow43_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.periodid_array.append_option(row.periodid());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
        builder
            .intervention_active_array
            .append_option({
                row.intervention_active
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .mwflow1_array
            .append_option({
                row.mwflow1
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow2_array
            .append_option({
                row.mwflow2
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow3_array
            .append_option({
                row.mwflow3
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow4_array
            .append_option({
                row.mwflow4
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow5_array
            .append_option({
                row.mwflow5
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow6_array
            .append_option({
                row.mwflow6
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow7_array
            .append_option({
                row.mwflow7
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow8_array
            .append_option({
                row.mwflow8
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow9_array
            .append_option({
                row.mwflow9
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow10_array
            .append_option({
                row.mwflow10
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow11_array
            .append_option({
                row.mwflow11
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow12_array
            .append_option({
                row.mwflow12
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow13_array
            .append_option({
                row.mwflow13
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow14_array
            .append_option({
                row.mwflow14
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow15_array
            .append_option({
                row.mwflow15
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow16_array
            .append_option({
                row.mwflow16
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow17_array
            .append_option({
                row.mwflow17
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow18_array
            .append_option({
                row.mwflow18
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow19_array
            .append_option({
                row.mwflow19
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow20_array
            .append_option({
                row.mwflow20
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow21_array
            .append_option({
                row.mwflow21
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow22_array
            .append_option({
                row.mwflow22
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow23_array
            .append_option({
                row.mwflow23
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow24_array
            .append_option({
                row.mwflow24
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow25_array
            .append_option({
                row.mwflow25
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow26_array
            .append_option({
                row.mwflow26
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow27_array
            .append_option({
                row.mwflow27
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow28_array
            .append_option({
                row.mwflow28
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow29_array
            .append_option({
                row.mwflow29
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow30_array
            .append_option({
                row.mwflow30
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow31_array
            .append_option({
                row.mwflow31
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow32_array
            .append_option({
                row.mwflow32
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow33_array
            .append_option({
                row.mwflow33
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow34_array
            .append_option({
                row.mwflow34
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow35_array
            .append_option({
                row.mwflow35
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow36_array
            .append_option({
                row.mwflow36
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow37_array
            .append_option({
                row.mwflow37
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow38_array
            .append_option({
                row.mwflow38
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow39_array
            .append_option({
                row.mwflow39
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow40_array
            .append_option({
                row.mwflow40
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow41_array
            .append_option({
                row.mwflow41
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow42_array
            .append_option({
                row.mwflow42
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mwflow43_array
            .append_option({
                row.mwflow43
                    .map(|mut val| {
                        val.rescale(5);
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_active_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow11_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow12_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow13_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow14_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow15_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow16_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow17_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow18_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow19_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow20_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow21_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow22_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow23_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow24_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow25_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow26_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow27_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow28_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow29_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow30_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow31_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow32_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow33_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow34_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow35_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow36_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow37_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow38_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow39_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow40_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow41_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow42_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow43_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchInterconnectrSens1Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_active_array: arrow::array::builder::Decimal128Builder,
    mwflow1_array: arrow::array::builder::Decimal128Builder,
    mwflow2_array: arrow::array::builder::Decimal128Builder,
    mwflow3_array: arrow::array::builder::Decimal128Builder,
    mwflow4_array: arrow::array::builder::Decimal128Builder,
    mwflow5_array: arrow::array::builder::Decimal128Builder,
    mwflow6_array: arrow::array::builder::Decimal128Builder,
    mwflow7_array: arrow::array::builder::Decimal128Builder,
    mwflow8_array: arrow::array::builder::Decimal128Builder,
    mwflow9_array: arrow::array::builder::Decimal128Builder,
    mwflow10_array: arrow::array::builder::Decimal128Builder,
    mwflow11_array: arrow::array::builder::Decimal128Builder,
    mwflow12_array: arrow::array::builder::Decimal128Builder,
    mwflow13_array: arrow::array::builder::Decimal128Builder,
    mwflow14_array: arrow::array::builder::Decimal128Builder,
    mwflow15_array: arrow::array::builder::Decimal128Builder,
    mwflow16_array: arrow::array::builder::Decimal128Builder,
    mwflow17_array: arrow::array::builder::Decimal128Builder,
    mwflow18_array: arrow::array::builder::Decimal128Builder,
    mwflow19_array: arrow::array::builder::Decimal128Builder,
    mwflow20_array: arrow::array::builder::Decimal128Builder,
    mwflow21_array: arrow::array::builder::Decimal128Builder,
    mwflow22_array: arrow::array::builder::Decimal128Builder,
    mwflow23_array: arrow::array::builder::Decimal128Builder,
    mwflow24_array: arrow::array::builder::Decimal128Builder,
    mwflow25_array: arrow::array::builder::Decimal128Builder,
    mwflow26_array: arrow::array::builder::Decimal128Builder,
    mwflow27_array: arrow::array::builder::Decimal128Builder,
    mwflow28_array: arrow::array::builder::Decimal128Builder,
    mwflow29_array: arrow::array::builder::Decimal128Builder,
    mwflow30_array: arrow::array::builder::Decimal128Builder,
    mwflow31_array: arrow::array::builder::Decimal128Builder,
    mwflow32_array: arrow::array::builder::Decimal128Builder,
    mwflow33_array: arrow::array::builder::Decimal128Builder,
    mwflow34_array: arrow::array::builder::Decimal128Builder,
    mwflow35_array: arrow::array::builder::Decimal128Builder,
    mwflow36_array: arrow::array::builder::Decimal128Builder,
    mwflow37_array: arrow::array::builder::Decimal128Builder,
    mwflow38_array: arrow::array::builder::Decimal128Builder,
    mwflow39_array: arrow::array::builder::Decimal128Builder,
    mwflow40_array: arrow::array::builder::Decimal128Builder,
    mwflow41_array: arrow::array::builder::Decimal128Builder,
    mwflow42_array: arrow::array::builder::Decimal128Builder,
    mwflow43_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct PredispatchUnitSolution4;
pub struct PredispatchUnitSolution4Mapping([usize; 64]);
/// # Summary
///
/// ## PREDISPATCHLOAD
///  _PREDISPATCHLOAD shows pre-dispatch targets for each dispatchable unit, including additional fields to handle the Ancillary Services functionality. No record is written where a unit is not dispatched. PREDISPATCHLOAD shows all the results for each period._
///
/// * Data Set Name: Predispatch
/// * File Name: Unit Solution
/// * Data Version: 4
///
/// # Description
///  Source Own (confidential) data updates every thirty minutes, with whole market data for the day before available as part of next day market data. Note ** A flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined using the low 3 bits as follows: Flag Name Bit Description Enabled 0 The unit is enabled to provide this ancillary service type. Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. Interpretation of the bit-flags as a number gives the following possibilities (i.e. other combinations are not possible): Numeric Value Bit (2,1,0) Meaning 0 000 Not stranded, not trapped, not enabled. 1 001 Not stranded, not trapped, is enabled. 3 011 Not stranded, is trapped, is enabled. 4 100 Is stranded, not trapped, not enabled. For example, testing for availability can be done by checking for odd (=available) or even (=unavailable) number (e.g.  mod(flag,2)  results in 0 for unavailable and 1 for available). *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values.
///
///
///
/// # Primary Key Columns
///
/// * DATETIME
/// * DUID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchUnitSolution4Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Dispatchable unit identifier for fast start
    pub duid: core::ops::Range<usize>,
    /// Not used
    pub tradetype: Option<rust_decimal::Decimal>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: core::ops::Range<usize>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
    /// Connection point identifier
    pub connectionpointid: core::ops::Range<usize>,
    /// AGC Status from EMS
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Dispatch mode of unit for fast start (1-4)
    pub dispatchmode: Option<rust_decimal::Decimal>,
    /// Initial MW at start of first period. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run. Negative values when Bi-directional Unit start from importing power, otherwise positive.
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period. Negative values when Bi-directional Unit is importing power, otherwise positive.
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Lower 5 min MW target in period
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec MW target in period
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec MW target in period
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min MW target in period
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec MW target in period
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec MW target in period
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Ramp down rate in period in MW/minute
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate in period in MW/minute
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    pub downepf: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    pub upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min from LP Solver
    pub marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds from LP Solver
    pub marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds from LP Solver
    pub marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy from LP Solver
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
    /// Period date and time
    pub datetime: chrono::NaiveDateTime,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// For Scheduled units, this is the MAXAVAIL bid availability For Semi-scheduled units, this is the lower of MAXAVAIL bid availability and UIGF
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise reg status flag
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag
    pub lowerregflags: Option<rust_decimal::Decimal>,
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
    /// BDU only. The projected energy storage based on cleared energy and regulation FCAS dispatch (MWh).<br>Participants may use negative values as an indicator of the relative “error” in profiling Max Availability to reflect energy limits
    pub energy_storage: Option<rust_decimal::Decimal>,
    /// BDU only - Minimum Energy Storage constraint limit (MWh)
    pub energy_storage_min: Option<rust_decimal::Decimal>,
    /// BDU only - Maximum Energy Storage constraint limit (MWh)
    pub energy_storage_max: Option<rust_decimal::Decimal>,
    /// BDU only. Load side availability (BidOfferPeriod.MAXAVAIL where DIRECTION = LOAD)
    pub min_availability: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchUnitSolution4Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
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
impl mmsdm_core::GetTable for PredispatchUnitSolution4 {
    const VERSION: i32 = 4;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "UNIT_SOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchUnitSolution4Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PREDISPATCHSEQNO",
        "RUNNO",
        "DUID",
        "TRADETYPE",
        "PERIODID",
        "INTERVENTION",
        "CONNECTIONPOINTID",
        "AGCSTATUS",
        "DISPATCHMODE",
        "INITIALMW",
        "TOTALCLEARED",
        "LOWER5MIN",
        "LOWER60SEC",
        "LOWER6SEC",
        "RAISE5MIN",
        "RAISE60SEC",
        "RAISE6SEC",
        "RAMPDOWNRATE",
        "RAMPUPRATE",
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
        "DATETIME",
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
        "RAISE6SECACTUALAVAILABILITY",
        "RAISE60SECACTUALAVAILABILITY",
        "RAISE5MINACTUALAVAILABILITY",
        "RAISEREGACTUALAVAILABILITY",
        "LOWER6SECACTUALAVAILABILITY",
        "LOWER60SECACTUALAVAILABILITY",
        "LOWER5MINACTUALAVAILABILITY",
        "LOWERREGACTUALAVAILABILITY",
        "SEMIDISPATCHCAP",
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
        "ENERGY_STORAGE_MIN",
        "ENERGY_STORAGE_MAX",
        "MIN_AVAILABILITY",
    ];
    type Row<'row> = PredispatchUnitSolution4Row<'row>;
    type FieldMapping = PredispatchUnitSolution4Mapping;
    type PrimaryKey = PredispatchUnitSolution4PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchUnitSolution4Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
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
            periodid: row.get_opt_range("periodid", field_mapping.0[4])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[6])?,
            agcstatus: row
                .get_opt_custom_parsed_at_idx(
                    "agcstatus",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchmode: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchmode",
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
            lower5min: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampdownrate: row
                .get_opt_custom_parsed_at_idx(
                    "rampdownrate",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampuprate: row
                .get_opt_custom_parsed_at_idx(
                    "rampuprate",
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
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[30],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lowerreg: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availability: row
                .get_opt_custom_parsed_at_idx(
                    "availability",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secflags",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secflags",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minflags",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregflags: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregflags",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secflags",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secflags",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minflags",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregflags: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregflags",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secactualavailability",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secactualavailability",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minactualavailability",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregactualavailability",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secactualavailability",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secactualavailability",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minactualavailability",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregactualavailability",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            semidispatchcap: row
                .get_opt_custom_parsed_at_idx(
                    "semidispatchcap",
                    field_mapping.0[50],
                    mmsdm_core::mms_decimal::parse,
                )?,
            conformance_mode: row
                .get_opt_custom_parsed_at_idx(
                    "conformance_mode",
                    field_mapping.0[51],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uigf: row
                .get_opt_custom_parsed_at_idx(
                    "uigf",
                    field_mapping.0[52],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise1sec",
                    field_mapping.0[53],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secflags",
                    field_mapping.0[54],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower1sec",
                    field_mapping.0[55],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secflags",
                    field_mapping.0[56],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secactualavailability",
                    field_mapping.0[57],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secactualavailability",
                    field_mapping.0[58],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initial_energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "initial_energy_storage",
                    field_mapping.0[59],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "energy_storage",
                    field_mapping.0[60],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_storage_min: row
                .get_opt_custom_parsed_at_idx(
                    "energy_storage_min",
                    field_mapping.0[61],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_storage_max: row
                .get_opt_custom_parsed_at_idx(
                    "energy_storage_max",
                    field_mapping.0[62],
                    mmsdm_core::mms_decimal::parse,
                )?,
            min_availability: row
                .get_opt_custom_parsed_at_idx(
                    "min_availability",
                    field_mapping.0[63],
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
        Ok(PredispatchUnitSolution4Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 34, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchUnitSolution4PrimaryKey {
        PredispatchUnitSolution4PrimaryKey {
            datetime: row.datetime,
            duid: row.duid().to_string(),
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_unit_solution_v4_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchUnitSolution4Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            duid: row.duid.clone(),
            tradetype: row.tradetype.clone(),
            periodid: row.periodid.clone(),
            intervention: row.intervention.clone(),
            connectionpointid: row.connectionpointid.clone(),
            agcstatus: row.agcstatus.clone(),
            dispatchmode: row.dispatchmode.clone(),
            initialmw: row.initialmw.clone(),
            totalcleared: row.totalcleared.clone(),
            lower5min: row.lower5min.clone(),
            lower60sec: row.lower60sec.clone(),
            lower6sec: row.lower6sec.clone(),
            raise5min: row.raise5min.clone(),
            raise60sec: row.raise60sec.clone(),
            raise6sec: row.raise6sec.clone(),
            rampdownrate: row.rampdownrate.clone(),
            rampuprate: row.rampuprate.clone(),
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
            datetime: row.datetime.clone(),
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
            raise6secactualavailability: row.raise6secactualavailability.clone(),
            raise60secactualavailability: row.raise60secactualavailability.clone(),
            raise5minactualavailability: row.raise5minactualavailability.clone(),
            raiseregactualavailability: row.raiseregactualavailability.clone(),
            lower6secactualavailability: row.lower6secactualavailability.clone(),
            lower60secactualavailability: row.lower60secactualavailability.clone(),
            lower5minactualavailability: row.lower5minactualavailability.clone(),
            lowerregactualavailability: row.lowerregactualavailability.clone(),
            semidispatchcap: row.semidispatchcap.clone(),
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
            energy_storage_min: row.energy_storage_min.clone(),
            energy_storage_max: row.energy_storage_max.clone(),
            min_availability: row.min_availability.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchUnitSolution4PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub duid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchUnitSolution4PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchUnitSolution4Row<'data> {
    type Row<'other> = PredispatchUnitSolution4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.duid() == row.duid()
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchUnitSolution4Row<'data> {
    type PrimaryKey = PredispatchUnitSolution4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid() == key.duid
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchUnitSolution4PrimaryKey {
    type Row<'other> = PredispatchUnitSolution4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.duid == row.duid()
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchUnitSolution4PrimaryKey {
    type PrimaryKey = PredispatchUnitSolution4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid == key.duid
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchUnitSolution4 {
    type Builder = PredispatchUnitSolution4Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
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
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
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
                    "agcstatus",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchmode",
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
                    "datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
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
                    "energy_storage_min",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energy_storage_max",
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
        PredispatchUnitSolution4Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            tradetype_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            periodid_array: arrow::array::builder::StringBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            agcstatus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            dispatchmode_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            initialmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalcleared_array: arrow::array::builder::Decimal128Builder::new()
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
            rampdownrate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rampuprate_array: arrow::array::builder::Decimal128Builder::new()
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
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            energy_storage_min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            energy_storage_max_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            min_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
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
        builder.periodid_array.append_option(row.periodid());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_option(row.connectionpointid());
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
            .dispatchmode_array
            .append_option({
                row.dispatchmode
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
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
            .energy_storage_min_array
            .append_option({
                row.energy_storage_min
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .energy_storage_max_array
            .append_option({
                row.energy_storage_max
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tradetype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.agcstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchmode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initialmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalcleared_array.finish())
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
                    alloc::sync::Arc::new(builder.rampdownrate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rampuprate_array.finish())
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
                    alloc::sync::Arc::new(builder.datetime_array.finish())
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
                    alloc::sync::Arc::new(builder.energy_storage_min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energy_storage_max_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.min_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchUnitSolution4Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    tradetype_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    agcstatus_array: arrow::array::builder::Decimal128Builder,
    dispatchmode_array: arrow::array::builder::Decimal128Builder,
    initialmw_array: arrow::array::builder::Decimal128Builder,
    totalcleared_array: arrow::array::builder::Decimal128Builder,
    lower5min_array: arrow::array::builder::Decimal128Builder,
    lower60sec_array: arrow::array::builder::Decimal128Builder,
    lower6sec_array: arrow::array::builder::Decimal128Builder,
    raise5min_array: arrow::array::builder::Decimal128Builder,
    raise60sec_array: arrow::array::builder::Decimal128Builder,
    raise6sec_array: arrow::array::builder::Decimal128Builder,
    rampdownrate_array: arrow::array::builder::Decimal128Builder,
    rampuprate_array: arrow::array::builder::Decimal128Builder,
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
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
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
    raise6secactualavailability_array: arrow::array::builder::Decimal128Builder,
    raise60secactualavailability_array: arrow::array::builder::Decimal128Builder,
    raise5minactualavailability_array: arrow::array::builder::Decimal128Builder,
    raiseregactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower6secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower60secactualavailability_array: arrow::array::builder::Decimal128Builder,
    lower5minactualavailability_array: arrow::array::builder::Decimal128Builder,
    lowerregactualavailability_array: arrow::array::builder::Decimal128Builder,
    semidispatchcap_array: arrow::array::builder::Decimal128Builder,
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
    energy_storage_min_array: arrow::array::builder::Decimal128Builder,
    energy_storage_max_array: arrow::array::builder::Decimal128Builder,
    min_availability_array: arrow::array::builder::Decimal128Builder,
}
pub struct PredispatchOffertrk1;
pub struct PredispatchOffertrk1Mapping([usize; 8]);
/// # Summary
///
/// ## PREDISPATCHOFFERTRK
///  _PREDISPATCHOFFERTRK is for the ancillary service bid tracking of predispatch processing. PREDISPATCHOFFERTRK identifies which bids from BIDDAYOFFER and BIDOFFERPERIOD were applied for a given unit and ancillary service for each predispatch run._
///
/// * Data Set Name: Predispatch
/// * File Name: Offertrk
/// * Data Version: 1
///
/// # Description
///  Source PREDISPATCHOFFERTRK updates every 30 minutes. The data is confidential to each participant until the next trading day.  Volume Approximately 45,000 records per day.
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * PERIODID
/// * PREDISPATCHSEQNO
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchOffertrk1Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// Dispatchable Unit identifier
    pub duid: core::ops::Range<usize>,
    /// Bid type Identifier - the ancillary service to which the bid applies
    pub bidtype: core::ops::Range<usize>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: core::ops::Range<usize>,
    /// Settlement date of bid applied
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    pub bidofferdate: Option<chrono::NaiveDateTime>,
    /// Period date and time
    pub datetime: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchOffertrk1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
    pub fn periodid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.periodid.clone())
    }
}
impl mmsdm_core::GetTable for PredispatchOffertrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "OFFERTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchOffertrk1Mapping([
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
        "PREDISPATCHSEQNO",
        "DUID",
        "BIDTYPE",
        "PERIODID",
        "BIDSETTLEMENTDATE",
        "BIDOFFERDATE",
        "DATETIME",
        "LASTCHANGED",
    ];
    type Row<'row> = PredispatchOffertrk1Row<'row>;
    type FieldMapping = PredispatchOffertrk1Mapping;
    type PrimaryKey = PredispatchOffertrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchOffertrk1Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            bidtype: row.get_range("bidtype", field_mapping.0[2])?,
            periodid: row.get_range("periodid", field_mapping.0[3])?,
            bidsettlementdate: row
                .get_opt_custom_parsed_at_idx(
                    "bidsettlementdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            bidofferdate: row
                .get_opt_custom_parsed_at_idx(
                    "bidofferdate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            datetime: row
                .get_opt_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(PredispatchOffertrk1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let predispatchseqno: mmsdm_core::TradingPeriod = row
            .get_parsed_at_idx("predispatchseqno", 4)?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(predispatchseqno).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchOffertrk1PrimaryKey {
        PredispatchOffertrk1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            duid: row.duid().to_string(),
            periodid: row.periodid().to_string(),
            predispatchseqno: row.predispatchseqno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.predispatchseqno).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_offertrk_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchOffertrk1Row {
            predispatchseqno: row.predispatchseqno.clone(),
            duid: row.duid.clone(),
            bidtype: row.bidtype.clone(),
            periodid: row.periodid.clone(),
            bidsettlementdate: row.bidsettlementdate.clone(),
            bidofferdate: row.bidofferdate.clone(),
            datetime: row.datetime.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchOffertrk1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub duid: alloc::string::String,
    pub periodid: alloc::string::String,
    pub predispatchseqno: mmsdm_core::TradingPeriod,
}
impl mmsdm_core::PrimaryKey for PredispatchOffertrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchOffertrk1Row<'data> {
    type Row<'other> = PredispatchOffertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.duid() == row.duid()
            && self.periodid() == row.periodid()
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchOffertrk1Row<'data> {
    type PrimaryKey = PredispatchOffertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.duid() == key.duid
            && self.periodid() == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchOffertrk1PrimaryKey {
    type Row<'other> = PredispatchOffertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.duid == row.duid()
            && self.periodid == row.periodid()
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchOffertrk1PrimaryKey {
    type PrimaryKey = PredispatchOffertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.duid == key.duid
            && self.periodid == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchOffertrk1 {
    type Builder = PredispatchOffertrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
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
                    "periodid",
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
                    "datetime",
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
        PredispatchOffertrk1Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
            bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            bidofferdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.bidtype_array.append_value(row.bidtype());
        builder.periodid_array.append_value(row.periodid());
        builder
            .bidsettlementdate_array
            .append_option(row.bidsettlementdate.map(|val| val.timestamp_millis()));
        builder
            .bidofferdate_array
            .append_option(row.bidofferdate.map(|val| val.timestamp_millis()));
        builder
            .datetime_array
            .append_option(row.datetime.map(|val| val.timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidsettlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidofferdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchOffertrk1Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
    bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    bidofferdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct PredispatchRegionPrices2;
pub struct PredispatchRegionPrices2Mapping([usize; 35]);
/// # Summary
///
/// ## PREDISPATCHPRICE
///  _PREDISPATCHPRICE records predispatch prices for each region by period for each predispatch run, including fields to handle the Ancillary Services functionality._
///
/// * Data Set Name: Predispatch
/// * File Name: Region Prices
/// * Data Version: 2
///
/// # Description
///  PREDISPATCHPRICE data is public, so is available to all participants. Source PREDISPATCHPRICE updates with every thirty-minute predispatch run.
///
///
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchRegionPrices2Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: core::ops::Range<usize>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: core::ops::Range<usize>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price
    pub eep: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp1: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep1: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp2: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep2: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp3: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep3: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp4: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep4: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp5: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep5: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp6: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep6: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp7: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep7: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp8: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep8: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    pub datetime: chrono::NaiveDateTime,
    /// Regional reference price for this dispatch period
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// Regional Raise 1Sec Price - R1Price attribute after capping /flooring
    pub raise1secrrp: Option<rust_decimal::Decimal>,
    /// Regional Lower 1Sec Price - RegionSolution element L1Price attribute
    pub lower1secrrp: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchRegionPrices2Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for PredispatchRegionPrices2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "REGION_PRICES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchRegionPrices2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PREDISPATCHSEQNO",
        "RUNNO",
        "REGIONID",
        "PERIODID",
        "INTERVENTION",
        "RRP",
        "EEP",
        "RRP1",
        "EEP1",
        "RRP2",
        "EEP2",
        "RRP3",
        "EEP3",
        "RRP4",
        "EEP4",
        "RRP5",
        "EEP5",
        "RRP6",
        "EEP6",
        "RRP7",
        "EEP7",
        "RRP8",
        "EEP8",
        "LASTCHANGED",
        "DATETIME",
        "RAISE6SECRRP",
        "RAISE60SECRRP",
        "RAISE5MINRRP",
        "RAISEREGRRP",
        "LOWER6SECRRP",
        "LOWER60SECRRP",
        "LOWER5MINRRP",
        "LOWERREGRRP",
        "RAISE1SECRRP",
        "LOWER1SECRRP",
    ];
    type Row<'row> = PredispatchRegionPrices2Row<'row>;
    type FieldMapping = PredispatchRegionPrices2Mapping;
    type PrimaryKey = PredispatchRegionPrices2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchRegionPrices2Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            periodid: row.get_opt_range("periodid", field_mapping.0[3])?,
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
            rrp1: row
                .get_opt_custom_parsed_at_idx(
                    "rrp1",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep1: row
                .get_opt_custom_parsed_at_idx(
                    "eep1",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp2: row
                .get_opt_custom_parsed_at_idx(
                    "rrp2",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep2: row
                .get_opt_custom_parsed_at_idx(
                    "eep2",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp3: row
                .get_opt_custom_parsed_at_idx(
                    "rrp3",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep3: row
                .get_opt_custom_parsed_at_idx(
                    "eep3",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp4: row
                .get_opt_custom_parsed_at_idx(
                    "rrp4",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep4: row
                .get_opt_custom_parsed_at_idx(
                    "eep4",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp5: row
                .get_opt_custom_parsed_at_idx(
                    "rrp5",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep5: row
                .get_opt_custom_parsed_at_idx(
                    "eep5",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp6: row
                .get_opt_custom_parsed_at_idx(
                    "rrp6",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep6: row
                .get_opt_custom_parsed_at_idx(
                    "eep6",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp7: row
                .get_opt_custom_parsed_at_idx(
                    "rrp7",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep7: row
                .get_opt_custom_parsed_at_idx(
                    "eep7",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp8: row
                .get_opt_custom_parsed_at_idx(
                    "rrp8",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep8: row
                .get_opt_custom_parsed_at_idx(
                    "eep8",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[23],
                    mmsdm_core::mms_datetime::parse,
                )?,
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[24],
                    mmsdm_core::mms_datetime::parse,
                )?,
            raise6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secrrp",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secrrp",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minrrp",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregrrp",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secrrp",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secrrp",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minrrp",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregrrp",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secrrp",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secrrp",
                    field_mapping.0[34],
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
        Ok(PredispatchRegionPrices2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 28, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchRegionPrices2PrimaryKey {
        PredispatchRegionPrices2PrimaryKey {
            datetime: row.datetime,
            regionid: row.regionid().to_string(),
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_region_prices_v2_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchRegionPrices2Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            intervention: row.intervention.clone(),
            rrp: row.rrp.clone(),
            eep: row.eep.clone(),
            rrp1: row.rrp1.clone(),
            eep1: row.eep1.clone(),
            rrp2: row.rrp2.clone(),
            eep2: row.eep2.clone(),
            rrp3: row.rrp3.clone(),
            eep3: row.eep3.clone(),
            rrp4: row.rrp4.clone(),
            eep4: row.eep4.clone(),
            rrp5: row.rrp5.clone(),
            eep5: row.eep5.clone(),
            rrp6: row.rrp6.clone(),
            eep6: row.eep6.clone(),
            rrp7: row.rrp7.clone(),
            eep7: row.eep7.clone(),
            rrp8: row.rrp8.clone(),
            eep8: row.eep8.clone(),
            lastchanged: row.lastchanged.clone(),
            datetime: row.datetime.clone(),
            raise6secrrp: row.raise6secrrp.clone(),
            raise60secrrp: row.raise60secrrp.clone(),
            raise5minrrp: row.raise5minrrp.clone(),
            raiseregrrp: row.raiseregrrp.clone(),
            lower6secrrp: row.lower6secrrp.clone(),
            lower60secrrp: row.lower60secrrp.clone(),
            lower5minrrp: row.lower5minrrp.clone(),
            lowerregrrp: row.lowerregrrp.clone(),
            raise1secrrp: row.raise1secrrp.clone(),
            lower1secrrp: row.lower1secrrp.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchRegionPrices2PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchRegionPrices2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchRegionPrices2Row<'data> {
    type Row<'other> = PredispatchRegionPrices2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.regionid() == row.regionid()
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchRegionPrices2Row<'data> {
    type PrimaryKey = PredispatchRegionPrices2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid() == key.regionid
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchRegionPrices2PrimaryKey {
    type Row<'other> = PredispatchRegionPrices2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid()
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchRegionPrices2PrimaryKey {
    type PrimaryKey = PredispatchRegionPrices2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchRegionPrices2 {
    type Builder = PredispatchRegionPrices2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
                    "rrp1",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep1",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp2",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep2",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp3",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep3",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp4",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep4",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp5",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep5",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp6",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep6",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp7",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep7",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp8",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep8",
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
                    "datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
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
                    "lower1secrrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        PredispatchRegionPrices2Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            eep8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            raise6secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregrrp_array: arrow::array::builder::Decimal128Builder::new()
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
            lower1secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.regionid_array.append_value(row.regionid());
        builder.periodid_array.append_option(row.periodid());
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
            .rrp1_array
            .append_option({
                row.rrp1
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep1_array
            .append_option({
                row.eep1
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp2_array
            .append_option({
                row.rrp2
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep2_array
            .append_option({
                row.eep2
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp3_array
            .append_option({
                row.rrp3
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep3_array
            .append_option({
                row.eep3
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp4_array
            .append_option({
                row.rrp4
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep4_array
            .append_option({
                row.eep4
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp5_array
            .append_option({
                row.rrp5
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep5_array
            .append_option({
                row.eep5
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp6_array
            .append_option({
                row.rrp6
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep6_array
            .append_option({
                row.eep6
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp7_array
            .append_option({
                row.rrp7
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep7_array
            .append_option({
                row.eep7
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp8_array
            .append_option({
                row.rrp8
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep8_array
            .append_option({
                row.eep8
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
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
            .lower1secrrp_array
            .append_option({
                row.lower1secrrp
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregrrp_array.finish())
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
                    alloc::sync::Arc::new(builder.lower1secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchRegionPrices2Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    eep_array: arrow::array::builder::Decimal128Builder,
    rrp1_array: arrow::array::builder::Decimal128Builder,
    eep1_array: arrow::array::builder::Decimal128Builder,
    rrp2_array: arrow::array::builder::Decimal128Builder,
    eep2_array: arrow::array::builder::Decimal128Builder,
    rrp3_array: arrow::array::builder::Decimal128Builder,
    eep3_array: arrow::array::builder::Decimal128Builder,
    rrp4_array: arrow::array::builder::Decimal128Builder,
    eep4_array: arrow::array::builder::Decimal128Builder,
    rrp5_array: arrow::array::builder::Decimal128Builder,
    eep5_array: arrow::array::builder::Decimal128Builder,
    rrp6_array: arrow::array::builder::Decimal128Builder,
    eep6_array: arrow::array::builder::Decimal128Builder,
    rrp7_array: arrow::array::builder::Decimal128Builder,
    eep7_array: arrow::array::builder::Decimal128Builder,
    rrp8_array: arrow::array::builder::Decimal128Builder,
    eep8_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    raise6secrrp_array: arrow::array::builder::Decimal128Builder,
    raise60secrrp_array: arrow::array::builder::Decimal128Builder,
    raise5minrrp_array: arrow::array::builder::Decimal128Builder,
    raiseregrrp_array: arrow::array::builder::Decimal128Builder,
    lower6secrrp_array: arrow::array::builder::Decimal128Builder,
    lower60secrrp_array: arrow::array::builder::Decimal128Builder,
    lower5minrrp_array: arrow::array::builder::Decimal128Builder,
    lowerregrrp_array: arrow::array::builder::Decimal128Builder,
    raise1secrrp_array: arrow::array::builder::Decimal128Builder,
    lower1secrrp_array: arrow::array::builder::Decimal128Builder,
}
pub struct PredispatchPricesensitivities1;
pub struct PredispatchPricesensitivities1Mapping([usize; 51]);
/// # Summary
///
/// ## PREDISPATCHPRICESENSITIVITIES
///  _PREDISPATCHPRICESENSITIVITIES sets out the sensitivity prices for each region by period._
///
/// * Data Set Name: Predispatch
/// * File Name: Pricesensitivities
/// * Data Version: 1
///
/// # Description
///  Source The plan is to provide this data every half-hour.
///
///
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchPricesensitivities1Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: core::ops::Range<usize>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: core::ops::Range<usize>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
    /// Regional Energy Price for scenario 1
    pub rrpeep1: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 2
    pub rrpeep2: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 3
    pub rrpeep3: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 4
    pub rrpeep4: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 5
    pub rrpeep5: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 6
    pub rrpeep6: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 7
    pub rrpeep7: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 8
    pub rrpeep8: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 9
    pub rrpeep9: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 10
    pub rrpeep10: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 11
    pub rrpeep11: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 12
    pub rrpeep12: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 13
    pub rrpeep13: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 14
    pub rrpeep14: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 15
    pub rrpeep15: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 16
    pub rrpeep16: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 17
    pub rrpeep17: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 18
    pub rrpeep18: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 19
    pub rrpeep19: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 20
    pub rrpeep20: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 21
    pub rrpeep21: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 22
    pub rrpeep22: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 23
    pub rrpeep23: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 24
    pub rrpeep24: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 25
    pub rrpeep25: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 26
    pub rrpeep26: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 27
    pub rrpeep27: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 28
    pub rrpeep28: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    pub datetime: chrono::NaiveDateTime,
    /// Regional Energy Price for scenario 29
    pub rrpeep29: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 30
    pub rrpeep30: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 31
    pub rrpeep31: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 32
    pub rrpeep32: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 33
    pub rrpeep33: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 34
    pub rrpeep34: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 35
    pub rrpeep35: Option<rust_decimal::Decimal>,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 36
    pub rrpeep36: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 37
    pub rrpeep37: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 38
    pub rrpeep38: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 39
    pub rrpeep39: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 40
    pub rrpeep40: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 41
    pub rrpeep41: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 42
    pub rrpeep42: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 43
    pub rrpeep43: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchPricesensitivities1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for PredispatchPricesensitivities1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "PRICESENSITIVITIES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchPricesensitivities1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PREDISPATCHSEQNO",
        "RUNNO",
        "REGIONID",
        "PERIODID",
        "INTERVENTION",
        "RRPEEP1",
        "RRPEEP2",
        "RRPEEP3",
        "RRPEEP4",
        "RRPEEP5",
        "RRPEEP6",
        "RRPEEP7",
        "RRPEEP8",
        "RRPEEP9",
        "RRPEEP10",
        "RRPEEP11",
        "RRPEEP12",
        "RRPEEP13",
        "RRPEEP14",
        "RRPEEP15",
        "RRPEEP16",
        "RRPEEP17",
        "RRPEEP18",
        "RRPEEP19",
        "RRPEEP20",
        "RRPEEP21",
        "RRPEEP22",
        "RRPEEP23",
        "RRPEEP24",
        "RRPEEP25",
        "RRPEEP26",
        "RRPEEP27",
        "RRPEEP28",
        "LASTCHANGED",
        "DATETIME",
        "RRPEEP29",
        "RRPEEP30",
        "RRPEEP31",
        "RRPEEP32",
        "RRPEEP33",
        "RRPEEP34",
        "RRPEEP35",
        "INTERVENTION_ACTIVE",
        "RRPEEP36",
        "RRPEEP37",
        "RRPEEP38",
        "RRPEEP39",
        "RRPEEP40",
        "RRPEEP41",
        "RRPEEP42",
        "RRPEEP43",
    ];
    type Row<'row> = PredispatchPricesensitivities1Row<'row>;
    type FieldMapping = PredispatchPricesensitivities1Mapping;
    type PrimaryKey = PredispatchPricesensitivities1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchPricesensitivities1Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            periodid: row.get_opt_range("periodid", field_mapping.0[3])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep1: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep1",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep2: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep2",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep3: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep3",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep4: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep4",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep5: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep5",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep6: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep6",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep7: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep7",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep8: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep8",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep9: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep9",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep10: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep10",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep11: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep11",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep12: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep12",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep13: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep13",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep14: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep14",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep15: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep15",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep16: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep16",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep17: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep17",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep18: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep18",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep19: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep19",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep20: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep20",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep21: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep21",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep22: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep22",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep23: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep23",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep24: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep24",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep25: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep25",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep26: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep26",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep27: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep27",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep28: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep28",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[33],
                    mmsdm_core::mms_datetime::parse,
                )?,
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[34],
                    mmsdm_core::mms_datetime::parse,
                )?,
            rrpeep29: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep29",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep30: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep30",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep31: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep31",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep32: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep32",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep33: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep33",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep34: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep34",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep35: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep35",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention_active: row
                .get_opt_custom_parsed_at_idx(
                    "intervention_active",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep36: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep36",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep37: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep37",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep38: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep38",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep39: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep39",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep40: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep40",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep41: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep41",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep42: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep42",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrpeep43: row
                .get_opt_custom_parsed_at_idx(
                    "rrpeep43",
                    field_mapping.0[50],
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
        Ok(PredispatchPricesensitivities1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 38, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchPricesensitivities1PrimaryKey {
        PredispatchPricesensitivities1PrimaryKey {
            datetime: row.datetime,
            regionid: row.regionid().to_string(),
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_pricesensitivities_v1_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchPricesensitivities1Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            intervention: row.intervention.clone(),
            rrpeep1: row.rrpeep1.clone(),
            rrpeep2: row.rrpeep2.clone(),
            rrpeep3: row.rrpeep3.clone(),
            rrpeep4: row.rrpeep4.clone(),
            rrpeep5: row.rrpeep5.clone(),
            rrpeep6: row.rrpeep6.clone(),
            rrpeep7: row.rrpeep7.clone(),
            rrpeep8: row.rrpeep8.clone(),
            rrpeep9: row.rrpeep9.clone(),
            rrpeep10: row.rrpeep10.clone(),
            rrpeep11: row.rrpeep11.clone(),
            rrpeep12: row.rrpeep12.clone(),
            rrpeep13: row.rrpeep13.clone(),
            rrpeep14: row.rrpeep14.clone(),
            rrpeep15: row.rrpeep15.clone(),
            rrpeep16: row.rrpeep16.clone(),
            rrpeep17: row.rrpeep17.clone(),
            rrpeep18: row.rrpeep18.clone(),
            rrpeep19: row.rrpeep19.clone(),
            rrpeep20: row.rrpeep20.clone(),
            rrpeep21: row.rrpeep21.clone(),
            rrpeep22: row.rrpeep22.clone(),
            rrpeep23: row.rrpeep23.clone(),
            rrpeep24: row.rrpeep24.clone(),
            rrpeep25: row.rrpeep25.clone(),
            rrpeep26: row.rrpeep26.clone(),
            rrpeep27: row.rrpeep27.clone(),
            rrpeep28: row.rrpeep28.clone(),
            lastchanged: row.lastchanged.clone(),
            datetime: row.datetime.clone(),
            rrpeep29: row.rrpeep29.clone(),
            rrpeep30: row.rrpeep30.clone(),
            rrpeep31: row.rrpeep31.clone(),
            rrpeep32: row.rrpeep32.clone(),
            rrpeep33: row.rrpeep33.clone(),
            rrpeep34: row.rrpeep34.clone(),
            rrpeep35: row.rrpeep35.clone(),
            intervention_active: row.intervention_active.clone(),
            rrpeep36: row.rrpeep36.clone(),
            rrpeep37: row.rrpeep37.clone(),
            rrpeep38: row.rrpeep38.clone(),
            rrpeep39: row.rrpeep39.clone(),
            rrpeep40: row.rrpeep40.clone(),
            rrpeep41: row.rrpeep41.clone(),
            rrpeep42: row.rrpeep42.clone(),
            rrpeep43: row.rrpeep43.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchPricesensitivities1PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchPricesensitivities1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchPricesensitivities1Row<'data> {
    type Row<'other> = PredispatchPricesensitivities1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.regionid() == row.regionid()
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for PredispatchPricesensitivities1Row<'data> {
    type PrimaryKey = PredispatchPricesensitivities1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid() == key.regionid
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchPricesensitivities1PrimaryKey {
    type Row<'other> = PredispatchPricesensitivities1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid()
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchPricesensitivities1PrimaryKey {
    type PrimaryKey = PredispatchPricesensitivities1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchPricesensitivities1 {
    type Builder = PredispatchPricesensitivities1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep1",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep2",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep3",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep4",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep5",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep6",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep7",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep8",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep9",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep10",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep11",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep12",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep13",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep14",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep15",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep16",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep17",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep18",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep19",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep20",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep21",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep22",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep23",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep24",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep25",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep26",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep27",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep28",
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
                    "datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep29",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep30",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep31",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep32",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep33",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep34",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep35",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervention_active",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep36",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep37",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep38",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep39",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep40",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep41",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep42",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrpeep43",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        PredispatchPricesensitivities1Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            rrpeep1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep11_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep12_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep13_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep14_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep15_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep16_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep17_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep18_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep19_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep20_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep21_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep22_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep23_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep24_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep25_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep26_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep27_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep28_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            rrpeep29_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep30_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep31_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep32_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep33_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep34_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep35_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            intervention_active_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            rrpeep36_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep37_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep38_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep39_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep40_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep41_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep42_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrpeep43_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.regionid_array.append_value(row.regionid());
        builder.periodid_array.append_option(row.periodid());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .rrpeep1_array
            .append_option({
                row.rrpeep1
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep2_array
            .append_option({
                row.rrpeep2
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep3_array
            .append_option({
                row.rrpeep3
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep4_array
            .append_option({
                row.rrpeep4
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep5_array
            .append_option({
                row.rrpeep5
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep6_array
            .append_option({
                row.rrpeep6
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep7_array
            .append_option({
                row.rrpeep7
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep8_array
            .append_option({
                row.rrpeep8
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep9_array
            .append_option({
                row.rrpeep9
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep10_array
            .append_option({
                row.rrpeep10
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep11_array
            .append_option({
                row.rrpeep11
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep12_array
            .append_option({
                row.rrpeep12
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep13_array
            .append_option({
                row.rrpeep13
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep14_array
            .append_option({
                row.rrpeep14
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep15_array
            .append_option({
                row.rrpeep15
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep16_array
            .append_option({
                row.rrpeep16
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep17_array
            .append_option({
                row.rrpeep17
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep18_array
            .append_option({
                row.rrpeep18
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep19_array
            .append_option({
                row.rrpeep19
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep20_array
            .append_option({
                row.rrpeep20
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep21_array
            .append_option({
                row.rrpeep21
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep22_array
            .append_option({
                row.rrpeep22
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep23_array
            .append_option({
                row.rrpeep23
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep24_array
            .append_option({
                row.rrpeep24
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep25_array
            .append_option({
                row.rrpeep25
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep26_array
            .append_option({
                row.rrpeep26
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep27_array
            .append_option({
                row.rrpeep27
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep28_array
            .append_option({
                row.rrpeep28
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
        builder
            .rrpeep29_array
            .append_option({
                row.rrpeep29
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep30_array
            .append_option({
                row.rrpeep30
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep31_array
            .append_option({
                row.rrpeep31
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep32_array
            .append_option({
                row.rrpeep32
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep33_array
            .append_option({
                row.rrpeep33
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep34_array
            .append_option({
                row.rrpeep34
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep35_array
            .append_option({
                row.rrpeep35
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .intervention_active_array
            .append_option({
                row.intervention_active
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep36_array
            .append_option({
                row.rrpeep36
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep37_array
            .append_option({
                row.rrpeep37
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep38_array
            .append_option({
                row.rrpeep38
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep39_array
            .append_option({
                row.rrpeep39
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep40_array
            .append_option({
                row.rrpeep40
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep41_array
            .append_option({
                row.rrpeep41
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep42_array
            .append_option({
                row.rrpeep42
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrpeep43_array
            .append_option({
                row.rrpeep43
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep11_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep12_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep13_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep14_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep15_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep16_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep17_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep18_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep19_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep20_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep21_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep22_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep23_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep24_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep25_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep26_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep27_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep28_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep29_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep30_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep31_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep32_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep33_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep34_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep35_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_active_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep36_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep37_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep38_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep39_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep40_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep41_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep42_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrpeep43_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchPricesensitivities1Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    rrpeep1_array: arrow::array::builder::Decimal128Builder,
    rrpeep2_array: arrow::array::builder::Decimal128Builder,
    rrpeep3_array: arrow::array::builder::Decimal128Builder,
    rrpeep4_array: arrow::array::builder::Decimal128Builder,
    rrpeep5_array: arrow::array::builder::Decimal128Builder,
    rrpeep6_array: arrow::array::builder::Decimal128Builder,
    rrpeep7_array: arrow::array::builder::Decimal128Builder,
    rrpeep8_array: arrow::array::builder::Decimal128Builder,
    rrpeep9_array: arrow::array::builder::Decimal128Builder,
    rrpeep10_array: arrow::array::builder::Decimal128Builder,
    rrpeep11_array: arrow::array::builder::Decimal128Builder,
    rrpeep12_array: arrow::array::builder::Decimal128Builder,
    rrpeep13_array: arrow::array::builder::Decimal128Builder,
    rrpeep14_array: arrow::array::builder::Decimal128Builder,
    rrpeep15_array: arrow::array::builder::Decimal128Builder,
    rrpeep16_array: arrow::array::builder::Decimal128Builder,
    rrpeep17_array: arrow::array::builder::Decimal128Builder,
    rrpeep18_array: arrow::array::builder::Decimal128Builder,
    rrpeep19_array: arrow::array::builder::Decimal128Builder,
    rrpeep20_array: arrow::array::builder::Decimal128Builder,
    rrpeep21_array: arrow::array::builder::Decimal128Builder,
    rrpeep22_array: arrow::array::builder::Decimal128Builder,
    rrpeep23_array: arrow::array::builder::Decimal128Builder,
    rrpeep24_array: arrow::array::builder::Decimal128Builder,
    rrpeep25_array: arrow::array::builder::Decimal128Builder,
    rrpeep26_array: arrow::array::builder::Decimal128Builder,
    rrpeep27_array: arrow::array::builder::Decimal128Builder,
    rrpeep28_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    rrpeep29_array: arrow::array::builder::Decimal128Builder,
    rrpeep30_array: arrow::array::builder::Decimal128Builder,
    rrpeep31_array: arrow::array::builder::Decimal128Builder,
    rrpeep32_array: arrow::array::builder::Decimal128Builder,
    rrpeep33_array: arrow::array::builder::Decimal128Builder,
    rrpeep34_array: arrow::array::builder::Decimal128Builder,
    rrpeep35_array: arrow::array::builder::Decimal128Builder,
    intervention_active_array: arrow::array::builder::Decimal128Builder,
    rrpeep36_array: arrow::array::builder::Decimal128Builder,
    rrpeep37_array: arrow::array::builder::Decimal128Builder,
    rrpeep38_array: arrow::array::builder::Decimal128Builder,
    rrpeep39_array: arrow::array::builder::Decimal128Builder,
    rrpeep40_array: arrow::array::builder::Decimal128Builder,
    rrpeep41_array: arrow::array::builder::Decimal128Builder,
    rrpeep42_array: arrow::array::builder::Decimal128Builder,
    rrpeep43_array: arrow::array::builder::Decimal128Builder,
}
pub struct PredispatchRegionSolution8;
pub struct PredispatchRegionSolution8Mapping([usize; 125]);
/// # Summary
///
/// ## PREDISPATCHREGIONSUM
///  _PREDISPATCHREGIONSUM sets out the overall regional Pre-Dispatch results for base case details (excluding price)._
///
/// * Data Set Name: Predispatch
/// * File Name: Region Solution
/// * Data Version: 8
///
/// # Description
///  PREDISPATCHREGIONSUM includes the forecast demand (total demand) and Frequency Control Ancillary Services (FCAS) requirements (specifically, for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations). PREDISPATCHREGIONSUM updates each half-hour with the latest Pre-Dispatch details for the remaining period. Regional demand can be calculated as total demand plus dispatchable load (i.e. Regional demand = Total Demand + Dispatchable Load) Source PREDISPATCHREGIONSUM updates every thirty minutes. Note *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values. From 16 February 2006, the old reserve values are no longer populated (i.e. are null), being LORSurplus and LRCSurplus. For more details on the changes to Reporting of Reserve Condition Data, refer to AEMO Communication 2042. For the best available indicator of reserve condition in each of the regions of the NEM for each trading interval, refer to the latest run of the Pre-Dispatch PASA (see table PDPASA_REGIONSOLUTION).
///
///
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchRegionSolution8Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// LP Solver Pre-Dispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: core::ops::Range<usize>,
    /// PERIODID is just a period count, starting from 1 for each Pre-Dispatch run. Use DATETIME to determine half hour period.
    pub periodid: core::ops::Range<usize>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
    /// Total demand in MW for period (less normally on loads)
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    pub availableload: Option<rust_decimal::Decimal>,
    /// Delta MW value only
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Generation dispatched in period
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Load dispatched in period
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Excess generation in period / Deficit generation if VOLL
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
    /// Not used since Dec 2003.  Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 5 min
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
    /// Period date and time
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period expressed as Date/Time
    pub datetime: chrono::NaiveDateTime,
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
    /// generation availability taking into account daily energy constraints
    pub decavailability: Option<rust_decimal::Decimal>,
    /// Not used after Feb 2006. Total short term generation capacity reserve used in assessing lack of reserve condition
    pub lorsurplus: Option<rust_decimal::Decimal>,
    /// Not used after Feb 2006. Total short term generation capacity reserve above the stated low reserve condition requirement
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
impl<'data> PredispatchRegionSolution8Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for PredispatchRegionSolution8 {
    const VERSION: i32 = 8;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "REGION_SOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchRegionSolution8Mapping([
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
        "PREDISPATCHSEQNO",
        "RUNNO",
        "REGIONID",
        "PERIODID",
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
        "LASTCHANGED",
        "DATETIME",
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
        "DECAVAILABILITY",
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
    type Row<'row> = PredispatchRegionSolution8Row<'row>;
    type FieldMapping = PredispatchRegionSolution8Mapping;
    type PrimaryKey = PredispatchRegionSolution8PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchRegionSolution8Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            periodid: row.get_opt_range("periodid", field_mapping.0[3])?,
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
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[61],
                    mmsdm_core::mms_datetime::parse,
                )?,
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[62],
                    mmsdm_core::mms_datetime::parse,
                )?,
            initialsupply: row
                .get_opt_custom_parsed_at_idx(
                    "initialsupply",
                    field_mapping.0[63],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedsupply: row
                .get_opt_custom_parsed_at_idx(
                    "clearedsupply",
                    field_mapping.0[64],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregimport: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregimport",
                    field_mapping.0[65],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocaldispatch",
                    field_mapping.0[66],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocalreq",
                    field_mapping.0[67],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregreq: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregreq",
                    field_mapping.0[68],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregimport: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregimport",
                    field_mapping.0[69],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocaldispatch",
                    field_mapping.0[70],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalreq",
                    field_mapping.0[71],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregreq: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregreq",
                    field_mapping.0[72],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalviolation",
                    field_mapping.0[73],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalviolation",
                    field_mapping.0[74],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalviolation",
                    field_mapping.0[75],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalviolation",
                    field_mapping.0[76],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalviolation",
                    field_mapping.0[77],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocalviolation",
                    field_mapping.0[78],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalviolation",
                    field_mapping.0[79],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalviolation",
                    field_mapping.0[80],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minviolation",
                    field_mapping.0[81],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregviolation",
                    field_mapping.0[82],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secviolation",
                    field_mapping.0[83],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secviolation",
                    field_mapping.0[84],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minviolation",
                    field_mapping.0[85],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregviolation",
                    field_mapping.0[86],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secviolation",
                    field_mapping.0[87],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secviolation",
                    field_mapping.0[88],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secactualavailability",
                    field_mapping.0[89],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secactualavailability",
                    field_mapping.0[90],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minactualavailability",
                    field_mapping.0[91],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregactualavailability",
                    field_mapping.0[92],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secactualavailability",
                    field_mapping.0[93],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secactualavailability",
                    field_mapping.0[94],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minactualavailability",
                    field_mapping.0[95],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregactualavailability: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregactualavailability",
                    field_mapping.0[96],
                    mmsdm_core::mms_decimal::parse,
                )?,
            decavailability: row
                .get_opt_custom_parsed_at_idx(
                    "decavailability",
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
        Ok(PredispatchRegionSolution8Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 66, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchRegionSolution8PrimaryKey {
        PredispatchRegionSolution8PrimaryKey {
            datetime: row.datetime,
            regionid: row.regionid().to_string(),
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_region_solution_v8_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchRegionSolution8Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
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
            lastchanged: row.lastchanged.clone(),
            datetime: row.datetime.clone(),
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
            decavailability: row.decavailability.clone(),
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
pub struct PredispatchRegionSolution8PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchRegionSolution8PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchRegionSolution8Row<'data> {
    type Row<'other> = PredispatchRegionSolution8Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.regionid() == row.regionid()
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchRegionSolution8Row<'data> {
    type PrimaryKey = PredispatchRegionSolution8PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid() == key.regionid
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchRegionSolution8PrimaryKey {
    type Row<'other> = PredispatchRegionSolution8Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid()
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchRegionSolution8PrimaryKey {
    type PrimaryKey = PredispatchRegionSolution8PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchRegionSolution8 {
    type Builder = PredispatchRegionSolution8Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
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
                    "decavailability",
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
        PredispatchRegionSolution8Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
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
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            decavailability_array: arrow::array::builder::Decimal128Builder::new()
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
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.regionid_array.append_value(row.regionid());
        builder.periodid_array.append_option(row.periodid());
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
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
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
            .decavailability_array
            .append_option({
                row.decavailability
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.datetime_array.finish())
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
                    alloc::sync::Arc::new(builder.decavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
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
pub struct PredispatchRegionSolution8Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
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
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
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
    decavailability_array: arrow::array::builder::Decimal128Builder,
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
pub struct PredispatchScenarioDemand1;
pub struct PredispatchScenarioDemand1Mapping([usize; 5]);
/// # Summary
///
/// ## PREDISPATCHSCENARIODEMAND
///  _PREDISPATCHSCENARIODEMAND defines the demand offsets that are applied for each of the predispatch sensitivity scenarios._
///
/// * Data Set Name: Predispatch
/// * File Name: Scenario Demand
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * SCENARIO
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchScenarioDemand1Row<'data> {
    /// The effective date of this set of scenarios
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub versionno: i64,
    /// The scenario identifier.
    pub scenario: i64,
    /// The region to which to apply the deltaMW for this SCENARIO.
    pub regionid: core::ops::Range<usize>,
    /// The MW offset that is applied for this scenario
    pub deltamw: Option<i64>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchScenarioDemand1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for PredispatchScenarioDemand1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "SCENARIO_DEMAND";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchScenarioDemand1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "SCENARIO",
        "REGIONID",
        "DELTAMW",
    ];
    type Row<'row> = PredispatchScenarioDemand1Row<'row>;
    type FieldMapping = PredispatchScenarioDemand1Mapping;
    type PrimaryKey = PredispatchScenarioDemand1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchScenarioDemand1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row.get_parsed_at_idx("versionno", field_mapping.0[1])?,
            scenario: row.get_parsed_at_idx("scenario", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            deltamw: row.get_opt_parsed_at_idx("deltamw", field_mapping.0[4])?,
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
        Ok(PredispatchScenarioDemand1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchScenarioDemand1PrimaryKey {
        PredispatchScenarioDemand1PrimaryKey {
            effectivedate: row.effectivedate,
            regionid: row.regionid().to_string(),
            scenario: row.scenario,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.effectivedate)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.effectivedate)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_scenario_demand_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchScenarioDemand1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            scenario: row.scenario.clone(),
            regionid: row.regionid.clone(),
            deltamw: row.deltamw.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchScenarioDemand1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub scenario: i64,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for PredispatchScenarioDemand1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchScenarioDemand1Row<'data> {
    type Row<'other> = PredispatchScenarioDemand1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid() == row.regionid()
            && self.scenario == row.scenario && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchScenarioDemand1Row<'data> {
    type PrimaryKey = PredispatchScenarioDemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid() == key.regionid
            && self.scenario == key.scenario && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchScenarioDemand1PrimaryKey {
    type Row<'other> = PredispatchScenarioDemand1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid()
            && self.scenario == row.scenario && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchScenarioDemand1PrimaryKey {
    type PrimaryKey = PredispatchScenarioDemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.scenario == key.scenario && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchScenarioDemand1 {
    type Builder = PredispatchScenarioDemand1Builder;
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
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "scenario",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "deltamw",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        PredispatchScenarioDemand1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Int64Builder::new(),
            scenario_array: arrow::array::builder::Int64Builder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            deltamw_array: arrow::array::builder::Int64Builder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder.versionno_array.append_value(row.versionno);
        builder.scenario_array.append_value(row.scenario);
        builder.regionid_array.append_value(row.regionid());
        builder.deltamw_array.append_option(row.deltamw);
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
                    alloc::sync::Arc::new(builder.scenario_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deltamw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchScenarioDemand1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Int64Builder,
    scenario_array: arrow::array::builder::Int64Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    deltamw_array: arrow::array::builder::Int64Builder,
}
pub struct PredispatchScenarioDemandTrk1;
pub struct PredispatchScenarioDemandTrk1Mapping([usize; 5]);
/// # Summary
///
/// ## PREDISPATCHSCENARIODEMANDTRK
///  _Tracks the predispatch scenario offset updates across time_
///
/// * Data Set Name: Predispatch
/// * File Name: Scenario Demand Trk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchScenarioDemandTrk1Row<'data> {
    /// The effective date of this set of scenarios
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub versionno: i64,
    /// The user that authorised the scenario update
    pub authorisedby: core::ops::Range<usize>,
    /// The datetime that the scenario update was authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// The datetime that the record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchScenarioDemandTrk1Row<'data> {
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
impl mmsdm_core::GetTable for PredispatchScenarioDemandTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "SCENARIO_DEMAND_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchScenarioDemandTrk1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = PredispatchScenarioDemandTrk1Row<'row>;
    type FieldMapping = PredispatchScenarioDemandTrk1Mapping;
    type PrimaryKey = PredispatchScenarioDemandTrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchScenarioDemandTrk1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row.get_parsed_at_idx("versionno", field_mapping.0[1])?,
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
        Ok(PredispatchScenarioDemandTrk1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchScenarioDemandTrk1PrimaryKey {
        PredispatchScenarioDemandTrk1PrimaryKey {
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.effectivedate)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.effectivedate)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_scenario_demand_trk_v1_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchScenarioDemandTrk1Row {
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
pub struct PredispatchScenarioDemandTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for PredispatchScenarioDemandTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchScenarioDemandTrk1Row<'data> {
    type Row<'other> = PredispatchScenarioDemandTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for PredispatchScenarioDemandTrk1Row<'data> {
    type PrimaryKey = PredispatchScenarioDemandTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchScenarioDemandTrk1PrimaryKey {
    type Row<'other> = PredispatchScenarioDemandTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchScenarioDemandTrk1PrimaryKey {
    type PrimaryKey = PredispatchScenarioDemandTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchScenarioDemandTrk1 {
    type Builder = PredispatchScenarioDemandTrk1Builder;
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
                    arrow::datatypes::DataType::Int64,
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
        PredispatchScenarioDemandTrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Int64Builder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder.versionno_array.append_value(row.versionno);
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(row.authoriseddate.map(|val| val.timestamp_millis()));
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
pub struct PredispatchScenarioDemandTrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Int64Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct PredispatchRegionfcasrequirement2;
pub struct PredispatchRegionfcasrequirement2Mapping([usize; 18]);
/// # Summary
///
/// ## PREDISPATCH_FCAS_REQ
///  _PREDISPATCH_FCAS_REQ shows Predispatch Constraint tracking for Regional FCAS Requirements._
///
/// * Data Set Name: Predispatch
/// * File Name: Regionfcasrequirement
/// * Data Version: 2
///
/// # Description
///  Source PREDISPATCH_FCAS_REQ updates with each pre-dispatch run (half hourly) Volume Approximately 2,000 rows per day. Note The PERIODID columns in tables PREDISPATCHCONSTRAINT and PREDISPATCH_FCAS_REQ have no consistent relationship with the other PERIODID values in the other tables in the PRE-DISPATCH package (such as PREDISPATCHPRICE). AEMO and many Participants appreciate the data model is inconsistent, but the cost of changing existing systems has been judged as being unjustifiable. An additional field DATETIME was added to allow joins between these data sets.
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DATETIME
/// * GENCONID
/// * REGIONID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchRegionfcasrequirement2Row<'data> {
    /// PreDispatch Sequence number
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// Case Run number
    pub runno: Option<rust_decimal::Decimal>,
    /// Intervention Flag
    pub intervention: rust_decimal::Decimal,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    pub periodid: mmsdm_core::TradingPeriod,
    /// Generic Constraint ID - Join to table GenConData
    pub genconid: core::ops::Range<usize>,
    /// Region ID
    pub regionid: core::ops::Range<usize>,
    /// Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// Generic Constraint EffectiveDate - Join to table GenConData
    pub genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    pub genconversionno: Option<rust_decimal::Decimal>,
    /// Marginal Value of generic constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Date and Time of trading interval
    pub datetime: chrono::NaiveDateTime,
    /// Last date and time record changed
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
impl<'data> PredispatchRegionfcasrequirement2Row<'data> {
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
impl mmsdm_core::GetTable for PredispatchRegionfcasrequirement2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "REGIONFCASREQUIREMENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchRegionfcasrequirement2Mapping([
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
        "PREDISPATCHSEQNO",
        "RUNNO",
        "INTERVENTION",
        "PERIODID",
        "GENCONID",
        "REGIONID",
        "BIDTYPE",
        "GENCONEFFECTIVEDATE",
        "GENCONVERSIONNO",
        "MARGINALVALUE",
        "DATETIME",
        "LASTCHANGED",
        "BASE_COST",
        "ADJUSTED_COST",
        "ESTIMATED_CMPF",
        "ESTIMATED_CRMPF",
        "RECOVERY_FACTOR_CMPF",
        "RECOVERY_FACTOR_CRMPF",
    ];
    type Row<'row> = PredispatchRegionfcasrequirement2Row<'row>;
    type FieldMapping = PredispatchRegionfcasrequirement2Mapping;
    type PrimaryKey = PredispatchRegionfcasrequirement2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchRegionfcasrequirement2Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            runno: row
                .get_opt_custom_parsed_at_idx(
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
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[3])?,
            genconid: row.get_range("genconid", field_mapping.0[4])?,
            regionid: row.get_range("regionid", field_mapping.0[5])?,
            bidtype: row.get_range("bidtype", field_mapping.0[6])?,
            genconeffectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "genconeffectivedate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            genconversionno: row
                .get_opt_custom_parsed_at_idx(
                    "genconversionno",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            base_cost: row
                .get_opt_custom_parsed_at_idx(
                    "base_cost",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            adjusted_cost: row
                .get_opt_custom_parsed_at_idx(
                    "adjusted_cost",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            estimated_cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "estimated_cmpf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            estimated_crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "estimated_crmpf",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_cmpf",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_crmpf",
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
        Ok(PredispatchRegionfcasrequirement2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 14, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchRegionfcasrequirement2PrimaryKey {
        PredispatchRegionfcasrequirement2PrimaryKey {
            bidtype: row.bidtype().to_string(),
            datetime: row.datetime,
            genconid: row.genconid().to_string(),
            regionid: row.regionid().to_string(),
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_regionfcasrequirement_v2_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchRegionfcasrequirement2Row {
            predispatchseqno: row.predispatchseqno.clone(),
            runno: row.runno.clone(),
            intervention: row.intervention.clone(),
            periodid: row.periodid.clone(),
            genconid: row.genconid.clone(),
            regionid: row.regionid.clone(),
            bidtype: row.bidtype.clone(),
            genconeffectivedate: row.genconeffectivedate.clone(),
            genconversionno: row.genconversionno.clone(),
            marginalvalue: row.marginalvalue.clone(),
            datetime: row.datetime.clone(),
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
pub struct PredispatchRegionfcasrequirement2PrimaryKey {
    pub bidtype: alloc::string::String,
    pub datetime: chrono::NaiveDateTime,
    pub genconid: alloc::string::String,
    pub regionid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for PredispatchRegionfcasrequirement2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchRegionfcasrequirement2Row<'data> {
    type Row<'other> = PredispatchRegionfcasrequirement2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.datetime == row.datetime
            && self.genconid() == row.genconid() && self.regionid() == row.regionid()
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for PredispatchRegionfcasrequirement2Row<'data> {
    type PrimaryKey = PredispatchRegionfcasrequirement2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.datetime == key.datetime
            && self.genconid() == key.genconid && self.regionid() == key.regionid
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchRegionfcasrequirement2PrimaryKey {
    type Row<'other> = PredispatchRegionfcasrequirement2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.datetime == row.datetime
            && self.genconid == row.genconid() && self.regionid == row.regionid()
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchRegionfcasrequirement2PrimaryKey {
    type PrimaryKey = PredispatchRegionfcasrequirement2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.datetime == key.datetime
            && self.genconid == key.genconid && self.regionid == key.regionid
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchRegionfcasrequirement2 {
    type Builder = PredispatchRegionfcasrequirement2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
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
                    "datetime",
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
        PredispatchRegionfcasrequirement2Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            periodid_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            genconid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            genconeffectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            genconversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder
            .runno_array
            .append_option({
                row.runno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder.periodid_array.append_value(row.periodid.start().timestamp_millis());
        builder.genconid_array.append_value(row.genconid());
        builder.regionid_array.append_value(row.regionid());
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .genconeffectivedate_array
            .append_option(row.genconeffectivedate.map(|val| val.timestamp_millis()));
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
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
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
                    alloc::sync::Arc::new(builder.datetime_array.finish())
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
pub struct PredispatchRegionfcasrequirement2Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::TimestampMillisecondBuilder,
    genconid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    genconeffectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    genconversionno_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    base_cost_array: arrow::array::builder::Decimal128Builder,
    adjusted_cost_array: arrow::array::builder::Decimal128Builder,
    estimated_cmpf_array: arrow::array::builder::Decimal128Builder,
    estimated_crmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_cmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_crmpf_array: arrow::array::builder::Decimal128Builder,
}
pub struct PredispatchLocalPrice1;
pub struct PredispatchLocalPrice1Mapping([usize; 7]);
/// # Summary
///
/// ## PREDISPATCH_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period_
///
/// * Data Set Name: Predispatch
/// * File Name: Local Price
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * DATETIME
/// * DUID
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchLocalPrice1Row<'data> {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// The unique identifier for the interval within this study
    pub datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// A period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period
    pub periodid: core::ops::Range<usize>,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchLocalPrice1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for PredispatchLocalPrice1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "LOCAL_PRICE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchLocalPrice1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PREDISPATCHSEQNO",
        "DATETIME",
        "DUID",
        "PERIODID",
        "LOCAL_PRICE_ADJUSTMENT",
        "LOCALLY_CONSTRAINED",
        "LASTCHANGED",
    ];
    type Row<'row> = PredispatchLocalPrice1Row<'row>;
    type FieldMapping = PredispatchLocalPrice1Mapping;
    type PrimaryKey = PredispatchLocalPrice1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchLocalPrice1Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            datetime: row
                .get_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[2])?,
            periodid: row.get_opt_range("periodid", field_mapping.0[3])?,
            local_price_adjustment: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained",
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
        Ok(PredispatchLocalPrice1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let datetime = row
            .get_custom_parsed_at_idx("datetime", 5, mmsdm_core::mms_datetime::parse)?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(30) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchLocalPrice1PrimaryKey {
        PredispatchLocalPrice1PrimaryKey {
            datetime: row.datetime,
            duid: row.duid().to_string(),
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        30,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                30,
                            ) {
                                Some(d) => d,
                                None => panic!("invalid"),
                            };
                            D
                        })
                        .month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_local_price_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchLocalPrice1Row {
            predispatchseqno: row.predispatchseqno.clone(),
            datetime: row.datetime.clone(),
            duid: row.duid.clone(),
            periodid: row.periodid.clone(),
            local_price_adjustment: row.local_price_adjustment.clone(),
            locally_constrained: row.locally_constrained.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchLocalPrice1PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub duid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for PredispatchLocalPrice1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchLocalPrice1Row<'data> {
    type Row<'other> = PredispatchLocalPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.duid() == row.duid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchLocalPrice1Row<'data> {
    type PrimaryKey = PredispatchLocalPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid() == key.duid
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchLocalPrice1PrimaryKey {
    type Row<'other> = PredispatchLocalPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.datetime == row.datetime && self.duid == row.duid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchLocalPrice1PrimaryKey {
    type PrimaryKey = PredispatchLocalPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid == key.duid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchLocalPrice1 {
    type Builder = PredispatchLocalPrice1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "datetime",
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
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
        PredispatchLocalPrice1Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
            local_price_adjustment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 2)),
            locally_constrained_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder.datetime_array.append_value(row.datetime.timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.periodid_array.append_option(row.periodid());
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.local_price_adjustment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.locally_constrained_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchLocalPrice1Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
    local_price_adjustment_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct PredispatchMnspbidtrk1;
pub struct PredispatchMnspbidtrk1Mapping([usize; 9]);
/// # Summary
///
/// ## PREDISPATCH_MNSPBIDTRK
///  _PREDISPATCH_MNSPBIDTRK shows the MNSP bid tracking, including the bid version used in each predispatch run for each MNSP Interconnector Link. PREDISPATCH_MNSPBIDTRK shows the audit trail of the bid used for each predispatch run._
///
/// * Data Set Name: Predispatch
/// * File Name: Mnspbidtrk
/// * Data Version: 1
///
/// # Description
///  Source Own (confidential) data updates every predispatch run. All bids are available to all participants as part of next day market data. Volume 1, 700, 000 per year
///
///
///
/// # Primary Key Columns
///
/// * LINKID
/// * PERIODID
/// * PREDISPATCHSEQNO
#[derive(Debug, PartialEq, Eq)]
pub struct PredispatchMnspbidtrk1Row<'data> {
    /// Predispatch run identifier
    pub predispatchseqno: mmsdm_core::TradingPeriod,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: core::ops::Range<usize>,
    /// Trading Interval number
    pub periodid: mmsdm_core::TradingPeriod,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// Market Date from which bid is active
    pub settlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Version No. for given offer date and settlement date used
    pub versionno: Option<rust_decimal::Decimal>,
    /// Period expressed as Date/Time
    pub datetime: Option<chrono::NaiveDateTime>,
    /// Record creation timestamp
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PredispatchMnspbidtrk1Row<'data> {
    pub fn linkid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.linkid.clone())
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
}
impl mmsdm_core::GetTable for PredispatchMnspbidtrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PREDISPATCH";
    const TABLE_NAME: &'static str = "MNSPBIDTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PredispatchMnspbidtrk1Mapping([
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
        "PREDISPATCHSEQNO",
        "LINKID",
        "PERIODID",
        "PARTICIPANTID",
        "SETTLEMENTDATE",
        "OFFERDATE",
        "VERSIONNO",
        "DATETIME",
        "LASTCHANGED",
    ];
    type Row<'row> = PredispatchMnspbidtrk1Row<'row>;
    type FieldMapping = PredispatchMnspbidtrk1Mapping;
    type PrimaryKey = PredispatchMnspbidtrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PredispatchMnspbidtrk1Row {
            predispatchseqno: row
                .get_parsed_at_idx("predispatchseqno", field_mapping.0[0])?,
            linkid: row.get_range("linkid", field_mapping.0[1])?,
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[2])?,
            participantid: row.get_opt_range("participantid", field_mapping.0[3])?,
            settlementdate: row
                .get_opt_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdate: row
                .get_opt_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            datetime: row
                .get_opt_custom_parsed_at_idx(
                    "datetime",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(PredispatchMnspbidtrk1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let predispatchseqno: mmsdm_core::TradingPeriod = row
            .get_parsed_at_idx("predispatchseqno", 4)?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(predispatchseqno).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PredispatchMnspbidtrk1PrimaryKey {
        PredispatchMnspbidtrk1PrimaryKey {
            linkid: row.linkid().to_string(),
            periodid: row.periodid,
            predispatchseqno: row.predispatchseqno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.predispatchseqno).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.predispatchseqno).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_mnspbidtrk_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PredispatchMnspbidtrk1Row {
            predispatchseqno: row.predispatchseqno.clone(),
            linkid: row.linkid.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            settlementdate: row.settlementdate.clone(),
            offerdate: row.offerdate.clone(),
            versionno: row.versionno.clone(),
            datetime: row.datetime.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredispatchMnspbidtrk1PrimaryKey {
    pub linkid: alloc::string::String,
    pub periodid: mmsdm_core::TradingPeriod,
    pub predispatchseqno: mmsdm_core::TradingPeriod,
}
impl mmsdm_core::PrimaryKey for PredispatchMnspbidtrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PredispatchMnspbidtrk1Row<'data> {
    type Row<'other> = PredispatchMnspbidtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid() == row.linkid() && self.periodid == row.periodid
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PredispatchMnspbidtrk1Row<'data> {
    type PrimaryKey = PredispatchMnspbidtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid() == key.linkid && self.periodid == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PredispatchMnspbidtrk1PrimaryKey {
    type Row<'other> = PredispatchMnspbidtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid == row.linkid() && self.periodid == row.periodid
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PredispatchMnspbidtrk1PrimaryKey {
    type PrimaryKey = PredispatchMnspbidtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid && self.periodid == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PredispatchMnspbidtrk1 {
    type Builder = PredispatchMnspbidtrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "predispatchseqno",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "linkid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
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
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
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
                    "datetime",
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
        PredispatchMnspbidtrk1Builder {
            predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            linkid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .predispatchseqno_array
            .append_value(row.predispatchseqno.start().timestamp_millis());
        builder.linkid_array.append_value(row.linkid());
        builder.periodid_array.append_value(row.periodid.start().timestamp_millis());
        builder.participantid_array.append_option(row.participantid());
        builder
            .settlementdate_array
            .append_option(row.settlementdate.map(|val| val.timestamp_millis()));
        builder
            .offerdate_array
            .append_option(row.offerdate.map(|val| val.timestamp_millis()));
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
            .datetime_array
            .append_option(row.datetime.map(|val| val.timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.predispatchseqno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.linkid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PredispatchMnspbidtrk1Builder {
    predispatchseqno_array: arrow::array::builder::TimestampMillisecondBuilder,
    linkid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::TimestampMillisecondBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
