#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct GdInstructGdinstruct1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &GdInstructGdinstruct1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl GdInstructGdinstruct1 {
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
pub struct GdInstructGdinstruct1Mapping([usize; 15]);
/// # Summary
///
/// ## GDINSTRUCT
///  _GDINSTRUCT shows all manually issued dispatch instructions for a dispatchable unit. Ancillary Service instructions are to enable and to disable (i.e. 2 separate instructions) a service. Non-conforming units are also instructed via this facility. However, this facility is not the same as the market notice._
///
/// * Data Set Name: Gd Instruct
/// * File Name: Gdinstruct
/// * Data Version: 1
///
/// # Description
///  Source GDINSTRUCT updates on issue of an instruction by AEMO, with visibility restricted on the day of issue to the relevant participant. All participants have previous days' data available.
///
///
///
/// # Primary Key Columns
///
/// * ID
#[derive(Debug, PartialEq, Eq)]
pub struct GdInstructGdinstruct1Row<'data> {
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Instruction ID (sequential number)
    pub id: rust_decimal::Decimal,
    /// Instruction type
    pub instructiontypeid: core::ops::Range<usize>,
    /// Instruction sub type
    pub instructionsubtypeid: core::ops::Range<usize>,
    /// Instruction class
    pub instructionclassid: core::ops::Range<usize>,
    /// Reason
    pub reason: core::ops::Range<usize>,
    /// Instruction target level
    pub instlevel: Option<rust_decimal::Decimal>,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorised by
    pub authorisedby: core::ops::Range<usize>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Date / time issued
    pub issuedtime: Option<chrono::NaiveDateTime>,
    /// Date / time instruction to apply
    pub targettime: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GdInstructGdinstruct1Row<'data> {
    pub fn duid(&self) -> Option<&str> {
        if self.duid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone()),
            )
        }
    }
    pub fn stationid(&self) -> Option<&str> {
        if self.stationid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.stationid.clone(),
                ),
            )
        }
    }
    pub fn regionid(&self) -> Option<&str> {
        if self.regionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionid.clone(),
                ),
            )
        }
    }
    pub fn instructiontypeid(&self) -> Option<&str> {
        if self.instructiontypeid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.instructiontypeid.clone(),
                ),
            )
        }
    }
    pub fn instructionsubtypeid(&self) -> Option<&str> {
        if self.instructionsubtypeid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.instructionsubtypeid.clone(),
                ),
            )
        }
    }
    pub fn instructionclassid(&self) -> Option<&str> {
        if self.instructionclassid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.instructionclassid.clone(),
                ),
            )
        }
    }
    pub fn reason(&self) -> Option<&str> {
        if self.reason.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reason.clone(),
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
impl mmsdm_core::GetTable for GdInstructGdinstruct1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "GD_INSTRUCT";
    const TABLE_NAME: &'static str = "GDINSTRUCT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GdInstructGdinstruct1Mapping([
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
        "DUID",
        "STATIONID",
        "REGIONID",
        "ID",
        "INSTRUCTIONTYPEID",
        "INSTRUCTIONSUBTYPEID",
        "INSTRUCTIONCLASSID",
        "REASON",
        "INSTLEVEL",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "PARTICIPANTID",
        "ISSUEDTIME",
        "TARGETTIME",
        "LASTCHANGED",
    ];
    type Row<'row> = GdInstructGdinstruct1Row<'row>;
    type FieldMapping = GdInstructGdinstruct1Mapping;
    type PrimaryKey = GdInstructGdinstruct1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GdInstructGdinstruct1Row {
            duid: row.get_opt_range("duid", field_mapping.0[0])?,
            stationid: row.get_opt_range("stationid", field_mapping.0[1])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[2])?,
            id: row
                .get_custom_parsed_at_idx(
                    "id",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            instructiontypeid: row
                .get_opt_range("instructiontypeid", field_mapping.0[4])?,
            instructionsubtypeid: row
                .get_opt_range("instructionsubtypeid", field_mapping.0[5])?,
            instructionclassid: row
                .get_opt_range("instructionclassid", field_mapping.0[6])?,
            reason: row.get_opt_range("reason", field_mapping.0[7])?,
            instlevel: row
                .get_opt_custom_parsed_at_idx(
                    "instlevel",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[10])?,
            participantid: row.get_opt_range("participantid", field_mapping.0[11])?,
            issuedtime: row
                .get_opt_custom_parsed_at_idx(
                    "issuedtime",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            targettime: row
                .get_opt_custom_parsed_at_idx(
                    "targettime",
                    field_mapping.0[13],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(GdInstructGdinstruct1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GdInstructGdinstruct1PrimaryKey {
        GdInstructGdinstruct1PrimaryKey {
            id: row.id,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("gd_instruct_gdinstruct_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GdInstructGdinstruct1Row {
            duid: row.duid.clone(),
            stationid: row.stationid.clone(),
            regionid: row.regionid.clone(),
            id: row.id.clone(),
            instructiontypeid: row.instructiontypeid.clone(),
            instructionsubtypeid: row.instructionsubtypeid.clone(),
            instructionclassid: row.instructionclassid.clone(),
            reason: row.reason.clone(),
            instlevel: row.instlevel.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            participantid: row.participantid.clone(),
            issuedtime: row.issuedtime.clone(),
            targettime: row.targettime.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GdInstructGdinstruct1PrimaryKey {
    pub id: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for GdInstructGdinstruct1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GdInstructGdinstruct1Row<'data> {
    type Row<'other> = GdInstructGdinstruct1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.id == row.id
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GdInstructGdinstruct1Row<'data> {
    type PrimaryKey = GdInstructGdinstruct1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.id == key.id
    }
}
impl<'data> mmsdm_core::CompareWithRow for GdInstructGdinstruct1PrimaryKey {
    type Row<'other> = GdInstructGdinstruct1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.id == row.id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructGdinstruct1PrimaryKey {
    type PrimaryKey = GdInstructGdinstruct1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.id == key.id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GdInstructGdinstruct1 {
    type Builder = GdInstructGdinstruct1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "id",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "instructiontypeid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "instructionsubtypeid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "instructionclassid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "instlevel",
                    arrow::datatypes::DataType::Decimal128(6, 0),
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
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "issuedtime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "targettime",
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
        GdInstructGdinstruct1Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            id_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            instructiontypeid_array: arrow::array::builder::StringBuilder::new(),
            instructionsubtypeid_array: arrow::array::builder::StringBuilder::new(),
            instructionclassid_array: arrow::array::builder::StringBuilder::new(),
            reason_array: arrow::array::builder::StringBuilder::new(),
            instlevel_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            issuedtime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            targettime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_option(row.duid());
        builder.stationid_array.append_option(row.stationid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .id_array
            .append_value({
                let mut val = row.id;
                val.rescale(0);
                val.mantissa()
            });
        builder.instructiontypeid_array.append_option(row.instructiontypeid());
        builder.instructionsubtypeid_array.append_option(row.instructionsubtypeid());
        builder.instructionclassid_array.append_option(row.instructionclassid());
        builder.reason_array.append_option(row.reason());
        builder
            .instlevel_array
            .append_option({
                row.instlevel
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder.participantid_array.append_option(row.participantid());
        builder
            .issuedtime_array
            .append_option(row.issuedtime.map(|val| val.and_utc().timestamp_millis()));
        builder
            .targettime_array
            .append_option(row.targettime.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.instructiontypeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.instructionsubtypeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.instructionclassid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.instlevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.issuedtime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.targettime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GdInstructGdinstruct1Builder {
    duid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    id_array: arrow::array::builder::Decimal128Builder,
    instructiontypeid_array: arrow::array::builder::StringBuilder,
    instructionsubtypeid_array: arrow::array::builder::StringBuilder,
    instructionclassid_array: arrow::array::builder::StringBuilder,
    reason_array: arrow::array::builder::StringBuilder,
    instlevel_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    issuedtime_array: arrow::array::builder::TimestampMillisecondBuilder,
    targettime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct GdInstructInstructionsubtype1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &GdInstructInstructionsubtype1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl GdInstructInstructionsubtype1 {
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
pub struct GdInstructInstructionsubtype1Mapping([usize; 4]);
/// # Summary
///
/// ## INSTRUCTIONSUBTYPE
///  _Each Dispatch instruction (GD instruct) has a type and subtype. INSTRUCTIONSUBTYPE, together with INSTRUCTIONTYPE, sets out valid instruction types._
///
/// * Data Set Name: Gd Instruct
/// * File Name: Instructionsubtype
/// * Data Version: 1
///
/// # Description
///  INSTRUCTIONSUBTYPE is public data, and is available to all participants. Source INSTRUCTIONSUBTYPE shows ad hoc updates to market configuration.
///
///
///
/// # Primary Key Columns
///
/// * INSTRUCTIONSUBTYPEID
/// * INSTRUCTIONTYPEID
#[derive(Debug, PartialEq, Eq)]
pub struct GdInstructInstructionsubtype1Row<'data> {
    /// Instruction type
    pub instructiontypeid: core::ops::Range<usize>,
    /// Subtype for each dispatch instruction type, for example governor off.
    pub instructionsubtypeid: core::ops::Range<usize>,
    /// Description of instruction subtype
    pub description: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GdInstructInstructionsubtype1Row<'data> {
    pub fn instructiontypeid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.instructiontypeid.clone(),
        )
    }
    pub fn instructionsubtypeid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.instructionsubtypeid.clone(),
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
impl mmsdm_core::GetTable for GdInstructInstructionsubtype1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "GD_INSTRUCT";
    const TABLE_NAME: &'static str = "INSTRUCTIONSUBTYPE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GdInstructInstructionsubtype1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INSTRUCTIONTYPEID",
        "INSTRUCTIONSUBTYPEID",
        "DESCRIPTION",
        "LASTCHANGED",
    ];
    type Row<'row> = GdInstructInstructionsubtype1Row<'row>;
    type FieldMapping = GdInstructInstructionsubtype1Mapping;
    type PrimaryKey = GdInstructInstructionsubtype1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GdInstructInstructionsubtype1Row {
            instructiontypeid: row.get_range("instructiontypeid", field_mapping.0[0])?,
            instructionsubtypeid: row
                .get_range("instructionsubtypeid", field_mapping.0[1])?,
            description: row.get_opt_range("description", field_mapping.0[2])?,
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
        Ok(GdInstructInstructionsubtype1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GdInstructInstructionsubtype1PrimaryKey {
        GdInstructInstructionsubtype1PrimaryKey {
            instructionsubtypeid: row.instructionsubtypeid().to_string(),
            instructiontypeid: row.instructiontypeid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("gd_instruct_instructionsubtype_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GdInstructInstructionsubtype1Row {
            instructiontypeid: row.instructiontypeid.clone(),
            instructionsubtypeid: row.instructionsubtypeid.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GdInstructInstructionsubtype1PrimaryKey {
    pub instructionsubtypeid: alloc::string::String,
    pub instructiontypeid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for GdInstructInstructionsubtype1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GdInstructInstructionsubtype1Row<'data> {
    type Row<'other> = GdInstructInstructionsubtype1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.instructionsubtypeid() == row.instructionsubtypeid()
            && self.instructiontypeid() == row.instructiontypeid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for GdInstructInstructionsubtype1Row<'data> {
    type PrimaryKey = GdInstructInstructionsubtype1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructionsubtypeid() == key.instructionsubtypeid
            && self.instructiontypeid() == key.instructiontypeid
    }
}
impl<'data> mmsdm_core::CompareWithRow for GdInstructInstructionsubtype1PrimaryKey {
    type Row<'other> = GdInstructInstructionsubtype1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.instructionsubtypeid == row.instructionsubtypeid()
            && self.instructiontypeid == row.instructiontypeid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructInstructionsubtype1PrimaryKey {
    type PrimaryKey = GdInstructInstructionsubtype1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructionsubtypeid == key.instructionsubtypeid
            && self.instructiontypeid == key.instructiontypeid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GdInstructInstructionsubtype1 {
    type Builder = GdInstructInstructionsubtype1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "instructiontypeid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "instructionsubtypeid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
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
        GdInstructInstructionsubtype1Builder {
            instructiontypeid_array: arrow::array::builder::StringBuilder::new(),
            instructionsubtypeid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.instructiontypeid_array.append_value(row.instructiontypeid());
        builder.instructionsubtypeid_array.append_value(row.instructionsubtypeid());
        builder.description_array.append_option(row.description());
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
                    alloc::sync::Arc::new(builder.instructiontypeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.instructionsubtypeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GdInstructInstructionsubtype1Builder {
    instructiontypeid_array: arrow::array::builder::StringBuilder,
    instructionsubtypeid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct GdInstructInstructiontype1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &GdInstructInstructiontype1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl GdInstructInstructiontype1 {
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
pub struct GdInstructInstructiontype1Mapping([usize; 4]);
/// # Summary
///
/// ## INSTRUCTIONTYPE
///  _Dispatch instruction (GD instruct) has types and subtypes. INSTRUCTIONTYPE, together with INSTRUCTIONSUBTYPE, sets out valid instruction types._
///
/// * Data Set Name: Gd Instruct
/// * File Name: Instructiontype
/// * Data Version: 1
///
/// # Description
///  INSTRUCTIONTYPE data is public to all participants. Source INSTRUCTIONTYPE shows ad hoc updates to market configuration.
///
///
///
/// # Primary Key Columns
///
/// * INSTRUCTIONTYPEID
#[derive(Debug, PartialEq, Eq)]
pub struct GdInstructInstructiontype1Row<'data> {
    /// Dispatch instruction type for example FCAS service.
    pub instructiontypeid: core::ops::Range<usize>,
    /// Description of instruction type
    pub description: core::ops::Range<usize>,
    /// Region id if regional instruction only.
    pub regionid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GdInstructInstructiontype1Row<'data> {
    pub fn instructiontypeid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.instructiontypeid.clone(),
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
    pub fn regionid(&self) -> Option<&str> {
        if self.regionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for GdInstructInstructiontype1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "GD_INSTRUCT";
    const TABLE_NAME: &'static str = "INSTRUCTIONTYPE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GdInstructInstructiontype1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INSTRUCTIONTYPEID",
        "DESCRIPTION",
        "REGIONID",
        "LASTCHANGED",
    ];
    type Row<'row> = GdInstructInstructiontype1Row<'row>;
    type FieldMapping = GdInstructInstructiontype1Mapping;
    type PrimaryKey = GdInstructInstructiontype1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GdInstructInstructiontype1Row {
            instructiontypeid: row.get_range("instructiontypeid", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[2])?,
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
        Ok(GdInstructInstructiontype1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GdInstructInstructiontype1PrimaryKey {
        GdInstructInstructiontype1PrimaryKey {
            instructiontypeid: row.instructiontypeid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("gd_instruct_instructiontype_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GdInstructInstructiontype1Row {
            instructiontypeid: row.instructiontypeid.clone(),
            description: row.description.clone(),
            regionid: row.regionid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GdInstructInstructiontype1PrimaryKey {
    pub instructiontypeid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for GdInstructInstructiontype1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GdInstructInstructiontype1Row<'data> {
    type Row<'other> = GdInstructInstructiontype1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.instructiontypeid() == row.instructiontypeid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GdInstructInstructiontype1Row<'data> {
    type PrimaryKey = GdInstructInstructiontype1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructiontypeid() == key.instructiontypeid
    }
}
impl<'data> mmsdm_core::CompareWithRow for GdInstructInstructiontype1PrimaryKey {
    type Row<'other> = GdInstructInstructiontype1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.instructiontypeid == row.instructiontypeid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructInstructiontype1PrimaryKey {
    type PrimaryKey = GdInstructInstructiontype1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructiontypeid == key.instructiontypeid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GdInstructInstructiontype1 {
    type Builder = GdInstructInstructiontype1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "instructiontypeid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
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
        GdInstructInstructiontype1Builder {
            instructiontypeid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.instructiontypeid_array.append_value(row.instructiontypeid());
        builder.description_array.append_option(row.description());
        builder.regionid_array.append_option(row.regionid());
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
                    alloc::sync::Arc::new(builder.instructiontypeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GdInstructInstructiontype1Builder {
    instructiontypeid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
