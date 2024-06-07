#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct P5minBlockedConstraints1;
pub struct P5minBlockedConstraints1Mapping([usize; 2]);
/// # Summary
///
/// ## P5MIN_BLOCKEDCONSTRAINT
///  _P5MIN Blocked Constraints lists any constraints that were blocked in a P5MIN run. If no constraints are blocked, there will be no rows for that 5 minute predispatch run._
///
/// * Data Set Name: P5min
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
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct P5minBlockedConstraints1Row<'data> {
    /// 5-minute Predispatch Run
    pub run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minBlockedConstraints1Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for P5minBlockedConstraints1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "BLOCKED_CONSTRAINTS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minBlockedConstraints1Mapping([
        4,
        5,
    ]);
    const COLUMNS: &'static [&'static str] = &["RUN_DATETIME", "CONSTRAINTID"];
    type Row<'row> = P5minBlockedConstraints1Row<'row>;
    type FieldMapping = P5minBlockedConstraints1Mapping;
    type PrimaryKey = P5minBlockedConstraints1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minBlockedConstraints1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
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
        Ok(P5minBlockedConstraints1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let run_datetime = row
            .get_custom_parsed_at_idx(
                "run_datetime",
                4,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
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
    fn primary_key(row: &Self::Row<'_>) -> P5minBlockedConstraints1PrimaryKey {
        P5minBlockedConstraints1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            run_datetime: row.run_datetime,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.run_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.run_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_blocked_constraints_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minBlockedConstraints1Row {
            run_datetime: row.run_datetime.clone(),
            constraintid: row.constraintid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minBlockedConstraints1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for P5minBlockedConstraints1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minBlockedConstraints1Row<'data> {
    type Row<'other> = P5minBlockedConstraints1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minBlockedConstraints1Row<'data> {
    type PrimaryKey = P5minBlockedConstraints1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minBlockedConstraints1PrimaryKey {
    type Row<'other> = P5minBlockedConstraints1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid() && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minBlockedConstraints1PrimaryKey {
    type PrimaryKey = P5minBlockedConstraints1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minBlockedConstraints1 {
    type Builder = P5minBlockedConstraints1Builder;
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
                    "constraintid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        P5minBlockedConstraints1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
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
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct P5minBlockedConstraints1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
}
pub struct P5minCasesolution2;
pub struct P5minCasesolution2Mapping([usize; 19]);
/// # Summary
///
/// ## P5MIN_CASESOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_CASESOLUTION shows one record containing results pertaining to the entire solution._
///
/// * Data Set Name: P5min
/// * File Name: Casesolution
/// * Data Version: 2
///
/// # Description
///  P5MIN_CASESOLUTION data is public, so is available to all participants. Source P5MIN_CASESOLUTION updates every 5 minutes. Volume Rows per day: 288
///
///
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minCasesolution2Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// Date and Time of first interval in study
    pub startinterval_datetime: core::ops::Range<usize>,
    /// The Objective function from the LP
    pub totalobjective: Option<rust_decimal::Decimal>,
    /// Flag to indicate non-physical losses occurred in this study
    pub nonphysicallosses: Option<rust_decimal::Decimal>,
    /// Sum of Regional Energy balance violations
    pub totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Sum of Interconnector violations of standing data limits
    pub totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Sum of Generic Constraint violations
    pub totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Sum of Unit Ramp Rate violations
    pub totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit capacity violations
    pub totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 5 min FCAS violations
    pub total5minviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional regulation FCAS violations
    pub totalregviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 6 sec FCAS violations
    pub total6secviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 60 sec FCAS violations
    pub total60secviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit energy constrained violations
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit offer violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit FCAS profile offer violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit Fast start profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    /// Last changed date and time of this record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    pub intervention: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minCasesolution2Row<'data> {
    pub fn startinterval_datetime(&self) -> Option<&str> {
        if self.startinterval_datetime.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.startinterval_datetime.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for P5minCasesolution2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "CASESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minCasesolution2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "STARTINTERVAL_DATETIME",
        "TOTALOBJECTIVE",
        "NONPHYSICALLOSSES",
        "TOTALAREAGENVIOLATION",
        "TOTALINTERCONNECTORVIOLATION",
        "TOTALGENERICVIOLATION",
        "TOTALRAMPRATEVIOLATION",
        "TOTALUNITMWCAPACITYVIOLATION",
        "TOTAL5MINVIOLATION",
        "TOTALREGVIOLATION",
        "TOTAL6SECVIOLATION",
        "TOTAL60SECVIOLATION",
        "TOTALENERGYCONSTRVIOLATION",
        "TOTALENERGYOFFERVIOLATION",
        "TOTALASPROFILEVIOLATION",
        "TOTALFASTSTARTVIOLATION",
        "LASTCHANGED",
        "INTERVENTION",
    ];
    type Row<'row> = P5minCasesolution2Row<'row>;
    type FieldMapping = P5minCasesolution2Mapping;
    type PrimaryKey = P5minCasesolution2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minCasesolution2Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startinterval_datetime: row
                .get_opt_range("startinterval_datetime", field_mapping.0[1])?,
            totalobjective: row
                .get_opt_custom_parsed_at_idx(
                    "totalobjective",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            nonphysicallosses: row
                .get_opt_custom_parsed_at_idx(
                    "nonphysicallosses",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalareagenviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalareagenviolation",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalinterconnectorviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalinterconnectorviolation",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalgenericviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalgenericviolation",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalramprateviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalramprateviolation",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalunitmwcapacityviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalunitmwcapacityviolation",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total5minviolation",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalregviolation",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total6secviolation",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total60secviolation",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalenergyconstrviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalenergyconstrviolation",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalenergyofferviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalenergyofferviolation",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalasprofileviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalasprofileviolation",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalfaststartviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalfaststartviolation",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[17],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[18],
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
        Ok(P5minCasesolution2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let run_datetime = row
            .get_custom_parsed_at_idx(
                "run_datetime",
                4,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
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
    fn primary_key(row: &Self::Row<'_>) -> P5minCasesolution2PrimaryKey {
        P5minCasesolution2PrimaryKey {
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.run_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.run_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_casesolution_v2_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minCasesolution2Row {
            run_datetime: row.run_datetime.clone(),
            startinterval_datetime: row.startinterval_datetime.clone(),
            totalobjective: row.totalobjective.clone(),
            nonphysicallosses: row.nonphysicallosses.clone(),
            totalareagenviolation: row.totalareagenviolation.clone(),
            totalinterconnectorviolation: row.totalinterconnectorviolation.clone(),
            totalgenericviolation: row.totalgenericviolation.clone(),
            totalramprateviolation: row.totalramprateviolation.clone(),
            totalunitmwcapacityviolation: row.totalunitmwcapacityviolation.clone(),
            total5minviolation: row.total5minviolation.clone(),
            totalregviolation: row.totalregviolation.clone(),
            total6secviolation: row.total6secviolation.clone(),
            total60secviolation: row.total60secviolation.clone(),
            totalenergyconstrviolation: row.totalenergyconstrviolation.clone(),
            totalenergyofferviolation: row.totalenergyofferviolation.clone(),
            totalasprofileviolation: row.totalasprofileviolation.clone(),
            totalfaststartviolation: row.totalfaststartviolation.clone(),
            lastchanged: row.lastchanged.clone(),
            intervention: row.intervention.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minCasesolution2PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minCasesolution2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minCasesolution2Row<'data> {
    type Row<'other> = P5minCasesolution2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minCasesolution2Row<'data> {
    type PrimaryKey = P5minCasesolution2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minCasesolution2PrimaryKey {
    type Row<'other> = P5minCasesolution2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minCasesolution2PrimaryKey {
    type PrimaryKey = P5minCasesolution2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minCasesolution2 {
    type Builder = P5minCasesolution2Builder;
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
                    "startinterval_datetime",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalobjective",
                    arrow::datatypes::DataType::Decimal128(27, 10),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nonphysicallosses",
                    arrow::datatypes::DataType::Decimal128(1, 0),
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
        P5minCasesolution2Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startinterval_datetime_array: arrow::array::builder::StringBuilder::new(),
            totalobjective_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 10)),
            nonphysicallosses_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
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
            totalenergyconstrviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalasprofileviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalfaststartviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder.startinterval_datetime_array.append_option(row.startinterval_datetime());
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
            .nonphysicallosses_array
            .append_option({
                row.nonphysicallosses
                    .map(|mut val| {
                        val.rescale(0);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startinterval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalobjective_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nonphysicallosses_array.finish())
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
                    alloc::sync::Arc::new(
                        builder.totalenergyconstrviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.totalenergyofferviolation_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalasprofileviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalfaststartviolation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
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
pub struct P5minCasesolution2Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    startinterval_datetime_array: arrow::array::builder::StringBuilder,
    totalobjective_array: arrow::array::builder::Decimal128Builder,
    nonphysicallosses_array: arrow::array::builder::Decimal128Builder,
    totalareagenviolation_array: arrow::array::builder::Decimal128Builder,
    totalinterconnectorviolation_array: arrow::array::builder::Decimal128Builder,
    totalgenericviolation_array: arrow::array::builder::Decimal128Builder,
    totalramprateviolation_array: arrow::array::builder::Decimal128Builder,
    totalunitmwcapacityviolation_array: arrow::array::builder::Decimal128Builder,
    total5minviolation_array: arrow::array::builder::Decimal128Builder,
    totalregviolation_array: arrow::array::builder::Decimal128Builder,
    total6secviolation_array: arrow::array::builder::Decimal128Builder,
    total60secviolation_array: arrow::array::builder::Decimal128Builder,
    totalenergyconstrviolation_array: arrow::array::builder::Decimal128Builder,
    totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder,
    totalasprofileviolation_array: arrow::array::builder::Decimal128Builder,
    totalfaststartviolation_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
}
pub struct P5minConstraintsolution6;
pub struct P5minConstraintsolution6Mapping([usize; 12]);
/// # Summary
///
/// ## P5MIN_CONSTRAINTSOLUTION
///  _The Five-Minute Pre-Dispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The Five-Minute Pre-dispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value._
///
/// * Data Set Name: P5min
/// * File Name: Constraintsolution
/// * Data Version: 6
///
/// # Description
///  P5MIN_CONSTRAINTSOLUTION is public data, so is available to all participants. Source P5MIN_CONSTRAINTSOLUTION updates every five minutes. Volume Rows per day: ~2.3 million
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minConstraintsolution6Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    /// Right Hand Side value in the capacity evaluation
    pub rhs: Option<rust_decimal::Decimal>,
    /// Marginal cost of constraint (&gt;0 if binding)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Amount of Violation (&gt;0 if  violating)
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
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minConstraintsolution6Row<'data> {
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
impl mmsdm_core::GetTable for P5minConstraintsolution6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "CONSTRAINTSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minConstraintsolution6Mapping([
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
        "INTERVAL_DATETIME",
        "CONSTRAINTID",
        "RHS",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "LASTCHANGED",
        "DUID",
        "GENCONID_EFFECTIVEDATE",
        "GENCONID_VERSIONNO",
        "LHS",
        "INTERVENTION",
    ];
    type Row<'row> = P5minConstraintsolution6Row<'row>;
    type FieldMapping = P5minConstraintsolution6Mapping;
    type PrimaryKey = P5minConstraintsolution6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minConstraintsolution6Row {
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
            constraintid: row.get_range("constraintid", field_mapping.0[2])?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violationdegree",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_opt_range("duid", field_mapping.0[7])?,
            genconid_effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "genconid_effectivedate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            genconid_versionno: row
                .get_opt_custom_parsed_at_idx(
                    "genconid_versionno",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lhs: row
                .get_opt_custom_parsed_at_idx(
                    "lhs",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[11],
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
        Ok(P5minConstraintsolution6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                5,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minConstraintsolution6PrimaryKey {
        P5minConstraintsolution6PrimaryKey {
            constraintid: row.constraintid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_constraintsolution_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minConstraintsolution6Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            constraintid: row.constraintid.clone(),
            rhs: row.rhs.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            lastchanged: row.lastchanged.clone(),
            duid: row.duid.clone(),
            genconid_effectivedate: row.genconid_effectivedate.clone(),
            genconid_versionno: row.genconid_versionno.clone(),
            lhs: row.lhs.clone(),
            intervention: row.intervention.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minConstraintsolution6PrimaryKey {
    pub constraintid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minConstraintsolution6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minConstraintsolution6Row<'data> {
    type Row<'other> = P5minConstraintsolution6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minConstraintsolution6Row<'data> {
    type PrimaryKey = P5minConstraintsolution6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minConstraintsolution6PrimaryKey {
    type Row<'other> = P5minConstraintsolution6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minConstraintsolution6PrimaryKey {
    type PrimaryKey = P5minConstraintsolution6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minConstraintsolution6 {
    type Builder = P5minConstraintsolution6Builder;
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
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        P5minConstraintsolution6Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
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
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
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
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
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
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct P5minConstraintsolution6Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
    rhs_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    genconid_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    genconid_versionno_array: arrow::array::builder::Decimal128Builder,
    lhs_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
}
pub struct P5minFcasRequirment1;
pub struct P5minFcasRequirment1Mapping([usize; 16]);
/// # Summary
///
/// ## P5MIN_FCAS_REQUIREMENT
///  _5-minute Predispatch constraint tracking for Regional FCAS recovery_
///
/// * Data Set Name: P5min
/// * File Name: Fcas Requirment
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minFcasRequirment1Row<'data> {
    /// First interval of the 5-minute Predispatch case
    pub run_datetime: chrono::NaiveDateTime,
    /// Datetime of the 5-minute Predispatch interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// ConstraintID Join to table GenConData
    pub constraintid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// DUID offered type
    pub bidtype: core::ops::Range<usize>,
    /// Intervention flag
    pub intervention: rust_decimal::Decimal,
    /// Constraint EffectiveDate Join to table GenConData
    pub constraint_effectivedate: Option<chrono::NaiveDateTime>,
    /// Constraint Version number Join to table GenConData
    pub constraint_versionno: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// The base cost of the constraint for this service, before the regulation/contingency split
    pub base_cost: Option<rust_decimal::Decimal>,
    /// The adjusted cost of the constraint for this service, after the regulation/contingency split
    pub adjusted_cost: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CMPF, based on 5- minute Predispatch data
    pub estimated_cmpf: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CRMPF, based on 5-minute Predispatch data
    pub estimated_crmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CMPF based recovery
    pub recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery for CRMPF based recovery
    pub recovery_factor_crmpf: Option<rust_decimal::Decimal>,
    /// Last changed date for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minFcasRequirment1Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for P5minFcasRequirment1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "FCAS_REQUIRMENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minFcasRequirment1Mapping([
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
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "CONSTRAINTID",
        "REGIONID",
        "BIDTYPE",
        "INTERVENTION",
        "CONSTRAINT_EFFECTIVEDATE",
        "CONSTRAINT_VERSIONNO",
        "MARGINALVALUE",
        "BASE_COST",
        "ADJUSTED_COST",
        "ESTIMATED_CMPF",
        "ESTIMATED_CRMPF",
        "RECOVERY_FACTOR_CMPF",
        "RECOVERY_FACTOR_CRMPF",
        "LASTCHANGED",
    ];
    type Row<'row> = P5minFcasRequirment1Row<'row>;
    type FieldMapping = P5minFcasRequirment1Mapping;
    type PrimaryKey = P5minFcasRequirment1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minFcasRequirment1Row {
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
            constraintid: row.get_range("constraintid", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            bidtype: row.get_range("bidtype", field_mapping.0[4])?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constraint_effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "constraint_effectivedate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraint_versionno: row
                .get_opt_custom_parsed_at_idx(
                    "constraint_versionno",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            base_cost: row
                .get_opt_custom_parsed_at_idx(
                    "base_cost",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            adjusted_cost: row
                .get_opt_custom_parsed_at_idx(
                    "adjusted_cost",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            estimated_cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "estimated_cmpf",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            estimated_crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "estimated_crmpf",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_cmpf",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_crmpf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[15],
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
        Ok(P5minFcasRequirment1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                5,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minFcasRequirment1PrimaryKey {
        P5minFcasRequirment1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            constraintid: row.constraintid().to_string(),
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_fcas_requirment_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minFcasRequirment1Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            constraintid: row.constraintid.clone(),
            regionid: row.regionid.clone(),
            bidtype: row.bidtype.clone(),
            intervention: row.intervention.clone(),
            constraint_effectivedate: row.constraint_effectivedate.clone(),
            constraint_versionno: row.constraint_versionno.clone(),
            marginalvalue: row.marginalvalue.clone(),
            base_cost: row.base_cost.clone(),
            adjusted_cost: row.adjusted_cost.clone(),
            estimated_cmpf: row.estimated_cmpf.clone(),
            estimated_crmpf: row.estimated_crmpf.clone(),
            recovery_factor_cmpf: row.recovery_factor_cmpf.clone(),
            recovery_factor_crmpf: row.recovery_factor_crmpf.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minFcasRequirment1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub constraintid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minFcasRequirment1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minFcasRequirment1Row<'data> {
    type Row<'other> = P5minFcasRequirment1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.constraintid() == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minFcasRequirment1Row<'data> {
    type PrimaryKey = P5minFcasRequirment1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.constraintid() == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minFcasRequirment1PrimaryKey {
    type Row<'other> = P5minFcasRequirment1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.constraintid == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minFcasRequirment1PrimaryKey {
    type PrimaryKey = P5minFcasRequirment1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minFcasRequirment1 {
    type Builder = P5minFcasRequirment1Builder;
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
                    "constraintid",
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraint_effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constraint_versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        P5minFcasRequirment1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            constraint_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraint_versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
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
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
        builder.regionid_array.append_value(row.regionid());
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .constraint_effectivedate_array
            .append_option(
                row.constraint_effectivedate.map(|val| val.timestamp_millis()),
            );
        builder
            .constraint_versionno_array
            .append_option({
                row.constraint_versionno
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
                        val.rescale(8);
                        val.mantissa()
                    })
            });
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.constraint_effectivedate_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraint_versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct P5minFcasRequirment1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    constraint_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraint_versionno_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    base_cost_array: arrow::array::builder::Decimal128Builder,
    adjusted_cost_array: arrow::array::builder::Decimal128Builder,
    estimated_cmpf_array: arrow::array::builder::Decimal128Builder,
    estimated_crmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_cmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_crmpf_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct P5minInterconnectorsoln4;
pub struct P5minInterconnectorsoln4Mapping([usize; 22]);
/// # Summary
///
/// ## P5MIN_INTERCONNECTORSOLN
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_INTERCONNECTORSOLN sets out the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval._
///
/// * Data Set Name: P5min
/// * File Name: Interconnectorsoln
/// * Data Version: 4
///
/// # Description
///  P5MIN_INTERCONNECTORSOLN is public data, so is available to all participants. Source P5MIN_INTERCONNECTORSOLN updates every 5 minutes. Volume Rows per day: 1440 Based on 200 interconnector/binding constraints per interval
///
///
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minInterconnectorsoln4Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// SCADA MW Flow measured at Run start. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Cleared Interconnector loading level (MW)
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Interconnector Losses at cleared flow
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Marginal cost of Interconnector standing data limits (if binding)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation of Interconnector standing data limits
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Flag indicating MNSP registration
    pub mnsp: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor at the cleared flow
    pub marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    pub exportgenconid: core::ops::Range<usize>,
    /// Generic Constraint setting the import limit
    pub importgenconid: core::ops::Range<usize>,
    /// Calculated export limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasimportlimit: Option<rust_decimal::Decimal>,
    /// Last changed date of this record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    pub local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    pub local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_import: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minInterconnectorsoln4Row<'data> {
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
impl mmsdm_core::GetTable for P5minInterconnectorsoln4 {
    const VERSION: i32 = 4;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "INTERCONNECTORSOLN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minInterconnectorsoln4Mapping([
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
        "RUN_DATETIME",
        "INTERCONNECTORID",
        "INTERVAL_DATETIME",
        "METEREDMWFLOW",
        "MWFLOW",
        "MWLOSSES",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "MNSP",
        "EXPORTLIMIT",
        "IMPORTLIMIT",
        "MARGINALLOSS",
        "EXPORTGENCONID",
        "IMPORTGENCONID",
        "FCASEXPORTLIMIT",
        "FCASIMPORTLIMIT",
        "LASTCHANGED",
        "LOCAL_PRICE_ADJUSTMENT_EXPORT",
        "LOCALLY_CONSTRAINED_EXPORT",
        "LOCAL_PRICE_ADJUSTMENT_IMPORT",
        "LOCALLY_CONSTRAINED_IMPORT",
        "INTERVENTION",
    ];
    type Row<'row> = P5minInterconnectorsoln4Row<'row>;
    type FieldMapping = P5minInterconnectorsoln4Mapping;
    type PrimaryKey = P5minInterconnectorsoln4PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minInterconnectorsoln4Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            meteredmwflow: row
                .get_opt_custom_parsed_at_idx(
                    "meteredmwflow",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwlosses: row
                .get_opt_custom_parsed_at_idx(
                    "mwlosses",
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
            mnsp: row
                .get_opt_custom_parsed_at_idx(
                    "mnsp",
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
            exportgenconid: row.get_opt_range("exportgenconid", field_mapping.0[12])?,
            importgenconid: row.get_opt_range("importgenconid", field_mapping.0[13])?,
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
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[16],
                    mmsdm_core::mms_datetime::parse,
                )?,
            local_price_adjustment_export: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_export",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_export: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_export",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            local_price_adjustment_import: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment_import",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained_import: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained_import",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
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
        Ok(P5minInterconnectorsoln4Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                6,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minInterconnectorsoln4PrimaryKey {
        P5minInterconnectorsoln4PrimaryKey {
            interconnectorid: row.interconnectorid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_interconnectorsoln_v4_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minInterconnectorsoln4Row {
            run_datetime: row.run_datetime.clone(),
            interconnectorid: row.interconnectorid.clone(),
            interval_datetime: row.interval_datetime.clone(),
            meteredmwflow: row.meteredmwflow.clone(),
            mwflow: row.mwflow.clone(),
            mwlosses: row.mwlosses.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            mnsp: row.mnsp.clone(),
            exportlimit: row.exportlimit.clone(),
            importlimit: row.importlimit.clone(),
            marginalloss: row.marginalloss.clone(),
            exportgenconid: row.exportgenconid.clone(),
            importgenconid: row.importgenconid.clone(),
            fcasexportlimit: row.fcasexportlimit.clone(),
            fcasimportlimit: row.fcasimportlimit.clone(),
            lastchanged: row.lastchanged.clone(),
            local_price_adjustment_export: row.local_price_adjustment_export.clone(),
            locally_constrained_export: row.locally_constrained_export.clone(),
            local_price_adjustment_import: row.local_price_adjustment_import.clone(),
            locally_constrained_import: row.locally_constrained_import.clone(),
            intervention: row.intervention.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minInterconnectorsoln4PrimaryKey {
    pub interconnectorid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minInterconnectorsoln4PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minInterconnectorsoln4Row<'data> {
    type Row<'other> = P5minInterconnectorsoln4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minInterconnectorsoln4Row<'data> {
    type PrimaryKey = P5minInterconnectorsoln4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minInterconnectorsoln4PrimaryKey {
    type Row<'other> = P5minInterconnectorsoln4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minInterconnectorsoln4PrimaryKey {
    type PrimaryKey = P5minInterconnectorsoln4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minInterconnectorsoln4 {
    type Builder = P5minInterconnectorsoln4Builder;
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
                    "interconnectorid",
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
                    "mnsp",
                    arrow::datatypes::DataType::Decimal128(1, 0),
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
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        P5minInterconnectorsoln4Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            mnsp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
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
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            local_price_adjustment_export_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 2)),
            locally_constrained_export_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            local_price_adjustment_import_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 2)),
            locally_constrained_import_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
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
            .mnsp_array
            .append_option({
                row.mnsp
                    .map(|mut val| {
                        val.rescale(0);
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
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
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
                    alloc::sync::Arc::new(builder.mnsp_array.finish())
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
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
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct P5minInterconnectorsoln4Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    meteredmwflow_array: arrow::array::builder::Decimal128Builder,
    mwflow_array: arrow::array::builder::Decimal128Builder,
    mwlosses_array: arrow::array::builder::Decimal128Builder,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    mnsp_array: arrow::array::builder::Decimal128Builder,
    exportlimit_array: arrow::array::builder::Decimal128Builder,
    importlimit_array: arrow::array::builder::Decimal128Builder,
    marginalloss_array: arrow::array::builder::Decimal128Builder,
    exportgenconid_array: arrow::array::builder::StringBuilder,
    importgenconid_array: arrow::array::builder::StringBuilder,
    fcasexportlimit_array: arrow::array::builder::Decimal128Builder,
    fcasimportlimit_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    local_price_adjustment_export_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_export_array: arrow::array::builder::Decimal128Builder,
    local_price_adjustment_import_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_import_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
}
pub struct P5minIntersensitivities1;
pub struct P5minIntersensitivities1Mapping([usize; 49]);
/// # Summary
///
/// ## P5MIN_INTERSENSITIVITIES
///  _Price Sensitivies for 5MinPD solution. New solution every 5 minutes. Current Scenarios defined in P5MIN_SCENARIODEMANDTRK/P5MIN_SCENARIODEMAND_
///
/// * Data Set Name: P5min
/// * File Name: Intersensitivities
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minIntersensitivities1Row<'data> {
    /// Definitive Run from which this solution derives
    pub run_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: rust_decimal::Decimal,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow1 = MW flow for given Interconnector for Scenario 1
    pub mwflow1: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow2 = MW flow for given Interconnector for Scenario 2
    pub mwflow2: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow3 = MW flow for given Interconnector for Scenario 3
    pub mwflow3: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow4 = MW flow for given Interconnector for Scenario 4
    pub mwflow4: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow5 = MW flow for given Interconnector for Scenario 5
    pub mwflow5: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow6 = MW flow for given Interconnector for Scenario 6
    pub mwflow6: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow7 = MW flow for given Interconnector for Scenario 7
    pub mwflow7: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow8 = MW flow for given Interconnector for Scenario 8
    pub mwflow8: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow9 = MW flow for given Interconnector for Scenario 9
    pub mwflow9: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow10 = MW flow for given Interconnector for Scenario 10
    pub mwflow10: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow11 = MW flow for given Interconnector for Scenario 11
    pub mwflow11: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow12 = MW flow for given Interconnector for Scenario 12
    pub mwflow12: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow13 = MW flow for given Interconnector for Scenario 13
    pub mwflow13: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow14 = MW flow for given Interconnector for Scenario 14
    pub mwflow14: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow15 = MW flow for given Interconnector for Scenario 15
    pub mwflow15: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow16 = MW flow for given Interconnector for Scenario 16
    pub mwflow16: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow17 = MW flow for given Interconnector for Scenario 17
    pub mwflow17: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow18 = MW flow for given Interconnector for Scenario 18
    pub mwflow18: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow19 = MW flow for given Interconnector for Scenario 19
    pub mwflow19: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow20 = MW flow for given Interconnector for Scenario 20
    pub mwflow20: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow21 = MW flow for given Interconnector for Scenario 21
    pub mwflow21: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow22 = MW flow for given Interconnector for Scenario 22
    pub mwflow22: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow23 = MW flow for given Interconnector for Scenario 23
    pub mwflow23: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow24 = MW flow for given Interconnector for Scenario 24
    pub mwflow24: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow25 = MW flow for given Interconnector for Scenario 25
    pub mwflow25: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow26 = MW flow for given Interconnector for Scenario 26
    pub mwflow26: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow27 = MW flow for given Interconnector for Scenario 27
    pub mwflow27: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow28 = MW flow for given Interconnector for Scenario 28
    pub mwflow28: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow29 = MW flow for given Interconnector for Scenario 29
    pub mwflow29: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow30 = MW flow for given Interconnector for Scenario 30
    pub mwflow30: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow31 = MW flow for given Interconnector for Scenario 31
    pub mwflow31: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow32 = MW flow for given Interconnector for Scenario 32
    pub mwflow32: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow33 = MW flow for given Interconnector for Scenario 33
    pub mwflow33: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow34 = MW flow for given Interconnector for Scenario 34
    pub mwflow34: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow35 = MW flow for given Interconnector for Scenario 35
    pub mwflow35: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow36 = MW flow for given Interconnector for Scenario 36
    pub mwflow36: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow37 = MW flow for given Interconnector for Scenario 37
    pub mwflow37: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow38 = MW flow for given Interconnector for Scenario 38
    pub mwflow38: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow39 = MW flow for given Interconnector for Scenario 39
    pub mwflow39: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow40 = MW flow for given Interconnector for Scenario 40
    pub mwflow40: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow41 = MW flow for given Interconnector for Scenario 41
    pub mwflow41: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow42 = MW flow for given Interconnector for Scenario 42
    pub mwflow42: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow43 = MW flow for given Interconnector for Scenario 43
    pub mwflow43: Option<rust_decimal::Decimal>,
    /// Timestamp when this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minIntersensitivities1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for P5minIntersensitivities1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "INTERSENSITIVITIES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minIntersensitivities1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERCONNECTORID",
        "INTERVAL_DATETIME",
        "INTERVENTION",
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
    type Row<'row> = P5minIntersensitivities1Row<'row>;
    type FieldMapping = P5minIntersensitivities1Mapping;
    type PrimaryKey = P5minIntersensitivities1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minIntersensitivities1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention_active: row
                .get_opt_custom_parsed_at_idx(
                    "intervention_active",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow1: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow1",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow2: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow2",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow3: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow3",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow4: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow4",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow5: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow5",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow6: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow6",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow7: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow7",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow8: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow8",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow9: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow9",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow10: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow10",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow11: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow11",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow12: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow12",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow13: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow13",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow14: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow14",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow15: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow15",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow16: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow16",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow17: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow17",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow18: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow18",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow19: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow19",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow20: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow20",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow21: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow21",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow22: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow22",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow23: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow23",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow24: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow24",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow25: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow25",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow26: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow26",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow27: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow27",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow28: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow28",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow29: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow29",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow30: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow30",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow31: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow31",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow32: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow32",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow33: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow33",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow34: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow34",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow35: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow35",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow36: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow36",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow37: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow37",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow38: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow38",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow39: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow39",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow40: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow40",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow41: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow41",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow42: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow42",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwflow43: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow43",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[48],
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
        Ok(P5minIntersensitivities1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                6,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minIntersensitivities1PrimaryKey {
        P5minIntersensitivities1PrimaryKey {
            interconnectorid: row.interconnectorid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_intersensitivities_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minIntersensitivities1Row {
            run_datetime: row.run_datetime.clone(),
            interconnectorid: row.interconnectorid.clone(),
            interval_datetime: row.interval_datetime.clone(),
            intervention: row.intervention.clone(),
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
pub struct P5minIntersensitivities1PrimaryKey {
    pub interconnectorid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minIntersensitivities1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minIntersensitivities1Row<'data> {
    type Row<'other> = P5minIntersensitivities1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minIntersensitivities1Row<'data> {
    type PrimaryKey = P5minIntersensitivities1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minIntersensitivities1PrimaryKey {
    type Row<'other> = P5minIntersensitivities1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minIntersensitivities1PrimaryKey {
    type PrimaryKey = P5minIntersensitivities1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minIntersensitivities1 {
    type Builder = P5minIntersensitivities1Builder;
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
                    "interconnectorid",
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(1, 0),
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
        P5minIntersensitivities1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
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
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
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
pub struct P5minIntersensitivities1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
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
pub struct P5minLocalPrice1;
pub struct P5minLocalPrice1Mapping([usize; 5]);
/// # Summary
///
/// ## P5MIN_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period_
///
/// * Data Set Name: P5min
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
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct P5minLocalPrice1Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minLocalPrice1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for P5minLocalPrice1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "LOCAL_PRICE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minLocalPrice1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "DUID",
        "LOCAL_PRICE_ADJUSTMENT",
        "LOCALLY_CONSTRAINED",
    ];
    type Row<'row> = P5minLocalPrice1Row<'row>;
    type FieldMapping = P5minLocalPrice1Mapping;
    type PrimaryKey = P5minLocalPrice1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minLocalPrice1Row {
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
            duid: row.get_range("duid", field_mapping.0[2])?,
            local_price_adjustment: row
                .get_opt_custom_parsed_at_idx(
                    "local_price_adjustment",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            locally_constrained: row
                .get_opt_custom_parsed_at_idx(
                    "locally_constrained",
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
        Ok(P5minLocalPrice1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                5,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minLocalPrice1PrimaryKey {
        P5minLocalPrice1PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_local_price_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minLocalPrice1Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            duid: row.duid.clone(),
            local_price_adjustment: row.local_price_adjustment.clone(),
            locally_constrained: row.locally_constrained.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minLocalPrice1PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for P5minLocalPrice1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minLocalPrice1Row<'data> {
    type Row<'other> = P5minLocalPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minLocalPrice1Row<'data> {
    type PrimaryKey = P5minLocalPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minLocalPrice1PrimaryKey {
    type Row<'other> = P5minLocalPrice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minLocalPrice1PrimaryKey {
    type PrimaryKey = P5minLocalPrice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minLocalPrice1 {
    type Builder = P5minLocalPrice1Builder;
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
        P5minLocalPrice1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            local_price_adjustment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 2)),
            locally_constrained_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
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
pub struct P5minLocalPrice1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    local_price_adjustment_array: arrow::array::builder::Decimal128Builder,
    locally_constrained_array: arrow::array::builder::Decimal128Builder,
}
pub struct P5minPricesensitivities1;
pub struct P5minPricesensitivities1Mapping([usize; 49]);
/// # Summary
///
/// ## P5MIN_PRICESENSITIVITIES
///  _Price Sensitivies for 5MinPD solution. New solution every 5 minutes. Current Scenarios defined in P5MIN_SCENARIODEMANDTRK/P5MIN_SCENARIODEMAND_
///
/// * Data Set Name: P5min
/// * File Name: Pricesensitivities
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minPricesensitivities1Row<'data> {
    /// Definitive Run from which this solution derives
    pub run_datetime: chrono::NaiveDateTime,
    /// Region
    pub regionid: core::ops::Range<usize>,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Whether an Intervention constraint was defined in this run
    pub intervention: rust_decimal::Decimal,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 1
    pub rrp1: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 2
    pub rrp2: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 3
    pub rrp3: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 4
    pub rrp4: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 5
    pub rrp5: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 6
    pub rrp6: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 7
    pub rrp7: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 8
    pub rrp8: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 9
    pub rrp9: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 10
    pub rrp10: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 11
    pub rrp11: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 12
    pub rrp12: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 13
    pub rrp13: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 14
    pub rrp14: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 15
    pub rrp15: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 16
    pub rrp16: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 17
    pub rrp17: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 18
    pub rrp18: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 19
    pub rrp19: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 20
    pub rrp20: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 21
    pub rrp21: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 22
    pub rrp22: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 23
    pub rrp23: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 24
    pub rrp24: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 25
    pub rrp25: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 26
    pub rrp26: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 27
    pub rrp27: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 28
    pub rrp28: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 29
    pub rrp29: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 30
    pub rrp30: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 31
    pub rrp31: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 32
    pub rrp32: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 33
    pub rrp33: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 34
    pub rrp34: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 35
    pub rrp35: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 36
    pub rrp36: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 37
    pub rrp37: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 38
    pub rrp38: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 39
    pub rrp39: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 40
    pub rrp40: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 41
    pub rrp41: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 42
    pub rrp42: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 43
    pub rrp43: Option<rust_decimal::Decimal>,
    /// Timestamp when this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minPricesensitivities1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for P5minPricesensitivities1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "PRICESENSITIVITIES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minPricesensitivities1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "REGIONID",
        "INTERVAL_DATETIME",
        "INTERVENTION",
        "INTERVENTION_ACTIVE",
        "RRP1",
        "RRP2",
        "RRP3",
        "RRP4",
        "RRP5",
        "RRP6",
        "RRP7",
        "RRP8",
        "RRP9",
        "RRP10",
        "RRP11",
        "RRP12",
        "RRP13",
        "RRP14",
        "RRP15",
        "RRP16",
        "RRP17",
        "RRP18",
        "RRP19",
        "RRP20",
        "RRP21",
        "RRP22",
        "RRP23",
        "RRP24",
        "RRP25",
        "RRP26",
        "RRP27",
        "RRP28",
        "RRP29",
        "RRP30",
        "RRP31",
        "RRP32",
        "RRP33",
        "RRP34",
        "RRP35",
        "RRP36",
        "RRP37",
        "RRP38",
        "RRP39",
        "RRP40",
        "RRP41",
        "RRP42",
        "RRP43",
        "LASTCHANGED",
    ];
    type Row<'row> = P5minPricesensitivities1Row<'row>;
    type FieldMapping = P5minPricesensitivities1Mapping;
    type PrimaryKey = P5minPricesensitivities1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minPricesensitivities1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
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
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention_active: row
                .get_opt_custom_parsed_at_idx(
                    "intervention_active",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp1: row
                .get_opt_custom_parsed_at_idx(
                    "rrp1",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp2: row
                .get_opt_custom_parsed_at_idx(
                    "rrp2",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp3: row
                .get_opt_custom_parsed_at_idx(
                    "rrp3",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp4: row
                .get_opt_custom_parsed_at_idx(
                    "rrp4",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp5: row
                .get_opt_custom_parsed_at_idx(
                    "rrp5",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp6: row
                .get_opt_custom_parsed_at_idx(
                    "rrp6",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp7: row
                .get_opt_custom_parsed_at_idx(
                    "rrp7",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp8: row
                .get_opt_custom_parsed_at_idx(
                    "rrp8",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp9: row
                .get_opt_custom_parsed_at_idx(
                    "rrp9",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp10: row
                .get_opt_custom_parsed_at_idx(
                    "rrp10",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp11: row
                .get_opt_custom_parsed_at_idx(
                    "rrp11",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp12: row
                .get_opt_custom_parsed_at_idx(
                    "rrp12",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp13: row
                .get_opt_custom_parsed_at_idx(
                    "rrp13",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp14: row
                .get_opt_custom_parsed_at_idx(
                    "rrp14",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp15: row
                .get_opt_custom_parsed_at_idx(
                    "rrp15",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp16: row
                .get_opt_custom_parsed_at_idx(
                    "rrp16",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp17: row
                .get_opt_custom_parsed_at_idx(
                    "rrp17",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp18: row
                .get_opt_custom_parsed_at_idx(
                    "rrp18",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp19: row
                .get_opt_custom_parsed_at_idx(
                    "rrp19",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp20: row
                .get_opt_custom_parsed_at_idx(
                    "rrp20",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp21: row
                .get_opt_custom_parsed_at_idx(
                    "rrp21",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp22: row
                .get_opt_custom_parsed_at_idx(
                    "rrp22",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp23: row
                .get_opt_custom_parsed_at_idx(
                    "rrp23",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp24: row
                .get_opt_custom_parsed_at_idx(
                    "rrp24",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp25: row
                .get_opt_custom_parsed_at_idx(
                    "rrp25",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp26: row
                .get_opt_custom_parsed_at_idx(
                    "rrp26",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp27: row
                .get_opt_custom_parsed_at_idx(
                    "rrp27",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp28: row
                .get_opt_custom_parsed_at_idx(
                    "rrp28",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp29: row
                .get_opt_custom_parsed_at_idx(
                    "rrp29",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp30: row
                .get_opt_custom_parsed_at_idx(
                    "rrp30",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp31: row
                .get_opt_custom_parsed_at_idx(
                    "rrp31",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp32: row
                .get_opt_custom_parsed_at_idx(
                    "rrp32",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp33: row
                .get_opt_custom_parsed_at_idx(
                    "rrp33",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp34: row
                .get_opt_custom_parsed_at_idx(
                    "rrp34",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp35: row
                .get_opt_custom_parsed_at_idx(
                    "rrp35",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp36: row
                .get_opt_custom_parsed_at_idx(
                    "rrp36",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp37: row
                .get_opt_custom_parsed_at_idx(
                    "rrp37",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp38: row
                .get_opt_custom_parsed_at_idx(
                    "rrp38",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp39: row
                .get_opt_custom_parsed_at_idx(
                    "rrp39",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp40: row
                .get_opt_custom_parsed_at_idx(
                    "rrp40",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp41: row
                .get_opt_custom_parsed_at_idx(
                    "rrp41",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp42: row
                .get_opt_custom_parsed_at_idx(
                    "rrp42",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp43: row
                .get_opt_custom_parsed_at_idx(
                    "rrp43",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[48],
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
        Ok(P5minPricesensitivities1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                6,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minPricesensitivities1PrimaryKey {
        P5minPricesensitivities1PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_pricesensitivities_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minPricesensitivities1Row {
            run_datetime: row.run_datetime.clone(),
            regionid: row.regionid.clone(),
            interval_datetime: row.interval_datetime.clone(),
            intervention: row.intervention.clone(),
            intervention_active: row.intervention_active.clone(),
            rrp1: row.rrp1.clone(),
            rrp2: row.rrp2.clone(),
            rrp3: row.rrp3.clone(),
            rrp4: row.rrp4.clone(),
            rrp5: row.rrp5.clone(),
            rrp6: row.rrp6.clone(),
            rrp7: row.rrp7.clone(),
            rrp8: row.rrp8.clone(),
            rrp9: row.rrp9.clone(),
            rrp10: row.rrp10.clone(),
            rrp11: row.rrp11.clone(),
            rrp12: row.rrp12.clone(),
            rrp13: row.rrp13.clone(),
            rrp14: row.rrp14.clone(),
            rrp15: row.rrp15.clone(),
            rrp16: row.rrp16.clone(),
            rrp17: row.rrp17.clone(),
            rrp18: row.rrp18.clone(),
            rrp19: row.rrp19.clone(),
            rrp20: row.rrp20.clone(),
            rrp21: row.rrp21.clone(),
            rrp22: row.rrp22.clone(),
            rrp23: row.rrp23.clone(),
            rrp24: row.rrp24.clone(),
            rrp25: row.rrp25.clone(),
            rrp26: row.rrp26.clone(),
            rrp27: row.rrp27.clone(),
            rrp28: row.rrp28.clone(),
            rrp29: row.rrp29.clone(),
            rrp30: row.rrp30.clone(),
            rrp31: row.rrp31.clone(),
            rrp32: row.rrp32.clone(),
            rrp33: row.rrp33.clone(),
            rrp34: row.rrp34.clone(),
            rrp35: row.rrp35.clone(),
            rrp36: row.rrp36.clone(),
            rrp37: row.rrp37.clone(),
            rrp38: row.rrp38.clone(),
            rrp39: row.rrp39.clone(),
            rrp40: row.rrp40.clone(),
            rrp41: row.rrp41.clone(),
            rrp42: row.rrp42.clone(),
            rrp43: row.rrp43.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minPricesensitivities1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minPricesensitivities1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minPricesensitivities1Row<'data> {
    type Row<'other> = P5minPricesensitivities1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minPricesensitivities1Row<'data> {
    type PrimaryKey = P5minPricesensitivities1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minPricesensitivities1PrimaryKey {
    type Row<'other> = P5minPricesensitivities1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minPricesensitivities1PrimaryKey {
    type PrimaryKey = P5minPricesensitivities1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minPricesensitivities1 {
    type Builder = P5minPricesensitivities1Builder;
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "intervention_active",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp1",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp2",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp3",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp4",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp5",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp6",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp7",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp8",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp9",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp10",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp11",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp12",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp13",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp14",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp15",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp16",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp17",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp18",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp19",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp20",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp21",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp22",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp23",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp24",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp25",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp26",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp27",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp28",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp29",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp30",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp31",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp32",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp33",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp34",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp35",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp36",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp37",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp38",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp39",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp40",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp41",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp42",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp43",
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
        P5minPricesensitivities1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            intervention_active_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            rrp1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp11_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp12_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp13_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp14_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp15_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp16_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp17_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp18_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp19_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp20_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp21_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp22_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp23_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp24_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp25_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp26_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp27_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp28_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp29_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp30_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp31_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp32_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp33_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp34_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp35_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp36_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp37_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp38_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp39_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp40_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp41_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp42_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp43_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
        builder
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
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
            .rrp1_array
            .append_option({
                row.rrp1
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
            .rrp3_array
            .append_option({
                row.rrp3
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
            .rrp5_array
            .append_option({
                row.rrp5
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
            .rrp7_array
            .append_option({
                row.rrp7
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
            .rrp9_array
            .append_option({
                row.rrp9
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp10_array
            .append_option({
                row.rrp10
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp11_array
            .append_option({
                row.rrp11
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp12_array
            .append_option({
                row.rrp12
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp13_array
            .append_option({
                row.rrp13
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp14_array
            .append_option({
                row.rrp14
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp15_array
            .append_option({
                row.rrp15
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp16_array
            .append_option({
                row.rrp16
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp17_array
            .append_option({
                row.rrp17
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp18_array
            .append_option({
                row.rrp18
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp19_array
            .append_option({
                row.rrp19
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp20_array
            .append_option({
                row.rrp20
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp21_array
            .append_option({
                row.rrp21
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp22_array
            .append_option({
                row.rrp22
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp23_array
            .append_option({
                row.rrp23
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp24_array
            .append_option({
                row.rrp24
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp25_array
            .append_option({
                row.rrp25
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp26_array
            .append_option({
                row.rrp26
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp27_array
            .append_option({
                row.rrp27
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp28_array
            .append_option({
                row.rrp28
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp29_array
            .append_option({
                row.rrp29
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp30_array
            .append_option({
                row.rrp30
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp31_array
            .append_option({
                row.rrp31
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp32_array
            .append_option({
                row.rrp32
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp33_array
            .append_option({
                row.rrp33
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp34_array
            .append_option({
                row.rrp34
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp35_array
            .append_option({
                row.rrp35
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp36_array
            .append_option({
                row.rrp36
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp37_array
            .append_option({
                row.rrp37
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp38_array
            .append_option({
                row.rrp38
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp39_array
            .append_option({
                row.rrp39
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp40_array
            .append_option({
                row.rrp40
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp41_array
            .append_option({
                row.rrp41
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp42_array
            .append_option({
                row.rrp42
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp43_array
            .append_option({
                row.rrp43
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_active_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp11_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp12_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp13_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp14_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp15_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp16_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp17_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp18_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp19_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp20_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp21_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp22_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp23_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp24_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp25_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp26_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp27_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp28_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp29_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp30_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp31_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp32_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp33_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp34_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp35_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp36_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp37_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp38_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp39_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp40_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp41_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp42_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp43_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct P5minPricesensitivities1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    intervention_active_array: arrow::array::builder::Decimal128Builder,
    rrp1_array: arrow::array::builder::Decimal128Builder,
    rrp2_array: arrow::array::builder::Decimal128Builder,
    rrp3_array: arrow::array::builder::Decimal128Builder,
    rrp4_array: arrow::array::builder::Decimal128Builder,
    rrp5_array: arrow::array::builder::Decimal128Builder,
    rrp6_array: arrow::array::builder::Decimal128Builder,
    rrp7_array: arrow::array::builder::Decimal128Builder,
    rrp8_array: arrow::array::builder::Decimal128Builder,
    rrp9_array: arrow::array::builder::Decimal128Builder,
    rrp10_array: arrow::array::builder::Decimal128Builder,
    rrp11_array: arrow::array::builder::Decimal128Builder,
    rrp12_array: arrow::array::builder::Decimal128Builder,
    rrp13_array: arrow::array::builder::Decimal128Builder,
    rrp14_array: arrow::array::builder::Decimal128Builder,
    rrp15_array: arrow::array::builder::Decimal128Builder,
    rrp16_array: arrow::array::builder::Decimal128Builder,
    rrp17_array: arrow::array::builder::Decimal128Builder,
    rrp18_array: arrow::array::builder::Decimal128Builder,
    rrp19_array: arrow::array::builder::Decimal128Builder,
    rrp20_array: arrow::array::builder::Decimal128Builder,
    rrp21_array: arrow::array::builder::Decimal128Builder,
    rrp22_array: arrow::array::builder::Decimal128Builder,
    rrp23_array: arrow::array::builder::Decimal128Builder,
    rrp24_array: arrow::array::builder::Decimal128Builder,
    rrp25_array: arrow::array::builder::Decimal128Builder,
    rrp26_array: arrow::array::builder::Decimal128Builder,
    rrp27_array: arrow::array::builder::Decimal128Builder,
    rrp28_array: arrow::array::builder::Decimal128Builder,
    rrp29_array: arrow::array::builder::Decimal128Builder,
    rrp30_array: arrow::array::builder::Decimal128Builder,
    rrp31_array: arrow::array::builder::Decimal128Builder,
    rrp32_array: arrow::array::builder::Decimal128Builder,
    rrp33_array: arrow::array::builder::Decimal128Builder,
    rrp34_array: arrow::array::builder::Decimal128Builder,
    rrp35_array: arrow::array::builder::Decimal128Builder,
    rrp36_array: arrow::array::builder::Decimal128Builder,
    rrp37_array: arrow::array::builder::Decimal128Builder,
    rrp38_array: arrow::array::builder::Decimal128Builder,
    rrp39_array: arrow::array::builder::Decimal128Builder,
    rrp40_array: arrow::array::builder::Decimal128Builder,
    rrp41_array: arrow::array::builder::Decimal128Builder,
    rrp42_array: arrow::array::builder::Decimal128Builder,
    rrp43_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct P5minRegionsolution9;
pub struct P5minRegionsolution9Mapping([usize; 117]);
/// # Summary
///
/// ## P5MIN_REGIONSOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_REGIONSOLUTION shows the results of the regional capacity, maximum surplus reserve and maximum spare capacity evaluations for each period of the study._
///
/// * Data Set Name: P5min
/// * File Name: Regionsolution
/// * Data Version: 9
///
/// # Description
///  P5MIN_REGIONSOLUTION is public data, so is available to all participants. Source P5MIN_REGIONSOLUTION updates every 5 minutes. Volume Rows per day: 1440
///
///
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minRegionsolution9Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Region Reference Price (Energy)
    pub rrp: Option<rust_decimal::Decimal>,
    /// Region Override Price (Energy)
    pub rop: Option<rust_decimal::Decimal>,
    /// Total Energy Imbalance (MW)
    pub excessgeneration: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise6Sec)
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise6Sec)
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise60Sec)
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise60Sec)
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise5Min)
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise5Min)
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (RaiseReg)
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (RaiseReg)
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower6Sec)
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower6Sec)
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower60Sec)
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower60Sec)
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower5Min)
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower5Min)
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (LowerReg)
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (LowerReg)
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// Regional Demand - NB NOT net of Interconnector flows or Loads
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Regional Available generation
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Available Load
    pub availableload: Option<rust_decimal::Decimal>,
    /// Predicted change in regional demand for this interval
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Regional Generation Dispatched
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Load Dispatched
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector Flows
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    pub lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    pub lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    pub lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    pub lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    pub lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    pub lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    pub lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    pub lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    pub lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    pub lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    pub lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    pub lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    pub lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    pub lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    pub lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Raise 5 min MW dispatch
    pub raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    pub raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    pub raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    pub raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    pub raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 50 sec local dispatch
    pub raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    pub raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec total requirement
    pub raise60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW dispatch
    pub raise6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW imported
    pub raise6secimport: Option<rust_decimal::Decimal>,
    /// Raise 6 sec local dispatch
    pub raise6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec local requirement
    pub raise6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec total requirement
    pub raise6secreq: Option<rust_decimal::Decimal>,
    /// Aggregate dispatch error applied
    pub aggregatedispatcherror: Option<rust_decimal::Decimal>,
    /// Sum of initial generation and import for region
    pub initialsupply: Option<rust_decimal::Decimal>,
    /// Sum of cleared generation and import for region
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation MW imported
    pub lowerregimport: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Lower Regulation dispatch
    pub lowerregdispatch: Option<rust_decimal::Decimal>,
    /// Lower Regulation local dispatch
    pub lowerreglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation local requirement
    pub lowerreglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation total requirement
    pub lowerregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation MW imported
    pub raiseregimport: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Raise Regulation dispatch
    pub raiseregdispatch: Option<rust_decimal::Decimal>,
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
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
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
    /// Regional Raise 1Sec Price - R1Price attribute after capping/flooring
    pub raise1secrrp: Option<rust_decimal::Decimal>,
    /// Raise1Sec Regional Original Price - uncapped/unfloored and unscaled
    pub raise1secrop: Option<rust_decimal::Decimal>,
    /// Regional Lower 1Sec Price - RegionSolution element L1Price attribute
    pub lower1secrrp: Option<rust_decimal::Decimal>,
    /// Lower1Sec Regional Original Price - uncapped/unfloored and unscaled
    pub lower1secrop: Option<rust_decimal::Decimal>,
    /// Total Raise1Sec Dispatched in Region - RegionSolution element R1Dispatch attribute
    pub raise1seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Total Lower1Sec Dispatched in Region - RegionSolution element L1Dispatch attribute
    pub lower1seclocaldispatch: Option<rust_decimal::Decimal>,
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
impl<'data> P5minRegionsolution9Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for P5minRegionsolution9 {
    const VERSION: i32 = 9;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "REGIONSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minRegionsolution9Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "REGIONID",
        "RRP",
        "ROP",
        "EXCESSGENERATION",
        "RAISE6SECRRP",
        "RAISE6SECROP",
        "RAISE60SECRRP",
        "RAISE60SECROP",
        "RAISE5MINRRP",
        "RAISE5MINROP",
        "RAISEREGRRP",
        "RAISEREGROP",
        "LOWER6SECRRP",
        "LOWER6SECROP",
        "LOWER60SECRRP",
        "LOWER60SECROP",
        "LOWER5MINRRP",
        "LOWER5MINROP",
        "LOWERREGRRP",
        "LOWERREGROP",
        "TOTALDEMAND",
        "AVAILABLEGENERATION",
        "AVAILABLELOAD",
        "DEMANDFORECAST",
        "DISPATCHABLEGENERATION",
        "DISPATCHABLELOAD",
        "NETINTERCHANGE",
        "LOWER5MINDISPATCH",
        "LOWER5MINIMPORT",
        "LOWER5MINLOCALDISPATCH",
        "LOWER5MINLOCALREQ",
        "LOWER5MINREQ",
        "LOWER60SECDISPATCH",
        "LOWER60SECIMPORT",
        "LOWER60SECLOCALDISPATCH",
        "LOWER60SECLOCALREQ",
        "LOWER60SECREQ",
        "LOWER6SECDISPATCH",
        "LOWER6SECIMPORT",
        "LOWER6SECLOCALDISPATCH",
        "LOWER6SECLOCALREQ",
        "LOWER6SECREQ",
        "RAISE5MINDISPATCH",
        "RAISE5MINIMPORT",
        "RAISE5MINLOCALDISPATCH",
        "RAISE5MINLOCALREQ",
        "RAISE5MINREQ",
        "RAISE60SECDISPATCH",
        "RAISE60SECIMPORT",
        "RAISE60SECLOCALDISPATCH",
        "RAISE60SECLOCALREQ",
        "RAISE60SECREQ",
        "RAISE6SECDISPATCH",
        "RAISE6SECIMPORT",
        "RAISE6SECLOCALDISPATCH",
        "RAISE6SECLOCALREQ",
        "RAISE6SECREQ",
        "AGGREGATEDISPATCHERROR",
        "INITIALSUPPLY",
        "CLEAREDSUPPLY",
        "LOWERREGIMPORT",
        "LOWERREGDISPATCH",
        "LOWERREGLOCALDISPATCH",
        "LOWERREGLOCALREQ",
        "LOWERREGREQ",
        "RAISEREGIMPORT",
        "RAISEREGDISPATCH",
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
        "LASTCHANGED",
        "TOTALINTERMITTENTGENERATION",
        "DEMAND_AND_NONSCHEDGEN",
        "UIGF",
        "SEMISCHEDULE_CLEAREDMW",
        "SEMISCHEDULE_COMPLIANCEMW",
        "INTERVENTION",
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
        "RAISE1SECRRP",
        "RAISE1SECROP",
        "LOWER1SECRRP",
        "LOWER1SECROP",
        "RAISE1SECLOCALDISPATCH",
        "LOWER1SECLOCALDISPATCH",
        "BDU_ENERGY_STORAGE",
        "BDU_MIN_AVAIL",
        "BDU_MAX_AVAIL",
        "BDU_CLEAREDMW_GEN",
        "BDU_CLEAREDMW_LOAD",
    ];
    type Row<'row> = P5minRegionsolution9Row<'row>;
    type FieldMapping = P5minRegionsolution9Mapping;
    type PrimaryKey = P5minRegionsolution9PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minRegionsolution9Row {
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
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rop: row
                .get_opt_custom_parsed_at_idx(
                    "rop",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            excessgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "excessgeneration",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secrrp",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secrop",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secrrp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secrop",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minrrp",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minrop",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregrrp",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregrop: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregrop",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secrrp",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secrop",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secrrp",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secrop",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minrrp",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minrop",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregrrp",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregrop: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregrop",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totaldemand: row
                .get_opt_custom_parsed_at_idx(
                    "totaldemand",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availablegeneration: row
                .get_opt_custom_parsed_at_idx(
                    "availablegeneration",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availableload: row
                .get_opt_custom_parsed_at_idx(
                    "availableload",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demandforecast: row
                .get_opt_custom_parsed_at_idx(
                    "demandforecast",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchablegeneration: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchablegeneration",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchableload: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchableload",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netinterchange: row
                .get_opt_custom_parsed_at_idx(
                    "netinterchange",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5mindispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower5mindispatch",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minimport",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocaldispatch",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalreq",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minreq",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secdispatch",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secimport",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocaldispatch",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalreq",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secreq",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secdispatch",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secimport",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocaldispatch",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalreq",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secreq",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5mindispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise5mindispatch",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minimport",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocaldispatch",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalreq",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minreq",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secdispatch",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secimport",
                    field_mapping.0[50],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocaldispatch",
                    field_mapping.0[51],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalreq",
                    field_mapping.0[52],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secreq",
                    field_mapping.0[53],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secdispatch",
                    field_mapping.0[54],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secimport",
                    field_mapping.0[55],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocaldispatch",
                    field_mapping.0[56],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalreq",
                    field_mapping.0[57],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secreq",
                    field_mapping.0[58],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregatedispatcherror: row
                .get_opt_custom_parsed_at_idx(
                    "aggregatedispatcherror",
                    field_mapping.0[59],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initialsupply: row
                .get_opt_custom_parsed_at_idx(
                    "initialsupply",
                    field_mapping.0[60],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedsupply: row
                .get_opt_custom_parsed_at_idx(
                    "clearedsupply",
                    field_mapping.0[61],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregimport: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregimport",
                    field_mapping.0[62],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregdispatch",
                    field_mapping.0[63],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocaldispatch",
                    field_mapping.0[64],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocalreq",
                    field_mapping.0[65],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregreq: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregreq",
                    field_mapping.0[66],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregimport: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregimport",
                    field_mapping.0[67],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregdispatch",
                    field_mapping.0[68],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocaldispatch",
                    field_mapping.0[69],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalreq",
                    field_mapping.0[70],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregreq: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregreq",
                    field_mapping.0[71],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalviolation",
                    field_mapping.0[72],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalviolation",
                    field_mapping.0[73],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalviolation",
                    field_mapping.0[74],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalviolation",
                    field_mapping.0[75],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalviolation",
                    field_mapping.0[76],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocalviolation",
                    field_mapping.0[77],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalviolation",
                    field_mapping.0[78],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalviolation",
                    field_mapping.0[79],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minviolation",
                    field_mapping.0[80],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregviolation",
                    field_mapping.0[81],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secviolation",
                    field_mapping.0[82],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secviolation",
                    field_mapping.0[83],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minviolation",
                    field_mapping.0[84],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregviolation",
                    field_mapping.0[85],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secviolation",
                    field_mapping.0[86],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secviolation",
                    field_mapping.0[87],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[88],
                    mmsdm_core::mms_datetime::parse,
                )?,
            totalintermittentgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "totalintermittentgeneration",
                    field_mapping.0[89],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand_and_nonschedgen: row
                .get_opt_custom_parsed_at_idx(
                    "demand_and_nonschedgen",
                    field_mapping.0[90],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uigf: row
                .get_opt_custom_parsed_at_idx(
                    "uigf",
                    field_mapping.0[91],
                    mmsdm_core::mms_decimal::parse,
                )?,
            semischedule_clearedmw: row
                .get_opt_custom_parsed_at_idx(
                    "semischedule_clearedmw",
                    field_mapping.0[92],
                    mmsdm_core::mms_decimal::parse,
                )?,
            semischedule_compliancemw: row
                .get_opt_custom_parsed_at_idx(
                    "semischedule_compliancemw",
                    field_mapping.0[93],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[94],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_uigf",
                    field_mapping.0[95],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_uigf",
                    field_mapping.0[96],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_clearedmw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_clearedmw",
                    field_mapping.0[97],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_clearedmw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_clearedmw",
                    field_mapping.0[98],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_compliancemw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_compliancemw",
                    field_mapping.0[99],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_compliancemw: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_compliancemw",
                    field_mapping.0[100],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_initialmw: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_initialmw",
                    field_mapping.0[101],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_available: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_available",
                    field_mapping.0[102],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_dispatched: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_dispatched",
                    field_mapping.0[103],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_availability: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_availability",
                    field_mapping.0[104],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_availability: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_availability",
                    field_mapping.0[105],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secrrp",
                    field_mapping.0[106],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secrop: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secrop",
                    field_mapping.0[107],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secrrp: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secrrp",
                    field_mapping.0[108],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secrop: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secrop",
                    field_mapping.0[109],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise1seclocaldispatch",
                    field_mapping.0[110],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower1seclocaldispatch",
                    field_mapping.0[111],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_energy_storage",
                    field_mapping.0[112],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_min_avail: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_min_avail",
                    field_mapping.0[113],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_max_avail: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_max_avail",
                    field_mapping.0[114],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_clearedmw_gen: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_clearedmw_gen",
                    field_mapping.0[115],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_clearedmw_load: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_clearedmw_load",
                    field_mapping.0[116],
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
        Ok(P5minRegionsolution9Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                5,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minRegionsolution9PrimaryKey {
        P5minRegionsolution9PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_regionsolution_v9_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minRegionsolution9Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            regionid: row.regionid.clone(),
            rrp: row.rrp.clone(),
            rop: row.rop.clone(),
            excessgeneration: row.excessgeneration.clone(),
            raise6secrrp: row.raise6secrrp.clone(),
            raise6secrop: row.raise6secrop.clone(),
            raise60secrrp: row.raise60secrrp.clone(),
            raise60secrop: row.raise60secrop.clone(),
            raise5minrrp: row.raise5minrrp.clone(),
            raise5minrop: row.raise5minrop.clone(),
            raiseregrrp: row.raiseregrrp.clone(),
            raiseregrop: row.raiseregrop.clone(),
            lower6secrrp: row.lower6secrrp.clone(),
            lower6secrop: row.lower6secrop.clone(),
            lower60secrrp: row.lower60secrrp.clone(),
            lower60secrop: row.lower60secrop.clone(),
            lower5minrrp: row.lower5minrrp.clone(),
            lower5minrop: row.lower5minrop.clone(),
            lowerregrrp: row.lowerregrrp.clone(),
            lowerregrop: row.lowerregrop.clone(),
            totaldemand: row.totaldemand.clone(),
            availablegeneration: row.availablegeneration.clone(),
            availableload: row.availableload.clone(),
            demandforecast: row.demandforecast.clone(),
            dispatchablegeneration: row.dispatchablegeneration.clone(),
            dispatchableload: row.dispatchableload.clone(),
            netinterchange: row.netinterchange.clone(),
            lower5mindispatch: row.lower5mindispatch.clone(),
            lower5minimport: row.lower5minimport.clone(),
            lower5minlocaldispatch: row.lower5minlocaldispatch.clone(),
            lower5minlocalreq: row.lower5minlocalreq.clone(),
            lower5minreq: row.lower5minreq.clone(),
            lower60secdispatch: row.lower60secdispatch.clone(),
            lower60secimport: row.lower60secimport.clone(),
            lower60seclocaldispatch: row.lower60seclocaldispatch.clone(),
            lower60seclocalreq: row.lower60seclocalreq.clone(),
            lower60secreq: row.lower60secreq.clone(),
            lower6secdispatch: row.lower6secdispatch.clone(),
            lower6secimport: row.lower6secimport.clone(),
            lower6seclocaldispatch: row.lower6seclocaldispatch.clone(),
            lower6seclocalreq: row.lower6seclocalreq.clone(),
            lower6secreq: row.lower6secreq.clone(),
            raise5mindispatch: row.raise5mindispatch.clone(),
            raise5minimport: row.raise5minimport.clone(),
            raise5minlocaldispatch: row.raise5minlocaldispatch.clone(),
            raise5minlocalreq: row.raise5minlocalreq.clone(),
            raise5minreq: row.raise5minreq.clone(),
            raise60secdispatch: row.raise60secdispatch.clone(),
            raise60secimport: row.raise60secimport.clone(),
            raise60seclocaldispatch: row.raise60seclocaldispatch.clone(),
            raise60seclocalreq: row.raise60seclocalreq.clone(),
            raise60secreq: row.raise60secreq.clone(),
            raise6secdispatch: row.raise6secdispatch.clone(),
            raise6secimport: row.raise6secimport.clone(),
            raise6seclocaldispatch: row.raise6seclocaldispatch.clone(),
            raise6seclocalreq: row.raise6seclocalreq.clone(),
            raise6secreq: row.raise6secreq.clone(),
            aggregatedispatcherror: row.aggregatedispatcherror.clone(),
            initialsupply: row.initialsupply.clone(),
            clearedsupply: row.clearedsupply.clone(),
            lowerregimport: row.lowerregimport.clone(),
            lowerregdispatch: row.lowerregdispatch.clone(),
            lowerreglocaldispatch: row.lowerreglocaldispatch.clone(),
            lowerreglocalreq: row.lowerreglocalreq.clone(),
            lowerregreq: row.lowerregreq.clone(),
            raiseregimport: row.raiseregimport.clone(),
            raiseregdispatch: row.raiseregdispatch.clone(),
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
            lastchanged: row.lastchanged.clone(),
            totalintermittentgeneration: row.totalintermittentgeneration.clone(),
            demand_and_nonschedgen: row.demand_and_nonschedgen.clone(),
            uigf: row.uigf.clone(),
            semischedule_clearedmw: row.semischedule_clearedmw.clone(),
            semischedule_compliancemw: row.semischedule_compliancemw.clone(),
            intervention: row.intervention.clone(),
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
            raise1secrrp: row.raise1secrrp.clone(),
            raise1secrop: row.raise1secrop.clone(),
            lower1secrrp: row.lower1secrrp.clone(),
            lower1secrop: row.lower1secrop.clone(),
            raise1seclocaldispatch: row.raise1seclocaldispatch.clone(),
            lower1seclocaldispatch: row.lower1seclocaldispatch.clone(),
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
pub struct P5minRegionsolution9PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minRegionsolution9PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minRegionsolution9Row<'data> {
    type Row<'other> = P5minRegionsolution9Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minRegionsolution9Row<'data> {
    type PrimaryKey = P5minRegionsolution9PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minRegionsolution9PrimaryKey {
    type Row<'other> = P5minRegionsolution9Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minRegionsolution9PrimaryKey {
    type PrimaryKey = P5minRegionsolution9PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minRegionsolution9 {
    type Builder = P5minRegionsolution9Builder;
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
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rop",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "excessgeneration",
                    arrow::datatypes::DataType::Decimal128(15, 5),
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
                    "lower5minlocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minreq",
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
                    "lower60seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secreq",
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
                    "lower6seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secreq",
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
                    "raise5minlocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minreq",
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
                    "raise60seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secreq",
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
                    "raise6seclocalreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secreq",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregatedispatcherror",
                    arrow::datatypes::DataType::Decimal128(15, 5),
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
                    "lowerregdispatch",
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
                    "raiseregdispatch",
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
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
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
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
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
        P5minRegionsolution9Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            excessgeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
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
            lower5mindispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minlocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minlocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5minreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5mindispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minlocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minlocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5minreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6seclocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6secreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            aggregatedispatcherror_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            initialsupply_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            clearedsupply_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregdispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerreglocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerreglocalreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerregreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregimport_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregdispatch_array: arrow::array::builder::Decimal128Builder::new()
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
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
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
            raise1secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1secrrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1secrop_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise1seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower1seclocaldispatch_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
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
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
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
            .rop_array
            .append_option({
                row.rop
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
            .lower5minlocalreq_array
            .append_option({
                row.lower5minlocalreq
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
            .lower60seclocalreq_array
            .append_option({
                row.lower60seclocalreq
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
            .lower6seclocalreq_array
            .append_option({
                row.lower6seclocalreq
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
            .raise5minlocalreq_array
            .append_option({
                row.raise5minlocalreq
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
            .raise60seclocalreq_array
            .append_option({
                row.raise60seclocalreq
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
            .raise6seclocalreq_array
            .append_option({
                row.raise6seclocalreq
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
            .aggregatedispatcherror_array
            .append_option({
                row.aggregatedispatcherror
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
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
            .lowerregdispatch_array
            .append_option({
                row.lowerregdispatch
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
            .raiseregdispatch_array
            .append_option({
                row.raiseregdispatch
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
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.excessgeneration_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregrop_array.finish())
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
                    alloc::sync::Arc::new(builder.lower5mindispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minlocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minlocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5mindispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minlocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minlocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6seclocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregatedispatcherror_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initialsupply_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedsupply_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregdispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreglocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreglocalreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregimport_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregdispatch_array.finish())
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
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
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
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
                    alloc::sync::Arc::new(builder.raise1secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secrrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secrop_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1seclocaldispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
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
pub struct P5minRegionsolution9Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    rop_array: arrow::array::builder::Decimal128Builder,
    excessgeneration_array: arrow::array::builder::Decimal128Builder,
    raise6secrrp_array: arrow::array::builder::Decimal128Builder,
    raise6secrop_array: arrow::array::builder::Decimal128Builder,
    raise60secrrp_array: arrow::array::builder::Decimal128Builder,
    raise60secrop_array: arrow::array::builder::Decimal128Builder,
    raise5minrrp_array: arrow::array::builder::Decimal128Builder,
    raise5minrop_array: arrow::array::builder::Decimal128Builder,
    raiseregrrp_array: arrow::array::builder::Decimal128Builder,
    raiseregrop_array: arrow::array::builder::Decimal128Builder,
    lower6secrrp_array: arrow::array::builder::Decimal128Builder,
    lower6secrop_array: arrow::array::builder::Decimal128Builder,
    lower60secrrp_array: arrow::array::builder::Decimal128Builder,
    lower60secrop_array: arrow::array::builder::Decimal128Builder,
    lower5minrrp_array: arrow::array::builder::Decimal128Builder,
    lower5minrop_array: arrow::array::builder::Decimal128Builder,
    lowerregrrp_array: arrow::array::builder::Decimal128Builder,
    lowerregrop_array: arrow::array::builder::Decimal128Builder,
    totaldemand_array: arrow::array::builder::Decimal128Builder,
    availablegeneration_array: arrow::array::builder::Decimal128Builder,
    availableload_array: arrow::array::builder::Decimal128Builder,
    demandforecast_array: arrow::array::builder::Decimal128Builder,
    dispatchablegeneration_array: arrow::array::builder::Decimal128Builder,
    dispatchableload_array: arrow::array::builder::Decimal128Builder,
    netinterchange_array: arrow::array::builder::Decimal128Builder,
    lower5mindispatch_array: arrow::array::builder::Decimal128Builder,
    lower5minimport_array: arrow::array::builder::Decimal128Builder,
    lower5minlocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower5minlocalreq_array: arrow::array::builder::Decimal128Builder,
    lower5minreq_array: arrow::array::builder::Decimal128Builder,
    lower60secdispatch_array: arrow::array::builder::Decimal128Builder,
    lower60secimport_array: arrow::array::builder::Decimal128Builder,
    lower60seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower60seclocalreq_array: arrow::array::builder::Decimal128Builder,
    lower60secreq_array: arrow::array::builder::Decimal128Builder,
    lower6secdispatch_array: arrow::array::builder::Decimal128Builder,
    lower6secimport_array: arrow::array::builder::Decimal128Builder,
    lower6seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower6seclocalreq_array: arrow::array::builder::Decimal128Builder,
    lower6secreq_array: arrow::array::builder::Decimal128Builder,
    raise5mindispatch_array: arrow::array::builder::Decimal128Builder,
    raise5minimport_array: arrow::array::builder::Decimal128Builder,
    raise5minlocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raise5minlocalreq_array: arrow::array::builder::Decimal128Builder,
    raise5minreq_array: arrow::array::builder::Decimal128Builder,
    raise60secdispatch_array: arrow::array::builder::Decimal128Builder,
    raise60secimport_array: arrow::array::builder::Decimal128Builder,
    raise60seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raise60seclocalreq_array: arrow::array::builder::Decimal128Builder,
    raise60secreq_array: arrow::array::builder::Decimal128Builder,
    raise6secdispatch_array: arrow::array::builder::Decimal128Builder,
    raise6secimport_array: arrow::array::builder::Decimal128Builder,
    raise6seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    raise6seclocalreq_array: arrow::array::builder::Decimal128Builder,
    raise6secreq_array: arrow::array::builder::Decimal128Builder,
    aggregatedispatcherror_array: arrow::array::builder::Decimal128Builder,
    initialsupply_array: arrow::array::builder::Decimal128Builder,
    clearedsupply_array: arrow::array::builder::Decimal128Builder,
    lowerregimport_array: arrow::array::builder::Decimal128Builder,
    lowerregdispatch_array: arrow::array::builder::Decimal128Builder,
    lowerreglocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lowerreglocalreq_array: arrow::array::builder::Decimal128Builder,
    lowerregreq_array: arrow::array::builder::Decimal128Builder,
    raiseregimport_array: arrow::array::builder::Decimal128Builder,
    raiseregdispatch_array: arrow::array::builder::Decimal128Builder,
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
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    totalintermittentgeneration_array: arrow::array::builder::Decimal128Builder,
    demand_and_nonschedgen_array: arrow::array::builder::Decimal128Builder,
    uigf_array: arrow::array::builder::Decimal128Builder,
    semischedule_clearedmw_array: arrow::array::builder::Decimal128Builder,
    semischedule_compliancemw_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
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
    raise1secrrp_array: arrow::array::builder::Decimal128Builder,
    raise1secrop_array: arrow::array::builder::Decimal128Builder,
    lower1secrrp_array: arrow::array::builder::Decimal128Builder,
    lower1secrop_array: arrow::array::builder::Decimal128Builder,
    raise1seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    lower1seclocaldispatch_array: arrow::array::builder::Decimal128Builder,
    bdu_energy_storage_array: arrow::array::builder::Decimal128Builder,
    bdu_min_avail_array: arrow::array::builder::Decimal128Builder,
    bdu_max_avail_array: arrow::array::builder::Decimal128Builder,
    bdu_clearedmw_gen_array: arrow::array::builder::Decimal128Builder,
    bdu_clearedmw_load_array: arrow::array::builder::Decimal128Builder,
}
pub struct P5minScenariodemand1;
pub struct P5minScenariodemand1Mapping([usize; 5]);
/// # Summary
///
/// ## P5MIN_SCENARIODEMAND
///  _The P5Min scenario MW offsets_
///
/// * Data Set Name: P5min
/// * File Name: Scenariodemand
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
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct P5minScenariodemand1Row<'data> {
    /// The effective date of this set of scenarios
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub version_datetime: chrono::NaiveDateTime,
    /// The scenario identifier
    pub scenario: rust_decimal::Decimal,
    /// The region to which to apply the deltaMW for this SCENARIO
    pub regionid: core::ops::Range<usize>,
    /// The MW offset to apply to region total demand for this SCENARIO
    pub deltamw: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minScenariodemand1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for P5minScenariodemand1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "SCENARIODEMAND";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minScenariodemand1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "SCENARIO",
        "REGIONID",
        "DELTAMW",
    ];
    type Row<'row> = P5minScenariodemand1Row<'row>;
    type FieldMapping = P5minScenariodemand1Mapping;
    type PrimaryKey = P5minScenariodemand1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minScenariodemand1Row {
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
            scenario: row
                .get_custom_parsed_at_idx(
                    "scenario",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            deltamw: row
                .get_opt_custom_parsed_at_idx(
                    "deltamw",
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
        Ok(P5minScenariodemand1Mapping(base_mapping))
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
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
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
    fn primary_key(row: &Self::Row<'_>) -> P5minScenariodemand1PrimaryKey {
        P5minScenariodemand1PrimaryKey {
            effectivedate: row.effectivedate,
            regionid: row.regionid().to_string(),
            scenario: row.scenario,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.effectivedate)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
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
                                5,
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
            "p5min_scenariodemand_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minScenariodemand1Row {
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            scenario: row.scenario.clone(),
            regionid: row.regionid.clone(),
            deltamw: row.deltamw.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minScenariodemand1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub scenario: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for P5minScenariodemand1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minScenariodemand1Row<'data> {
    type Row<'other> = P5minScenariodemand1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid() == row.regionid()
            && self.scenario == row.scenario
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minScenariodemand1Row<'data> {
    type PrimaryKey = P5minScenariodemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid() == key.regionid
            && self.scenario == key.scenario
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minScenariodemand1PrimaryKey {
    type Row<'other> = P5minScenariodemand1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid()
            && self.scenario == row.scenario
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minScenariodemand1PrimaryKey {
    type PrimaryKey = P5minScenariodemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.scenario == key.scenario
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minScenariodemand1 {
    type Builder = P5minScenariodemand1Builder;
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
                    "scenario",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "deltamw",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        P5minScenariodemand1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            scenario_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            deltamw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.timestamp_millis());
        builder
            .scenario_array
            .append_value({
                let mut val = row.scenario;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .deltamw_array
            .append_option({
                row.deltamw
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
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
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
pub struct P5minScenariodemand1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    scenario_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    deltamw_array: arrow::array::builder::Decimal128Builder,
}
pub struct P5minScenariodemandtrk1;
pub struct P5minScenariodemandtrk1Mapping([usize; 4]);
/// # Summary
///
/// ## P5MIN_SCENARIODEMANDTRK
///  _Tracks the 5Min scenario offset updates across time_
///
/// * Data Set Name: P5min
/// * File Name: Scenariodemandtrk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct P5minScenariodemandtrk1Row<'data> {
    /// The effective date of this set of scenarios
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub version_datetime: chrono::NaiveDateTime,
    /// The datetime that the scenario update was authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// The datetime that the record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> P5minScenariodemandtrk1Row<'data> {}
impl mmsdm_core::GetTable for P5minScenariodemandtrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "SCENARIODEMANDTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minScenariodemandtrk1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = P5minScenariodemandtrk1Row<'row>;
    type FieldMapping = P5minScenariodemandtrk1Mapping;
    type PrimaryKey = P5minScenariodemandtrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minScenariodemandtrk1Row {
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
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
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
        Ok(P5minScenariodemandtrk1Mapping(base_mapping))
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
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
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
    fn primary_key(row: &Self::Row<'_>) -> P5minScenariodemandtrk1PrimaryKey {
        P5minScenariodemandtrk1PrimaryKey {
            effectivedate: row.effectivedate,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.effectivedate)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
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
                                5,
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
            "p5min_scenariodemandtrk_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minScenariodemandtrk1Row {
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minScenariodemandtrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for P5minScenariodemandtrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minScenariodemandtrk1Row<'data> {
    type Row<'other> = P5minScenariodemandtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minScenariodemandtrk1Row<'data> {
    type PrimaryKey = P5minScenariodemandtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minScenariodemandtrk1PrimaryKey {
    type Row<'other> = P5minScenariodemandtrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minScenariodemandtrk1PrimaryKey {
    type PrimaryKey = P5minScenariodemandtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minScenariodemandtrk1 {
    type Builder = P5minScenariodemandtrk1Builder;
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
        P5minScenariodemandtrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.timestamp_millis());
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
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
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
pub struct P5minScenariodemandtrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct P5minUnitsolution6;
pub struct P5minUnitsolution6Mapping([usize; 42]);
/// # Summary
///
/// ## P5MIN_UNITSOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_UNITSOLUTION shows the Unit results from the capacity evaluations for each period of the study._
///
/// * Data Set Name: P5min
/// * File Name: Unitsolution
/// * Data Version: 6
///
/// # Description
///  P5MIN_UNITSOLUTION data is confidential, so shows own details for participant. Source P5MIN_UNITSOLUTION updates every 5 minutes for all units, even zero targets. Volume Rows per day: 57600 Based on 200 units per Interval Note A bitwise flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The SPD Formulation document details the logic determining whether a unit is "trapped" or "stranded". The flag is defined as follows: Flagged Condition Bit Description Field value FCAS profile active 0 The bid profile for this service has been activated such that the unit is available to be cleared to provide this ancillary service type. 1 or 3 Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. 3 Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. 4
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct P5minUnitsolution6Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Connection point identifier for DUID
    pub connectionpointid: core::ops::Range<usize>,
    /// Generator or Load
    pub tradetype: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS: 1 = on, 0 = off
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run. Negative values when Bi-directional Unit start from importing power, otherwise positive.
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period. Negative values when Bi-directional Unit is importing power, otherwise positive.
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate (lessor of bid or telemetered rate).
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lessor of bid or telemetered rate).
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
    /// Raise Reg status flag
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag
    pub lowerregflags: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: rust_decimal::Decimal,
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
    /// BDU only. The energy storage at the start of this dispatch interval (MWh)
    pub initial_energy_storage: Option<rust_decimal::Decimal>,
    /// BDU only. The projected energy storage based on cleared energy and regulation FCAS dispatch (MWh)
    pub energy_storage: Option<rust_decimal::Decimal>,
    /// BDU only - Minimum Energy Storage constraint limit (MWh)
    pub energy_storage_min: Option<rust_decimal::Decimal>,
    /// BDU only - Maximum Energy Storage constraint limit (MWh)
    pub energy_storage_max: Option<rust_decimal::Decimal>,
    /// BDU only. Load side availability (BidOfferPeriod.MAXAVAIL where DIRECTION = LOAD).
    pub min_availability: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> P5minUnitsolution6Row<'data> {
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
impl mmsdm_core::GetTable for P5minUnitsolution6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "P5MIN";
    const TABLE_NAME: &'static str = "UNITSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = P5minUnitsolution6Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "DUID",
        "CONNECTIONPOINTID",
        "TRADETYPE",
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
        "LASTCHANGED",
        "SEMIDISPATCHCAP",
        "INTERVENTION",
        "DISPATCHMODETIME",
        "CONFORMANCE_MODE",
        "UIGF",
        "RAISE1SEC",
        "RAISE1SECFLAGS",
        "LOWER1SEC",
        "LOWER1SECFLAGS",
        "INITIAL_ENERGY_STORAGE",
        "ENERGY_STORAGE",
        "ENERGY_STORAGE_MIN",
        "ENERGY_STORAGE_MAX",
        "MIN_AVAILABILITY",
    ];
    type Row<'row> = P5minUnitsolution6Row<'row>;
    type FieldMapping = P5minUnitsolution6Mapping;
    type PrimaryKey = P5minUnitsolution6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(P5minUnitsolution6Row {
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
            duid: row.get_range("duid", field_mapping.0[2])?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[3])?,
            tradetype: row
                .get_opt_custom_parsed_at_idx(
                    "tradetype",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            agcstatus: row
                .get_opt_custom_parsed_at_idx(
                    "agcstatus",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initialmw: row
                .get_opt_custom_parsed_at_idx(
                    "initialmw",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalcleared: row
                .get_opt_custom_parsed_at_idx(
                    "totalcleared",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampdownrate: row
                .get_opt_custom_parsed_at_idx(
                    "rampdownrate",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampuprate: row
                .get_opt_custom_parsed_at_idx(
                    "rampuprate",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5min: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreg: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availability: row
                .get_opt_custom_parsed_at_idx(
                    "availability",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secflags",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secflags",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minflags",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregflags: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregflags",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secflags",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secflags",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minflags",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregflags: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregflags",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[27],
                    mmsdm_core::mms_datetime::parse,
                )?,
            semidispatchcap: row
                .get_opt_custom_parsed_at_idx(
                    "semidispatchcap",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchmodetime: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchmodetime",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            conformance_mode: row
                .get_opt_custom_parsed_at_idx(
                    "conformance_mode",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uigf: row
                .get_opt_custom_parsed_at_idx(
                    "uigf",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise1sec",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secflags",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower1sec",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secflags",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initial_energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "initial_energy_storage",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_storage: row
                .get_opt_custom_parsed_at_idx(
                    "energy_storage",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_storage_min: row
                .get_opt_custom_parsed_at_idx(
                    "energy_storage_min",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_storage_max: row
                .get_opt_custom_parsed_at_idx(
                    "energy_storage_max",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            min_availability: row
                .get_opt_custom_parsed_at_idx(
                    "min_availability",
                    field_mapping.0[41],
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
        Ok(P5minUnitsolution6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                5,
                mmsdm_core::mms_datetime::parse,
            )?
            - {
                const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(5) {
                    Some(d) => d,
                    None => panic!("invalid"),
                };
                D
            };
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> P5minUnitsolution6PrimaryKey {
        P5minUnitsolution6PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            intervention: row.intervention,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: (chrono::NaiveDateTime::from(row.interval_datetime)
                - {
                    const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                        5,
                    ) {
                        Some(d) => d,
                        None => panic!("invalid"),
                    };
                    D
                })
                .year(),
            month: num_traits::FromPrimitive::from_u32(
                    (chrono::NaiveDateTime::from(row.interval_datetime)
                        - {
                            const D: chrono::TimeDelta = match chrono::TimeDelta::try_minutes(
                                5,
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
            "p5min_unitsolution_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        P5minUnitsolution6Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            duid: row.duid.clone(),
            connectionpointid: row.connectionpointid.clone(),
            tradetype: row.tradetype.clone(),
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
            lastchanged: row.lastchanged.clone(),
            semidispatchcap: row.semidispatchcap.clone(),
            intervention: row.intervention.clone(),
            dispatchmodetime: row.dispatchmodetime.clone(),
            conformance_mode: row.conformance_mode.clone(),
            uigf: row.uigf.clone(),
            raise1sec: row.raise1sec.clone(),
            raise1secflags: row.raise1secflags.clone(),
            lower1sec: row.lower1sec.clone(),
            lower1secflags: row.lower1secflags.clone(),
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
pub struct P5minUnitsolution6PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub intervention: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for P5minUnitsolution6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for P5minUnitsolution6Row<'data> {
    type Row<'other> = P5minUnitsolution6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for P5minUnitsolution6Row<'data> {
    type PrimaryKey = P5minUnitsolution6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for P5minUnitsolution6PrimaryKey {
    type Row<'other> = P5minUnitsolution6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
            && self.intervention == row.intervention
    }
}
impl mmsdm_core::CompareWithPrimaryKey for P5minUnitsolution6PrimaryKey {
    type PrimaryKey = P5minUnitsolution6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
            && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for P5minUnitsolution6 {
    type Builder = P5minUnitsolution6Builder;
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
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tradetype",
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
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "semidispatchcap",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
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
        P5minUnitsolution6Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            tradetype_array: arrow::array::builder::Decimal128Builder::new()
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
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            semidispatchcap_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
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
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.connectionpointid_array.append_option(row.connectionpointid());
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
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
            .intervention_array
            .append_value({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tradetype_array.finish())
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.semidispatchcap_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
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
pub struct P5minUnitsolution6Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    tradetype_array: arrow::array::builder::Decimal128Builder,
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
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    semidispatchcap_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    dispatchmodetime_array: arrow::array::builder::Decimal128Builder,
    conformance_mode_array: arrow::array::builder::Decimal128Builder,
    uigf_array: arrow::array::builder::Decimal128Builder,
    raise1sec_array: arrow::array::builder::Decimal128Builder,
    raise1secflags_array: arrow::array::builder::Decimal128Builder,
    lower1sec_array: arrow::array::builder::Decimal128Builder,
    lower1secflags_array: arrow::array::builder::Decimal128Builder,
    initial_energy_storage_array: arrow::array::builder::Decimal128Builder,
    energy_storage_array: arrow::array::builder::Decimal128Builder,
    energy_storage_min_array: arrow::array::builder::Decimal128Builder,
    energy_storage_max_array: arrow::array::builder::Decimal128Builder,
    min_availability_array: arrow::array::builder::Decimal128Builder,
}
