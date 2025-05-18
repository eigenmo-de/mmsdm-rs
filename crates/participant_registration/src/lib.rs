#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct ParticipantRegistrationAdgDetail1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationAdgDetail1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationAdgDetail1 {
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
pub struct ParticipantRegistrationAdgDetail1Mapping([usize; 7]);
/// # Summary
///
/// ## ADG_DETAIL
///
/// Table for tracking evolving Aggregate Dispatch Group attributes
///
/// * Data Set Name: Participant Registration
/// * File Name: Adg Detail
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
/// * ADG_ID
/// * EFFECTIVEDATE
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationAdgDetail1Row<'data> {
    /// Identifies the Aggregate Dispatch Group
    pub adg_id: core::ops::Range<usize>,
    /// Effective calendar date of record
    pub effectivedate: chrono::NaiveDateTime,
    /// Date and time of the version of Dispatchable Unit details
    pub version_datetime: chrono::NaiveDateTime,
    /// Conformance Type for the Aggregate Dispatch Group.   One of the following: CAP, MIXED, TARGET
    pub adg_type: core::ops::Range<usize>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationAdgDetail1Row<'data> {
    pub fn adg_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.adg_id.clone())
    }
    pub fn adg_type(&self) -> Option<&str> {
        if self.adg_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.adg_type.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationAdgDetail1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "ADG_DETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationAdgDetail1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "ADG_ID",
        "EFFECTIVEDATE",
        "VERSION_DATETIME",
        "ADG_TYPE",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationAdgDetail1Row<'row>;
    type FieldMapping = ParticipantRegistrationAdgDetail1Mapping;
    type PrimaryKey = ParticipantRegistrationAdgDetail1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationAdgDetail1Row {
            adg_id: row.get_range("adg_id", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            adg_type: row.get_opt_range("adg_type", field_mapping.0[3])?,
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
        Ok(ParticipantRegistrationAdgDetail1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationAdgDetail1PrimaryKey {
        ParticipantRegistrationAdgDetail1PrimaryKey {
            adg_id: row.adg_id().to_string(),
            effectivedate: row.effectivedate,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_adg_detail_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationAdgDetail1Row {
            adg_id: row.adg_id.clone(),
            effectivedate: row.effectivedate.clone(),
            version_datetime: row.version_datetime.clone(),
            adg_type: row.adg_type.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationAdgDetail1PrimaryKey {
    pub adg_id: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationAdgDetail1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationAdgDetail1Row<'data> {
    type Row<'other> = ParticipantRegistrationAdgDetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.adg_id() == row.adg_id() && self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationAdgDetail1Row<'data> {
    type PrimaryKey = ParticipantRegistrationAdgDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adg_id() == key.adg_id && self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationAdgDetail1PrimaryKey {
    type Row<'other> = ParticipantRegistrationAdgDetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.adg_id == row.adg_id() && self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationAdgDetail1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationAdgDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adg_id == key.adg_id && self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationAdgDetail1 {
    type Builder = ParticipantRegistrationAdgDetail1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "adg_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
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
                    "adg_type",
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
        ParticipantRegistrationAdgDetail1Builder {
            adg_id_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            adg_type_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.adg_id_array.append_value(row.adg_id());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder.adg_type_array.append_option(row.adg_type());
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
                    alloc::sync::Arc::new(builder.adg_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.adg_type_array.finish())
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
pub struct ParticipantRegistrationAdgDetail1Builder {
    adg_id_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    adg_type_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationAggregateDispatchGroup1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationAggregateDispatchGroup1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationAggregateDispatchGroup1 {
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
pub struct ParticipantRegistrationAggregateDispatchGroup1Mapping([usize; 3]);
/// # Summary
///
/// ## AGGREGATE_DISPATCH_GROUP
///
/// Entity allowing for compliance monitoring over grouped DUIDs
///
/// * Data Set Name: Participant Registration
/// * File Name: Aggregate Dispatch Group
/// * Data Version: 1
///
/// # Description
/// BIDDUIDDETAILS data is public to participants.SourceBIDDUIDDETAILS updates as dispatchable unit registration details are modified.VolumeApproximately 1000 records per year.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * ADG_ID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationAggregateDispatchGroup1Row<'data> {
    /// Aggregate Dispatch Group ID
    pub adg_id: core::ops::Range<usize>,
    /// A participant provided comment
    pub comments: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationAggregateDispatchGroup1Row<'data> {
    pub fn adg_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.adg_id.clone())
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
}
impl mmsdm_core::GetTable for ParticipantRegistrationAggregateDispatchGroup1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "AGGREGATE_DISPATCH_GROUP";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationAggregateDispatchGroup1Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &["ADG_ID", "COMMENTS", "LASTCHANGED"];
    type Row<'row> = ParticipantRegistrationAggregateDispatchGroup1Row<'row>;
    type FieldMapping = ParticipantRegistrationAggregateDispatchGroup1Mapping;
    type PrimaryKey = ParticipantRegistrationAggregateDispatchGroup1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationAggregateDispatchGroup1Row {
            adg_id: row.get_range("adg_id", field_mapping.0[0])?,
            comments: row.get_opt_range("comments", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
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
        Ok(ParticipantRegistrationAggregateDispatchGroup1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationAggregateDispatchGroup1PrimaryKey {
        ParticipantRegistrationAggregateDispatchGroup1PrimaryKey {
            adg_id: row.adg_id().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_aggregate_dispatch_group_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationAggregateDispatchGroup1Row {
            adg_id: row.adg_id.clone(),
            comments: row.comments.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationAggregateDispatchGroup1PrimaryKey {
    pub adg_id: alloc::string::String,
}
impl mmsdm_core::PrimaryKey
for ParticipantRegistrationAggregateDispatchGroup1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationAggregateDispatchGroup1Row<'data> {
    type Row<'other> = ParticipantRegistrationAggregateDispatchGroup1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.adg_id() == row.adg_id()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationAggregateDispatchGroup1Row<'data> {
    type PrimaryKey = ParticipantRegistrationAggregateDispatchGroup1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adg_id() == key.adg_id
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationAggregateDispatchGroup1PrimaryKey {
    type Row<'other> = ParticipantRegistrationAggregateDispatchGroup1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.adg_id == row.adg_id()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationAggregateDispatchGroup1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationAggregateDispatchGroup1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adg_id == key.adg_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationAggregateDispatchGroup1 {
    type Builder = ParticipantRegistrationAggregateDispatchGroup1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "adg_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationAggregateDispatchGroup1Builder {
            adg_id_array: arrow::array::builder::StringBuilder::new(),
            comments_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.adg_id_array.append_value(row.adg_id());
        builder.comments_array.append_option(row.comments());
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
                    alloc::sync::Arc::new(builder.adg_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.comments_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationAggregateDispatchGroup1Builder {
    adg_id_array: arrow::array::builder::StringBuilder,
    comments_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationBidduiddetails1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationBidduiddetails1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationBidduiddetails1 {
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
pub struct ParticipantRegistrationBidduiddetails1Mapping([usize; 10]);
/// # Summary
///
/// ## BIDDUIDDETAILS
///
/// BIDDUIDDETAILS and the associated tracking object BIDDUIDDETAILSTRK define the registration data for each ancillary service a dispatchable unit is registered to provide. The registration data is required to validate a dispatchable unit bid submitted for that ancillary service.
///
/// * Data Set Name: Participant Registration
/// * File Name: Bidduiddetails
/// * Data Version: 1
///
/// # Description
/// BIDDUIDDETAILS data is public to participants.SourceBIDDUIDDETAILS updates as dispatchable unit registration details are modified.VolumeApproximately 1000 records per year.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationBidduiddetails1Row<'data> {
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Market date starting at 04:30 inclusive
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// Maximum Capacity of this DUID for this BIDTYPE
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which this ancillary service becomes available (AS Only)
    pub minenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which this ancillary service can be supplied (AS Only)
    pub maxenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the lower end of the ancillary service profile (Degrees)
    pub maxlowerangle: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the upper end of the ancillary service profile (Degrees)
    pub maxupperangle: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationBidduiddetails1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationBidduiddetails1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "BIDDUIDDETAILS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationBidduiddetails1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "BIDTYPE",
        "MAXCAPACITY",
        "MINENABLEMENTLEVEL",
        "MAXENABLEMENTLEVEL",
        "MAXLOWERANGLE",
        "MAXUPPERANGLE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationBidduiddetails1Row<'row>;
    type FieldMapping = ParticipantRegistrationBidduiddetails1Mapping;
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationBidduiddetails1Row {
            duid: row.get_range("duid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bidtype: row.get_range("bidtype", field_mapping.0[3])?,
            maxcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxcapacity",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            minenablementlevel: row
                .get_opt_custom_parsed_at_idx(
                    "minenablementlevel",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxenablementlevel: row
                .get_opt_custom_parsed_at_idx(
                    "maxenablementlevel",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxlowerangle: row
                .get_opt_custom_parsed_at_idx(
                    "maxlowerangle",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxupperangle: row
                .get_opt_custom_parsed_at_idx(
                    "maxupperangle",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
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
        Ok(ParticipantRegistrationBidduiddetails1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationBidduiddetails1PrimaryKey {
        ParticipantRegistrationBidduiddetails1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            duid: row.duid().to_string(),
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_bidduiddetails_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationBidduiddetails1Row {
            duid: row.duid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            bidtype: row.bidtype.clone(),
            maxcapacity: row.maxcapacity.clone(),
            minenablementlevel: row.minenablementlevel.clone(),
            maxenablementlevel: row.maxenablementlevel.clone(),
            maxlowerangle: row.maxlowerangle.clone(),
            maxupperangle: row.maxupperangle.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationBidduiddetails1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub duid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationBidduiddetails1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationBidduiddetails1Row<'data> {
    type Row<'other> = ParticipantRegistrationBidduiddetails1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.duid() == row.duid()
            && self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationBidduiddetails1Row<'data> {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.duid() == key.duid
            && self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationBidduiddetails1PrimaryKey {
    type Row<'other> = ParticipantRegistrationBidduiddetails1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.duid == row.duid()
            && self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationBidduiddetails1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.duid == key.duid
            && self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationBidduiddetails1 {
    type Builder = ParticipantRegistrationBidduiddetails1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
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
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "maxcapacity",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "minenablementlevel",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxenablementlevel",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxlowerangle",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxupperangle",
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
        ParticipantRegistrationBidduiddetails1Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            maxcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            minenablementlevel_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            maxenablementlevel_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            maxlowerangle_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            maxupperangle_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .maxcapacity_array
            .append_option({
                row.maxcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .minenablementlevel_array
            .append_option({
                row.minenablementlevel
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxenablementlevel_array
            .append_option({
                row.maxenablementlevel
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxlowerangle_array
            .append_option({
                row.maxlowerangle
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxupperangle_array
            .append_option({
                row.maxupperangle
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.minenablementlevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxenablementlevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxlowerangle_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxupperangle_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationBidduiddetails1Builder {
    duid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    bidtype_array: arrow::array::builder::StringBuilder,
    maxcapacity_array: arrow::array::builder::Decimal128Builder,
    minenablementlevel_array: arrow::array::builder::Decimal128Builder,
    maxenablementlevel_array: arrow::array::builder::Decimal128Builder,
    maxlowerangle_array: arrow::array::builder::Decimal128Builder,
    maxupperangle_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationBidduiddetailstrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationBidduiddetailstrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationBidduiddetailstrk1 {
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
pub struct ParticipantRegistrationBidduiddetailstrk1Mapping([usize; 6]);
/// # Summary
///
/// ## BIDDUIDDETAILSTRK
///
/// BIDDUIDDETAILSTRK shows the tracking for the associated object BIDDUIDDETAILS. Together, BIDDUIDDETAILSTRK and BIDDUIDDETAILS define the registration data for each ancillary service a dispatchable unit is registered to provide. The registration data is required to validate a dispatchable unit bid submitted for that ancillary service.
///
/// * Data Set Name: Participant Registration
/// * File Name: Bidduiddetailstrk
/// * Data Version: 1
///
/// # Description
/// BIDDUIDDETAILSTRK data is public to participants.SourceBIDDUIDDETAILSTRK updates as dispatchable unit registration details are modified.VolumeApproximately 200 records per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationBidduiddetailstrk1Row<'data> {
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Market date starting at 04:30 inclusive
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Date of record authorisation. A NULL value indicates the record is not authorised.
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User that authorised record. A NULL value indicates the record is not authorised.
    pub authorisedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationBidduiddetailstrk1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
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
impl mmsdm_core::GetTable for ParticipantRegistrationBidduiddetailstrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "BIDDUIDDETAILSTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationBidduiddetailstrk1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationBidduiddetailstrk1Row<'row>;
    type FieldMapping = ParticipantRegistrationBidduiddetailstrk1Mapping;
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationBidduiddetailstrk1Row {
            duid: row.get_range("duid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[4])?,
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
        Ok(ParticipantRegistrationBidduiddetailstrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
        ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
            duid: row.duid().to_string(),
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_bidduiddetailstrk_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationBidduiddetailstrk1Row {
            duid: row.duid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    pub duid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationBidduiddetailstrk1Row<'data> {
    type Row<'other> = ParticipantRegistrationBidduiddetailstrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationBidduiddetailstrk1Row<'data> {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    type Row<'other> = ParticipantRegistrationBidduiddetailstrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationBidduiddetailstrk1 {
    type Builder = ParticipantRegistrationBidduiddetailstrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
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
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        ParticipantRegistrationBidduiddetailstrk1Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
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
pub struct ParticipantRegistrationBidduiddetailstrk1Builder {
    duid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationDispatchableunit1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationDispatchableunit1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationDispatchableunit1 {
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
pub struct ParticipantRegistrationDispatchableunit1Mapping([usize; 4]);
/// # Summary
///
/// ## DISPATCHABLEUNIT
///
/// DISPATCHABLEUNIT sets out the unit name and type of each dispatchable unit in the market.
///
/// * Data Set Name: Participant Registration
/// * File Name: Dispatchableunit
/// * Data Version: 1
///
/// # Description
/// DISPATCHABLEUNIT data is public data, and is available to all participants.SourceDISPATCHABLEUNIT pdates as new units added or names changed.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DUID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationDispatchableunit1Row<'data> {
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Dispatchable Unit full description
    pub duname: core::ops::Range<usize>,
    /// Identifies LOAD, GENERATOR or BIDIRECTIONAL
    pub unittype: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationDispatchableunit1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn duname(&self) -> Option<&str> {
        if self.duname.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.duname.clone(),
                ),
            )
        }
    }
    pub fn unittype(&self) -> Option<&str> {
        if self.unittype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unittype.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationDispatchableunit1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "DISPATCHABLEUNIT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationDispatchableunit1Mapping([
        4, 5, 6, 7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "DUNAME",
        "UNITTYPE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationDispatchableunit1Row<'row>;
    type FieldMapping = ParticipantRegistrationDispatchableunit1Mapping;
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationDispatchableunit1Row {
            duid: row.get_range("duid", field_mapping.0[0])?,
            duname: row.get_opt_range("duname", field_mapping.0[1])?,
            unittype: row.get_opt_range("unittype", field_mapping.0[2])?,
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
        Ok(ParticipantRegistrationDispatchableunit1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationDispatchableunit1PrimaryKey {
        ParticipantRegistrationDispatchableunit1PrimaryKey {
            duid: row.duid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_dispatchableunit_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationDispatchableunit1Row {
            duid: row.duid.clone(),
            duname: row.duname.clone(),
            unittype: row.unittype.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDispatchableunit1PrimaryKey {
    pub duid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDispatchableunit1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationDispatchableunit1Row<'data> {
    type Row<'other> = ParticipantRegistrationDispatchableunit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationDispatchableunit1Row<'data> {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationDispatchableunit1PrimaryKey {
    type Row<'other> = ParticipantRegistrationDispatchableunit1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationDispatchableunit1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDispatchableunit1 {
    type Builder = ParticipantRegistrationDispatchableunit1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duname",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unittype",
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
        ParticipantRegistrationDispatchableunit1Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            duname_array: arrow::array::builder::StringBuilder::new(),
            unittype_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder.duname_array.append_option(row.duname());
        builder.unittype_array.append_option(row.unittype());
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
                    alloc::sync::Arc::new(builder.duname_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unittype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationDispatchableunit1Builder {
    duid_array: arrow::array::builder::StringBuilder,
    duname_array: arrow::array::builder::StringBuilder,
    unittype_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationDualloc1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationDualloc1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationDualloc1 {
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
pub struct ParticipantRegistrationDualloc1Mapping([usize; 5]);
/// # Summary
///
/// ## DUALLOC
///
/// DUALLOC cross references dispatch unit identifier to genset ID for each participant.
///
/// * Data Set Name: Participant Registration
/// * File Name: Dualloc
/// * Data Version: 1
///
/// # Description
/// SourceDUALLOC updates where changed.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DUID
/// * EFFECTIVEDATE
/// * GENSETID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationDualloc1Row<'data> {
    /// Effective calendar date of record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    pub versionno: rust_decimal::Decimal,
    /// Dispatchable Unit identifier
    pub duid: core::ops::Range<usize>,
    /// Physical unit identifier
    pub gensetid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationDualloc1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn gensetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.gensetid.clone())
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationDualloc1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "DUALLOC";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationDualloc1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "DUID",
        "GENSETID",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationDualloc1Row<'row>;
    type FieldMapping = ParticipantRegistrationDualloc1Mapping;
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationDualloc1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[2])?,
            gensetid: row.get_range("gensetid", field_mapping.0[3])?,
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
        Ok(ParticipantRegistrationDualloc1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationDualloc1PrimaryKey {
        ParticipantRegistrationDualloc1PrimaryKey {
            duid: row.duid().to_string(),
            effectivedate: row.effectivedate,
            gensetid: row.gensetid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_dualloc_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationDualloc1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            duid: row.duid.clone(),
            gensetid: row.gensetid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDualloc1PrimaryKey {
    pub duid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub gensetid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDualloc1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationDualloc1Row<'data> {
    type Row<'other> = ParticipantRegistrationDualloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.effectivedate == row.effectivedate
            && self.gensetid() == row.gensetid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationDualloc1Row<'data> {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.effectivedate == key.effectivedate
            && self.gensetid() == key.gensetid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationDualloc1PrimaryKey {
    type Row<'other> = ParticipantRegistrationDualloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.effectivedate == row.effectivedate
            && self.gensetid == row.gensetid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDualloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.effectivedate == key.effectivedate
            && self.gensetid == key.gensetid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDualloc1 {
    type Builder = ParticipantRegistrationDualloc1Builder;
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
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "gensetid",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationDualloc1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            gensetid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.duid_array.append_value(row.duid());
        builder.gensetid_array.append_value(row.gensetid());
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
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gensetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationDualloc1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    gensetid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationDudetail7 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationDudetail7Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationDudetail7 {
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
pub struct ParticipantRegistrationDudetail7Mapping([usize; 34]);
/// # Summary
///
/// ## DUDETAIL
///
/// DUDETAIL sets out a records specific details for each unit including start type and whether normally on or off load. Much of this data is information only and is not used in dispatch or settlements.
///
/// * Data Set Name: Participant Registration
/// * File Name: Dudetail
/// * Data Version: 7
///
/// # Description
/// DUDETAIL is public data, and is available to all participants.SourceDUDETAIL updates only when registration details change.NoteTo find the current set of details for selected dispatchable units, query the participant's local database as follows.Select du.* from dudetail duwhere (du.EFFECTIVEDATE, du.VERSIONNO) =(select effectivedate, max(versionno)from dudetailwhere EFFECTIVEDATE = (select max(effectivedate)from  dudetailwhere EFFECTIVEDATE <= sysdateand duid = du.duidand authoriseddate is not null)and duid = du.duidand authoriseddate is not nullgroup by effectivedate)and du.duid in ('UNIT1', 'UNIT2');The following notes apply to this SQL code:·This table is specific to dispatch units only.·If you wish to query details for a different date, substitute a date expression for "sysdate"in the "where EFFECTIVEDATE <= sysdate"clause.·If you wish to list all the units, remove the line "and du.duid in ('UNIT1', 'UNIT2')"·The DUDETAIL table does not indicate if a unit is active;  this is done through ownership (STADUALLOC) by an active station owned by an active participant (STATIONOWNER )·If you wish to query Station details refer to STATION, STATIONOWNER and STADUALLOC.·If you wish to look at connection point loss factors, refer to TRANSMISSIONLOSSFACTOR.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationDudetail7Row<'data> {
    /// Effective calendar date of record
    pub effectivedate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// version of Dispatchable Unit details for this effective date
    pub versionno: rust_decimal::Decimal,
    /// Country wide - Unique id of a connection point
    pub connectionpointid: core::ops::Range<usize>,
    /// Voltage Level
    pub voltlevel: core::ops::Range<usize>,
    /// Registered capacity for normal operations
    pub registeredcapacity: Option<rust_decimal::Decimal>,
    /// AGC Capability flag
    pub agccapability: core::ops::Range<usize>,
    /// Identifies LOAD, GENERATOR or BIDIRECTIONAL.
    pub dispatchtype: core::ops::Range<usize>,
    /// Maximum Capacity as used for bid validation
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Identify unit as Fast or Slow
    pub starttype: core::ops::Range<usize>,
    /// For a dispatchable load indicates that the load is normally on or off.
    pub normallyonflag: core::ops::Range<usize>,
    /// Indicates that the physical details for this unit are to be recorded
    pub physicaldetailsflag: core::ops::Range<usize>,
    /// Indicates spinning reserve capability
    pub spinningreserveflag: core::ops::Range<usize>,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Indicate whether a unit is intermittent (e.g. a wind farm)
    pub intermittentflag: core::ops::Range<usize>,
    /// Indicates if the DUID is a Semi-Scheduled Unit
    pub semi_schedule_flag: core::ops::Range<usize>,
    /// Maximum ramp up rate for Unit (Mw/min)
    pub maxrateofchangeup: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min)
    pub maxrateofchangedown: Option<rust_decimal::Decimal>,
    /// Additional information for DISPATCHTYPE. For DISPATCHTYPE = LOAD, subtype value is WDR for wholesale demand response units. For DISPATCHTYPE = LOAD, subtype value is NULL for Scheduled Loads. For DISPATCHTYPE = GENERATOR type, the subtype value is NULL.
    pub dispatchsubtype: core::ops::Range<usize>,
    /// Aggregate Dispatch Group to which this dispatch unit belongs
    pub adg_id: core::ops::Range<usize>,
    /// Minimum capacity only for load side of BDU, otherwise 0 (MW)
    pub mincapacity: Option<rust_decimal::Decimal>,
    /// Registered minimum capacity only for load side of BDU, otherwise 0 (MW)
    pub registeredmincapacity: Option<rust_decimal::Decimal>,
    /// Raise Ramp rate applied to BDU Load component (MW/min)
    pub maxrateofchangeup_load: Option<rust_decimal::Decimal>,
    /// Lower Ramp rate applied to BDU Load component (MW/min)
    pub maxrateofchangedown_load: Option<rust_decimal::Decimal>,
    /// The rated storage capacity (MWh), information only
    pub maxstoragecapacity: Option<rust_decimal::Decimal>,
    /// The storage energy import conversion efficiency. Number from 0 to 1 where 1 is lossless. Calculated as (increase in stored energy / increase in imported energy)
    pub storageimportefficiencyfactor: Option<rust_decimal::Decimal>,
    /// The storage energy export conversion efficiency. Number from 0 to 1 where 1 is lossless. Calculated as (decrease in exported energy / decrease in stored energy)
    pub storageexportefficiencyfactor: Option<rust_decimal::Decimal>,
    /// Calculated Minimum Ramp Rate Up value accepted for Energy Offers or Bids with explanation for energy imports (all DUID types and BDU Generation side) (MW/min)
    pub min_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// Calculated Minimum Ramp Rate Down value accepted for Energy Offers or Bids with explanation for energy imports (all DUID types and BDU Generation side) (MW/min)
    pub min_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Calculated Minimum Ramp Rate Up value accepted for Energy Offers or Bids on BDU Load component with explanation for energy imports (MW/min)
    pub load_min_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// Calculated Minimum Ramp Rate Down value accepted for Energy Offers or Bids on BDU Load component with explanation for energy imports (MW/min)
    pub load_min_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Identifies if a unit is aggregated. This flag was initially added in GENUNITS_UNIT table which is now deprecated with IESS release.
    pub aggregated: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationDudetail7Row<'data> {
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
    pub fn voltlevel(&self) -> Option<&str> {
        if self.voltlevel.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.voltlevel.clone(),
                ),
            )
        }
    }
    pub fn agccapability(&self) -> Option<&str> {
        if self.agccapability.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.agccapability.clone(),
                ),
            )
        }
    }
    pub fn dispatchtype(&self) -> Option<&str> {
        if self.dispatchtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.dispatchtype.clone(),
                ),
            )
        }
    }
    pub fn starttype(&self) -> Option<&str> {
        if self.starttype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.starttype.clone(),
                ),
            )
        }
    }
    pub fn normallyonflag(&self) -> Option<&str> {
        if self.normallyonflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.normallyonflag.clone(),
                ),
            )
        }
    }
    pub fn physicaldetailsflag(&self) -> Option<&str> {
        if self.physicaldetailsflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.physicaldetailsflag.clone(),
                ),
            )
        }
    }
    pub fn spinningreserveflag(&self) -> Option<&str> {
        if self.spinningreserveflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spinningreserveflag.clone(),
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
    pub fn intermittentflag(&self) -> Option<&str> {
        if self.intermittentflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.intermittentflag.clone(),
                ),
            )
        }
    }
    pub fn semi_schedule_flag(&self) -> Option<&str> {
        if self.semi_schedule_flag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.semi_schedule_flag.clone(),
                ),
            )
        }
    }
    pub fn dispatchsubtype(&self) -> Option<&str> {
        if self.dispatchsubtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.dispatchsubtype.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationDudetail7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "DUDETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationDudetail7Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "DUID",
        "VERSIONNO",
        "CONNECTIONPOINTID",
        "VOLTLEVEL",
        "REGISTEREDCAPACITY",
        "AGCCAPABILITY",
        "DISPATCHTYPE",
        "MAXCAPACITY",
        "STARTTYPE",
        "NORMALLYONFLAG",
        "PHYSICALDETAILSFLAG",
        "SPINNINGRESERVEFLAG",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
        "INTERMITTENTFLAG",
        "SemiSchedule_Flag",
        "MAXRATEOFCHANGEUP",
        "MAXRATEOFCHANGEDOWN",
        "DISPATCHSUBTYPE",
        "ADG_ID",
        "MINCAPACITY",
        "REGISTEREDMINCAPACITY",
        "MAXRATEOFCHANGEUP_LOAD",
        "MAXRATEOFCHANGEDOWN_LOAD",
        "MAXSTORAGECAPACITY",
        "STORAGEIMPORTEFFICIENCYFACTOR",
        "STORAGEEXPORTEFFICIENCYFACTOR",
        "MIN_RAMP_RATE_UP",
        "MIN_RAMP_RATE_DOWN",
        "LOAD_MIN_RAMP_RATE_UP",
        "LOAD_MIN_RAMP_RATE_DOWN",
        "AGGREGATED",
    ];
    type Row<'row> = ParticipantRegistrationDudetail7Row<'row>;
    type FieldMapping = ParticipantRegistrationDudetail7Mapping;
    type PrimaryKey = ParticipantRegistrationDudetail7PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationDudetail7Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[3])?,
            voltlevel: row.get_opt_range("voltlevel", field_mapping.0[4])?,
            registeredcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "registeredcapacity",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            agccapability: row.get_opt_range("agccapability", field_mapping.0[6])?,
            dispatchtype: row.get_opt_range("dispatchtype", field_mapping.0[7])?,
            maxcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxcapacity",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            starttype: row.get_opt_range("starttype", field_mapping.0[9])?,
            normallyonflag: row.get_opt_range("normallyonflag", field_mapping.0[10])?,
            physicaldetailsflag: row
                .get_opt_range("physicaldetailsflag", field_mapping.0[11])?,
            spinningreserveflag: row
                .get_opt_range("spinningreserveflag", field_mapping.0[12])?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[13])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[14],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[15],
                    mmsdm_core::mms_datetime::parse,
                )?,
            intermittentflag: row
                .get_opt_range("intermittentflag", field_mapping.0[16])?,
            semi_schedule_flag: row
                .get_opt_range("semi_schedule_flag", field_mapping.0[17])?,
            maxrateofchangeup: row
                .get_opt_custom_parsed_at_idx(
                    "maxrateofchangeup",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxrateofchangedown: row
                .get_opt_custom_parsed_at_idx(
                    "maxrateofchangedown",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchsubtype: row.get_opt_range("dispatchsubtype", field_mapping.0[20])?,
            adg_id: row.get_opt_range("adg_id", field_mapping.0[21])?,
            mincapacity: row
                .get_opt_custom_parsed_at_idx(
                    "mincapacity",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            registeredmincapacity: row
                .get_opt_custom_parsed_at_idx(
                    "registeredmincapacity",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxrateofchangeup_load: row
                .get_opt_custom_parsed_at_idx(
                    "maxrateofchangeup_load",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxrateofchangedown_load: row
                .get_opt_custom_parsed_at_idx(
                    "maxrateofchangedown_load",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxstoragecapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxstoragecapacity",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            storageimportefficiencyfactor: row
                .get_opt_custom_parsed_at_idx(
                    "storageimportefficiencyfactor",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            storageexportefficiencyfactor: row
                .get_opt_custom_parsed_at_idx(
                    "storageexportefficiencyfactor",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            min_ramp_rate_up: row
                .get_opt_custom_parsed_at_idx(
                    "min_ramp_rate_up",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            min_ramp_rate_down: row
                .get_opt_custom_parsed_at_idx(
                    "min_ramp_rate_down",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_min_ramp_rate_up: row
                .get_opt_custom_parsed_at_idx(
                    "load_min_ramp_rate_up",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_min_ramp_rate_down: row
                .get_opt_custom_parsed_at_idx(
                    "load_min_ramp_rate_down",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregated: row
                .get_opt_custom_parsed_at_idx(
                    "aggregated",
                    field_mapping.0[33],
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
        Ok(ParticipantRegistrationDudetail7Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationDudetail7PrimaryKey {
        ParticipantRegistrationDudetail7PrimaryKey {
            duid: row.duid().to_string(),
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_dudetail_v7_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationDudetail7Row {
            effectivedate: row.effectivedate.clone(),
            duid: row.duid.clone(),
            versionno: row.versionno.clone(),
            connectionpointid: row.connectionpointid.clone(),
            voltlevel: row.voltlevel.clone(),
            registeredcapacity: row.registeredcapacity.clone(),
            agccapability: row.agccapability.clone(),
            dispatchtype: row.dispatchtype.clone(),
            maxcapacity: row.maxcapacity.clone(),
            starttype: row.starttype.clone(),
            normallyonflag: row.normallyonflag.clone(),
            physicaldetailsflag: row.physicaldetailsflag.clone(),
            spinningreserveflag: row.spinningreserveflag.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            intermittentflag: row.intermittentflag.clone(),
            semi_schedule_flag: row.semi_schedule_flag.clone(),
            maxrateofchangeup: row.maxrateofchangeup.clone(),
            maxrateofchangedown: row.maxrateofchangedown.clone(),
            dispatchsubtype: row.dispatchsubtype.clone(),
            adg_id: row.adg_id.clone(),
            mincapacity: row.mincapacity.clone(),
            registeredmincapacity: row.registeredmincapacity.clone(),
            maxrateofchangeup_load: row.maxrateofchangeup_load.clone(),
            maxrateofchangedown_load: row.maxrateofchangedown_load.clone(),
            maxstoragecapacity: row.maxstoragecapacity.clone(),
            storageimportefficiencyfactor: row.storageimportefficiencyfactor.clone(),
            storageexportefficiencyfactor: row.storageexportefficiencyfactor.clone(),
            min_ramp_rate_up: row.min_ramp_rate_up.clone(),
            min_ramp_rate_down: row.min_ramp_rate_down.clone(),
            load_min_ramp_rate_up: row.load_min_ramp_rate_up.clone(),
            load_min_ramp_rate_down: row.load_min_ramp_rate_down.clone(),
            aggregated: row.aggregated.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDudetail7PrimaryKey {
    pub duid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDudetail7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationDudetail7Row<'data> {
    type Row<'other> = ParticipantRegistrationDudetail7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationDudetail7Row<'data> {
    type PrimaryKey = ParticipantRegistrationDudetail7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationDudetail7PrimaryKey {
    type Row<'other> = ParticipantRegistrationDudetail7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDudetail7PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDudetail7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDudetail7 {
    type Builder = ParticipantRegistrationDudetail7Builder;
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
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "voltlevel",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "registeredcapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "agccapability",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxcapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "starttype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "normallyonflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "physicaldetailsflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spinningreserveflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
                arrow::datatypes::Field::new(
                    "intermittentflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "semi_schedule_flag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxrateofchangeup",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxrateofchangedown",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchsubtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "adg_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mincapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "registeredmincapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxrateofchangeup_load",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxrateofchangedown_load",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxstoragecapacity",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "storageimportefficiencyfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "storageexportefficiencyfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "min_ramp_rate_up",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "min_ramp_rate_down",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_min_ramp_rate_up",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_min_ramp_rate_down",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregated",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationDudetail7Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            voltlevel_array: arrow::array::builder::StringBuilder::new(),
            registeredcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            agccapability_array: arrow::array::builder::StringBuilder::new(),
            dispatchtype_array: arrow::array::builder::StringBuilder::new(),
            maxcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            starttype_array: arrow::array::builder::StringBuilder::new(),
            normallyonflag_array: arrow::array::builder::StringBuilder::new(),
            physicaldetailsflag_array: arrow::array::builder::StringBuilder::new(),
            spinningreserveflag_array: arrow::array::builder::StringBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            intermittentflag_array: arrow::array::builder::StringBuilder::new(),
            semi_schedule_flag_array: arrow::array::builder::StringBuilder::new(),
            maxrateofchangeup_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            maxrateofchangedown_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            dispatchsubtype_array: arrow::array::builder::StringBuilder::new(),
            adg_id_array: arrow::array::builder::StringBuilder::new(),
            mincapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            registeredmincapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            maxrateofchangeup_load_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            maxrateofchangedown_load_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            maxstoragecapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            storageimportefficiencyfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            storageexportefficiencyfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            load_min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            load_min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            aggregated_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_option(row.connectionpointid());
        builder.voltlevel_array.append_option(row.voltlevel());
        builder
            .registeredcapacity_array
            .append_option({
                row.registeredcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.agccapability_array.append_option(row.agccapability());
        builder.dispatchtype_array.append_option(row.dispatchtype());
        builder
            .maxcapacity_array
            .append_option({
                row.maxcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.starttype_array.append_option(row.starttype());
        builder.normallyonflag_array.append_option(row.normallyonflag());
        builder.physicaldetailsflag_array.append_option(row.physicaldetailsflag());
        builder.spinningreserveflag_array.append_option(row.spinningreserveflag());
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.intermittentflag_array.append_option(row.intermittentflag());
        builder.semi_schedule_flag_array.append_option(row.semi_schedule_flag());
        builder
            .maxrateofchangeup_array
            .append_option({
                row.maxrateofchangeup
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxrateofchangedown_array
            .append_option({
                row.maxrateofchangedown
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.dispatchsubtype_array.append_option(row.dispatchsubtype());
        builder.adg_id_array.append_option(row.adg_id());
        builder
            .mincapacity_array
            .append_option({
                row.mincapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .registeredmincapacity_array
            .append_option({
                row.registeredmincapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxrateofchangeup_load_array
            .append_option({
                row.maxrateofchangeup_load
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxrateofchangedown_load_array
            .append_option({
                row.maxrateofchangedown_load
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxstoragecapacity_array
            .append_option({
                row.maxstoragecapacity
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .storageimportefficiencyfactor_array
            .append_option({
                row.storageimportefficiencyfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .storageexportefficiencyfactor_array
            .append_option({
                row.storageexportefficiencyfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .min_ramp_rate_up_array
            .append_option({
                row.min_ramp_rate_up
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .min_ramp_rate_down_array
            .append_option({
                row.min_ramp_rate_down
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .load_min_ramp_rate_up_array
            .append_option({
                row.load_min_ramp_rate_up
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .load_min_ramp_rate_down_array
            .append_option({
                row.load_min_ramp_rate_down
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .aggregated_array
            .append_option({
                row.aggregated
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.voltlevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.registeredcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.agccapability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.starttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.normallyonflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.physicaldetailsflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spinningreserveflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intermittentflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.semi_schedule_flag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxrateofchangeup_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxrateofchangedown_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchsubtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.adg_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mincapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.registeredmincapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxrateofchangeup_load_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.maxrateofchangedown_load_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxstoragecapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.storageimportefficiencyfactor_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.storageexportefficiencyfactor_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.min_ramp_rate_up_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.min_ramp_rate_down_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_min_ramp_rate_up_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_min_ramp_rate_down_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregated_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationDudetail7Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    voltlevel_array: arrow::array::builder::StringBuilder,
    registeredcapacity_array: arrow::array::builder::Decimal128Builder,
    agccapability_array: arrow::array::builder::StringBuilder,
    dispatchtype_array: arrow::array::builder::StringBuilder,
    maxcapacity_array: arrow::array::builder::Decimal128Builder,
    starttype_array: arrow::array::builder::StringBuilder,
    normallyonflag_array: arrow::array::builder::StringBuilder,
    physicaldetailsflag_array: arrow::array::builder::StringBuilder,
    spinningreserveflag_array: arrow::array::builder::StringBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    intermittentflag_array: arrow::array::builder::StringBuilder,
    semi_schedule_flag_array: arrow::array::builder::StringBuilder,
    maxrateofchangeup_array: arrow::array::builder::Decimal128Builder,
    maxrateofchangedown_array: arrow::array::builder::Decimal128Builder,
    dispatchsubtype_array: arrow::array::builder::StringBuilder,
    adg_id_array: arrow::array::builder::StringBuilder,
    mincapacity_array: arrow::array::builder::Decimal128Builder,
    registeredmincapacity_array: arrow::array::builder::Decimal128Builder,
    maxrateofchangeup_load_array: arrow::array::builder::Decimal128Builder,
    maxrateofchangedown_load_array: arrow::array::builder::Decimal128Builder,
    maxstoragecapacity_array: arrow::array::builder::Decimal128Builder,
    storageimportefficiencyfactor_array: arrow::array::builder::Decimal128Builder,
    storageexportefficiencyfactor_array: arrow::array::builder::Decimal128Builder,
    min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder,
    min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder,
    load_min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder,
    load_min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder,
    aggregated_array: arrow::array::builder::Decimal128Builder,
}
pub struct ParticipantRegistrationDudetailsummary7 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationDudetailsummary7Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationDudetailsummary7 {
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
pub struct ParticipantRegistrationDudetailsummary7Mapping([usize; 29]);
/// # Summary
///
/// ## DUDETAILSUMMARY
///
/// DUDETAILSUMMARY sets out a single summary unit table so reducing the need for participants to use the various dispatchable unit detail and owner tables to establish generating unit specific details.
///
/// * Data Set Name: Participant Registration
/// * File Name: Dudetailsummary
/// * Data Version: 7
///
/// # Description
/// DUDETAILSUMMARY is a public table, and is available to all participants.SourceDUDETAILSUMMARY updates only when registration details change.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DUID
/// * START_DATE
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationDudetailsummary7Row<'data> {
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Start date for effective record
    pub start_date: chrono::NaiveDateTime,
    /// End date for effective record
    pub end_date: chrono::NaiveDateTime,
    /// Identifies LOAD, GENERATOR or BIDIRECTIONAL. This will likely expand to more generic models as new technology types are integrated into the NEM
    pub dispatchtype: core::ops::Range<usize>,
    /// Country wide - Unique id of a connection point
    pub connectionpointid: core::ops::Range<usize>,
    /// Region identifier that unit is in
    pub regionid: core::ops::Range<usize>,
    /// Station that unit is in
    pub stationid: core::ops::Range<usize>,
    /// Participant that owns unit during effective record period
    pub participantid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Used in Bidding, Dispatch and Settlements. For Bidding and Dispatch, where the DUID is a BDU with DISPATCHTYPE of BIDIRECTIONAL, the TLF for the load component of the BDU. For Settlements, where dual TLFs apply, the primary TLF is applied to all energy (load and generation) when the Net Energy Flow of the ConnectionPointID in the interval is negative (net load).
    pub transmissionlossfactor: Option<rust_decimal::Decimal>,
    /// Unit start type. At this time restricted to Fast, Slow or Non Dispatched
    pub starttype: core::ops::Range<usize>,
    /// The distribution loss factor to the currently assigned connection point
    pub distributionlossfactor: Option<rust_decimal::Decimal>,
    /// Floored Offer/Bid Energy Price adjusted for TLF, DLF and MPF
    pub minimum_energy_price: Option<rust_decimal::Decimal>,
    /// Capped Offer/Bid Energy Price adjusted for TLF, DLF and VoLL
    pub maximum_energy_price: Option<rust_decimal::Decimal>,
    /// Scheduled status of the unit:   'SCHEDULED'   'NON-SCHEDULED'   'SEMI-SCHEDULED'
    pub schedule_type: core::ops::Range<usize>,
    /// MW/Min. Calculated Minimum Ramp Rate Up value accepted for Energy Offers or Bids with explanation
    pub min_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// MW/Min. Calculated Minimum Ramp Rate Down value accepted for Energy Offers or Bids with explanation
    pub min_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Maximum ramp up rate for Unit (Mw/min) - from DUDetail table
    pub max_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min) - from DUDetail table
    pub max_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Whether the DUID is classified as an "Aggregated Unit"under the rules. This impacts the Minimum Ramp Rate calculation
    pub is_aggregated: Option<rust_decimal::Decimal>,
    /// Additional information for DISPATCHTYPE. For DISPATCHTYPE = LOAD, subtype value is WDR for wholesale demand response units For DISPATCHTYPE = LOAD, subtype value is NULL for Scheduled Loads. For DISPATCHTYPE = GENERATOR type, subtype value is NULL.
    pub dispatchsubtype: core::ops::Range<usize>,
    /// Aggregate Dispatch Group. Group into which the DUID is aggregated for Conformance. Null if DUID not aggregated for Conformance
    pub adg_id: core::ops::Range<usize>,
    /// BDU only. Floored Offer/Bid Energy Price adjusted for TLF, DLF and MPF for energy imports
    pub load_minimum_energy_price: Option<rust_decimal::Decimal>,
    /// BDU only. Capped Offer/Bid Energy Price adjusted for TLF, DLF and VoLL for energy imports
    pub load_maximum_energy_price: Option<rust_decimal::Decimal>,
    /// BDU only. MW/Min. Calculated Minimum Ramp Rate Up value accepted for Energy Offers or Bids with explanation for energy imports
    pub load_min_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// BDU only. MW/Min. Calculated Minimum Ramp Rate Down value accepted for Energy Offers or Bids with explanation for energy imports
    pub load_min_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// BDU only. MW/Min. Registered Maximum Ramp Rate Up value accepted for Energy Offers or Bids for energy imports
    pub load_max_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// BDU only. MW/Min. Registered Maximum Ramp Rate Down value accepted for Energy Offers or Bids for energy imports
    pub load_max_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Used in Bidding, Dispatch and Settlements, only populated where Dual TLFs apply. For Bidding and Dispatch, the TLF for the generation component of a BDU, when null the TRANSMISSIONLOSSFACTOR is used for both the load and generation components. For Settlements, the secondary TLF is applied to all energy (load and generation) when the Net Energy Flow of the ConnectionPointID in the interval is positive (net generation).
    pub secondary_tlf: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationDudetailsummary7Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn dispatchtype(&self) -> Option<&str> {
        if self.dispatchtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.dispatchtype.clone(),
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
    pub fn starttype(&self) -> Option<&str> {
        if self.starttype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.starttype.clone(),
                ),
            )
        }
    }
    pub fn schedule_type(&self) -> Option<&str> {
        if self.schedule_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.schedule_type.clone(),
                ),
            )
        }
    }
    pub fn dispatchsubtype(&self) -> Option<&str> {
        if self.dispatchsubtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.dispatchsubtype.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationDudetailsummary7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "DUDETAILSUMMARY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationDudetailsummary7Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "START_DATE",
        "END_DATE",
        "DISPATCHTYPE",
        "CONNECTIONPOINTID",
        "REGIONID",
        "STATIONID",
        "PARTICIPANTID",
        "LASTCHANGED",
        "TRANSMISSIONLOSSFACTOR",
        "STARTTYPE",
        "DISTRIBUTIONLOSSFACTOR",
        "MINIMUM_ENERGY_PRICE",
        "MAXIMUM_ENERGY_PRICE",
        "SCHEDULE_TYPE",
        "MIN_RAMP_RATE_UP",
        "MIN_RAMP_RATE_DOWN",
        "MAX_RAMP_RATE_UP",
        "MAX_RAMP_RATE_DOWN",
        "IS_AGGREGATED",
        "DISPATCHSUBTYPE",
        "ADG_ID",
        "LOAD_MINIMUM_ENERGY_PRICE",
        "LOAD_MAXIMUM_ENERGY_PRICE",
        "LOAD_MIN_RAMP_RATE_UP",
        "LOAD_MIN_RAMP_RATE_DOWN",
        "LOAD_MAX_RAMP_RATE_UP",
        "LOAD_MAX_RAMP_RATE_DOWN",
        "SECONDARY_TLF",
    ];
    type Row<'row> = ParticipantRegistrationDudetailsummary7Row<'row>;
    type FieldMapping = ParticipantRegistrationDudetailsummary7Mapping;
    type PrimaryKey = ParticipantRegistrationDudetailsummary7PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationDudetailsummary7Row {
            duid: row.get_range("duid", field_mapping.0[0])?,
            start_date: row
                .get_custom_parsed_at_idx(
                    "start_date",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            end_date: row
                .get_custom_parsed_at_idx(
                    "end_date",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            dispatchtype: row.get_opt_range("dispatchtype", field_mapping.0[3])?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[4])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[5])?,
            stationid: row.get_opt_range("stationid", field_mapping.0[6])?,
            participantid: row.get_opt_range("participantid", field_mapping.0[7])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            transmissionlossfactor: row
                .get_opt_custom_parsed_at_idx(
                    "transmissionlossfactor",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            starttype: row.get_opt_range("starttype", field_mapping.0[10])?,
            distributionlossfactor: row
                .get_opt_custom_parsed_at_idx(
                    "distributionlossfactor",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            minimum_energy_price: row
                .get_opt_custom_parsed_at_idx(
                    "minimum_energy_price",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maximum_energy_price: row
                .get_opt_custom_parsed_at_idx(
                    "maximum_energy_price",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            schedule_type: row.get_opt_range("schedule_type", field_mapping.0[14])?,
            min_ramp_rate_up: row
                .get_opt_custom_parsed_at_idx(
                    "min_ramp_rate_up",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            min_ramp_rate_down: row
                .get_opt_custom_parsed_at_idx(
                    "min_ramp_rate_down",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            max_ramp_rate_up: row
                .get_opt_custom_parsed_at_idx(
                    "max_ramp_rate_up",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            max_ramp_rate_down: row
                .get_opt_custom_parsed_at_idx(
                    "max_ramp_rate_down",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            is_aggregated: row
                .get_opt_custom_parsed_at_idx(
                    "is_aggregated",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchsubtype: row.get_opt_range("dispatchsubtype", field_mapping.0[20])?,
            adg_id: row.get_opt_range("adg_id", field_mapping.0[21])?,
            load_minimum_energy_price: row
                .get_opt_custom_parsed_at_idx(
                    "load_minimum_energy_price",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_maximum_energy_price: row
                .get_opt_custom_parsed_at_idx(
                    "load_maximum_energy_price",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_min_ramp_rate_up: row
                .get_opt_custom_parsed_at_idx(
                    "load_min_ramp_rate_up",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_min_ramp_rate_down: row
                .get_opt_custom_parsed_at_idx(
                    "load_min_ramp_rate_down",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_max_ramp_rate_up: row
                .get_opt_custom_parsed_at_idx(
                    "load_max_ramp_rate_up",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_max_ramp_rate_down: row
                .get_opt_custom_parsed_at_idx(
                    "load_max_ramp_rate_down",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            secondary_tlf: row
                .get_opt_custom_parsed_at_idx(
                    "secondary_tlf",
                    field_mapping.0[28],
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
        Ok(ParticipantRegistrationDudetailsummary7Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationDudetailsummary7PrimaryKey {
        ParticipantRegistrationDudetailsummary7PrimaryKey {
            duid: row.duid().to_string(),
            start_date: row.start_date,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_dudetailsummary_v7_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationDudetailsummary7Row {
            duid: row.duid.clone(),
            start_date: row.start_date.clone(),
            end_date: row.end_date.clone(),
            dispatchtype: row.dispatchtype.clone(),
            connectionpointid: row.connectionpointid.clone(),
            regionid: row.regionid.clone(),
            stationid: row.stationid.clone(),
            participantid: row.participantid.clone(),
            lastchanged: row.lastchanged.clone(),
            transmissionlossfactor: row.transmissionlossfactor.clone(),
            starttype: row.starttype.clone(),
            distributionlossfactor: row.distributionlossfactor.clone(),
            minimum_energy_price: row.minimum_energy_price.clone(),
            maximum_energy_price: row.maximum_energy_price.clone(),
            schedule_type: row.schedule_type.clone(),
            min_ramp_rate_up: row.min_ramp_rate_up.clone(),
            min_ramp_rate_down: row.min_ramp_rate_down.clone(),
            max_ramp_rate_up: row.max_ramp_rate_up.clone(),
            max_ramp_rate_down: row.max_ramp_rate_down.clone(),
            is_aggregated: row.is_aggregated.clone(),
            dispatchsubtype: row.dispatchsubtype.clone(),
            adg_id: row.adg_id.clone(),
            load_minimum_energy_price: row.load_minimum_energy_price.clone(),
            load_maximum_energy_price: row.load_maximum_energy_price.clone(),
            load_min_ramp_rate_up: row.load_min_ramp_rate_up.clone(),
            load_min_ramp_rate_down: row.load_min_ramp_rate_down.clone(),
            load_max_ramp_rate_up: row.load_max_ramp_rate_up.clone(),
            load_max_ramp_rate_down: row.load_max_ramp_rate_down.clone(),
            secondary_tlf: row.secondary_tlf.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDudetailsummary7PrimaryKey {
    pub duid: alloc::string::String,
    pub start_date: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDudetailsummary7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationDudetailsummary7Row<'data> {
    type Row<'other> = ParticipantRegistrationDudetailsummary7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.start_date == row.start_date
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationDudetailsummary7Row<'data> {
    type PrimaryKey = ParticipantRegistrationDudetailsummary7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.start_date == key.start_date
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationDudetailsummary7PrimaryKey {
    type Row<'other> = ParticipantRegistrationDudetailsummary7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.start_date == row.start_date
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationDudetailsummary7PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDudetailsummary7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.start_date == key.start_date
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDudetailsummary7 {
    type Builder = ParticipantRegistrationDudetailsummary7Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "start_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "end_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "dispatchtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
                    "transmissionlossfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "starttype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "distributionlossfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "minimum_energy_price",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maximum_energy_price",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "schedule_type",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "min_ramp_rate_up",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "min_ramp_rate_down",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "max_ramp_rate_up",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "max_ramp_rate_down",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "is_aggregated",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchsubtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "adg_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_minimum_energy_price",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_maximum_energy_price",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_min_ramp_rate_up",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_min_ramp_rate_down",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_max_ramp_rate_up",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_max_ramp_rate_down",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "secondary_tlf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationDudetailsummary7Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            start_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            end_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            dispatchtype_array: arrow::array::builder::StringBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            transmissionlossfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            starttype_array: arrow::array::builder::StringBuilder::new(),
            distributionlossfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            minimum_energy_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            maximum_energy_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            schedule_type_array: arrow::array::builder::StringBuilder::new(),
            min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            max_ramp_rate_up_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            max_ramp_rate_down_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            is_aggregated_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            dispatchsubtype_array: arrow::array::builder::StringBuilder::new(),
            adg_id_array: arrow::array::builder::StringBuilder::new(),
            load_minimum_energy_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            load_maximum_energy_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            load_min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            load_min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            load_max_ramp_rate_up_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            load_max_ramp_rate_down_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            secondary_tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder
            .start_date_array
            .append_value(row.start_date.and_utc().timestamp_millis());
        builder.end_date_array.append_value(row.end_date.and_utc().timestamp_millis());
        builder.dispatchtype_array.append_option(row.dispatchtype());
        builder.connectionpointid_array.append_option(row.connectionpointid());
        builder.regionid_array.append_option(row.regionid());
        builder.stationid_array.append_option(row.stationid());
        builder.participantid_array.append_option(row.participantid());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .transmissionlossfactor_array
            .append_option({
                row.transmissionlossfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder.starttype_array.append_option(row.starttype());
        builder
            .distributionlossfactor_array
            .append_option({
                row.distributionlossfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .minimum_energy_price_array
            .append_option({
                row.minimum_energy_price
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .maximum_energy_price_array
            .append_option({
                row.maximum_energy_price
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder.schedule_type_array.append_option(row.schedule_type());
        builder
            .min_ramp_rate_up_array
            .append_option({
                row.min_ramp_rate_up
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .min_ramp_rate_down_array
            .append_option({
                row.min_ramp_rate_down
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .max_ramp_rate_up_array
            .append_option({
                row.max_ramp_rate_up
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .max_ramp_rate_down_array
            .append_option({
                row.max_ramp_rate_down
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .is_aggregated_array
            .append_option({
                row.is_aggregated
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.dispatchsubtype_array.append_option(row.dispatchsubtype());
        builder.adg_id_array.append_option(row.adg_id());
        builder
            .load_minimum_energy_price_array
            .append_option({
                row.load_minimum_energy_price
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .load_maximum_energy_price_array
            .append_option({
                row.load_maximum_energy_price
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .load_min_ramp_rate_up_array
            .append_option({
                row.load_min_ramp_rate_up
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .load_min_ramp_rate_down_array
            .append_option({
                row.load_min_ramp_rate_down
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .load_max_ramp_rate_up_array
            .append_option({
                row.load_max_ramp_rate_up
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .load_max_ramp_rate_down_array
            .append_option({
                row.load_max_ramp_rate_down
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .secondary_tlf_array
            .append_option({
                row.secondary_tlf
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.start_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.end_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transmissionlossfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.starttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.distributionlossfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.minimum_energy_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maximum_energy_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.schedule_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.min_ramp_rate_up_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.min_ramp_rate_down_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.max_ramp_rate_up_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.max_ramp_rate_down_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.is_aggregated_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchsubtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.adg_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.load_minimum_energy_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.load_maximum_energy_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_min_ramp_rate_up_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_min_ramp_rate_down_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_max_ramp_rate_up_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_max_ramp_rate_down_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.secondary_tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationDudetailsummary7Builder {
    duid_array: arrow::array::builder::StringBuilder,
    start_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    end_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    dispatchtype_array: arrow::array::builder::StringBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    transmissionlossfactor_array: arrow::array::builder::Decimal128Builder,
    starttype_array: arrow::array::builder::StringBuilder,
    distributionlossfactor_array: arrow::array::builder::Decimal128Builder,
    minimum_energy_price_array: arrow::array::builder::Decimal128Builder,
    maximum_energy_price_array: arrow::array::builder::Decimal128Builder,
    schedule_type_array: arrow::array::builder::StringBuilder,
    min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder,
    min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder,
    max_ramp_rate_up_array: arrow::array::builder::Decimal128Builder,
    max_ramp_rate_down_array: arrow::array::builder::Decimal128Builder,
    is_aggregated_array: arrow::array::builder::Decimal128Builder,
    dispatchsubtype_array: arrow::array::builder::StringBuilder,
    adg_id_array: arrow::array::builder::StringBuilder,
    load_minimum_energy_price_array: arrow::array::builder::Decimal128Builder,
    load_maximum_energy_price_array: arrow::array::builder::Decimal128Builder,
    load_min_ramp_rate_up_array: arrow::array::builder::Decimal128Builder,
    load_min_ramp_rate_down_array: arrow::array::builder::Decimal128Builder,
    load_max_ramp_rate_up_array: arrow::array::builder::Decimal128Builder,
    load_max_ramp_rate_down_array: arrow::array::builder::Decimal128Builder,
    secondary_tlf_array: arrow::array::builder::Decimal128Builder,
}
pub struct ParticipantRegistrationGenmeter1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationGenmeter1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationGenmeter1 {
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
pub struct ParticipantRegistrationGenmeter1Mapping([usize; 16]);
/// # Summary
///
/// ## GENMETER
///
/// GENMETER shows details of generator meter sets.
///
/// * Data Set Name: Participant Registration
/// * File Name: Genmeter
/// * Data Version: 1
///
/// # Description
/// GENMETER is a public table, and is available to all participants.SourceGENMETER updates only when meter details change.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * APPLYDATE
/// * METERID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationGenmeter1Row<'data> {
    /// Meter Id
    pub meterid: core::ops::Range<usize>,
    /// Generator Set ID
    pub gensetid: core::ops::Range<usize>,
    /// Not used
    pub connectionpointid: core::ops::Range<usize>,
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// LOAD
    pub metertype: core::ops::Range<usize>,
    /// WATT or AUXILARY
    pub meterclass: core::ops::Range<usize>,
    /// Voltage
    pub voltagelevel: Option<rust_decimal::Decimal>,
    /// Application date
    pub applydate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// AEMO user authorising
    pub authorisedby: core::ops::Range<usize>,
    /// Date authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub comdate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub decomdate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationGenmeter1Row<'data> {
    pub fn meterid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.meterid.clone())
    }
    pub fn gensetid(&self) -> Option<&str> {
        if self.gensetid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.gensetid.clone(),
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
    pub fn metertype(&self) -> Option<&str> {
        if self.metertype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.metertype.clone(),
                ),
            )
        }
    }
    pub fn meterclass(&self) -> Option<&str> {
        if self.meterclass.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.meterclass.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationGenmeter1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "GENMETER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationGenmeter1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "METERID",
        "GENSETID",
        "CONNECTIONPOINTID",
        "STATIONID",
        "METERTYPE",
        "METERCLASS",
        "VOLTAGELEVEL",
        "APPLYDATE",
        "VERSIONNO",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "COMDATE",
        "DECOMDATE",
        "ENDDATE",
        "STARTDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationGenmeter1Row<'row>;
    type FieldMapping = ParticipantRegistrationGenmeter1Mapping;
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationGenmeter1Row {
            meterid: row.get_range("meterid", field_mapping.0[0])?,
            gensetid: row.get_opt_range("gensetid", field_mapping.0[1])?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[2])?,
            stationid: row.get_opt_range("stationid", field_mapping.0[3])?,
            metertype: row.get_opt_range("metertype", field_mapping.0[4])?,
            meterclass: row.get_opt_range("meterclass", field_mapping.0[5])?,
            voltagelevel: row
                .get_opt_custom_parsed_at_idx(
                    "voltagelevel",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            applydate: row
                .get_custom_parsed_at_idx(
                    "applydate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[9])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            comdate: row
                .get_opt_custom_parsed_at_idx(
                    "comdate",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            decomdate: row
                .get_opt_custom_parsed_at_idx(
                    "decomdate",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[13],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[14],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(ParticipantRegistrationGenmeter1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationGenmeter1PrimaryKey {
        ParticipantRegistrationGenmeter1PrimaryKey {
            applydate: row.applydate,
            meterid: row.meterid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_genmeter_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationGenmeter1Row {
            meterid: row.meterid.clone(),
            gensetid: row.gensetid.clone(),
            connectionpointid: row.connectionpointid.clone(),
            stationid: row.stationid.clone(),
            metertype: row.metertype.clone(),
            meterclass: row.meterclass.clone(),
            voltagelevel: row.voltagelevel.clone(),
            applydate: row.applydate.clone(),
            versionno: row.versionno.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            comdate: row.comdate.clone(),
            decomdate: row.decomdate.clone(),
            enddate: row.enddate.clone(),
            startdate: row.startdate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationGenmeter1PrimaryKey {
    pub applydate: chrono::NaiveDateTime,
    pub meterid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationGenmeter1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationGenmeter1Row<'data> {
    type Row<'other> = ParticipantRegistrationGenmeter1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.applydate == row.applydate && self.meterid() == row.meterid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationGenmeter1Row<'data> {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applydate == key.applydate && self.meterid() == key.meterid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationGenmeter1PrimaryKey {
    type Row<'other> = ParticipantRegistrationGenmeter1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.applydate == row.applydate && self.meterid == row.meterid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenmeter1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applydate == key.applydate && self.meterid == key.meterid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationGenmeter1 {
    type Builder = ParticipantRegistrationGenmeter1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "meterid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "gensetid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "metertype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meterclass",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "voltagelevel",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "applydate",
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
                    "comdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "decomdate",
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
                    "startdate",
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
        ParticipantRegistrationGenmeter1Builder {
            meterid_array: arrow::array::builder::StringBuilder::new(),
            gensetid_array: arrow::array::builder::StringBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            metertype_array: arrow::array::builder::StringBuilder::new(),
            meterclass_array: arrow::array::builder::StringBuilder::new(),
            voltagelevel_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            applydate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            comdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            decomdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.meterid_array.append_value(row.meterid());
        builder.gensetid_array.append_option(row.gensetid());
        builder.connectionpointid_array.append_option(row.connectionpointid());
        builder.stationid_array.append_option(row.stationid());
        builder.metertype_array.append_option(row.metertype());
        builder.meterclass_array.append_option(row.meterclass());
        builder
            .voltagelevel_array
            .append_option({
                row.voltagelevel
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.applydate_array.append_value(row.applydate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .comdate_array
            .append_option(row.comdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .decomdate_array
            .append_option(row.decomdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.meterid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gensetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.metertype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meterclass_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.voltagelevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.applydate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.comdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.decomdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationGenmeter1Builder {
    meterid_array: arrow::array::builder::StringBuilder,
    gensetid_array: arrow::array::builder::StringBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    metertype_array: arrow::array::builder::StringBuilder,
    meterclass_array: arrow::array::builder::StringBuilder,
    voltagelevel_array: arrow::array::builder::Decimal128Builder,
    applydate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    comdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    decomdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationGenunits3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationGenunits3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationGenunits3 {
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
pub struct ParticipantRegistrationGenunits3Mapping([usize; 22]);
/// # Summary
///
/// ## GENUNITS
///
/// GENUNITS shows Genset details for each physical unit with the relevant station.
///
/// * Data Set Name: Participant Registration
/// * File Name: Genunits
/// * Data Version: 3
///
/// # Description
/// GENUNITS is a public table, and is available to all participants.SourceGENUNITS updates whenever plant details change.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * GENSETID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationGenunits3Row<'data> {
    /// Physical Unit identifier
    pub gensetid: core::ops::Range<usize>,
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// Not used
    pub setlossfactor: Option<rust_decimal::Decimal>,
    /// Centrally dispatched Indicator
    pub cdindicator: core::ops::Range<usize>,
    /// AGC Available flag
    pub agcflag: core::ops::Range<usize>,
    /// Not used
    pub spinningflag: core::ops::Range<usize>,
    /// Voltage level
    pub voltlevel: Option<rust_decimal::Decimal>,
    /// Registered capacity
    pub registeredcapacity: Option<rust_decimal::Decimal>,
    /// Identifies LOAD, GENERATOR or BIDIRECTIONAL. This will likely expand to more generic models as new technology types are integrated into the NEM.
    pub dispatchtype: core::ops::Range<usize>,
    /// Fast / Slow / Not Dispatched
    pub starttype: core::ops::Range<usize>,
    /// Market Generator Indicator Flag
    pub mktgeneratorind: core::ops::Range<usize>,
    /// On / Off for load
    pub normalstatus: core::ops::Range<usize>,
    /// Maximum capacity
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Genset type
    pub gensettype: core::ops::Range<usize>,
    /// Genset name
    pub gensetname: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The emissions factor for the generating unit, as calculated by Settlements staff members
    pub co2e_emissions_factor: Option<rust_decimal::Decimal>,
    /// The energy source for the generating unit, as used in the calculation of the CO2-e emissions factor.  Distinct from the Energy Source for a generating unit published as part of the Registration Master List
    pub co2e_energy_source: core::ops::Range<usize>,
    /// An indicator as to the source of the emission factor used in the calculation of the index. The applicable values for this field would be NTNDP which indicates the emission factor is quoted from the National Transmission Network Development Plan or Estimated to indicate the emission factor has been calculated using an internal AEMO procedure based upon the Department of Climate Change and Energy Efficiency NGA factors
    pub co2e_data_source: core::ops::Range<usize>,
    /// Minimum capacity only for load side of BDU, otherwise 0 (MW)
    pub mincapacity: Option<rust_decimal::Decimal>,
    /// Registered minimum capacity only for load side of BDU, otherwise 0 (MW)
    pub registeredmincapacity: Option<rust_decimal::Decimal>,
    /// The rated storage capacity (MWh), information only
    pub maxstoragecapacity: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationGenunits3Row<'data> {
    pub fn gensetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.gensetid.clone())
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
    pub fn cdindicator(&self) -> Option<&str> {
        if self.cdindicator.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.cdindicator.clone(),
                ),
            )
        }
    }
    pub fn agcflag(&self) -> Option<&str> {
        if self.agcflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.agcflag.clone(),
                ),
            )
        }
    }
    pub fn spinningflag(&self) -> Option<&str> {
        if self.spinningflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spinningflag.clone(),
                ),
            )
        }
    }
    pub fn dispatchtype(&self) -> Option<&str> {
        if self.dispatchtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.dispatchtype.clone(),
                ),
            )
        }
    }
    pub fn starttype(&self) -> Option<&str> {
        if self.starttype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.starttype.clone(),
                ),
            )
        }
    }
    pub fn mktgeneratorind(&self) -> Option<&str> {
        if self.mktgeneratorind.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.mktgeneratorind.clone(),
                ),
            )
        }
    }
    pub fn normalstatus(&self) -> Option<&str> {
        if self.normalstatus.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.normalstatus.clone(),
                ),
            )
        }
    }
    pub fn gensettype(&self) -> Option<&str> {
        if self.gensettype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.gensettype.clone(),
                ),
            )
        }
    }
    pub fn gensetname(&self) -> Option<&str> {
        if self.gensetname.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.gensetname.clone(),
                ),
            )
        }
    }
    pub fn co2e_energy_source(&self) -> Option<&str> {
        if self.co2e_energy_source.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.co2e_energy_source.clone(),
                ),
            )
        }
    }
    pub fn co2e_data_source(&self) -> Option<&str> {
        if self.co2e_data_source.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.co2e_data_source.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationGenunits3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "GENUNITS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationGenunits3Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "GENSETID",
        "STATIONID",
        "SETLOSSFACTOR",
        "CDINDICATOR",
        "AGCFLAG",
        "SPINNINGFLAG",
        "VOLTLEVEL",
        "REGISTEREDCAPACITY",
        "DISPATCHTYPE",
        "STARTTYPE",
        "MKTGENERATORIND",
        "NORMALSTATUS",
        "MAXCAPACITY",
        "GENSETTYPE",
        "GENSETNAME",
        "LASTCHANGED",
        "CO2E_EMISSIONS_FACTOR",
        "CO2E_ENERGY_SOURCE",
        "CO2E_DATA_SOURCE",
        "MINCAPACITY",
        "REGISTEREDMINCAPACITY",
        "MAXSTORAGECAPACITY",
    ];
    type Row<'row> = ParticipantRegistrationGenunits3Row<'row>;
    type FieldMapping = ParticipantRegistrationGenunits3Mapping;
    type PrimaryKey = ParticipantRegistrationGenunits3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationGenunits3Row {
            gensetid: row.get_range("gensetid", field_mapping.0[0])?,
            stationid: row.get_opt_range("stationid", field_mapping.0[1])?,
            setlossfactor: row
                .get_opt_custom_parsed_at_idx(
                    "setlossfactor",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cdindicator: row.get_opt_range("cdindicator", field_mapping.0[3])?,
            agcflag: row.get_opt_range("agcflag", field_mapping.0[4])?,
            spinningflag: row.get_opt_range("spinningflag", field_mapping.0[5])?,
            voltlevel: row
                .get_opt_custom_parsed_at_idx(
                    "voltlevel",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            registeredcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "registeredcapacity",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchtype: row.get_opt_range("dispatchtype", field_mapping.0[8])?,
            starttype: row.get_opt_range("starttype", field_mapping.0[9])?,
            mktgeneratorind: row.get_opt_range("mktgeneratorind", field_mapping.0[10])?,
            normalstatus: row.get_opt_range("normalstatus", field_mapping.0[11])?,
            maxcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxcapacity",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            gensettype: row.get_opt_range("gensettype", field_mapping.0[13])?,
            gensetname: row.get_opt_range("gensetname", field_mapping.0[14])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[15],
                    mmsdm_core::mms_datetime::parse,
                )?,
            co2e_emissions_factor: row
                .get_opt_custom_parsed_at_idx(
                    "co2e_emissions_factor",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            co2e_energy_source: row
                .get_opt_range("co2e_energy_source", field_mapping.0[17])?,
            co2e_data_source: row
                .get_opt_range("co2e_data_source", field_mapping.0[18])?,
            mincapacity: row
                .get_opt_custom_parsed_at_idx(
                    "mincapacity",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            registeredmincapacity: row
                .get_opt_custom_parsed_at_idx(
                    "registeredmincapacity",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxstoragecapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxstoragecapacity",
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
        Ok(ParticipantRegistrationGenunits3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationGenunits3PrimaryKey {
        ParticipantRegistrationGenunits3PrimaryKey {
            gensetid: row.gensetid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_genunits_v3_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationGenunits3Row {
            gensetid: row.gensetid.clone(),
            stationid: row.stationid.clone(),
            setlossfactor: row.setlossfactor.clone(),
            cdindicator: row.cdindicator.clone(),
            agcflag: row.agcflag.clone(),
            spinningflag: row.spinningflag.clone(),
            voltlevel: row.voltlevel.clone(),
            registeredcapacity: row.registeredcapacity.clone(),
            dispatchtype: row.dispatchtype.clone(),
            starttype: row.starttype.clone(),
            mktgeneratorind: row.mktgeneratorind.clone(),
            normalstatus: row.normalstatus.clone(),
            maxcapacity: row.maxcapacity.clone(),
            gensettype: row.gensettype.clone(),
            gensetname: row.gensetname.clone(),
            lastchanged: row.lastchanged.clone(),
            co2e_emissions_factor: row.co2e_emissions_factor.clone(),
            co2e_energy_source: row.co2e_energy_source.clone(),
            co2e_data_source: row.co2e_data_source.clone(),
            mincapacity: row.mincapacity.clone(),
            registeredmincapacity: row.registeredmincapacity.clone(),
            maxstoragecapacity: row.maxstoragecapacity.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationGenunits3PrimaryKey {
    pub gensetid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationGenunits3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationGenunits3Row<'data> {
    type Row<'other> = ParticipantRegistrationGenunits3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.gensetid() == row.gensetid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationGenunits3Row<'data> {
    type PrimaryKey = ParticipantRegistrationGenunits3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.gensetid() == key.gensetid
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationGenunits3PrimaryKey {
    type Row<'other> = ParticipantRegistrationGenunits3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.gensetid == row.gensetid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenunits3PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenunits3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.gensetid == key.gensetid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationGenunits3 {
    type Builder = ParticipantRegistrationGenunits3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "gensetid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "setlossfactor",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cdindicator",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "agcflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spinningflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "voltlevel",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "registeredcapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dispatchtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "starttype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mktgeneratorind",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "normalstatus",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxcapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gensettype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gensetname",
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
                    "co2e_emissions_factor",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "co2e_energy_source",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "co2e_data_source",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mincapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "registeredmincapacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxstoragecapacity",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationGenunits3Builder {
            gensetid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            setlossfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            cdindicator_array: arrow::array::builder::StringBuilder::new(),
            agcflag_array: arrow::array::builder::StringBuilder::new(),
            spinningflag_array: arrow::array::builder::StringBuilder::new(),
            voltlevel_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            registeredcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            dispatchtype_array: arrow::array::builder::StringBuilder::new(),
            starttype_array: arrow::array::builder::StringBuilder::new(),
            mktgeneratorind_array: arrow::array::builder::StringBuilder::new(),
            normalstatus_array: arrow::array::builder::StringBuilder::new(),
            maxcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            gensettype_array: arrow::array::builder::StringBuilder::new(),
            gensetname_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            co2e_emissions_factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            co2e_energy_source_array: arrow::array::builder::StringBuilder::new(),
            co2e_data_source_array: arrow::array::builder::StringBuilder::new(),
            mincapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            registeredmincapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            maxstoragecapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.gensetid_array.append_value(row.gensetid());
        builder.stationid_array.append_option(row.stationid());
        builder
            .setlossfactor_array
            .append_option({
                row.setlossfactor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.cdindicator_array.append_option(row.cdindicator());
        builder.agcflag_array.append_option(row.agcflag());
        builder.spinningflag_array.append_option(row.spinningflag());
        builder
            .voltlevel_array
            .append_option({
                row.voltlevel
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .registeredcapacity_array
            .append_option({
                row.registeredcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.dispatchtype_array.append_option(row.dispatchtype());
        builder.starttype_array.append_option(row.starttype());
        builder.mktgeneratorind_array.append_option(row.mktgeneratorind());
        builder.normalstatus_array.append_option(row.normalstatus());
        builder
            .maxcapacity_array
            .append_option({
                row.maxcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.gensettype_array.append_option(row.gensettype());
        builder.gensetname_array.append_option(row.gensetname());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .co2e_emissions_factor_array
            .append_option({
                row.co2e_emissions_factor
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder.co2e_energy_source_array.append_option(row.co2e_energy_source());
        builder.co2e_data_source_array.append_option(row.co2e_data_source());
        builder
            .mincapacity_array
            .append_option({
                row.mincapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .registeredmincapacity_array
            .append_option({
                row.registeredmincapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxstoragecapacity_array
            .append_option({
                row.maxstoragecapacity
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
                    alloc::sync::Arc::new(builder.gensetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.setlossfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cdindicator_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.agcflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spinningflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.voltlevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.registeredcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.starttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mktgeneratorind_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.normalstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gensettype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gensetname_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.co2e_emissions_factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.co2e_energy_source_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.co2e_data_source_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mincapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.registeredmincapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxstoragecapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationGenunits3Builder {
    gensetid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    setlossfactor_array: arrow::array::builder::Decimal128Builder,
    cdindicator_array: arrow::array::builder::StringBuilder,
    agcflag_array: arrow::array::builder::StringBuilder,
    spinningflag_array: arrow::array::builder::StringBuilder,
    voltlevel_array: arrow::array::builder::Decimal128Builder,
    registeredcapacity_array: arrow::array::builder::Decimal128Builder,
    dispatchtype_array: arrow::array::builder::StringBuilder,
    starttype_array: arrow::array::builder::StringBuilder,
    mktgeneratorind_array: arrow::array::builder::StringBuilder,
    normalstatus_array: arrow::array::builder::StringBuilder,
    maxcapacity_array: arrow::array::builder::Decimal128Builder,
    gensettype_array: arrow::array::builder::StringBuilder,
    gensetname_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    co2e_emissions_factor_array: arrow::array::builder::Decimal128Builder,
    co2e_energy_source_array: arrow::array::builder::StringBuilder,
    co2e_data_source_array: arrow::array::builder::StringBuilder,
    mincapacity_array: arrow::array::builder::Decimal128Builder,
    registeredmincapacity_array: arrow::array::builder::Decimal128Builder,
    maxstoragecapacity_array: arrow::array::builder::Decimal128Builder,
}
pub struct ParticipantRegistrationGenunitsUnit2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationGenunitsUnit2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationGenunitsUnit2 {
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
pub struct ParticipantRegistrationGenunitsUnit2Mapping([usize; 13]);
/// # Summary
///
/// ## GENUNITS_UNIT
///
/// Physical units within a Gen Unit Set
///
/// * Data Set Name: Participant Registration
/// * File Name: Genunits Unit
/// * Data Version: 2
///
/// # Description
/// MNSP_INTERCONNECTOR data is public, so is available to all participants.SourceMNSP_INTERCONNECTOR changes infrequently, typically annually.VolumeTwice the number of MNSPs.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENSETID
/// * UNIT_GROUPING_LABEL
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationGenunitsUnit2Row<'data> {
    /// System wide unique Generating Set ID
    pub gensetid: core::ops::Range<usize>,
    /// Effective Date of this detail record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    pub versionno: rust_decimal::Decimal,
    /// Label of Physical Units within the station
    pub unit_grouping_label: core::ops::Range<usize>,
    /// Number of units in this Gen Unit grouping
    pub unit_count: Option<rust_decimal::Decimal>,
    /// Nameplate Capacity for each unit in this grouping
    pub unit_size: Option<rust_decimal::Decimal>,
    /// Maximum Capacity for each unit in this grouping
    pub unit_max_size: Option<rust_decimal::Decimal>,
    /// Deprecated as this flag is moved to DUDETAIL table with IESS release.
    pub aggregation_flag: Option<rust_decimal::Decimal>,
    /// Date/Time when record was changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Only applicable for the LOAD side of BDU (MW)
    pub unitminsize: Option<rust_decimal::Decimal>,
    /// The rated storage capacity (MWh), information only
    pub maxstoragecapacity: Option<rust_decimal::Decimal>,
    /// Registered capacity for normal operations
    pub registeredcapacity: Option<rust_decimal::Decimal>,
    /// Only applicable for the LOAD side of BDU (MW)
    pub registeredmincapacity: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationGenunitsUnit2Row<'data> {
    pub fn gensetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.gensetid.clone())
    }
    pub fn unit_grouping_label(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.unit_grouping_label.clone(),
        )
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationGenunitsUnit2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "GENUNITS_UNIT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationGenunitsUnit2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "GENSETID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "UNIT_GROUPING_LABEL",
        "UNIT_COUNT",
        "UNIT_SIZE",
        "UNIT_MAX_SIZE",
        "AGGREGATION_FLAG",
        "LASTCHANGED",
        "UNITMINSIZE",
        "MAXSTORAGECAPACITY",
        "REGISTEREDCAPACITY",
        "REGISTEREDMINCAPACITY",
    ];
    type Row<'row> = ParticipantRegistrationGenunitsUnit2Row<'row>;
    type FieldMapping = ParticipantRegistrationGenunitsUnit2Mapping;
    type PrimaryKey = ParticipantRegistrationGenunitsUnit2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationGenunitsUnit2Row {
            gensetid: row.get_range("gensetid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unit_grouping_label: row
                .get_range("unit_grouping_label", field_mapping.0[3])?,
            unit_count: row
                .get_opt_custom_parsed_at_idx(
                    "unit_count",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unit_size: row
                .get_opt_custom_parsed_at_idx(
                    "unit_size",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unit_max_size: row
                .get_opt_custom_parsed_at_idx(
                    "unit_max_size",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregation_flag: row
                .get_opt_custom_parsed_at_idx(
                    "aggregation_flag",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            unitminsize: row
                .get_opt_custom_parsed_at_idx(
                    "unitminsize",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxstoragecapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxstoragecapacity",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            registeredcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "registeredcapacity",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            registeredmincapacity: row
                .get_opt_custom_parsed_at_idx(
                    "registeredmincapacity",
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
        Ok(ParticipantRegistrationGenunitsUnit2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationGenunitsUnit2PrimaryKey {
        ParticipantRegistrationGenunitsUnit2PrimaryKey {
            effectivedate: row.effectivedate,
            gensetid: row.gensetid().to_string(),
            unit_grouping_label: row.unit_grouping_label().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_genunits_unit_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationGenunitsUnit2Row {
            gensetid: row.gensetid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            unit_grouping_label: row.unit_grouping_label.clone(),
            unit_count: row.unit_count.clone(),
            unit_size: row.unit_size.clone(),
            unit_max_size: row.unit_max_size.clone(),
            aggregation_flag: row.aggregation_flag.clone(),
            lastchanged: row.lastchanged.clone(),
            unitminsize: row.unitminsize.clone(),
            maxstoragecapacity: row.maxstoragecapacity.clone(),
            registeredcapacity: row.registeredcapacity.clone(),
            registeredmincapacity: row.registeredmincapacity.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationGenunitsUnit2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub gensetid: alloc::string::String,
    pub unit_grouping_label: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationGenunitsUnit2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationGenunitsUnit2Row<'data> {
    type Row<'other> = ParticipantRegistrationGenunitsUnit2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.gensetid() == row.gensetid()
            && self.unit_grouping_label() == row.unit_grouping_label()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationGenunitsUnit2Row<'data> {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.gensetid() == key.gensetid
            && self.unit_grouping_label() == key.unit_grouping_label
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationGenunitsUnit2PrimaryKey {
    type Row<'other> = ParticipantRegistrationGenunitsUnit2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.gensetid == row.gensetid()
            && self.unit_grouping_label == row.unit_grouping_label()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationGenunitsUnit2PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.gensetid == key.gensetid
            && self.unit_grouping_label == key.unit_grouping_label
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationGenunitsUnit2 {
    type Builder = ParticipantRegistrationGenunitsUnit2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "gensetid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
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
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "unit_grouping_label",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "unit_count",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unit_size",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unit_max_size",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregation_flag",
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
                arrow::datatypes::Field::new(
                    "unitminsize",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxstoragecapacity",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "registeredcapacity",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "registeredmincapacity",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationGenunitsUnit2Builder {
            gensetid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            unit_grouping_label_array: arrow::array::builder::StringBuilder::new(),
            unit_count_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            unit_size_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            unit_max_size_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            aggregation_flag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            unitminsize_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            maxstoragecapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            registeredcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            registeredmincapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.gensetid_array.append_value(row.gensetid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.unit_grouping_label_array.append_value(row.unit_grouping_label());
        builder
            .unit_count_array
            .append_option({
                row.unit_count
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .unit_size_array
            .append_option({
                row.unit_size
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .unit_max_size_array
            .append_option({
                row.unit_max_size
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .aggregation_flag_array
            .append_option({
                row.aggregation_flag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .unitminsize_array
            .append_option({
                row.unitminsize
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .maxstoragecapacity_array
            .append_option({
                row.maxstoragecapacity
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .registeredcapacity_array
            .append_option({
                row.registeredcapacity
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .registeredmincapacity_array
            .append_option({
                row.registeredmincapacity
                    .map(|mut val| {
                        val.rescale(3);
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
                    alloc::sync::Arc::new(builder.gensetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unit_grouping_label_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unit_count_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unit_size_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unit_max_size_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregation_flag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitminsize_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxstoragecapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.registeredcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.registeredmincapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationGenunitsUnit2Builder {
    gensetid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    unit_grouping_label_array: arrow::array::builder::StringBuilder,
    unit_count_array: arrow::array::builder::Decimal128Builder,
    unit_size_array: arrow::array::builder::Decimal128Builder,
    unit_max_size_array: arrow::array::builder::Decimal128Builder,
    aggregation_flag_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    unitminsize_array: arrow::array::builder::Decimal128Builder,
    maxstoragecapacity_array: arrow::array::builder::Decimal128Builder,
    registeredcapacity_array: arrow::array::builder::Decimal128Builder,
    registeredmincapacity_array: arrow::array::builder::Decimal128Builder,
}
pub struct ParticipantRegistrationMnspInterconnector2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationMnspInterconnector2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationMnspInterconnector2 {
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
pub struct ParticipantRegistrationMnspInterconnector2Mapping([usize; 15]);
/// # Summary
///
/// ## MNSP_INTERCONNECTOR
///
/// MNSP_INTERCONNECTOR sets out attributes of each interconnector.
///
/// * Data Set Name: Participant Registration
/// * File Name: Mnsp Interconnector
/// * Data Version: 2
///
/// # Description
/// MNSP_INTERCONNECTOR data is public, so is available to all participants.SourceMNSP_INTERCONNECTOR changes infrequently, typically annually.VolumeTwice the number of MNSPs.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * LINKID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationMnspInterconnector2Row<'data> {
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: core::ops::Range<usize>,
    /// Date when Interconnector becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregion: core::ops::Range<usize>,
    /// Nominated destination region for Interconnector
    pub toregion: core::ops::Range<usize>,
    /// Maximum capacity
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor (redundant from May 2012)
    pub tlf: Option<rust_decimal::Decimal>,
    /// Factor applied to the LHS of constraint equations; set by AEMO
    pub lhsfactor: Option<rust_decimal::Decimal>,
    /// Obsolete; no longer applied. Ignore.
    pub meterflowconstant: Option<rust_decimal::Decimal>,
    /// Date of authorisation. Nominal date but required to enable Interconnector.
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer
    pub authorisedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Transmission Loss Factor for Link "From Region"end
    pub from_region_tlf: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor for Link at "To Region"end
    pub to_region_tlf: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationMnspInterconnector2Row<'data> {
    pub fn linkid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.linkid.clone())
    }
    pub fn interconnectorid(&self) -> Option<&str> {
        if self.interconnectorid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.interconnectorid.clone(),
                ),
            )
        }
    }
    pub fn fromregion(&self) -> Option<&str> {
        if self.fromregion.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.fromregion.clone(),
                ),
            )
        }
    }
    pub fn toregion(&self) -> Option<&str> {
        if self.toregion.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.toregion.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationMnspInterconnector2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "MNSP_INTERCONNECTOR";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationMnspInterconnector2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "LINKID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "INTERCONNECTORID",
        "FROMREGION",
        "TOREGION",
        "MAXCAPACITY",
        "TLF",
        "LHSFACTOR",
        "METERFLOWCONSTANT",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
        "FROM_REGION_TLF",
        "TO_REGION_TLF",
    ];
    type Row<'row> = ParticipantRegistrationMnspInterconnector2Row<'row>;
    type FieldMapping = ParticipantRegistrationMnspInterconnector2Mapping;
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationMnspInterconnector2Row {
            linkid: row.get_range("linkid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_opt_range("interconnectorid", field_mapping.0[3])?,
            fromregion: row.get_opt_range("fromregion", field_mapping.0[4])?,
            toregion: row.get_opt_range("toregion", field_mapping.0[5])?,
            maxcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxcapacity",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lhsfactor: row
                .get_opt_custom_parsed_at_idx(
                    "lhsfactor",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meterflowconstant: row
                .get_opt_custom_parsed_at_idx(
                    "meterflowconstant",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[11])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            from_region_tlf: row
                .get_opt_custom_parsed_at_idx(
                    "from_region_tlf",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            to_region_tlf: row
                .get_opt_custom_parsed_at_idx(
                    "to_region_tlf",
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
        Ok(ParticipantRegistrationMnspInterconnector2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationMnspInterconnector2PrimaryKey {
        ParticipantRegistrationMnspInterconnector2PrimaryKey {
            effectivedate: row.effectivedate,
            linkid: row.linkid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_mnsp_interconnector_v2_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationMnspInterconnector2Row {
            linkid: row.linkid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregion: row.fromregion.clone(),
            toregion: row.toregion.clone(),
            maxcapacity: row.maxcapacity.clone(),
            tlf: row.tlf.clone(),
            lhsfactor: row.lhsfactor.clone(),
            meterflowconstant: row.meterflowconstant.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            from_region_tlf: row.from_region_tlf.clone(),
            to_region_tlf: row.to_region_tlf.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationMnspInterconnector2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub linkid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationMnspInterconnector2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationMnspInterconnector2Row<'data> {
    type Row<'other> = ParticipantRegistrationMnspInterconnector2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.linkid() == row.linkid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationMnspInterconnector2Row<'data> {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.linkid() == key.linkid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationMnspInterconnector2PrimaryKey {
    type Row<'other> = ParticipantRegistrationMnspInterconnector2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.linkid == row.linkid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationMnspInterconnector2PrimaryKey {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.linkid == key.linkid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationMnspInterconnector2 {
    type Builder = ParticipantRegistrationMnspInterconnector2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "linkid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
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
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fromregion",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "toregion",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxcapacity",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(12, 7),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lhsfactor",
                    arrow::datatypes::DataType::Decimal128(12, 7),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meterflowconstant",
                    arrow::datatypes::DataType::Decimal128(12, 7),
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
                arrow::datatypes::Field::new(
                    "from_region_tlf",
                    arrow::datatypes::DataType::Decimal128(12, 7),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "to_region_tlf",
                    arrow::datatypes::DataType::Decimal128(12, 7),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationMnspInterconnector2Builder {
            linkid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregion_array: arrow::array::builder::StringBuilder::new(),
            toregion_array: arrow::array::builder::StringBuilder::new(),
            maxcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 7)),
            lhsfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 7)),
            meterflowconstant_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 7)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            from_region_tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 7)),
            to_region_tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 7)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.linkid_array.append_value(row.linkid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_option(row.interconnectorid());
        builder.fromregion_array.append_option(row.fromregion());
        builder.toregion_array.append_option(row.toregion());
        builder
            .maxcapacity_array
            .append_option({
                row.maxcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(7);
                        val.mantissa()
                    })
            });
        builder
            .lhsfactor_array
            .append_option({
                row.lhsfactor
                    .map(|mut val| {
                        val.rescale(7);
                        val.mantissa()
                    })
            });
        builder
            .meterflowconstant_array
            .append_option({
                row.meterflowconstant
                    .map(|mut val| {
                        val.rescale(7);
                        val.mantissa()
                    })
            });
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .from_region_tlf_array
            .append_option({
                row.from_region_tlf
                    .map(|mut val| {
                        val.rescale(7);
                        val.mantissa()
                    })
            });
        builder
            .to_region_tlf_array
            .append_option({
                row.to_region_tlf
                    .map(|mut val| {
                        val.rescale(7);
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
                    alloc::sync::Arc::new(builder.linkid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.toregion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lhsfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meterflowconstant_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.from_region_tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.to_region_tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationMnspInterconnector2Builder {
    linkid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregion_array: arrow::array::builder::StringBuilder,
    toregion_array: arrow::array::builder::StringBuilder,
    maxcapacity_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    lhsfactor_array: arrow::array::builder::Decimal128Builder,
    meterflowconstant_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    from_region_tlf_array: arrow::array::builder::Decimal128Builder,
    to_region_tlf_array: arrow::array::builder::Decimal128Builder,
}
pub struct ParticipantRegistrationMnspParticipant1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationMnspParticipant1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationMnspParticipant1 {
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
pub struct ParticipantRegistrationMnspParticipant1Mapping([usize; 5]);
/// # Summary
///
/// ## MNSP_PARTICIPANT
///
/// MNSP_PARTICIPANT registers MNSP ownership.
///
/// * Data Set Name: Participant Registration
/// * File Name: Mnsp Participant
/// * Data Version: 1
///
/// # Description
/// MNSP_PARTICIPANT data is public, so is available to all participants.SourceMNSP_PARTICIPANT updates infrequently, typically annually.VolumeNumber of MNSPs.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationMnspParticipant1Row<'data> {
    /// Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Calendar date when Interconnector ownership becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationMnspParticipant1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationMnspParticipant1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "MNSP_PARTICIPANT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationMnspParticipant1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERCONNECTORID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationMnspParticipant1Row<'row>;
    type FieldMapping = ParticipantRegistrationMnspParticipant1Mapping;
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationMnspParticipant1Row {
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
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
        Ok(ParticipantRegistrationMnspParticipant1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationMnspParticipant1PrimaryKey {
        ParticipantRegistrationMnspParticipant1PrimaryKey {
            effectivedate: row.effectivedate,
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_mnsp_participant_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationMnspParticipant1Row {
            interconnectorid: row.interconnectorid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationMnspParticipant1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationMnspParticipant1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationMnspParticipant1Row<'data> {
    type Row<'other> = ParticipantRegistrationMnspParticipant1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationMnspParticipant1Row<'data> {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationMnspParticipant1PrimaryKey {
    type Row<'other> = ParticipantRegistrationMnspParticipant1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationMnspParticipant1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationMnspParticipant1 {
    type Builder = ParticipantRegistrationMnspParticipant1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
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
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationMnspParticipant1Builder {
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
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
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationMnspParticipant1Builder {
    interconnectorid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationParticipant1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationParticipant1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationParticipant1 {
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
pub struct ParticipantRegistrationParticipant1Mapping([usize; 7]);
/// # Summary
///
/// ## PARTICIPANT
///
/// PARTICIPANT sets out Participant ID, name and class for all participants.
///
/// * Data Set Name: Participant Registration
/// * File Name: Participant
/// * Data Version: 1
///
/// # Description
/// PARTICIPANT is public data, so is available to all participants.SourcePARTICIPANT updates as new participants register or existing participants change details.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationParticipant1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Class of participant
    pub participantclassid: core::ops::Range<usize>,
    /// Full name of participant
    pub name: core::ops::Range<usize>,
    /// Not used
    pub description: core::ops::Range<usize>,
    /// Australian Company Number; Nine Numbers XXX-XXX-XXX
    pub acn: core::ops::Range<usize>,
    /// Identifies primary business activity of participant
    pub primarybusiness: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationParticipant1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn participantclassid(&self) -> Option<&str> {
        if self.participantclassid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.participantclassid.clone(),
                ),
            )
        }
    }
    pub fn name(&self) -> Option<&str> {
        if self.name.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.name.clone()),
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
    pub fn acn(&self) -> Option<&str> {
        if self.acn.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.acn.clone()))
        }
    }
    pub fn primarybusiness(&self) -> Option<&str> {
        if self.primarybusiness.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.primarybusiness.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipant1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PARTICIPANT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationParticipant1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "PARTICIPANTCLASSID",
        "NAME",
        "DESCRIPTION",
        "ACN",
        "PRIMARYBUSINESS",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationParticipant1Row<'row>;
    type FieldMapping = ParticipantRegistrationParticipant1Mapping;
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationParticipant1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            participantclassid: row
                .get_opt_range("participantclassid", field_mapping.0[1])?,
            name: row.get_opt_range("name", field_mapping.0[2])?,
            description: row.get_opt_range("description", field_mapping.0[3])?,
            acn: row.get_opt_range("acn", field_mapping.0[4])?,
            primarybusiness: row.get_opt_range("primarybusiness", field_mapping.0[5])?,
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
        Ok(ParticipantRegistrationParticipant1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationParticipant1PrimaryKey {
        ParticipantRegistrationParticipant1PrimaryKey {
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_participant_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationParticipant1Row {
            participantid: row.participantid.clone(),
            participantclassid: row.participantclassid.clone(),
            name: row.name.clone(),
            description: row.description.clone(),
            acn: row.acn.clone(),
            primarybusiness: row.primarybusiness.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipant1PrimaryKey {
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipant1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipant1Row<'data> {
    type Row<'other> = ParticipantRegistrationParticipant1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipant1Row<'data> {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipant1PrimaryKey {
    type Row<'other> = ParticipantRegistrationParticipant1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipant1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipant1 {
    type Builder = ParticipantRegistrationParticipant1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantclassid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "name",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "acn",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "primarybusiness",
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
        ParticipantRegistrationParticipant1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            participantclassid_array: arrow::array::builder::StringBuilder::new(),
            name_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            acn_array: arrow::array::builder::StringBuilder::new(),
            primarybusiness_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
        builder.participantclassid_array.append_option(row.participantclassid());
        builder.name_array.append_option(row.name());
        builder.description_array.append_option(row.description());
        builder.acn_array.append_option(row.acn());
        builder.primarybusiness_array.append_option(row.primarybusiness());
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
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantclassid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.name_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.acn_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.primarybusiness_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationParticipant1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    participantclassid_array: arrow::array::builder::StringBuilder,
    name_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    acn_array: arrow::array::builder::StringBuilder,
    primarybusiness_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationParticipantaccount1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationParticipantaccount1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationParticipantaccount1 {
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
pub struct ParticipantRegistrationParticipantaccount1Mapping([usize; 15]);
/// # Summary
///
/// ## PARTICIPANTACCOUNT
///
/// PARTICIPANTACCOUNT shows financial details on participants.
///
/// * Data Set Name: Participant Registration
/// * File Name: Participantaccount
/// * Data Version: 1
///
/// # Description
/// PARTICIPANTACCOUNT data is confidential to the relevant participant.SourcePARTICIPANTACCOUNT updates as new participants register or existing participants change details.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationParticipantaccount1Row<'data> {
    /// Name of the account
    pub accountname: core::ops::Range<usize>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Account number
    pub accountnumber: core::ops::Range<usize>,
    /// Bank name
    pub bankname: core::ops::Range<usize>,
    /// Bank number
    pub banknumber: Option<rust_decimal::Decimal>,
    /// Branch name
    pub branchname: core::ops::Range<usize>,
    /// Branch number
    pub branchnumber: Option<rust_decimal::Decimal>,
    /// BSB number
    pub bsbnumber: core::ops::Range<usize>,
    /// AEMO credit account number
    pub nemmcocreditaccountnumber: Option<rust_decimal::Decimal>,
    /// AEMO debit account number
    pub nemmcodebitaccountnumber: Option<rust_decimal::Decimal>,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Date record authorised
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Australian Business Number
    pub abn: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationParticipantaccount1Row<'data> {
    pub fn accountname(&self) -> Option<&str> {
        if self.accountname.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.accountname.clone(),
                ),
            )
        }
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn accountnumber(&self) -> Option<&str> {
        if self.accountnumber.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.accountnumber.clone(),
                ),
            )
        }
    }
    pub fn bankname(&self) -> Option<&str> {
        if self.bankname.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.bankname.clone(),
                ),
            )
        }
    }
    pub fn branchname(&self) -> Option<&str> {
        if self.branchname.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.branchname.clone(),
                ),
            )
        }
    }
    pub fn bsbnumber(&self) -> Option<&str> {
        if self.bsbnumber.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.bsbnumber.clone(),
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
    pub fn abn(&self) -> Option<&str> {
        if self.abn.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.abn.clone()))
        }
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantaccount1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PARTICIPANTACCOUNT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationParticipantaccount1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "ACCOUNTNAME",
        "PARTICIPANTID",
        "ACCOUNTNUMBER",
        "BANKNAME",
        "BANKNUMBER",
        "BRANCHNAME",
        "BRANCHNUMBER",
        "BSBNUMBER",
        "NEMMCOCREDITACCOUNTNUMBER",
        "NEMMCODEBITACCOUNTNUMBER",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "EFFECTIVEDATE",
        "LASTCHANGED",
        "ABN",
    ];
    type Row<'row> = ParticipantRegistrationParticipantaccount1Row<'row>;
    type FieldMapping = ParticipantRegistrationParticipantaccount1Mapping;
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationParticipantaccount1Row {
            accountname: row.get_opt_range("accountname", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            accountnumber: row.get_opt_range("accountnumber", field_mapping.0[2])?,
            bankname: row.get_opt_range("bankname", field_mapping.0[3])?,
            banknumber: row
                .get_opt_custom_parsed_at_idx(
                    "banknumber",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            branchname: row.get_opt_range("branchname", field_mapping.0[5])?,
            branchnumber: row
                .get_opt_custom_parsed_at_idx(
                    "branchnumber",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bsbnumber: row.get_opt_range("bsbnumber", field_mapping.0[7])?,
            nemmcocreditaccountnumber: row
                .get_opt_custom_parsed_at_idx(
                    "nemmcocreditaccountnumber",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            nemmcodebitaccountnumber: row
                .get_opt_custom_parsed_at_idx(
                    "nemmcodebitaccountnumber",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[10])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[13],
                    mmsdm_core::mms_datetime::parse,
                )?,
            abn: row.get_opt_range("abn", field_mapping.0[14])?,
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
        Ok(ParticipantRegistrationParticipantaccount1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationParticipantaccount1PrimaryKey {
        ParticipantRegistrationParticipantaccount1PrimaryKey {
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_participantaccount_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationParticipantaccount1Row {
            accountname: row.accountname.clone(),
            participantid: row.participantid.clone(),
            accountnumber: row.accountnumber.clone(),
            bankname: row.bankname.clone(),
            banknumber: row.banknumber.clone(),
            branchname: row.branchname.clone(),
            branchnumber: row.branchnumber.clone(),
            bsbnumber: row.bsbnumber.clone(),
            nemmcocreditaccountnumber: row.nemmcocreditaccountnumber.clone(),
            nemmcodebitaccountnumber: row.nemmcodebitaccountnumber.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            effectivedate: row.effectivedate.clone(),
            lastchanged: row.lastchanged.clone(),
            abn: row.abn.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantaccount1PrimaryKey {
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantaccount1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantaccount1Row<'data> {
    type Row<'other> = ParticipantRegistrationParticipantaccount1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantaccount1Row<'data> {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantaccount1PrimaryKey {
    type Row<'other> = ParticipantRegistrationParticipantaccount1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantaccount1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantaccount1 {
    type Builder = ParticipantRegistrationParticipantaccount1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "accountname",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "accountnumber",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bankname",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "banknumber",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "branchname",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "branchnumber",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bsbnumber",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nemmcocreditaccountnumber",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nemmcodebitaccountnumber",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
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
                    "effectivedate",
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
                    "abn",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationParticipantaccount1Builder {
            accountname_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            accountnumber_array: arrow::array::builder::StringBuilder::new(),
            bankname_array: arrow::array::builder::StringBuilder::new(),
            banknumber_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            branchname_array: arrow::array::builder::StringBuilder::new(),
            branchnumber_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            bsbnumber_array: arrow::array::builder::StringBuilder::new(),
            nemmcocreditaccountnumber_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            nemmcodebitaccountnumber_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            abn_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.accountname_array.append_option(row.accountname());
        builder.participantid_array.append_value(row.participantid());
        builder.accountnumber_array.append_option(row.accountnumber());
        builder.bankname_array.append_option(row.bankname());
        builder
            .banknumber_array
            .append_option({
                row.banknumber
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.branchname_array.append_option(row.branchname());
        builder
            .branchnumber_array
            .append_option({
                row.branchnumber
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.bsbnumber_array.append_option(row.bsbnumber());
        builder
            .nemmcocreditaccountnumber_array
            .append_option({
                row.nemmcocreditaccountnumber
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .nemmcodebitaccountnumber_array
            .append_option({
                row.nemmcodebitaccountnumber
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .effectivedate_array
            .append_option(
                row.effectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.abn_array.append_option(row.abn());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.accountname_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.accountnumber_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bankname_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.banknumber_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.branchname_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.branchnumber_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bsbnumber_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.nemmcocreditaccountnumber_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.nemmcodebitaccountnumber_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.abn_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationParticipantaccount1Builder {
    accountname_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    accountnumber_array: arrow::array::builder::StringBuilder,
    bankname_array: arrow::array::builder::StringBuilder,
    banknumber_array: arrow::array::builder::Decimal128Builder,
    branchname_array: arrow::array::builder::StringBuilder,
    branchnumber_array: arrow::array::builder::Decimal128Builder,
    bsbnumber_array: arrow::array::builder::StringBuilder,
    nemmcocreditaccountnumber_array: arrow::array::builder::Decimal128Builder,
    nemmcodebitaccountnumber_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    abn_array: arrow::array::builder::StringBuilder,
}
pub struct ParticipantRegistrationParticipantcategory1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationParticipantcategory1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationParticipantcategory1 {
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
pub struct ParticipantRegistrationParticipantcategory1Mapping([usize; 3]);
/// # Summary
///
/// ## PARTICIPANTCATEGORY
///
/// PARTICIPANTCATEGORY sets out valid participant categories.
///
/// * Data Set Name: Participant Registration
/// * File Name: Participantcategory
/// * Data Version: 1
///
/// # Description
/// PARTICIPANTCATEGORY is public data, so is available to all participants.SourcePARTICIPANTCATEGORY updates as categories change. PARTICIPANTCATEGORY changes infrequently.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PARTICIPANTCATEGORYID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationParticipantcategory1Row<'data> {
    /// Participant category identifier
    pub participantcategoryid: core::ops::Range<usize>,
    /// Category description
    pub description: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationParticipantcategory1Row<'data> {
    pub fn participantcategoryid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participantcategoryid.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantcategory1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PARTICIPANTCATEGORY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationParticipantcategory1Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTCATEGORYID",
        "DESCRIPTION",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationParticipantcategory1Row<'row>;
    type FieldMapping = ParticipantRegistrationParticipantcategory1Mapping;
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationParticipantcategory1Row {
            participantcategoryid: row
                .get_range("participantcategoryid", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
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
        Ok(ParticipantRegistrationParticipantcategory1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationParticipantcategory1PrimaryKey {
        ParticipantRegistrationParticipantcategory1PrimaryKey {
            participantcategoryid: row.participantcategoryid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_participantcategory_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationParticipantcategory1Row {
            participantcategoryid: row.participantcategoryid.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantcategory1PrimaryKey {
    pub participantcategoryid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantcategory1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantcategory1Row<'data> {
    type Row<'other> = ParticipantRegistrationParticipantcategory1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantcategoryid() == row.participantcategoryid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantcategory1Row<'data> {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid() == key.participantcategoryid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantcategory1PrimaryKey {
    type Row<'other> = ParticipantRegistrationParticipantcategory1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantcategoryid == row.participantcategoryid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantcategory1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantcategory1 {
    type Builder = ParticipantRegistrationParticipantcategory1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantcategoryid",
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
        ParticipantRegistrationParticipantcategory1Builder {
            participantcategoryid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantcategoryid_array.append_value(row.participantcategoryid());
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
                    alloc::sync::Arc::new(builder.participantcategoryid_array.finish())
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
pub struct ParticipantRegistrationParticipantcategory1Builder {
    participantcategoryid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationParticipantcategoryalloc1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationParticipantcategoryalloc1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationParticipantcategoryalloc1 {
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
pub struct ParticipantRegistrationParticipantcategoryalloc1Mapping([usize; 3]);
/// # Summary
///
/// ## PARTICIPANTCATEGORYALLOC
///
/// PARTICIPANTCATEGORYALLOC sets out the assignment of participants to particular categories.
///
/// * Data Set Name: Participant Registration
/// * File Name: Participantcategoryalloc
/// * Data Version: 1
///
/// # Description
/// PARTICIPANTCATEGORYALLOC data is public, so is available to all participants.SourcePARTICIPANTCATEGORYALLOC updates for new participants or when categories change. PARTICIPANTCATEGORYALLOC changes infrequently.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationParticipantcategoryalloc1Row<'data> {
    /// Category unique identifier
    pub participantcategoryid: core::ops::Range<usize>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationParticipantcategoryalloc1Row<'data> {
    pub fn participantcategoryid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participantcategoryid.clone(),
        )
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantcategoryalloc1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PARTICIPANTCATEGORYALLOC";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationParticipantcategoryalloc1Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTCATEGORYID",
        "PARTICIPANTID",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationParticipantcategoryalloc1Row<'row>;
    type FieldMapping = ParticipantRegistrationParticipantcategoryalloc1Mapping;
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationParticipantcategoryalloc1Row {
            participantcategoryid: row
                .get_range("participantcategoryid", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
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
        Ok(ParticipantRegistrationParticipantcategoryalloc1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
        ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
            participantcategoryid: row.participantcategoryid().to_string(),
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_participantcategoryalloc_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationParticipantcategoryalloc1Row {
            participantcategoryid: row.participantcategoryid.clone(),
            participantid: row.participantid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    pub participantcategoryid: alloc::string::String,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey
for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantcategoryalloc1Row<'data> {
    type Row<'other> = ParticipantRegistrationParticipantcategoryalloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantcategoryid() == row.participantcategoryid()
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantcategoryalloc1Row<'data> {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid() == key.participantcategoryid
            && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    type Row<'other> = ParticipantRegistrationParticipantcategoryalloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantcategoryid == row.participantcategoryid()
            && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantcategoryalloc1 {
    type Builder = ParticipantRegistrationParticipantcategoryalloc1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantcategoryid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationParticipantcategoryalloc1Builder {
            participantcategoryid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantcategoryid_array.append_value(row.participantcategoryid());
        builder.participantid_array.append_value(row.participantid());
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
                    alloc::sync::Arc::new(builder.participantcategoryid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationParticipantcategoryalloc1Builder {
    participantcategoryid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationParticipantclass1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationParticipantclass1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationParticipantclass1 {
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
pub struct ParticipantRegistrationParticipantclass1Mapping([usize; 3]);
/// # Summary
///
/// ## PARTICIPANTCLASS
///
/// PARTICIPANTCLASS sets out valid participant classifications.
///
/// * Data Set Name: Participant Registration
/// * File Name: Participantclass
/// * Data Version: 1
///
/// # Description
/// PARTICIPANTCLASS data is public, so is available to all participants.SourcePARTICIPANTCLASS updates only if classifications change. This table changes infrequently.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PARTICIPANTCLASSID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationParticipantclass1Row<'data> {
    /// Class of participant
    pub participantclassid: core::ops::Range<usize>,
    /// Description of participant class
    pub description: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationParticipantclass1Row<'data> {
    pub fn participantclassid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participantclassid.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantclass1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PARTICIPANTCLASS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationParticipantclass1Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTCLASSID",
        "DESCRIPTION",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationParticipantclass1Row<'row>;
    type FieldMapping = ParticipantRegistrationParticipantclass1Mapping;
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationParticipantclass1Row {
            participantclassid: row.get_range("participantclassid", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
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
        Ok(ParticipantRegistrationParticipantclass1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationParticipantclass1PrimaryKey {
        ParticipantRegistrationParticipantclass1PrimaryKey {
            participantclassid: row.participantclassid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_participantclass_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationParticipantclass1Row {
            participantclassid: row.participantclassid.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantclass1PrimaryKey {
    pub participantclassid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantclass1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantclass1Row<'data> {
    type Row<'other> = ParticipantRegistrationParticipantclass1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantclassid() == row.participantclassid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantclass1Row<'data> {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantclassid() == key.participantclassid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantclass1PrimaryKey {
    type Row<'other> = ParticipantRegistrationParticipantclass1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantclassid == row.participantclassid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantclass1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantclassid == key.participantclassid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantclass1 {
    type Builder = ParticipantRegistrationParticipantclass1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantclassid",
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
        ParticipantRegistrationParticipantclass1Builder {
            participantclassid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantclassid_array.append_value(row.participantclassid());
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
                    alloc::sync::Arc::new(builder.participantclassid_array.finish())
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
pub struct ParticipantRegistrationParticipantclass1Builder {
    participantclassid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationParticipantcreditdetail1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationParticipantcreditdetail1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationParticipantcreditdetail1 {
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
pub struct ParticipantRegistrationParticipantcreditdetail1Mapping([usize; 6]);
/// # Summary
///
/// ## PARTICIPANTCREDITDETAIL
///
///
///
/// * Data Set Name: Participant Registration
/// * File Name: Participantcreditdetail
/// * Data Version: 1
///
/// # Description
/// PARTICIPANTCREDITDETAIL data is confidential to each participant.SourcePARTICIPANTCREDITDETAIL updates infrequently.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationParticipantcreditdetail1Row<'data> {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: core::ops::Range<usize>,
    pub creditlimit: Option<rust_decimal::Decimal>,
    pub authorisedby: core::ops::Range<usize>,
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationParticipantcreditdetail1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
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
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantcreditdetail1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PARTICIPANTCREDITDETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationParticipantcreditdetail1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "PARTICIPANTID",
        "CREDITLIMIT",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationParticipantcreditdetail1Row<'row>;
    type FieldMapping = ParticipantRegistrationParticipantcreditdetail1Mapping;
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationParticipantcreditdetail1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            creditlimit: row
                .get_opt_custom_parsed_at_idx(
                    "creditlimit",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[3])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
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
        Ok(ParticipantRegistrationParticipantcreditdetail1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
        ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
            effectivedate: row.effectivedate,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_participantcreditdetail_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationParticipantcreditdetail1Row {
            effectivedate: row.effectivedate.clone(),
            participantid: row.participantid.clone(),
            creditlimit: row.creditlimit.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey
for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantcreditdetail1Row<'data> {
    type Row<'other> = ParticipantRegistrationParticipantcreditdetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantcreditdetail1Row<'data> {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    type Row<'other> = ParticipantRegistrationParticipantcreditdetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantcreditdetail1 {
    type Builder = ParticipantRegistrationParticipantcreditdetail1Builder;
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
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "creditlimit",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
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
        ParticipantRegistrationParticipantcreditdetail1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            creditlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.participantid_array.append_value(row.participantid());
        builder
            .creditlimit_array
            .append_option({
                row.creditlimit
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
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
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.creditlimit_array.finish())
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
pub struct ParticipantRegistrationParticipantcreditdetail1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    creditlimit_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationPmsGroup1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationPmsGroup1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationPmsGroup1 {
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
pub struct ParticipantRegistrationPmsGroup1Mapping([usize; 3]);
/// # Summary
///
/// ## PMS_GROUP
///
/// Entity table for group
///
/// * Data Set Name: Participant Registration
/// * File Name: Pms Group
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
/// * GROUPID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationPmsGroup1Row<'data> {
    /// Abstract identifier for the group
    pub groupid: rust_decimal::Decimal,
    /// Date record was created
    pub createddate: Option<chrono::NaiveDateTime>,
    /// Date record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> ParticipantRegistrationPmsGroup1Row<'data> {}
impl mmsdm_core::GetTable for ParticipantRegistrationPmsGroup1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PMS_GROUP";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationPmsGroup1Mapping([
        4, 5, 6,
    ]);
    const COLUMNS: &'static [&'static str] = &["GROUPID", "CREATEDDATE", "LASTCHANGED"];
    type Row<'row> = ParticipantRegistrationPmsGroup1Row<'row>;
    type FieldMapping = ParticipantRegistrationPmsGroup1Mapping;
    type PrimaryKey = ParticipantRegistrationPmsGroup1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationPmsGroup1Row {
            groupid: row
                .get_custom_parsed_at_idx(
                    "groupid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            createddate: row
                .get_opt_custom_parsed_at_idx(
                    "createddate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(ParticipantRegistrationPmsGroup1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationPmsGroup1PrimaryKey {
        ParticipantRegistrationPmsGroup1PrimaryKey {
            groupid: row.groupid,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_pms_group_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationPmsGroup1Row {
            groupid: row.groupid.clone(),
            createddate: row.createddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationPmsGroup1PrimaryKey {
    pub groupid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationPmsGroup1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroup1Row<'data> {
    type Row<'other> = ParticipantRegistrationPmsGroup1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.groupid == row.groupid
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationPmsGroup1Row<'data> {
    type PrimaryKey = ParticipantRegistrationPmsGroup1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupid == key.groupid
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroup1PrimaryKey {
    type Row<'other> = ParticipantRegistrationPmsGroup1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.groupid == row.groupid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationPmsGroup1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationPmsGroup1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupid == key.groupid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationPmsGroup1 {
    type Builder = ParticipantRegistrationPmsGroup1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "groupid",
                    arrow::datatypes::DataType::Decimal128(20, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "createddate",
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
        ParticipantRegistrationPmsGroup1Builder {
            groupid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 0)),
            createddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .groupid_array
            .append_value({
                let mut val = row.groupid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .createddate_array
            .append_option(row.createddate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.groupid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.createddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationPmsGroup1Builder {
    groupid_array: arrow::array::builder::Decimal128Builder,
    createddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationPmsGroupnmi1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationPmsGroupnmi1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationPmsGroupnmi1 {
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
pub struct ParticipantRegistrationPmsGroupnmi1Mapping([usize; 17]);
/// # Summary
///
/// ## PMS_GROUPNMI
///
/// Describe the NMIs that a group uses to provide its service
///
/// * Data Set Name: Participant Registration
/// * File Name: Pms Groupnmi
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * GROUPNMIID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationPmsGroupnmi1Row<'data> {
    /// Record Identifier of the NMI within a Group. When data is updated, existing record identifier is terminated, and new record identifier(s) are allocated.
    pub groupnmiid: rust_decimal::Decimal,
    /// Group id of the Group which the NMI belongs in.
    pub groupid: Option<rust_decimal::Decimal>,
    /// Date for which this version is effective from
    pub versionfrom: Option<chrono::NaiveDateTime>,
    /// Date for which this version is effective to. Will be set to current day plus one if it is the current active record or past date if the record has been superseded/ended.
    pub versionto: Option<chrono::NaiveDateTime>,
    /// Effective date of when this service started operation
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Date for which this version is effective to. Will be set to current day plus one if it is the current active record or past date if the record has been superseded/ended.
    pub enddate: Option<chrono::NaiveDateTime>,
    /// National Meter Identifier linked to the group.
    pub nmi: core::ops::Range<usize>,
    /// Site name
    pub sitename: core::ops::Range<usize>,
    /// Specifies whether NMI is in a NERR aggregated premises (TRUE = 1/FALSE = 0)
    pub nerrgrouppremises: Option<rust_decimal::Decimal>,
    /// Baseline methodology to be used for the PoL and Baseline assessment of the NMI
    pub baselinemethodologyid: core::ops::Range<usize>,
    /// Maximum responsive component for the NMI
    pub mrc: Option<rust_decimal::Decimal>,
    /// Reason for the MRC
    pub mrcreason: core::ops::Range<usize>,
    /// Retail customer of the NMI
    pub retailcustomer: core::ops::Range<usize>,
    /// Indicates whether the NMI has been suspended from use. (TRUE = 1/FALSE = 0)
    pub suspended: Option<rust_decimal::Decimal>,
    /// Indicates whether the NMI is unavailable for use. (TRUE = 1/FALSE = 0)
    pub unavailable: Option<rust_decimal::Decimal>,
    /// Date which this record was approved
    pub approveddate: Option<chrono::NaiveDateTime>,
    /// Date time which record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationPmsGroupnmi1Row<'data> {
    pub fn nmi(&self) -> Option<&str> {
        if self.nmi.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.nmi.clone()))
        }
    }
    pub fn sitename(&self) -> Option<&str> {
        if self.sitename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.sitename.clone(),
                ),
            )
        }
    }
    pub fn baselinemethodologyid(&self) -> Option<&str> {
        if self.baselinemethodologyid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.baselinemethodologyid.clone(),
                ),
            )
        }
    }
    pub fn mrcreason(&self) -> Option<&str> {
        if self.mrcreason.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.mrcreason.clone(),
                ),
            )
        }
    }
    pub fn retailcustomer(&self) -> Option<&str> {
        if self.retailcustomer.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.retailcustomer.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationPmsGroupnmi1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PMS_GROUPNMI";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationPmsGroupnmi1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "GROUPNMIID",
        "GROUPID",
        "VERSIONFROM",
        "VERSIONTO",
        "STARTDATE",
        "ENDDATE",
        "NMI",
        "SITENAME",
        "NERRGROUPPREMISES",
        "BASELINEMETHODOLOGYID",
        "MRC",
        "MRCREASON",
        "RETAILCUSTOMER",
        "SUSPENDED",
        "UNAVAILABLE",
        "APPROVEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationPmsGroupnmi1Row<'row>;
    type FieldMapping = ParticipantRegistrationPmsGroupnmi1Mapping;
    type PrimaryKey = ParticipantRegistrationPmsGroupnmi1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationPmsGroupnmi1Row {
            groupnmiid: row
                .get_custom_parsed_at_idx(
                    "groupnmiid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            groupid: row
                .get_opt_custom_parsed_at_idx(
                    "groupid",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            versionfrom: row
                .get_opt_custom_parsed_at_idx(
                    "versionfrom",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionto: row
                .get_opt_custom_parsed_at_idx(
                    "versionto",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            nmi: row.get_opt_range("nmi", field_mapping.0[6])?,
            sitename: row.get_opt_range("sitename", field_mapping.0[7])?,
            nerrgrouppremises: row
                .get_opt_custom_parsed_at_idx(
                    "nerrgrouppremises",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            baselinemethodologyid: row
                .get_opt_range("baselinemethodologyid", field_mapping.0[9])?,
            mrc: row
                .get_opt_custom_parsed_at_idx(
                    "mrc",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mrcreason: row.get_opt_range("mrcreason", field_mapping.0[11])?,
            retailcustomer: row.get_opt_range("retailcustomer", field_mapping.0[12])?,
            suspended: row
                .get_opt_custom_parsed_at_idx(
                    "suspended",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unavailable: row
                .get_opt_custom_parsed_at_idx(
                    "unavailable",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            approveddate: row
                .get_opt_custom_parsed_at_idx(
                    "approveddate",
                    field_mapping.0[15],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[16],
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
        Ok(ParticipantRegistrationPmsGroupnmi1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationPmsGroupnmi1PrimaryKey {
        ParticipantRegistrationPmsGroupnmi1PrimaryKey {
            groupnmiid: row.groupnmiid,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_pms_groupnmi_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationPmsGroupnmi1Row {
            groupnmiid: row.groupnmiid.clone(),
            groupid: row.groupid.clone(),
            versionfrom: row.versionfrom.clone(),
            versionto: row.versionto.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            nmi: row.nmi.clone(),
            sitename: row.sitename.clone(),
            nerrgrouppremises: row.nerrgrouppremises.clone(),
            baselinemethodologyid: row.baselinemethodologyid.clone(),
            mrc: row.mrc.clone(),
            mrcreason: row.mrcreason.clone(),
            retailcustomer: row.retailcustomer.clone(),
            suspended: row.suspended.clone(),
            unavailable: row.unavailable.clone(),
            approveddate: row.approveddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationPmsGroupnmi1PrimaryKey {
    pub groupnmiid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationPmsGroupnmi1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationPmsGroupnmi1Row<'data> {
    type Row<'other> = ParticipantRegistrationPmsGroupnmi1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.groupnmiid == row.groupnmiid
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationPmsGroupnmi1Row<'data> {
    type PrimaryKey = ParticipantRegistrationPmsGroupnmi1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupnmiid == key.groupnmiid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationPmsGroupnmi1PrimaryKey {
    type Row<'other> = ParticipantRegistrationPmsGroupnmi1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.groupnmiid == row.groupnmiid
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationPmsGroupnmi1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationPmsGroupnmi1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupnmiid == key.groupnmiid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationPmsGroupnmi1 {
    type Builder = ParticipantRegistrationPmsGroupnmi1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "groupnmiid",
                    arrow::datatypes::DataType::Decimal128(20, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "groupid",
                    arrow::datatypes::DataType::Decimal128(20, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionfrom",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionto",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
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
                    "nmi",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sitename",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nerrgrouppremises",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "baselinemethodologyid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mrc",
                    arrow::datatypes::DataType::Decimal128(10, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mrcreason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "retailcustomer",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "suspended",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unavailable",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "approveddate",
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
        ParticipantRegistrationPmsGroupnmi1Builder {
            groupnmiid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 0)),
            groupid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 0)),
            versionfrom_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionto_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            nmi_array: arrow::array::builder::StringBuilder::new(),
            sitename_array: arrow::array::builder::StringBuilder::new(),
            nerrgrouppremises_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            baselinemethodologyid_array: arrow::array::builder::StringBuilder::new(),
            mrc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 3)),
            mrcreason_array: arrow::array::builder::StringBuilder::new(),
            retailcustomer_array: arrow::array::builder::StringBuilder::new(),
            suspended_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            unavailable_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            approveddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .groupnmiid_array
            .append_value({
                let mut val = row.groupnmiid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .groupid_array
            .append_option({
                row.groupid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .versionfrom_array
            .append_option(row.versionfrom.map(|val| val.and_utc().timestamp_millis()));
        builder
            .versionto_array
            .append_option(row.versionto.map(|val| val.and_utc().timestamp_millis()));
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder.nmi_array.append_option(row.nmi());
        builder.sitename_array.append_option(row.sitename());
        builder
            .nerrgrouppremises_array
            .append_option({
                row.nerrgrouppremises
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.baselinemethodologyid_array.append_option(row.baselinemethodologyid());
        builder
            .mrc_array
            .append_option({
                row.mrc
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder.mrcreason_array.append_option(row.mrcreason());
        builder.retailcustomer_array.append_option(row.retailcustomer());
        builder
            .suspended_array
            .append_option({
                row.suspended
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .unavailable_array
            .append_option({
                row.unavailable
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .approveddate_array
            .append_option(row.approveddate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.groupnmiid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.groupid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionfrom_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nmi_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sitename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nerrgrouppremises_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.baselinemethodologyid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mrc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mrcreason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.retailcustomer_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.suspended_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unavailable_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.approveddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationPmsGroupnmi1Builder {
    groupnmiid_array: arrow::array::builder::Decimal128Builder,
    groupid_array: arrow::array::builder::Decimal128Builder,
    versionfrom_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionto_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    nmi_array: arrow::array::builder::StringBuilder,
    sitename_array: arrow::array::builder::StringBuilder,
    nerrgrouppremises_array: arrow::array::builder::Decimal128Builder,
    baselinemethodologyid_array: arrow::array::builder::StringBuilder,
    mrc_array: arrow::array::builder::Decimal128Builder,
    mrcreason_array: arrow::array::builder::StringBuilder,
    retailcustomer_array: arrow::array::builder::StringBuilder,
    suspended_array: arrow::array::builder::Decimal128Builder,
    unavailable_array: arrow::array::builder::Decimal128Builder,
    approveddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationPmsGroupservice1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationPmsGroupservice1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationPmsGroupservice1 {
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
pub struct ParticipantRegistrationPmsGroupservice1Mapping([usize; 16]);
/// # Summary
///
/// ## PMS_GROUPSERVICE
///
/// Describe the services a group provides and its relation to a market
///
/// * Data Set Name: Participant Registration
/// * File Name: Pms Groupservice
/// * Data Version: 1
///
/// # Description
/// STADUALLOC is public data, and is available to all participants.SourceSTADUALLOC is updated whenever there is a station configuration change or new unit registration.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * GROUPSERVICEID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationPmsGroupservice1Row<'data> {
    /// Record identifier of the Service allocated to the Group. When data is updated, existing record identifier is terminated, and new record identifier(s) are allocated.
    pub groupserviceid: rust_decimal::Decimal,
    /// Group id of the Group where the Service is attached to.
    pub groupid: Option<rust_decimal::Decimal>,
    /// Date for which this version is effective from.
    pub versionfrom: Option<chrono::NaiveDateTime>,
    /// Date for which this version is effective to. Will be set to max date 9999/12/31 23:59:59.999 until this version ends or a change to the version is required.
    pub versionto: Option<chrono::NaiveDateTime>,
    /// Effective date of when this service started operation
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Effective date of when this service ended operation. Will be set to max date 9999/12/31 23:59:59.999 until its service ends or a change to the service is required.
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Market that this group is operating its service in. Will only be NEM initially.
    pub market: core::ops::Range<usize>,
    /// Service that this group is operating. Will be only be ENERGY initially
    pub servicetype: core::ops::Range<usize>,
    /// Describes the entity that is operating. Will only be WDRU initially.
    pub entitytype: core::ops::Range<usize>,
    /// Describe the entity ID in the market that it will be operating in. Will only contain the DUID of the group initially.
    pub entityid: core::ops::Range<usize>,
    /// Maximum responsive component for the service offering
    pub mrc: Option<rust_decimal::Decimal>,
    /// Reason for the MRC.
    pub mrcreason: core::ops::Range<usize>,
    /// Maximum ramp rate MW per minute of the service.
    pub maximumrampratepermin: Option<rust_decimal::Decimal>,
    /// Region the group is operating this service in One of NSW1, QLD1, VIC1, SA1 or TAS1
    pub region: core::ops::Range<usize>,
    /// Date which this record was approved
    pub approveddate: Option<chrono::NaiveDateTime>,
    /// Date time which record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationPmsGroupservice1Row<'data> {
    pub fn market(&self) -> Option<&str> {
        if self.market.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.market.clone(),
                ),
            )
        }
    }
    pub fn servicetype(&self) -> Option<&str> {
        if self.servicetype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.servicetype.clone(),
                ),
            )
        }
    }
    pub fn entitytype(&self) -> Option<&str> {
        if self.entitytype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.entitytype.clone(),
                ),
            )
        }
    }
    pub fn entityid(&self) -> Option<&str> {
        if self.entityid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.entityid.clone(),
                ),
            )
        }
    }
    pub fn mrcreason(&self) -> Option<&str> {
        if self.mrcreason.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.mrcreason.clone(),
                ),
            )
        }
    }
    pub fn region(&self) -> Option<&str> {
        if self.region.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.region.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationPmsGroupservice1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "PMS_GROUPSERVICE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationPmsGroupservice1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "GROUPSERVICEID",
        "GROUPID",
        "VERSIONFROM",
        "VERSIONTO",
        "STARTDATE",
        "ENDDATE",
        "MARKET",
        "SERVICETYPE",
        "ENTITYTYPE",
        "ENTITYID",
        "MRC",
        "MRCREASON",
        "MAXIMUMRAMPRATEPERMIN",
        "REGION",
        "APPROVEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationPmsGroupservice1Row<'row>;
    type FieldMapping = ParticipantRegistrationPmsGroupservice1Mapping;
    type PrimaryKey = ParticipantRegistrationPmsGroupservice1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationPmsGroupservice1Row {
            groupserviceid: row
                .get_custom_parsed_at_idx(
                    "groupserviceid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            groupid: row
                .get_opt_custom_parsed_at_idx(
                    "groupid",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            versionfrom: row
                .get_opt_custom_parsed_at_idx(
                    "versionfrom",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionto: row
                .get_opt_custom_parsed_at_idx(
                    "versionto",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            market: row.get_opt_range("market", field_mapping.0[6])?,
            servicetype: row.get_opt_range("servicetype", field_mapping.0[7])?,
            entitytype: row.get_opt_range("entitytype", field_mapping.0[8])?,
            entityid: row.get_opt_range("entityid", field_mapping.0[9])?,
            mrc: row
                .get_opt_custom_parsed_at_idx(
                    "mrc",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mrcreason: row.get_opt_range("mrcreason", field_mapping.0[11])?,
            maximumrampratepermin: row
                .get_opt_custom_parsed_at_idx(
                    "maximumrampratepermin",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            region: row.get_opt_range("region", field_mapping.0[13])?,
            approveddate: row
                .get_opt_custom_parsed_at_idx(
                    "approveddate",
                    field_mapping.0[14],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(ParticipantRegistrationPmsGroupservice1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationPmsGroupservice1PrimaryKey {
        ParticipantRegistrationPmsGroupservice1PrimaryKey {
            groupserviceid: row.groupserviceid,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_pms_groupservice_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationPmsGroupservice1Row {
            groupserviceid: row.groupserviceid.clone(),
            groupid: row.groupid.clone(),
            versionfrom: row.versionfrom.clone(),
            versionto: row.versionto.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            market: row.market.clone(),
            servicetype: row.servicetype.clone(),
            entitytype: row.entitytype.clone(),
            entityid: row.entityid.clone(),
            mrc: row.mrc.clone(),
            mrcreason: row.mrcreason.clone(),
            maximumrampratepermin: row.maximumrampratepermin.clone(),
            region: row.region.clone(),
            approveddate: row.approveddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationPmsGroupservice1PrimaryKey {
    pub groupserviceid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationPmsGroupservice1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationPmsGroupservice1Row<'data> {
    type Row<'other> = ParticipantRegistrationPmsGroupservice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.groupserviceid == row.groupserviceid
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationPmsGroupservice1Row<'data> {
    type PrimaryKey = ParticipantRegistrationPmsGroupservice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupserviceid == key.groupserviceid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationPmsGroupservice1PrimaryKey {
    type Row<'other> = ParticipantRegistrationPmsGroupservice1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.groupserviceid == row.groupserviceid
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationPmsGroupservice1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationPmsGroupservice1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupserviceid == key.groupserviceid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationPmsGroupservice1 {
    type Builder = ParticipantRegistrationPmsGroupservice1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "groupserviceid",
                    arrow::datatypes::DataType::Decimal128(20, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "groupid",
                    arrow::datatypes::DataType::Decimal128(20, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionfrom",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionto",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
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
                    "market",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "servicetype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "entitytype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "entityid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mrc",
                    arrow::datatypes::DataType::Decimal128(10, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mrcreason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maximumrampratepermin",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "region",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "approveddate",
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
        ParticipantRegistrationPmsGroupservice1Builder {
            groupserviceid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 0)),
            groupid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 0)),
            versionfrom_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionto_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            market_array: arrow::array::builder::StringBuilder::new(),
            servicetype_array: arrow::array::builder::StringBuilder::new(),
            entitytype_array: arrow::array::builder::StringBuilder::new(),
            entityid_array: arrow::array::builder::StringBuilder::new(),
            mrc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 3)),
            mrcreason_array: arrow::array::builder::StringBuilder::new(),
            maximumrampratepermin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            region_array: arrow::array::builder::StringBuilder::new(),
            approveddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .groupserviceid_array
            .append_value({
                let mut val = row.groupserviceid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .groupid_array
            .append_option({
                row.groupid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .versionfrom_array
            .append_option(row.versionfrom.map(|val| val.and_utc().timestamp_millis()));
        builder
            .versionto_array
            .append_option(row.versionto.map(|val| val.and_utc().timestamp_millis()));
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder.market_array.append_option(row.market());
        builder.servicetype_array.append_option(row.servicetype());
        builder.entitytype_array.append_option(row.entitytype());
        builder.entityid_array.append_option(row.entityid());
        builder
            .mrc_array
            .append_option({
                row.mrc
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder.mrcreason_array.append_option(row.mrcreason());
        builder
            .maximumrampratepermin_array
            .append_option({
                row.maximumrampratepermin
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.region_array.append_option(row.region());
        builder
            .approveddate_array
            .append_option(row.approveddate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.groupserviceid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.groupid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionfrom_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.market_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.servicetype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.entitytype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.entityid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mrc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mrcreason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maximumrampratepermin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.region_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.approveddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationPmsGroupservice1Builder {
    groupserviceid_array: arrow::array::builder::Decimal128Builder,
    groupid_array: arrow::array::builder::Decimal128Builder,
    versionfrom_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionto_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    market_array: arrow::array::builder::StringBuilder,
    servicetype_array: arrow::array::builder::StringBuilder,
    entitytype_array: arrow::array::builder::StringBuilder,
    entityid_array: arrow::array::builder::StringBuilder,
    mrc_array: arrow::array::builder::Decimal128Builder,
    mrcreason_array: arrow::array::builder::StringBuilder,
    maximumrampratepermin_array: arrow::array::builder::Decimal128Builder,
    region_array: arrow::array::builder::StringBuilder,
    approveddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationStadualloc1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationStadualloc1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationStadualloc1 {
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
pub struct ParticipantRegistrationStadualloc1Mapping([usize; 5]);
/// # Summary
///
/// ## STADUALLOC
///
/// STADUALLOC sets out details on the allocation of dispatchable units to particular sites or stations.
///
/// * Data Set Name: Participant Registration
/// * File Name: Stadualloc
/// * Data Version: 1
///
/// # Description
/// STADUALLOC is public data, and is available to all participants.SourceSTADUALLOC is updated whenever there is a station configuration change or new unit registration.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * DUID
/// * EFFECTIVEDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationStadualloc1Row<'data> {
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationStadualloc1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn stationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.stationid.clone())
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationStadualloc1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "STADUALLOC";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationStadualloc1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "EFFECTIVEDATE",
        "STATIONID",
        "VERSIONNO",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationStadualloc1Row<'row>;
    type FieldMapping = ParticipantRegistrationStadualloc1Mapping;
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationStadualloc1Row {
            duid: row.get_range("duid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            stationid: row.get_range("stationid", field_mapping.0[2])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
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
        Ok(ParticipantRegistrationStadualloc1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationStadualloc1PrimaryKey {
        ParticipantRegistrationStadualloc1PrimaryKey {
            duid: row.duid().to_string(),
            effectivedate: row.effectivedate,
            stationid: row.stationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_stadualloc_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationStadualloc1Row {
            duid: row.duid.clone(),
            effectivedate: row.effectivedate.clone(),
            stationid: row.stationid.clone(),
            versionno: row.versionno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStadualloc1PrimaryKey {
    pub duid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub stationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStadualloc1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationStadualloc1Row<'data> {
    type Row<'other> = ParticipantRegistrationStadualloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.effectivedate == row.effectivedate
            && self.stationid() == row.stationid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStadualloc1Row<'data> {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.effectivedate == key.effectivedate
            && self.stationid() == key.stationid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationStadualloc1PrimaryKey {
    type Row<'other> = ParticipantRegistrationStadualloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.effectivedate == row.effectivedate
            && self.stationid == row.stationid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStadualloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.effectivedate == key.effectivedate
            && self.stationid == key.stationid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStadualloc1 {
    type Builder = ParticipantRegistrationStadualloc1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        ParticipantRegistrationStadualloc1Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.stationid_array.append_value(row.stationid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
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
pub struct ParticipantRegistrationStadualloc1Builder {
    duid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationStation1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationStation1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationStation1 {
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
pub struct ParticipantRegistrationStation1Mapping([usize; 11]);
/// # Summary
///
/// ## STATION
///
/// STATION sets out valid station identifiers.
///
/// * Data Set Name: Participant Registration
/// * File Name: Station
/// * Data Version: 1
///
/// # Description
/// STATION is public data, and is available to all participants.SourceSTATION updates whenever there is a station configuration change or new unit registration.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * STATIONID
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationStation1Row<'data> {
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// Full name of station
    pub stationname: core::ops::Range<usize>,
    /// Station Address
    pub address1: core::ops::Range<usize>,
    /// Station Address
    pub address2: core::ops::Range<usize>,
    /// Station Address
    pub address3: core::ops::Range<usize>,
    /// Station Address
    pub address4: core::ops::Range<usize>,
    /// City
    pub city: core::ops::Range<usize>,
    /// State of Australia
    pub state: core::ops::Range<usize>,
    /// Post Code
    pub postcode: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used. Do not use as the Connection Point Identifier for station load
    pub connectionpointid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationStation1Row<'data> {
    pub fn stationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.stationid.clone())
    }
    pub fn stationname(&self) -> Option<&str> {
        if self.stationname.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.stationname.clone(),
                ),
            )
        }
    }
    pub fn address1(&self) -> Option<&str> {
        if self.address1.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.address1.clone(),
                ),
            )
        }
    }
    pub fn address2(&self) -> Option<&str> {
        if self.address2.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.address2.clone(),
                ),
            )
        }
    }
    pub fn address3(&self) -> Option<&str> {
        if self.address3.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.address3.clone(),
                ),
            )
        }
    }
    pub fn address4(&self) -> Option<&str> {
        if self.address4.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.address4.clone(),
                ),
            )
        }
    }
    pub fn city(&self) -> Option<&str> {
        if self.city.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.city.clone()),
            )
        }
    }
    pub fn state(&self) -> Option<&str> {
        if self.state.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.state.clone()),
            )
        }
    }
    pub fn postcode(&self) -> Option<&str> {
        if self.postcode.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.postcode.clone(),
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
impl mmsdm_core::GetTable for ParticipantRegistrationStation1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "STATION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationStation1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "STATIONID",
        "STATIONNAME",
        "ADDRESS1",
        "ADDRESS2",
        "ADDRESS3",
        "ADDRESS4",
        "CITY",
        "STATE",
        "POSTCODE",
        "LASTCHANGED",
        "CONNECTIONPOINTID",
    ];
    type Row<'row> = ParticipantRegistrationStation1Row<'row>;
    type FieldMapping = ParticipantRegistrationStation1Mapping;
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationStation1Row {
            stationid: row.get_range("stationid", field_mapping.0[0])?,
            stationname: row.get_opt_range("stationname", field_mapping.0[1])?,
            address1: row.get_opt_range("address1", field_mapping.0[2])?,
            address2: row.get_opt_range("address2", field_mapping.0[3])?,
            address3: row.get_opt_range("address3", field_mapping.0[4])?,
            address4: row.get_opt_range("address4", field_mapping.0[5])?,
            city: row.get_opt_range("city", field_mapping.0[6])?,
            state: row.get_opt_range("state", field_mapping.0[7])?,
            postcode: row.get_opt_range("postcode", field_mapping.0[8])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[10])?,
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
        Ok(ParticipantRegistrationStation1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ParticipantRegistrationStation1PrimaryKey {
        ParticipantRegistrationStation1PrimaryKey {
            stationid: row.stationid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_station_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationStation1Row {
            stationid: row.stationid.clone(),
            stationname: row.stationname.clone(),
            address1: row.address1.clone(),
            address2: row.address2.clone(),
            address3: row.address3.clone(),
            address4: row.address4.clone(),
            city: row.city.clone(),
            state: row.state.clone(),
            postcode: row.postcode.clone(),
            lastchanged: row.lastchanged.clone(),
            connectionpointid: row.connectionpointid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStation1PrimaryKey {
    pub stationid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStation1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationStation1Row<'data> {
    type Row<'other> = ParticipantRegistrationStation1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.stationid() == row.stationid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStation1Row<'data> {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.stationid() == key.stationid
    }
}
impl<'data> mmsdm_core::CompareWithRow for ParticipantRegistrationStation1PrimaryKey {
    type Row<'other> = ParticipantRegistrationStation1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.stationid == row.stationid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStation1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.stationid == key.stationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStation1 {
    type Builder = ParticipantRegistrationStation1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "stationname",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "address1",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "address2",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "address3",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "address4",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "city",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "state",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "postcode",
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
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ParticipantRegistrationStation1Builder {
            stationid_array: arrow::array::builder::StringBuilder::new(),
            stationname_array: arrow::array::builder::StringBuilder::new(),
            address1_array: arrow::array::builder::StringBuilder::new(),
            address2_array: arrow::array::builder::StringBuilder::new(),
            address3_array: arrow::array::builder::StringBuilder::new(),
            address4_array: arrow::array::builder::StringBuilder::new(),
            city_array: arrow::array::builder::StringBuilder::new(),
            state_array: arrow::array::builder::StringBuilder::new(),
            postcode_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.stationid_array.append_value(row.stationid());
        builder.stationname_array.append_option(row.stationname());
        builder.address1_array.append_option(row.address1());
        builder.address2_array.append_option(row.address2());
        builder.address3_array.append_option(row.address3());
        builder.address4_array.append_option(row.address4());
        builder.city_array.append_option(row.city());
        builder.state_array.append_option(row.state());
        builder.postcode_array.append_option(row.postcode());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.connectionpointid_array.append_option(row.connectionpointid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationname_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.address1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.address2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.address3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.address4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.city_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.state_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.postcode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ParticipantRegistrationStation1Builder {
    stationid_array: arrow::array::builder::StringBuilder,
    stationname_array: arrow::array::builder::StringBuilder,
    address1_array: arrow::array::builder::StringBuilder,
    address2_array: arrow::array::builder::StringBuilder,
    address3_array: arrow::array::builder::StringBuilder,
    address4_array: arrow::array::builder::StringBuilder,
    city_array: arrow::array::builder::StringBuilder,
    state_array: arrow::array::builder::StringBuilder,
    postcode_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
}
pub struct ParticipantRegistrationStationoperatingstatus1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationStationoperatingstatus1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationStationoperatingstatus1 {
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
pub struct ParticipantRegistrationStationoperatingstatus1Mapping([usize; 7]);
/// # Summary
///
/// ## STATIONOPERATINGSTATUS
///
/// STATIONOPERATINGSTATUS sets out the operating status of each station.
///
/// * Data Set Name: Participant Registration
/// * File Name: Stationoperatingstatus
/// * Data Version: 1
///
/// # Description
/// STATIONOWNER is public data, and is available to all participants.SourceSTATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationStationoperatingstatus1Row<'data> {
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique station identifier
    pub stationid: core::ops::Range<usize>,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// The operating status of this station, valid values are COMMISSIONED and DECOMMISSIONED
    pub status: core::ops::Range<usize>,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationStationoperatingstatus1Row<'data> {
    pub fn stationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.stationid.clone())
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
impl mmsdm_core::GetTable for ParticipantRegistrationStationoperatingstatus1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "STATIONOPERATINGSTATUS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationStationoperatingstatus1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "STATIONID",
        "VERSIONNO",
        "STATUS",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationStationoperatingstatus1Row<'row>;
    type FieldMapping = ParticipantRegistrationStationoperatingstatus1Mapping;
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationStationoperatingstatus1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            stationid: row.get_range("stationid", field_mapping.0[1])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            status: row.get_opt_range("status", field_mapping.0[3])?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[4])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(ParticipantRegistrationStationoperatingstatus1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationStationoperatingstatus1PrimaryKey {
        ParticipantRegistrationStationoperatingstatus1PrimaryKey {
            effectivedate: row.effectivedate,
            stationid: row.stationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_stationoperatingstatus_v1_{}", self
            .partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationStationoperatingstatus1Row {
            effectivedate: row.effectivedate.clone(),
            stationid: row.stationid.clone(),
            versionno: row.versionno.clone(),
            status: row.status.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub stationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey
for ParticipantRegistrationStationoperatingstatus1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationStationoperatingstatus1Row<'data> {
    type Row<'other> = ParticipantRegistrationStationoperatingstatus1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.stationid() == row.stationid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStationoperatingstatus1Row<'data> {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.stationid() == key.stationid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    type Row<'other> = ParticipantRegistrationStationoperatingstatus1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.stationid == row.stationid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStationoperatingstatus1 {
    type Builder = ParticipantRegistrationStationoperatingstatus1Builder;
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
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "status",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
        ParticipantRegistrationStationoperatingstatus1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            status_array: arrow::array::builder::StringBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.stationid_array.append_value(row.stationid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.status_array.append_option(row.status());
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
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
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
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
pub struct ParticipantRegistrationStationoperatingstatus1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    status_array: arrow::array::builder::StringBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationStationowner1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationStationowner1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationStationowner1 {
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
pub struct ParticipantRegistrationStationowner1Mapping([usize; 5]);
/// # Summary
///
/// ## STATIONOWNER
///
/// STATIONOWNER sets out the owner details of each station.
///
/// * Data Set Name: Participant Registration
/// * File Name: Stationowner
/// * Data Version: 1
///
/// # Description
/// STATIONOWNER is public data, and is available to all participants.SourceSTATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationStationowner1Row<'data> {
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationStationowner1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn stationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.stationid.clone())
    }
}
impl mmsdm_core::GetTable for ParticipantRegistrationStationowner1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "STATIONOWNER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationStationowner1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "PARTICIPANTID",
        "STATIONID",
        "VERSIONNO",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationStationowner1Row<'row>;
    type FieldMapping = ParticipantRegistrationStationowner1Mapping;
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationStationowner1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            stationid: row.get_range("stationid", field_mapping.0[2])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
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
        Ok(ParticipantRegistrationStationowner1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationStationowner1PrimaryKey {
        ParticipantRegistrationStationowner1PrimaryKey {
            effectivedate: row.effectivedate,
            participantid: row.participantid().to_string(),
            stationid: row.stationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_stationowner_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationStationowner1Row {
            effectivedate: row.effectivedate.clone(),
            participantid: row.participantid.clone(),
            stationid: row.stationid.clone(),
            versionno: row.versionno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStationowner1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub stationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStationowner1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationStationowner1Row<'data> {
    type Row<'other> = ParticipantRegistrationStationowner1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid() == row.participantid()
            && self.stationid() == row.stationid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStationowner1Row<'data> {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid() == key.participantid
            && self.stationid() == key.stationid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationStationowner1PrimaryKey {
    type Row<'other> = ParticipantRegistrationStationowner1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid()
            && self.stationid == row.stationid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStationowner1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStationowner1 {
    type Builder = ParticipantRegistrationStationowner1Builder;
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
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        ParticipantRegistrationStationowner1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.participantid_array.append_value(row.participantid());
        builder.stationid_array.append_value(row.stationid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
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
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
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
pub struct ParticipantRegistrationStationowner1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct ParticipantRegistrationStationownertrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &ParticipantRegistrationStationownertrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl ParticipantRegistrationStationownertrk1 {
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
pub struct ParticipantRegistrationStationownertrk1Mapping([usize; 6]);
/// # Summary
///
/// ## STATIONOWNERTRK
///
/// STATIONOWNERTRK shows the tracking for the associated object STATIONOWNER. Together, STATIONOWNERTRK and STATIONOWNER sets out the owner details of each station.
///
/// * Data Set Name: Participant Registration
/// * File Name: Stationownertrk
/// * Data Version: 1
///
/// # Description
/// STATIONOWNER is public data, and is available to all participants.SourceSTATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ParticipantRegistrationStationownertrk1Row<'data> {
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ParticipantRegistrationStationownertrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
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
impl mmsdm_core::GetTable for ParticipantRegistrationStationownertrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PARTICIPANT_REGISTRATION";
    const TABLE_NAME: &'static str = "STATIONOWNERTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ParticipantRegistrationStationownertrk1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "PARTICIPANTID",
        "VERSIONNO",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ParticipantRegistrationStationownertrk1Row<'row>;
    type FieldMapping = ParticipantRegistrationStationownertrk1Mapping;
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ParticipantRegistrationStationownertrk1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[3])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
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
        Ok(ParticipantRegistrationStationownertrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ParticipantRegistrationStationownertrk1PrimaryKey {
        ParticipantRegistrationStationownertrk1PrimaryKey {
            effectivedate: row.effectivedate,
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "participant_registration_stationownertrk_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ParticipantRegistrationStationownertrk1Row {
            effectivedate: row.effectivedate.clone(),
            participantid: row.participantid.clone(),
            versionno: row.versionno.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStationownertrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStationownertrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationStationownertrk1Row<'data> {
    type Row<'other> = ParticipantRegistrationStationownertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStationownertrk1Row<'data> {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ParticipantRegistrationStationownertrk1PrimaryKey {
    type Row<'other> = ParticipantRegistrationStationownertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ParticipantRegistrationStationownertrk1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStationownertrk1 {
    type Builder = ParticipantRegistrationStationownertrk1Builder;
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
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        ParticipantRegistrationStationownertrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder.participantid_array.append_value(row.participantid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
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
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
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
pub struct ParticipantRegistrationStationownertrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
