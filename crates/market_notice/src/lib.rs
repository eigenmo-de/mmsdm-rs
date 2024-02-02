#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct MarketNoticeMarketnoticedata1;
pub struct MarketNoticeMarketnoticedata1Mapping([usize; 7]);
/// # Summary
///
/// ## MARKETNOTICEDATA
///  _MARKETNOTICEDATA shows market notices data provided to all participants (market) and specific participants (participant)._
///
/// * Data Set Name: Market Notice
/// * File Name: Marketnoticedata
/// * Data Version: 1
///
/// # Description
///  MARKETNOTICEDATA data is confidential to each participant, although some notices are sent to all participants. Source MARKETNOTICEDATA updates immediately available.
///
///
///
/// # Primary Key Columns
///
/// * NOTICEID
#[derive(Debug, PartialEq, Eq)]
pub struct MarketNoticeMarketnoticedata1Row<'data> {
    /// Notice Identifier
    pub noticeid: rust_decimal::Decimal,
    /// Effective Date of Market notice
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Market Notice Type Identifier (Market - all participants. Participant - selected participants)
    pub typeid: core::ops::Range<usize>,
    /// Market Notice Type
    pub noticetype: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Detail of market notices.
    pub reason: core::ops::Range<usize>,
    /// External Reference for extra data pertaining to market notice
    pub externalreference: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketNoticeMarketnoticedata1Row<'data> {
    pub fn typeid(&self) -> Option<&str> {
        if self.typeid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.typeid.clone(),
                ),
            )
        }
    }
    pub fn noticetype(&self) -> Option<&str> {
        if self.noticetype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.noticetype.clone(),
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
    pub fn externalreference(&self) -> Option<&str> {
        if self.externalreference.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.externalreference.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MarketNoticeMarketnoticedata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_NOTICE";
    const TABLE_NAME: &'static str = "MARKETNOTICEDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketNoticeMarketnoticedata1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "NOTICEID",
        "EFFECTIVEDATE",
        "TYPEID",
        "NOTICETYPE",
        "LASTCHANGED",
        "REASON",
        "EXTERNALREFERENCE",
    ];
    type Row<'row> = MarketNoticeMarketnoticedata1Row<'row>;
    type FieldMapping = MarketNoticeMarketnoticedata1Mapping;
    type PrimaryKey = MarketNoticeMarketnoticedata1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketNoticeMarketnoticedata1Row {
            noticeid: row
                .get_custom_parsed_at_idx(
                    "noticeid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            typeid: row.get_opt_range("typeid", field_mapping.0[2])?,
            noticetype: row.get_opt_range("noticetype", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reason: row.get_opt_range("reason", field_mapping.0[5])?,
            externalreference: row
                .get_opt_range("externalreference", field_mapping.0[6])?,
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
        Ok(MarketNoticeMarketnoticedata1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketNoticeMarketnoticedata1PrimaryKey {
        MarketNoticeMarketnoticedata1PrimaryKey {
            noticeid: row.noticeid,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "market_notice_marketnoticedata_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketNoticeMarketnoticedata1Row {
            noticeid: row.noticeid.clone(),
            effectivedate: row.effectivedate.clone(),
            typeid: row.typeid.clone(),
            noticetype: row.noticetype.clone(),
            lastchanged: row.lastchanged.clone(),
            reason: row.reason.clone(),
            externalreference: row.externalreference.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketNoticeMarketnoticedata1PrimaryKey {
    pub noticeid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketNoticeMarketnoticedata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketNoticeMarketnoticedata1Row<'data> {
    type Row<'other> = MarketNoticeMarketnoticedata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.noticeid == row.noticeid
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketNoticeMarketnoticedata1Row<'data> {
    type PrimaryKey = MarketNoticeMarketnoticedata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketNoticeMarketnoticedata1PrimaryKey {
    type Row<'other> = MarketNoticeMarketnoticedata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.noticeid == row.noticeid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketNoticeMarketnoticedata1PrimaryKey {
    type PrimaryKey = MarketNoticeMarketnoticedata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketNoticeMarketnoticedata1 {
    type Builder = MarketNoticeMarketnoticedata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "noticeid",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "typeid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "noticetype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "externalreference",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketNoticeMarketnoticedata1Builder {
            noticeid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            effectivedate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            typeid_array: arrow::array::builder::StringBuilder::new(),
            noticetype_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
            reason_array: arrow::array::builder::StringBuilder::new(),
            externalreference_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .noticeid_array
            .append_value({
                let mut val = row.noticeid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .effectivedate_array
            .append_option(row.effectivedate.map(|val| val.timestamp()));
        builder.typeid_array.append_option(row.typeid());
        builder.noticetype_array.append_option(row.noticetype());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
        builder.reason_array.append_option(row.reason());
        builder.externalreference_array.append_option(row.externalreference());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.noticeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.typeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.noticetype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.externalreference_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketNoticeMarketnoticedata1Builder {
    noticeid_array: arrow::array::builder::Decimal128Builder,
    effectivedate_array: arrow::array::builder::TimestampSecondBuilder,
    typeid_array: arrow::array::builder::StringBuilder,
    noticetype_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
    reason_array: arrow::array::builder::StringBuilder,
    externalreference_array: arrow::array::builder::StringBuilder,
}
pub struct MarketNoticeMarketnoticetype1;
pub struct MarketNoticeMarketnoticetype1Mapping([usize; 4]);
/// # Summary
///
/// ## MARKETNOTICETYPE
///  _MARKETNOTICETYPE sets out the different types of market notices (e.g. market systems)._
///
/// * Data Set Name: Market Notice
/// * File Name: Marketnoticetype
/// * Data Version: 1
///
/// # Description
///  MARKETNOTICETYPE data is public, so is available to all participants. Source MARKETNOTICETYPE updates whenever market notice types change.
///
///
///
/// # Primary Key Columns
///
/// * TYPEID
#[derive(Debug, PartialEq, Eq)]
pub struct MarketNoticeMarketnoticetype1Row<'data> {
    /// Identifier for market notice type
    pub typeid: core::ops::Range<usize>,
    /// Type description
    pub description: core::ops::Range<usize>,
    /// Not used
    pub raisedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketNoticeMarketnoticetype1Row<'data> {
    pub fn typeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.typeid.clone())
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
    pub fn raisedby(&self) -> Option<&str> {
        if self.raisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.raisedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MarketNoticeMarketnoticetype1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_NOTICE";
    const TABLE_NAME: &'static str = "MARKETNOTICETYPE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketNoticeMarketnoticetype1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TYPEID",
        "DESCRIPTION",
        "RAISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketNoticeMarketnoticetype1Row<'row>;
    type FieldMapping = MarketNoticeMarketnoticetype1Mapping;
    type PrimaryKey = MarketNoticeMarketnoticetype1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketNoticeMarketnoticetype1Row {
            typeid: row.get_range("typeid", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            raisedby: row.get_opt_range("raisedby", field_mapping.0[2])?,
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
        Ok(MarketNoticeMarketnoticetype1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketNoticeMarketnoticetype1PrimaryKey {
        MarketNoticeMarketnoticetype1PrimaryKey {
            typeid: row.typeid().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "market_notice_marketnoticetype_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketNoticeMarketnoticetype1Row {
            typeid: row.typeid.clone(),
            description: row.description.clone(),
            raisedby: row.raisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketNoticeMarketnoticetype1PrimaryKey {
    pub typeid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MarketNoticeMarketnoticetype1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketNoticeMarketnoticetype1Row<'data> {
    type Row<'other> = MarketNoticeMarketnoticetype1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.typeid() == row.typeid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketNoticeMarketnoticetype1Row<'data> {
    type PrimaryKey = MarketNoticeMarketnoticetype1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.typeid() == key.typeid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketNoticeMarketnoticetype1PrimaryKey {
    type Row<'other> = MarketNoticeMarketnoticetype1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.typeid == row.typeid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketNoticeMarketnoticetype1PrimaryKey {
    type PrimaryKey = MarketNoticeMarketnoticetype1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.typeid == key.typeid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketNoticeMarketnoticetype1 {
    type Builder = MarketNoticeMarketnoticetype1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "typeid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketNoticeMarketnoticetype1Builder {
            typeid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            raisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.typeid_array.append_value(row.typeid());
        builder.description_array.append_option(row.description());
        builder.raisedby_array.append_option(row.raisedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.typeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketNoticeMarketnoticetype1Builder {
    typeid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    raisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct MarketNoticeParticipantnoticetrk1;
pub struct MarketNoticeParticipantnoticetrk1Mapping([usize; 3]);
/// # Summary
///
/// ## PARTICIPANTNOTICETRK
///  _PARTICIPANTNOTICETRK provides the cross-reference between participant market notices and participants._
///
/// * Data Set Name: Market Notice
/// * File Name: Participantnoticetrk
/// * Data Version: 1
///
/// # Description
///  PARTICIPANTNOTICETRK data is Confidential to the relevant participant. Source PARTICIPANTNOTICETRK updates immediately, whenever a participant notice is issued.
///
///
///
/// # Primary Key Columns
///
/// * NOTICEID
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct MarketNoticeParticipantnoticetrk1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Market notice identifier
    pub noticeid: rust_decimal::Decimal,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketNoticeParticipantnoticetrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for MarketNoticeParticipantnoticetrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_NOTICE";
    const TABLE_NAME: &'static str = "PARTICIPANTNOTICETRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketNoticeParticipantnoticetrk1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "NOTICEID",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketNoticeParticipantnoticetrk1Row<'row>;
    type FieldMapping = MarketNoticeParticipantnoticetrk1Mapping;
    type PrimaryKey = MarketNoticeParticipantnoticetrk1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketNoticeParticipantnoticetrk1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            noticeid: row
                .get_custom_parsed_at_idx(
                    "noticeid",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
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
        Ok(MarketNoticeParticipantnoticetrk1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketNoticeParticipantnoticetrk1PrimaryKey {
        MarketNoticeParticipantnoticetrk1PrimaryKey {
            noticeid: row.noticeid,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "market_notice_participantnoticetrk_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketNoticeParticipantnoticetrk1Row {
            participantid: row.participantid.clone(),
            noticeid: row.noticeid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketNoticeParticipantnoticetrk1PrimaryKey {
    pub noticeid: rust_decimal::Decimal,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MarketNoticeParticipantnoticetrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketNoticeParticipantnoticetrk1Row<'data> {
    type Row<'other> = MarketNoticeParticipantnoticetrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.noticeid == row.noticeid && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketNoticeParticipantnoticetrk1Row<'data> {
    type PrimaryKey = MarketNoticeParticipantnoticetrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketNoticeParticipantnoticetrk1PrimaryKey {
    type Row<'other> = MarketNoticeParticipantnoticetrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.noticeid == row.noticeid && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketNoticeParticipantnoticetrk1PrimaryKey {
    type PrimaryKey = MarketNoticeParticipantnoticetrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketNoticeParticipantnoticetrk1 {
    type Builder = MarketNoticeParticipantnoticetrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "noticeid",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketNoticeParticipantnoticetrk1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            noticeid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
        builder
            .noticeid_array
            .append_value({
                let mut val = row.noticeid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.noticeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketNoticeParticipantnoticetrk1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    noticeid_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
