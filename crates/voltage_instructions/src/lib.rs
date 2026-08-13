#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct VoltageInstructionInstruction2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &VoltageInstructionInstruction2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl VoltageInstructionInstruction2 {
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
pub struct VoltageInstructionInstruction2Mapping([usize; 13]);
/// # Summary
///
/// ## VOLTAGE_INSTRUCTION
///
/// Child record for Voltage Instructions (MVAr Dispatch)
///
/// * Data Set Name: Voltage Instruction
/// * File Name: Instruction
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
/// * EMS_ID
/// * RUN_DATETIME
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct VoltageInstructionInstruction2Row<'data> {
    /// MVAr Interval – a timestamp of when instructions issued
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for reference within AEMO –matches equipment names between NOS and EMS
    pub ems_id: core::ops::Range<usize>,
    /// The NEM id of the participant who owns the equipment
    pub participantid: core::ops::Range<usize>,
    /// The id of the station where the control equipment resides
    pub station_id: core::ops::Range<usize>,
    /// The company/participant preferred name of an equipment
    pub device_id: core::ops::Range<usize>,
    /// One of REACTOR, CAPACITOR, GEN, SVC, TRANS or GRPGEN but may be extended to other types
    pub device_type: core::ops::Range<usize>,
    /// One of VOLTAGE, TAP, MVAR, SWITCH or COMMIT but may be extended to other types
    pub control_type: core::ops::Range<usize>,
    /// Instruction for the device, for this interval null denotes no instruction
    pub target: Option<rust_decimal::Decimal>,
    /// [0,1] Denotes if the Device is currently conforming
    pub conforming: Option<rust_decimal::Decimal>,
    /// Verbose summary of instruction
    pub instruction_summary: core::ops::Range<usize>,
    /// Datetime the file was published by VDS - Versions differ from Run_DateTime only for Supplemental runs
    pub version_datetime: chrono::NaiveDateTime,
    /// Order for execution of Instruction
    pub instruction_sequence: Option<rust_decimal::Decimal>,
    /// Additional information pertaining to a particular instruction, e.g. Previously issued instruction revoked
    pub additional_notes: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> VoltageInstructionInstruction2Row<'data> {
    pub fn ems_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.ems_id.clone())
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
    pub fn station_id(&self) -> Option<&str> {
        if self.station_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.station_id.clone(),
                ),
            )
        }
    }
    pub fn device_id(&self) -> Option<&str> {
        if self.device_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.device_id.clone(),
                ),
            )
        }
    }
    pub fn device_type(&self) -> Option<&str> {
        if self.device_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.device_type.clone(),
                ),
            )
        }
    }
    pub fn control_type(&self) -> Option<&str> {
        if self.control_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.control_type.clone(),
                ),
            )
        }
    }
    pub fn instruction_summary(&self) -> Option<&str> {
        if self.instruction_summary.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.instruction_summary.clone(),
                ),
            )
        }
    }
    pub fn additional_notes(&self) -> Option<&str> {
        if self.additional_notes.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.additional_notes.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for VoltageInstructionInstruction2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "VOLTAGE_INSTRUCTION";
    const TABLE_NAME: &'static str = "INSTRUCTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = VoltageInstructionInstruction2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "EMS_ID",
        "PARTICIPANTID",
        "STATION_ID",
        "DEVICE_ID",
        "DEVICE_TYPE",
        "CONTROL_TYPE",
        "TARGET",
        "CONFORMING",
        "INSTRUCTION_SUMMARY",
        "VERSION_DATETIME",
        "INSTRUCTION_SEQUENCE",
        "ADDITIONAL_NOTES",
    ];
    type Row<'row> = VoltageInstructionInstruction2Row<'row>;
    type FieldMapping = VoltageInstructionInstruction2Mapping;
    type PrimaryKey = VoltageInstructionInstruction2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(VoltageInstructionInstruction2Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            ems_id: row.get_range("ems_id", field_mapping.0[1])?,
            participantid: row.get_opt_range("participantid", field_mapping.0[2])?,
            station_id: row.get_opt_range("station_id", field_mapping.0[3])?,
            device_id: row.get_opt_range("device_id", field_mapping.0[4])?,
            device_type: row.get_opt_range("device_type", field_mapping.0[5])?,
            control_type: row.get_opt_range("control_type", field_mapping.0[6])?,
            target: row
                .get_opt_custom_parsed_at_idx(
                    "target",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            conforming: row
                .get_opt_custom_parsed_at_idx(
                    "conforming",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            instruction_summary: row
                .get_opt_range("instruction_summary", field_mapping.0[9])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            instruction_sequence: row
                .get_opt_custom_parsed_at_idx(
                    "instruction_sequence",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            additional_notes: row
                .get_opt_range("additional_notes", field_mapping.0[12])?,
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
        Ok(VoltageInstructionInstruction2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> VoltageInstructionInstruction2PrimaryKey {
        VoltageInstructionInstruction2PrimaryKey {
            ems_id: row.ems_id().to_string(),
            run_datetime: row.run_datetime,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "voltage_instruction_instruction_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        VoltageInstructionInstruction2Row {
            run_datetime: row.run_datetime.clone(),
            ems_id: row.ems_id.clone(),
            participantid: row.participantid.clone(),
            station_id: row.station_id.clone(),
            device_id: row.device_id.clone(),
            device_type: row.device_type.clone(),
            control_type: row.control_type.clone(),
            target: row.target.clone(),
            conforming: row.conforming.clone(),
            instruction_summary: row.instruction_summary.clone(),
            version_datetime: row.version_datetime.clone(),
            instruction_sequence: row.instruction_sequence.clone(),
            additional_notes: row.additional_notes.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoltageInstructionInstruction2PrimaryKey {
    pub ems_id: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for VoltageInstructionInstruction2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for VoltageInstructionInstruction2Row<'data> {
    type Row<'other> = VoltageInstructionInstruction2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.ems_id() == row.ems_id() && self.run_datetime == row.run_datetime
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for VoltageInstructionInstruction2Row<'data> {
    type PrimaryKey = VoltageInstructionInstruction2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.ems_id() == key.ems_id && self.run_datetime == key.run_datetime
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for VoltageInstructionInstruction2PrimaryKey {
    type Row<'other> = VoltageInstructionInstruction2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.ems_id == row.ems_id() && self.run_datetime == row.run_datetime
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for VoltageInstructionInstruction2PrimaryKey {
    type PrimaryKey = VoltageInstructionInstruction2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.ems_id == key.ems_id && self.run_datetime == key.run_datetime
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for VoltageInstructionInstruction2 {
    type Builder = VoltageInstructionInstruction2Builder;
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
                    "ems_id",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "station_id",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "device_id",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "device_type",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "control_type",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "target",
                    arrow::datatypes::DataType::Decimal128(20, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "conforming",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "instruction_summary",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int64),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
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
                    "instruction_sequence",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "additional_notes",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        VoltageInstructionInstruction2Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            ems_id_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            participantid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            station_id_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            device_id_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            device_type_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            control_type_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            target_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 5)),
            conforming_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            instruction_summary_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int64Type,
            >::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            instruction_sequence_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            additional_notes_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.ems_id_array.append_value(row.ems_id());
        builder.participantid_array.append_option(row.participantid());
        builder.station_id_array.append_option(row.station_id());
        builder.device_id_array.append_option(row.device_id());
        builder.device_type_array.append_option(row.device_type());
        builder.control_type_array.append_option(row.control_type());
        builder
            .target_array
            .append_option({
                row.target
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .conforming_array
            .append_option({
                row.conforming
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.instruction_summary_array.append_option(row.instruction_summary());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .instruction_sequence_array
            .append_option({
                row.instruction_sequence
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.additional_notes_array.append_option(row.additional_notes());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ems_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.station_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.device_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.device_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.control_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.target_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.conforming_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.instruction_summary_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.instruction_sequence_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.additional_notes_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct VoltageInstructionInstruction2Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    ems_id_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    participantid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    station_id_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    device_id_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    device_type_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    control_type_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    target_array: arrow::array::builder::Decimal128Builder,
    conforming_array: arrow::array::builder::Decimal128Builder,
    instruction_summary_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int64Type,
    >,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    instruction_sequence_array: arrow::array::builder::Decimal128Builder,
    additional_notes_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
}
pub struct VoltageInstructionTrack2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &VoltageInstructionTrack2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl VoltageInstructionTrack2 {
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
pub struct VoltageInstructionTrack2Mapping([usize; 10]);
/// # Summary
///
/// ## VOLTAGE_INSTRUCTION_TRK
///
/// Parent record for Voltage Instructions (MVAr Dispatch). 'SIGNAL' records will have no children; 'INSTRUCTION' records will have children
///
/// * Data Set Name: Voltage Instruction
/// * File Name: Track
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
/// * RUN_DATETIME
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct VoltageInstructionTrack2Row<'data> {
    /// MVAr Interval - a timestamp of when instructions issued
    pub run_datetime: chrono::NaiveDateTime,
    /// Either 'SIGNAL' (childless) or 'INSTRUCTION'
    pub file_type: core::ops::Range<usize>,
    /// Datetime the file was published by VDS - Versions differ from Run_DateTime only for Supplemental runs
    pub version_datetime: chrono::NaiveDateTime,
    /// State Estimator start time, when a snapshot is taken of SCADA values
    pub se_datetime: Option<chrono::NaiveDateTime>,
    /// VDS solver solution category. Valid values: SUCCESS, WARNING, FAILURE
    pub solution_category: core::ops::Range<usize>,
    /// VDS solver solution status. Valid values: NOACTCNV [Solved with no instructions], NOVIOACT, CONVERGE, UNMANAGE, UNMANCTG, CTGDIV, SENHDIV [Failed with too many violations], BCDIV
    pub solution_status: core::ops::Range<usize>,
    /// The current VDS operating mode. Valid values: AUTO, AUTO-VERFIED, MANUAL
    pub operating_mode: core::ops::Range<usize>,
    /// Unstructured code and message from AEMO
    pub operating_status: core::ops::Range<usize>,
    /// Estimated expiry time of current Instruction set
    pub est_expiry: Option<chrono::NaiveDateTime>,
    /// Estimated issue time of next Instruction set
    pub est_next_instruction: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> VoltageInstructionTrack2Row<'data> {
    pub fn file_type(&self) -> Option<&str> {
        if self.file_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.file_type.clone(),
                ),
            )
        }
    }
    pub fn solution_category(&self) -> Option<&str> {
        if self.solution_category.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.solution_category.clone(),
                ),
            )
        }
    }
    pub fn solution_status(&self) -> Option<&str> {
        if self.solution_status.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.solution_status.clone(),
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
    pub fn operating_status(&self) -> Option<&str> {
        if self.operating_status.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.operating_status.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for VoltageInstructionTrack2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "VOLTAGE_INSTRUCTION";
    const TABLE_NAME: &'static str = "TRACK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = VoltageInstructionTrack2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "FILE_TYPE",
        "VERSION_DATETIME",
        "SE_DATETIME",
        "SOLUTION_CATEGORY",
        "SOLUTION_STATUS",
        "OPERATING_MODE",
        "OPERATING_STATUS",
        "EST_EXPIRY",
        "EST_NEXT_INSTRUCTION",
    ];
    type Row<'row> = VoltageInstructionTrack2Row<'row>;
    type FieldMapping = VoltageInstructionTrack2Mapping;
    type PrimaryKey = VoltageInstructionTrack2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(VoltageInstructionTrack2Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            file_type: row.get_opt_range("file_type", field_mapping.0[1])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            se_datetime: row
                .get_opt_custom_parsed_at_idx(
                    "se_datetime",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            solution_category: row
                .get_opt_range("solution_category", field_mapping.0[4])?,
            solution_status: row.get_opt_range("solution_status", field_mapping.0[5])?,
            operating_mode: row.get_opt_range("operating_mode", field_mapping.0[6])?,
            operating_status: row.get_opt_range("operating_status", field_mapping.0[7])?,
            est_expiry: row
                .get_opt_custom_parsed_at_idx(
                    "est_expiry",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            est_next_instruction: row
                .get_opt_custom_parsed_at_idx(
                    "est_next_instruction",
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
        Ok(VoltageInstructionTrack2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> VoltageInstructionTrack2PrimaryKey {
        VoltageInstructionTrack2PrimaryKey {
            run_datetime: row.run_datetime,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("voltage_instruction_track_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        VoltageInstructionTrack2Row {
            run_datetime: row.run_datetime.clone(),
            file_type: row.file_type.clone(),
            version_datetime: row.version_datetime.clone(),
            se_datetime: row.se_datetime.clone(),
            solution_category: row.solution_category.clone(),
            solution_status: row.solution_status.clone(),
            operating_mode: row.operating_mode.clone(),
            operating_status: row.operating_status.clone(),
            est_expiry: row.est_expiry.clone(),
            est_next_instruction: row.est_next_instruction.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoltageInstructionTrack2PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for VoltageInstructionTrack2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for VoltageInstructionTrack2Row<'data> {
    type Row<'other> = VoltageInstructionTrack2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for VoltageInstructionTrack2Row<'data> {
    type PrimaryKey = VoltageInstructionTrack2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for VoltageInstructionTrack2PrimaryKey {
    type Row<'other> = VoltageInstructionTrack2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for VoltageInstructionTrack2PrimaryKey {
    type PrimaryKey = VoltageInstructionTrack2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for VoltageInstructionTrack2 {
    type Builder = VoltageInstructionTrack2Builder;
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
                    "file_type",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
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
                    "se_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "solution_category",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "solution_status",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operating_mode",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operating_status",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "est_expiry",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "est_next_instruction",
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
        VoltageInstructionTrack2Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            file_type_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            se_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            solution_category_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            solution_status_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            operating_mode_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            operating_status_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            est_expiry_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            est_next_instruction_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.file_type_array.append_option(row.file_type());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .se_datetime_array
            .append_option(row.se_datetime.map(|val| val.and_utc().timestamp_millis()));
        builder.solution_category_array.append_option(row.solution_category());
        builder.solution_status_array.append_option(row.solution_status());
        builder.operating_mode_array.append_option(row.operating_mode());
        builder.operating_status_array.append_option(row.operating_status());
        builder
            .est_expiry_array
            .append_option(row.est_expiry.map(|val| val.and_utc().timestamp_millis()));
        builder
            .est_next_instruction_array
            .append_option(
                row.est_next_instruction.map(|val| val.and_utc().timestamp_millis()),
            );
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.file_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.se_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.solution_category_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.solution_status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.operating_mode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.operating_status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.est_expiry_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.est_next_instruction_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct VoltageInstructionTrack2Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    file_type_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    se_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    solution_category_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    solution_status_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    operating_mode_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    operating_status_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    est_expiry_array: arrow::array::builder::TimestampMillisecondBuilder,
    est_next_instruction_array: arrow::array::builder::TimestampMillisecondBuilder,
}
