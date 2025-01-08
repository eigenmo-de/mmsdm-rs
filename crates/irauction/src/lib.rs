#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct IrauctionConfigAuction1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionConfigAuction1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionConfigAuction1 {
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
pub struct IrauctionConfigAuction1Mapping([usize; 9]);
/// # Summary
///
/// ## AUCTION
///  _AUCTION holds auction details. AUCTION is new in March 2003 to support SRA Inter-Temporal Linking._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction
/// * Data Version: 1
///
/// # Description
///  AUCTION is public data, and is available to all participants. Source Static. Volume 4 records per year
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionConfigAuction1Row<'data> {
    /// Unique id for each auction date
    pub auctionid: core::ops::Range<usize>,
    /// Auction date
    pub auctiondate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Open date for bidding
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Close date for bidding
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Description of an auction
    pub description: core::ops::Range<usize>,
    /// &nbsp;
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub authorisedby: core::ops::Range<usize>,
    /// &nbsp;
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionConfigAuction1Row<'data> {
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
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
impl mmsdm_core::GetTable for IrauctionConfigAuction1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION_CONFIG";
    const TABLE_NAME: &'static str = "AUCTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionConfigAuction1Mapping([
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
        "AUCTIONID",
        "AUCTIONDATE",
        "NOTIFYDATE",
        "STARTDATE",
        "ENDDATE",
        "DESCRIPTION",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionConfigAuction1Row<'row>;
    type FieldMapping = IrauctionConfigAuction1Mapping;
    type PrimaryKey = IrauctionConfigAuction1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionConfigAuction1Row {
            auctionid: row.get_range("auctionid", field_mapping.0[0])?,
            auctiondate: row
                .get_opt_custom_parsed_at_idx(
                    "auctiondate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            notifydate: row
                .get_opt_custom_parsed_at_idx(
                    "notifydate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            description: row.get_opt_range("description", field_mapping.0[5])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[7])?,
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
        Ok(IrauctionConfigAuction1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionConfigAuction1PrimaryKey {
        IrauctionConfigAuction1PrimaryKey {
            auctionid: row.auctionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_config_auction_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionConfigAuction1Row {
            auctionid: row.auctionid.clone(),
            auctiondate: row.auctiondate.clone(),
            notifydate: row.notifydate.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            description: row.description.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuction1PrimaryKey {
    pub auctionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionConfigAuction1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuction1Row<'data> {
    type Row<'other> = IrauctionConfigAuction1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionConfigAuction1Row<'data> {
    type PrimaryKey = IrauctionConfigAuction1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuction1PrimaryKey {
    type Row<'other> = IrauctionConfigAuction1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionConfigAuction1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuction1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionConfigAuction1 {
    type Builder = IrauctionConfigAuction1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "auctiondate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "notifydate",
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
                    "description",
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
        IrauctionConfigAuction1Builder {
            auctionid_array: arrow::array::builder::StringBuilder::new(),
            auctiondate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            notifydate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.auctionid_array.append_value(row.auctionid());
        builder
            .auctiondate_array
            .append_option(row.auctiondate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .notifydate_array
            .append_option(row.notifydate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder.description_array.append_option(row.description());
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
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctiondate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.notifydate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
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
pub struct IrauctionConfigAuction1Builder {
    auctionid_array: arrow::array::builder::StringBuilder,
    auctiondate_array: arrow::array::builder::TimestampMillisecondBuilder,
    notifydate_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    description_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionConfigAuctionCalendar2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionConfigAuctionCalendar2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionConfigAuctionCalendar2 {
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
pub struct IrauctionConfigAuctionCalendar2Mapping([usize; 12]);
/// # Summary
///
/// ## AUCTION_CALENDAR
///  _AUCTION_CALENDAR holds the definitions of each auction quarter in a contract year. AUCTION_CALENDAR supports the Settlement Residue Auction._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Calendar
/// * Data Version: 2
///
/// # Description
///  AUCTION_CALENDAR is public data, and is available to all participants. Source Updates are usually quarterly by the SRA team. Volume AUCTION_CALENDAR shows a maximum of 16 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionConfigAuctionCalendar2Row<'data> {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// First day of SRA Contract Quarter expressed as Date
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last day of SRA Contract Quarter expressed as Date
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Default notification date
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Date for payment by Participant
    pub paymentdate: Option<chrono::NaiveDateTime>,
    /// Date of reconciliation for the quarter
    pub reconciliationdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The date the Prelim Purchase Statement is generated
    pub prelimpurchasestmtdate: Option<chrono::NaiveDateTime>,
    /// The date the Prelim Proceeds Statement is generated
    pub prelimproceedsstmtdate: Option<chrono::NaiveDateTime>,
    /// The date the Final Purchase Statement is generated
    pub finalpurchasestmtdate: Option<chrono::NaiveDateTime>,
    /// The date the Final Proceeds Statement is generated
    pub finalproceedsstmtdate: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> IrauctionConfigAuctionCalendar2Row<'data> {}
impl mmsdm_core::GetTable for IrauctionConfigAuctionCalendar2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "IRAUCTION_CONFIG";
    const TABLE_NAME: &'static str = "AUCTION_CALENDAR";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionConfigAuctionCalendar2Mapping([
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
        "CONTRACTYEAR",
        "QUARTER",
        "STARTDATE",
        "ENDDATE",
        "NOTIFYDATE",
        "PAYMENTDATE",
        "RECONCILIATIONDATE",
        "LASTCHANGED",
        "PRELIMPURCHASESTMTDATE",
        "PRELIMPROCEEDSSTMTDATE",
        "FINALPURCHASESTMTDATE",
        "FINALPROCEEDSSTMTDATE",
    ];
    type Row<'row> = IrauctionConfigAuctionCalendar2Row<'row>;
    type FieldMapping = IrauctionConfigAuctionCalendar2Mapping;
    type PrimaryKey = IrauctionConfigAuctionCalendar2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionConfigAuctionCalendar2Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            notifydate: row
                .get_opt_custom_parsed_at_idx(
                    "notifydate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            paymentdate: row
                .get_opt_custom_parsed_at_idx(
                    "paymentdate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reconciliationdate: row
                .get_opt_custom_parsed_at_idx(
                    "reconciliationdate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            prelimpurchasestmtdate: row
                .get_opt_custom_parsed_at_idx(
                    "prelimpurchasestmtdate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            prelimproceedsstmtdate: row
                .get_opt_custom_parsed_at_idx(
                    "prelimproceedsstmtdate",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            finalpurchasestmtdate: row
                .get_opt_custom_parsed_at_idx(
                    "finalpurchasestmtdate",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            finalproceedsstmtdate: row
                .get_opt_custom_parsed_at_idx(
                    "finalproceedsstmtdate",
                    field_mapping.0[11],
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
        Ok(IrauctionConfigAuctionCalendar2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionConfigAuctionCalendar2PrimaryKey {
        IrauctionConfigAuctionCalendar2PrimaryKey {
            contractyear: row.contractyear,
            quarter: row.quarter,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_config_auction_calendar_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionConfigAuctionCalendar2Row {
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            notifydate: row.notifydate.clone(),
            paymentdate: row.paymentdate.clone(),
            reconciliationdate: row.reconciliationdate.clone(),
            lastchanged: row.lastchanged.clone(),
            prelimpurchasestmtdate: row.prelimpurchasestmtdate.clone(),
            prelimproceedsstmtdate: row.prelimproceedsstmtdate.clone(),
            finalpurchasestmtdate: row.finalpurchasestmtdate.clone(),
            finalproceedsstmtdate: row.finalproceedsstmtdate.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionCalendar2PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionConfigAuctionCalendar2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuctionCalendar2Row<'data> {
    type Row<'other> = IrauctionConfigAuctionCalendar2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionCalendar2Row<'data> {
    type PrimaryKey = IrauctionConfigAuctionCalendar2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuctionCalendar2PrimaryKey {
    type Row<'other> = IrauctionConfigAuctionCalendar2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionConfigAuctionCalendar2PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionCalendar2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionConfigAuctionCalendar2 {
    type Builder = IrauctionConfigAuctionCalendar2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
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
                    "notifydate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "paymentdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reconciliationdate",
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
                    "prelimpurchasestmtdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "prelimproceedsstmtdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "finalpurchasestmtdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "finalproceedsstmtdate",
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
        IrauctionConfigAuctionCalendar2Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            notifydate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            paymentdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            reconciliationdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            prelimpurchasestmtdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            prelimproceedsstmtdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            finalpurchasestmtdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            finalproceedsstmtdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .notifydate_array
            .append_option(row.notifydate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .paymentdate_array
            .append_option(row.paymentdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .reconciliationdate_array
            .append_option(
                row.reconciliationdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .prelimpurchasestmtdate_array
            .append_option(
                row.prelimpurchasestmtdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .prelimproceedsstmtdate_array
            .append_option(
                row.prelimproceedsstmtdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .finalpurchasestmtdate_array
            .append_option(
                row.finalpurchasestmtdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .finalproceedsstmtdate_array
            .append_option(
                row.finalproceedsstmtdate.map(|val| val.and_utc().timestamp_millis()),
            );
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.notifydate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.paymentdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reconciliationdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.prelimpurchasestmtdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.prelimproceedsstmtdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.finalpurchasestmtdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.finalproceedsstmtdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionConfigAuctionCalendar2Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    notifydate_array: arrow::array::builder::TimestampMillisecondBuilder,
    paymentdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    reconciliationdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    prelimpurchasestmtdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    prelimproceedsstmtdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    finalpurchasestmtdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    finalproceedsstmtdate_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionConfigAuctionIcAllocations2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionConfigAuctionIcAllocations2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionConfigAuctionIcAllocations2 {
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
pub struct IrauctionConfigAuctionIcAllocations2Mapping([usize; 12]);
/// # Summary
///
/// ## AUCTION_IC_ALLOCATIONS
///  _AUCTION_IC_ALLOCATIONS supports the Settlement Residue Auction by providing the basis for setting up contracts for individual tranches. AUCTION_IC_ALLOCATIONS shows the default definitions for the total number of units and proportion applicable to each directional interconnector for a specified auction quarter._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Ic Allocations
/// * Data Version: 2
///
/// # Description
///  AUCTION_IC_ALLOCATIONS is public data, and is available to all participants. Source Updates are usually quarterly as auctions are held from Settlement Residue Auction team's SRIS interface. Volume AUCTION_IC_ALLOCATIONS contains a maximum of 100 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * QUARTER
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionConfigAuctionIcAllocations2Row<'data> {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Number of units on the interconnector
    pub maximumunits: Option<rust_decimal::Decimal>,
    /// Percentage of the total residue for each Unit
    pub proportion: Option<rust_decimal::Decimal>,
    /// Daily auction fee
    pub auctionfee: Option<rust_decimal::Decimal>,
    /// Authorisation date
    pub changedate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this data set
    pub changedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Fees for Cancelled Units.
    pub auctionfee_sales: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionConfigAuctionIcAllocations2Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
    pub fn changedby(&self) -> Option<&str> {
        if self.changedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.changedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionConfigAuctionIcAllocations2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "IRAUCTION_CONFIG";
    const TABLE_NAME: &'static str = "AUCTION_IC_ALLOCATIONS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionConfigAuctionIcAllocations2Mapping([
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
        "CONTRACTYEAR",
        "QUARTER",
        "VERSIONNO",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "MAXIMUMUNITS",
        "PROPORTION",
        "AUCTIONFEE",
        "CHANGEDATE",
        "CHANGEDBY",
        "LASTCHANGED",
        "AUCTIONFEE_SALES",
    ];
    type Row<'row> = IrauctionConfigAuctionIcAllocations2Row<'row>;
    type FieldMapping = IrauctionConfigAuctionIcAllocations2Mapping;
    type PrimaryKey = IrauctionConfigAuctionIcAllocations2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionConfigAuctionIcAllocations2Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[3])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[4])?,
            maximumunits: row
                .get_opt_custom_parsed_at_idx(
                    "maximumunits",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            proportion: row
                .get_opt_custom_parsed_at_idx(
                    "proportion",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            auctionfee: row
                .get_opt_custom_parsed_at_idx(
                    "auctionfee",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            changedate: row
                .get_opt_custom_parsed_at_idx(
                    "changedate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            changedby: row.get_opt_range("changedby", field_mapping.0[9])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            auctionfee_sales: row
                .get_opt_custom_parsed_at_idx(
                    "auctionfee_sales",
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
        Ok(IrauctionConfigAuctionIcAllocations2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> IrauctionConfigAuctionIcAllocations2PrimaryKey {
        IrauctionConfigAuctionIcAllocations2PrimaryKey {
            contractyear: row.contractyear,
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            quarter: row.quarter,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_config_auction_ic_allocations_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionConfigAuctionIcAllocations2Row {
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            maximumunits: row.maximumunits.clone(),
            proportion: row.proportion.clone(),
            auctionfee: row.auctionfee.clone(),
            changedate: row.changedate.clone(),
            changedby: row.changedby.clone(),
            lastchanged: row.lastchanged.clone(),
            auctionfee_sales: row.auctionfee_sales.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionIcAllocations2PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub quarter: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionConfigAuctionIcAllocations2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionConfigAuctionIcAllocations2Row<'data> {
    type Row<'other> = IrauctionConfigAuctionIcAllocations2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.quarter == row.quarter && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionIcAllocations2Row<'data> {
    type PrimaryKey = IrauctionConfigAuctionIcAllocations2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.quarter == key.quarter && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionConfigAuctionIcAllocations2PrimaryKey {
    type Row<'other> = IrauctionConfigAuctionIcAllocations2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.quarter == row.quarter && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionIcAllocations2PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionIcAllocations2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.quarter == key.quarter && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionConfigAuctionIcAllocations2 {
    type Builder = IrauctionConfigAuctionIcAllocations2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
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
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "maximumunits",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "proportion",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "auctionfee",
                    arrow::datatypes::DataType::Decimal128(17, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "changedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "changedby",
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
                    "auctionfee_sales",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionConfigAuctionIcAllocations2Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            maximumunits_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            proportion_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            auctionfee_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            changedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            changedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            auctionfee_sales_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .maximumunits_array
            .append_option({
                row.maximumunits
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .proportion_array
            .append_option({
                row.proportion
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .auctionfee_array
            .append_option({
                row.auctionfee
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .changedate_array
            .append_option(row.changedate.map(|val| val.and_utc().timestamp_millis()));
        builder.changedby_array.append_option(row.changedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .auctionfee_sales_array
            .append_option({
                row.auctionfee_sales
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
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maximumunits_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.proportion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionfee_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.changedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.changedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionfee_sales_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionConfigAuctionIcAllocations2Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    maximumunits_array: arrow::array::builder::Decimal128Builder,
    proportion_array: arrow::array::builder::Decimal128Builder,
    auctionfee_array: arrow::array::builder::Decimal128Builder,
    changedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    changedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    auctionfee_sales_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionConfigAuctionRevenueEstimate1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionConfigAuctionRevenueEstimate1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionConfigAuctionRevenueEstimate1 {
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
pub struct IrauctionConfigAuctionRevenueEstimate1Mapping([usize; 11]);
/// # Summary
///
/// ## AUCTION_REVENUE_ESTIMATE
///  _AUCTION_REVENUE_ESTIMATE supports the Settlement Residue Auction, by holding the evaluator’s estimates of revenue for each month of a given quarter.<br>Since reserve prices are no longer applicable from the end of 2001, zero is used as a default to avoid rewriting the system._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Revenue Estimate
/// * Data Version: 1
///
/// # Description
///  AUCTION_REVENUE_ESTIMATE is public data, and is available to all participants. Source Updates are quarterly from SRA team via SRIS interface Volume AUCTION_REVENUE_ESTIMATE contains a maximum of 300 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * MONTHNO
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionConfigAuctionRevenueEstimate1Row<'data> {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: core::ops::Range<usize>,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Month number within quarter (1..3)
    pub monthno: rust_decimal::Decimal,
    /// First day of month as date
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last day of month as date
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Estimated Revenue
    pub revenue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionConfigAuctionRevenueEstimate1Row<'data> {
    pub fn valuationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.valuationid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionConfigAuctionRevenueEstimate1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION_CONFIG";
    const TABLE_NAME: &'static str = "AUCTION_REVENUE_ESTIMATE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionConfigAuctionRevenueEstimate1Mapping([
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
        "CONTRACTYEAR",
        "QUARTER",
        "VALUATIONID",
        "VERSIONNO",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "MONTHNO",
        "STARTDATE",
        "ENDDATE",
        "REVENUE",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionConfigAuctionRevenueEstimate1Row<'row>;
    type FieldMapping = IrauctionConfigAuctionRevenueEstimate1Mapping;
    type PrimaryKey = IrauctionConfigAuctionRevenueEstimate1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionConfigAuctionRevenueEstimate1Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            valuationid: row.get_range("valuationid", field_mapping.0[2])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[4])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[5])?,
            monthno: row
                .get_custom_parsed_at_idx(
                    "monthno",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            revenue: row
                .get_opt_custom_parsed_at_idx(
                    "revenue",
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
        Ok(IrauctionConfigAuctionRevenueEstimate1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
        IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
            contractyear: row.contractyear,
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            monthno: row.monthno,
            quarter: row.quarter,
            valuationid: row.valuationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_config_auction_revenue_estimate_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionConfigAuctionRevenueEstimate1Row {
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            valuationid: row.valuationid.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            monthno: row.monthno.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            revenue: row.revenue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub monthno: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionConfigAuctionRevenueEstimate1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionConfigAuctionRevenueEstimate1Row<'data> {
    type Row<'other> = IrauctionConfigAuctionRevenueEstimate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.monthno == row.monthno && self.quarter == row.quarter
            && self.valuationid() == row.valuationid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionRevenueEstimate1Row<'data> {
    type PrimaryKey = IrauctionConfigAuctionRevenueEstimate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.monthno == key.monthno && self.quarter == key.quarter
            && self.valuationid() == key.valuationid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
    type Row<'other> = IrauctionConfigAuctionRevenueEstimate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.monthno == row.monthno && self.quarter == row.quarter
            && self.valuationid == row.valuationid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionRevenueEstimate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.monthno == key.monthno && self.quarter == key.quarter
            && self.valuationid == key.valuationid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionConfigAuctionRevenueEstimate1 {
    type Builder = IrauctionConfigAuctionRevenueEstimate1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "valuationid",
                    arrow::datatypes::DataType::Utf8,
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
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "monthno",
                    arrow::datatypes::DataType::Decimal128(1, 0),
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
                    "revenue",
                    arrow::datatypes::DataType::Decimal128(17, 5),
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
        IrauctionConfigAuctionRevenueEstimate1Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            valuationid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            monthno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            revenue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder.valuationid_array.append_value(row.valuationid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .monthno_array
            .append_value({
                let mut val = row.monthno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .revenue_array
            .append_option({
                row.revenue
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
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.valuationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.monthno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.revenue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionConfigAuctionRevenueEstimate1Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    valuationid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    monthno_array: arrow::array::builder::Decimal128Builder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    revenue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionConfigAuctionRevenueTrack1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionConfigAuctionRevenueTrack1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionConfigAuctionRevenueTrack1 {
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
pub struct IrauctionConfigAuctionRevenueTrack1Mapping([usize; 10]);
/// # Summary
///
/// ## AUCTION_REVENUE_TRACK
///  _AUCTION_REVENUE_TRACK supports the Settlement Residue Auction, by holding the tracking information for each evaluator’s estimates for a given quarter. The status field is dynamic and is used for selection of estimates to be published._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Revenue Track
/// * Data Version: 1
///
/// # Description
///  AUCTION_REVENUE_TRACK is public data, and is available to all participants. Source Updates are quarterly after SRA team updates SRIS interface. Volume AUCTION_REVENUE_TRACK contains a maximum of 100 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionConfigAuctionRevenueTrack1Row<'data> {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: core::ops::Range<usize>,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Date from which the record change is applicable
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Internal use
    pub status: core::ops::Range<usize>,
    /// Reference to methodology document
    pub documentref: core::ops::Range<usize>,
    /// Date of authorisation for this record
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this record
    pub authorisedby: core::ops::Range<usize>,
    /// Date and time this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionConfigAuctionRevenueTrack1Row<'data> {
    pub fn valuationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.valuationid.clone())
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
    pub fn documentref(&self) -> Option<&str> {
        if self.documentref.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.documentref.clone(),
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
impl mmsdm_core::GetTable for IrauctionConfigAuctionRevenueTrack1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION_CONFIG";
    const TABLE_NAME: &'static str = "AUCTION_REVENUE_TRACK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionConfigAuctionRevenueTrack1Mapping([
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
        "CONTRACTYEAR",
        "QUARTER",
        "VALUATIONID",
        "VERSIONNO",
        "EFFECTIVEDATE",
        "STATUS",
        "DOCUMENTREF",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionConfigAuctionRevenueTrack1Row<'row>;
    type FieldMapping = IrauctionConfigAuctionRevenueTrack1Mapping;
    type PrimaryKey = IrauctionConfigAuctionRevenueTrack1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionConfigAuctionRevenueTrack1Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            valuationid: row.get_range("valuationid", field_mapping.0[2])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            status: row.get_opt_range("status", field_mapping.0[5])?,
            documentref: row.get_opt_range("documentref", field_mapping.0[6])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[8])?,
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
        Ok(IrauctionConfigAuctionRevenueTrack1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> IrauctionConfigAuctionRevenueTrack1PrimaryKey {
        IrauctionConfigAuctionRevenueTrack1PrimaryKey {
            contractyear: row.contractyear,
            quarter: row.quarter,
            valuationid: row.valuationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_config_auction_revenue_track_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionConfigAuctionRevenueTrack1Row {
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            valuationid: row.valuationid.clone(),
            versionno: row.versionno.clone(),
            effectivedate: row.effectivedate.clone(),
            status: row.status.clone(),
            documentref: row.documentref.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionRevenueTrack1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionConfigAuctionRevenueTrack1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionConfigAuctionRevenueTrack1Row<'data> {
    type Row<'other> = IrauctionConfigAuctionRevenueTrack1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
            && self.valuationid() == row.valuationid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionRevenueTrack1Row<'data> {
    type PrimaryKey = IrauctionConfigAuctionRevenueTrack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
            && self.valuationid() == key.valuationid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionConfigAuctionRevenueTrack1PrimaryKey {
    type Row<'other> = IrauctionConfigAuctionRevenueTrack1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
            && self.valuationid == row.valuationid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionRevenueTrack1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionRevenueTrack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
            && self.valuationid == key.valuationid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionConfigAuctionRevenueTrack1 {
    type Builder = IrauctionConfigAuctionRevenueTrack1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "valuationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
                    "status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "documentref",
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
        IrauctionConfigAuctionRevenueTrack1Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            valuationid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            status_array: arrow::array::builder::StringBuilder::new(),
            documentref_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder.valuationid_array.append_value(row.valuationid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .effectivedate_array
            .append_option(
                row.effectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.status_array.append_option(row.status());
        builder.documentref_array.append_option(row.documentref());
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
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.valuationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.documentref_array.finish())
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
pub struct IrauctionConfigAuctionRevenueTrack1Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    valuationid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    status_array: arrow::array::builder::StringBuilder,
    documentref_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionConfigAuctionRpEstimate1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionConfigAuctionRpEstimate1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionConfigAuctionRpEstimate1 {
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
pub struct IrauctionConfigAuctionRpEstimate1Mapping([usize; 8]);
/// # Summary
///
/// ## AUCTION_RP_ESTIMATE
///  _AUCTION_RP_ESTIMATE supports the Settlement Residue Auction, by holding the evaluator’s estimates of revenue prices for a given quarter.<br>Since reserve prices are no longer applicable from the end of 2001, zero is used as a default to avoid rewriting the system._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Rp Estimate
/// * Data Version: 1
///
/// # Description
///  AUCTION_RP_ESTIMATE is public data, and is available to all participants. Source Updates are quarterly by SRA team via SRIS interface. Volume This view contains a maximum of 100 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionConfigAuctionRpEstimate1Row<'data> {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: core::ops::Range<usize>,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Estimate of reserve price
    pub rpestimate: Option<rust_decimal::Decimal>,
    /// Last date and time record was changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionConfigAuctionRpEstimate1Row<'data> {
    pub fn valuationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.valuationid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionConfigAuctionRpEstimate1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION_CONFIG";
    const TABLE_NAME: &'static str = "AUCTION_RP_ESTIMATE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionConfigAuctionRpEstimate1Mapping([
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
        "CONTRACTYEAR",
        "QUARTER",
        "VALUATIONID",
        "VERSIONNO",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "RPESTIMATE",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionConfigAuctionRpEstimate1Row<'row>;
    type FieldMapping = IrauctionConfigAuctionRpEstimate1Mapping;
    type PrimaryKey = IrauctionConfigAuctionRpEstimate1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionConfigAuctionRpEstimate1Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            valuationid: row.get_range("valuationid", field_mapping.0[2])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[4])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[5])?,
            rpestimate: row
                .get_opt_custom_parsed_at_idx(
                    "rpestimate",
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
        Ok(IrauctionConfigAuctionRpEstimate1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionConfigAuctionRpEstimate1PrimaryKey {
        IrauctionConfigAuctionRpEstimate1PrimaryKey {
            contractyear: row.contractyear,
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            quarter: row.quarter,
            valuationid: row.valuationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_config_auction_rp_estimate_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionConfigAuctionRpEstimate1Row {
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            valuationid: row.valuationid.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            rpestimate: row.rpestimate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionRpEstimate1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionConfigAuctionRpEstimate1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuctionRpEstimate1Row<'data> {
    type Row<'other> = IrauctionConfigAuctionRpEstimate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.quarter == row.quarter && self.valuationid() == row.valuationid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionRpEstimate1Row<'data> {
    type PrimaryKey = IrauctionConfigAuctionRpEstimate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.quarter == key.quarter && self.valuationid() == key.valuationid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuctionRpEstimate1PrimaryKey {
    type Row<'other> = IrauctionConfigAuctionRpEstimate1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.quarter == row.quarter && self.valuationid == row.valuationid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionConfigAuctionRpEstimate1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionRpEstimate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.quarter == key.quarter && self.valuationid == key.valuationid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionConfigAuctionRpEstimate1 {
    type Builder = IrauctionConfigAuctionRpEstimate1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "valuationid",
                    arrow::datatypes::DataType::Utf8,
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
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rpestimate",
                    arrow::datatypes::DataType::Decimal128(17, 5),
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
        IrauctionConfigAuctionRpEstimate1Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            valuationid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            rpestimate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder.valuationid_array.append_value(row.valuationid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .rpestimate_array
            .append_option({
                row.rpestimate
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
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.valuationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rpestimate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionConfigAuctionRpEstimate1Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    valuationid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    rpestimate_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionConfigAuctionTranche1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionConfigAuctionTranche1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionConfigAuctionTranche1 {
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
pub struct IrauctionConfigAuctionTranche1Mapping([usize; 10]);
/// # Summary
///
/// ## AUCTION_TRANCHE
///  _AUCTION_TRANCHE supports the Settlement Residue Auction, by holding the default definitions for the percentage number of units allocated and dates applicable to each tranche for a specified auction quarter. This information provides the basis for setting up contracts for individual tranches._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Tranche
/// * Data Version: 1
///
/// # Description
///  AUCTION_TRANCHE is public data, and is available to all participants. Source Updates are quarterly from SRA team via SRIS interface. Volume AUCTION_TRANCHE contains a maximum of 100 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * TRANCHE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionConfigAuctionTranche1Row<'data> {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    pub tranche: rust_decimal::Decimal,
    /// Default date of the auction
    pub auctiondate: Option<chrono::NaiveDateTime>,
    /// Default date participants notified of details
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Percentage of units allocated to the tranche
    pub unitallocation: Option<rust_decimal::Decimal>,
    /// Date of changing this record
    pub changedate: Option<chrono::NaiveDateTime>,
    /// Name of person who changed this record
    pub changedby: core::ops::Range<usize>,
    /// Date and time record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionConfigAuctionTranche1Row<'data> {
    pub fn changedby(&self) -> Option<&str> {
        if self.changedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.changedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionConfigAuctionTranche1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION_CONFIG";
    const TABLE_NAME: &'static str = "AUCTION_TRANCHE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionConfigAuctionTranche1Mapping([
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
        "CONTRACTYEAR",
        "QUARTER",
        "VERSIONNO",
        "TRANCHE",
        "AUCTIONDATE",
        "NOTIFYDATE",
        "UNITALLOCATION",
        "CHANGEDATE",
        "CHANGEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionConfigAuctionTranche1Row<'row>;
    type FieldMapping = IrauctionConfigAuctionTranche1Mapping;
    type PrimaryKey = IrauctionConfigAuctionTranche1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionConfigAuctionTranche1Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tranche: row
                .get_custom_parsed_at_idx(
                    "tranche",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            auctiondate: row
                .get_opt_custom_parsed_at_idx(
                    "auctiondate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            notifydate: row
                .get_opt_custom_parsed_at_idx(
                    "notifydate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            unitallocation: row
                .get_opt_custom_parsed_at_idx(
                    "unitallocation",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            changedate: row
                .get_opt_custom_parsed_at_idx(
                    "changedate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            changedby: row.get_opt_range("changedby", field_mapping.0[8])?,
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
        Ok(IrauctionConfigAuctionTranche1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionConfigAuctionTranche1PrimaryKey {
        IrauctionConfigAuctionTranche1PrimaryKey {
            contractyear: row.contractyear,
            quarter: row.quarter,
            tranche: row.tranche,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_config_auction_tranche_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionConfigAuctionTranche1Row {
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            versionno: row.versionno.clone(),
            tranche: row.tranche.clone(),
            auctiondate: row.auctiondate.clone(),
            notifydate: row.notifydate.clone(),
            unitallocation: row.unitallocation.clone(),
            changedate: row.changedate.clone(),
            changedby: row.changedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionTranche1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub tranche: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionConfigAuctionTranche1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuctionTranche1Row<'data> {
    type Row<'other> = IrauctionConfigAuctionTranche1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
            && self.tranche == row.tranche && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionConfigAuctionTranche1Row<'data> {
    type PrimaryKey = IrauctionConfigAuctionTranche1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
            && self.tranche == key.tranche && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionConfigAuctionTranche1PrimaryKey {
    type Row<'other> = IrauctionConfigAuctionTranche1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
            && self.tranche == row.tranche && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionConfigAuctionTranche1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionTranche1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
            && self.tranche == key.tranche && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionConfigAuctionTranche1 {
    type Builder = IrauctionConfigAuctionTranche1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tranche",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "auctiondate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "notifydate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitallocation",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "changedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "changedby",
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
        IrauctionConfigAuctionTranche1Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            tranche_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            auctiondate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            notifydate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            unitallocation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            changedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            changedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .tranche_array
            .append_value({
                let mut val = row.tranche;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .auctiondate_array
            .append_option(row.auctiondate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .notifydate_array
            .append_option(row.notifydate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .unitallocation_array
            .append_option({
                row.unitallocation
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .changedate_array
            .append_option(row.changedate.map(|val| val.and_utc().timestamp_millis()));
        builder.changedby_array.append_option(row.changedby());
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
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tranche_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctiondate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.notifydate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitallocation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.changedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.changedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionConfigAuctionTranche1Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    tranche_array: arrow::array::builder::Decimal128Builder,
    auctiondate_array: arrow::array::builder::TimestampMillisecondBuilder,
    notifydate_array: arrow::array::builder::TimestampMillisecondBuilder,
    unitallocation_array: arrow::array::builder::Decimal128Builder,
    changedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    changedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementConfigResiduecontractpayments1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementConfigResiduecontractpayments1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementConfigResiduecontractpayments1 {
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
pub struct SettlementConfigResiduecontractpayments1Mapping([usize; 3]);
/// # Summary
///
/// ## RESIDUECONTRACTPAYMENTS
///  _RESIDUECONTRACTPAYMENTS shows Settlement Residue Auction payment Participant notifications._
///
/// * Data Set Name: Settlement Config
/// * File Name: Residuecontractpayments
/// * Data Version: 1
///
/// # Description
///  RESIDUECONTRACTPAYMENTS data is confidential to the relevant participant.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementConfigResiduecontractpayments1Row<'data> {
    /// SRA Contract ID
    pub contractid: core::ops::Range<usize>,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// Date and time this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementConfigResiduecontractpayments1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementConfigResiduecontractpayments1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENT_CONFIG";
    const TABLE_NAME: &'static str = "RESIDUECONTRACTPAYMENTS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementConfigResiduecontractpayments1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "PARTICIPANTID",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementConfigResiduecontractpayments1Row<'row>;
    type FieldMapping = SettlementConfigResiduecontractpayments1Mapping;
    type PrimaryKey = SettlementConfigResiduecontractpayments1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementConfigResiduecontractpayments1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
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
        Ok(SettlementConfigResiduecontractpayments1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> SettlementConfigResiduecontractpayments1PrimaryKey {
        SettlementConfigResiduecontractpayments1PrimaryKey {
            contractid: row.contractid().to_string(),
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlement_config_residuecontractpayments_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementConfigResiduecontractpayments1Row {
            contractid: row.contractid.clone(),
            participantid: row.participantid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigResiduecontractpayments1PrimaryKey {
    pub contractid: alloc::string::String,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SettlementConfigResiduecontractpayments1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigResiduecontractpayments1Row<'data> {
    type Row<'other> = SettlementConfigResiduecontractpayments1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementConfigResiduecontractpayments1Row<'data> {
    type PrimaryKey = SettlementConfigResiduecontractpayments1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow
for SettlementConfigResiduecontractpayments1PrimaryKey {
    type Row<'other> = SettlementConfigResiduecontractpayments1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for SettlementConfigResiduecontractpayments1PrimaryKey {
    type PrimaryKey = SettlementConfigResiduecontractpayments1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementConfigResiduecontractpayments1 {
    type Builder = SettlementConfigResiduecontractpayments1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
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
        SettlementConfigResiduecontractpayments1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
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
pub struct SettlementConfigResiduecontractpayments1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionBidsFileTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionBidsFileTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionBidsFileTrk1 {
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
pub struct IrauctionBidsFileTrk1Mapping([usize; 8]);
/// # Summary
///
/// ## RESIDUEFILETRK
///  _RESIDUEFILETRK records all Settlement Residue Auction offers submitted by participants._
///
/// * Data Set Name: Irauction Bids
/// * File Name: File Trk
/// * Data Version: 1
///
/// # Description
///  RESIDUEFILETRK data is confidential to each participant Source RESIDUEFILETRK updates are ad hoc from participants Volume Assuming quarterly contracts RESIDUEFILETRK contains a maximum of 5,000 records per annum. Each bid file can contain many bids for each auction. Participants can input multiple bids (with the last acknowledged file being used in the auction).
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionBidsFileTrk1Row<'data> {
    /// SRA ContractID
    pub contractid: core::ops::Range<usize>,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// Date-Time SRA offer was loaded
    pub loaddate: chrono::NaiveDateTime,
    /// SRA offer file name
    pub filename: core::ops::Range<usize>,
    /// SRA acknowledgment file name
    pub ackfilename: core::ops::Range<usize>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionBidsFileTrk1Row<'data> {
    pub fn contractid(&self) -> Option<&str> {
        if self.contractid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.contractid.clone(),
                ),
            )
        }
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
    }
    pub fn ackfilename(&self) -> Option<&str> {
        if self.ackfilename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.ackfilename.clone(),
                ),
            )
        }
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
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionBidsFileTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION_BIDS";
    const TABLE_NAME: &'static str = "FILE_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionBidsFileTrk1Mapping([
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
        "CONTRACTID",
        "PARTICIPANTID",
        "LOADDATE",
        "FILENAME",
        "ACKFILENAME",
        "STATUS",
        "LASTCHANGED",
        "AUCTIONID",
    ];
    type Row<'row> = IrauctionBidsFileTrk1Row<'row>;
    type FieldMapping = IrauctionBidsFileTrk1Mapping;
    type PrimaryKey = IrauctionBidsFileTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionBidsFileTrk1Row {
            contractid: row.get_opt_range("contractid", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            loaddate: row
                .get_custom_parsed_at_idx(
                    "loaddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            filename: row.get_opt_range("filename", field_mapping.0[3])?,
            ackfilename: row.get_opt_range("ackfilename", field_mapping.0[4])?,
            status: row.get_opt_range("status", field_mapping.0[5])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            auctionid: row.get_range("auctionid", field_mapping.0[7])?,
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
        Ok(IrauctionBidsFileTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionBidsFileTrk1PrimaryKey {
        IrauctionBidsFileTrk1PrimaryKey {
            auctionid: row.auctionid().to_string(),
            loaddate: row.loaddate,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_bids_file_trk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionBidsFileTrk1Row {
            contractid: row.contractid.clone(),
            participantid: row.participantid.clone(),
            loaddate: row.loaddate.clone(),
            filename: row.filename.clone(),
            ackfilename: row.ackfilename.clone(),
            status: row.status.clone(),
            lastchanged: row.lastchanged.clone(),
            auctionid: row.auctionid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionBidsFileTrk1PrimaryKey {
    pub auctionid: alloc::string::String,
    pub loaddate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionBidsFileTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionBidsFileTrk1Row<'data> {
    type Row<'other> = IrauctionBidsFileTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid() && self.loaddate == row.loaddate
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionBidsFileTrk1Row<'data> {
    type PrimaryKey = IrauctionBidsFileTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid && self.loaddate == key.loaddate
            && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionBidsFileTrk1PrimaryKey {
    type Row<'other> = IrauctionBidsFileTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid() && self.loaddate == row.loaddate
            && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionBidsFileTrk1PrimaryKey {
    type PrimaryKey = IrauctionBidsFileTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.loaddate == key.loaddate
            && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionBidsFileTrk1 {
    type Builder = IrauctionBidsFileTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "loaddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "filename",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ackfilename",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "status",
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
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionBidsFileTrk1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            loaddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            ackfilename_array: arrow::array::builder::StringBuilder::new(),
            status_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            auctionid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_option(row.contractid());
        builder.participantid_array.append_value(row.participantid());
        builder.loaddate_array.append_value(row.loaddate.and_utc().timestamp_millis());
        builder.filename_array.append_option(row.filename());
        builder.ackfilename_array.append_option(row.ackfilename());
        builder.status_array.append_option(row.status());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.auctionid_array.append_value(row.auctionid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.loaddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ackfilename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionBidsFileTrk1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    loaddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    ackfilename_array: arrow::array::builder::StringBuilder,
    status_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    auctionid_array: arrow::array::builder::StringBuilder,
}
pub struct IrauctionResidueBidTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResidueBidTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResidueBidTrk1 {
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
pub struct IrauctionResidueBidTrk1Mapping([usize; 6]);
/// # Summary
///
/// ## RESIDUE_BID_TRK
///  _RESIDUE_BID_TRK supports the Settlement Residue Auction, by detailing which bid was used for which SRA Contract run._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Bid Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_BID_TRK updates are usually quarterly from participants before an Auction. RESIDUE_BID_TRK data is confidential to the relevant participant. RESIDUE_BID_TRK excludes contracts and versions without a valid publication date (i.e invalid bids are ignored). Volume Assuming monthly contracts, RESIDUE_BID_TRK shows a maximum of 500 records per year.
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResidueBidTrk1Row<'data> {
    /// SRA Contract unique identifier
    pub contractid: core::ops::Range<usize>,
    /// Version of Bid used
    pub versionno: rust_decimal::Decimal,
    /// Identifier of participant
    pub participantid: core::ops::Range<usize>,
    /// Date and time bid used
    pub bidloaddate: Option<chrono::NaiveDateTime>,
    /// Date and time this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResidueBidTrk1Row<'data> {
    pub fn contractid(&self) -> Option<&str> {
        if self.contractid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.contractid.clone(),
                ),
            )
        }
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResidueBidTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_BID_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResidueBidTrk1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "VERSIONNO",
        "PARTICIPANTID",
        "BIDLOADDATE",
        "LASTCHANGED",
        "AUCTIONID",
    ];
    type Row<'row> = IrauctionResidueBidTrk1Row<'row>;
    type FieldMapping = IrauctionResidueBidTrk1Mapping;
    type PrimaryKey = IrauctionResidueBidTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResidueBidTrk1Row {
            contractid: row.get_opt_range("contractid", field_mapping.0[0])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            bidloaddate: row
                .get_opt_custom_parsed_at_idx(
                    "bidloaddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            auctionid: row.get_range("auctionid", field_mapping.0[5])?,
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
        Ok(IrauctionResidueBidTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResidueBidTrk1PrimaryKey {
        IrauctionResidueBidTrk1PrimaryKey {
            auctionid: row.auctionid().to_string(),
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_residue_bid_trk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResidueBidTrk1Row {
            contractid: row.contractid.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            bidloaddate: row.bidloaddate.clone(),
            lastchanged: row.lastchanged.clone(),
            auctionid: row.auctionid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueBidTrk1PrimaryKey {
    pub auctionid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionResidueBidTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueBidTrk1Row<'data> {
    type Row<'other> = IrauctionResidueBidTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid()
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionResidueBidTrk1Row<'data> {
    type PrimaryKey = IrauctionResidueBidTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueBidTrk1PrimaryKey {
    type Row<'other> = IrauctionResidueBidTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid() && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResidueBidTrk1PrimaryKey {
    type PrimaryKey = IrauctionResidueBidTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResidueBidTrk1 {
    type Builder = IrauctionResidueBidTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    true,
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
                    "bidloaddate",
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
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionResidueBidTrk1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            bidloaddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            auctionid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_option(row.contractid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder
            .bidloaddate_array
            .append_option(row.bidloaddate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.auctionid_array.append_value(row.auctionid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidloaddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResidueBidTrk1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    bidloaddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    auctionid_array: arrow::array::builder::StringBuilder,
}
pub struct IrauctionResidueContracts1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResidueContracts1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResidueContracts1 {
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
pub struct IrauctionResidueContracts1Mapping([usize; 18]);
/// # Summary
///
/// ## RESIDUE_CONTRACTS
///  _RESIDUE_CONTRACTS supports the Settlement Residue Auction, by holding the contract details for each period for which a residue contract will be offered._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Contracts
/// * Data Version: 1
///
/// # Description
///  RESIDUE_CONTRACTS data is public, so is available to all participants. Source RESIDUE_CONTRACTS updates are quarterly by AEMO. Volume Assuming quarterly contracts, RESIDUE_CONTRACTS contains a maximum of 50 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * TRANCHE
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResidueContracts1Row<'data> {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    pub tranche: rust_decimal::Decimal,
    /// Unique identifier for each SRA Contract as specified by AEMO
    pub contractid: core::ops::Range<usize>,
    /// SRA Quarter start date
    pub startdate: Option<chrono::NaiveDateTime>,
    /// SRA Quarter end date
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Open date of bidding, calculated as RNOTIFYDATE business days before the auction date
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Close date of bidding, calculated as RAUCDATE business days before the contract start date
    pub auctiondate: Option<chrono::NaiveDateTime>,
    /// Identifies methodology used
    pub calcmethod: core::ops::Range<usize>,
    /// Authorisation date for this record
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    pub authorisedby: core::ops::Range<usize>,
    /// Date notification posted
    pub notifypostdate: Option<chrono::NaiveDateTime>,
    /// Name of notifying person
    pub notifyby: core::ops::Range<usize>,
    /// Date of publishing the auction results
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Name of publishing officer or process
    pub postedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Description of Contract
    pub description: core::ops::Range<usize>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResidueContracts1Row<'data> {
    pub fn contractid(&self) -> Option<&str> {
        if self.contractid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.contractid.clone(),
                ),
            )
        }
    }
    pub fn calcmethod(&self) -> Option<&str> {
        if self.calcmethod.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.calcmethod.clone(),
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
    pub fn notifyby(&self) -> Option<&str> {
        if self.notifyby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.notifyby.clone(),
                ),
            )
        }
    }
    pub fn postedby(&self) -> Option<&str> {
        if self.postedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.postedby.clone(),
                ),
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
    pub fn auctionid(&self) -> Option<&str> {
        if self.auctionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.auctionid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionResidueContracts1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_CONTRACTS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResidueContracts1Mapping([
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
        "CONTRACTYEAR",
        "QUARTER",
        "TRANCHE",
        "CONTRACTID",
        "STARTDATE",
        "ENDDATE",
        "NOTIFYDATE",
        "AUCTIONDATE",
        "CALCMETHOD",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "NOTIFYPOSTDATE",
        "NOTIFYBY",
        "POSTDATE",
        "POSTEDBY",
        "LASTCHANGED",
        "DESCRIPTION",
        "AUCTIONID",
    ];
    type Row<'row> = IrauctionResidueContracts1Row<'row>;
    type FieldMapping = IrauctionResidueContracts1Mapping;
    type PrimaryKey = IrauctionResidueContracts1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResidueContracts1Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tranche: row
                .get_custom_parsed_at_idx(
                    "tranche",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractid: row.get_opt_range("contractid", field_mapping.0[3])?,
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
            notifydate: row
                .get_opt_custom_parsed_at_idx(
                    "notifydate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            auctiondate: row
                .get_opt_custom_parsed_at_idx(
                    "auctiondate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            calcmethod: row.get_opt_range("calcmethod", field_mapping.0[8])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[10])?,
            notifypostdate: row
                .get_opt_custom_parsed_at_idx(
                    "notifypostdate",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            notifyby: row.get_opt_range("notifyby", field_mapping.0[12])?,
            postdate: row
                .get_opt_custom_parsed_at_idx(
                    "postdate",
                    field_mapping.0[13],
                    mmsdm_core::mms_datetime::parse,
                )?,
            postedby: row.get_opt_range("postedby", field_mapping.0[14])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[15],
                    mmsdm_core::mms_datetime::parse,
                )?,
            description: row.get_opt_range("description", field_mapping.0[16])?,
            auctionid: row.get_opt_range("auctionid", field_mapping.0[17])?,
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
        Ok(IrauctionResidueContracts1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResidueContracts1PrimaryKey {
        IrauctionResidueContracts1PrimaryKey {
            contractyear: row.contractyear,
            quarter: row.quarter,
            tranche: row.tranche,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_residue_contracts_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResidueContracts1Row {
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            tranche: row.tranche.clone(),
            contractid: row.contractid.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            notifydate: row.notifydate.clone(),
            auctiondate: row.auctiondate.clone(),
            calcmethod: row.calcmethod.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            notifypostdate: row.notifypostdate.clone(),
            notifyby: row.notifyby.clone(),
            postdate: row.postdate.clone(),
            postedby: row.postedby.clone(),
            lastchanged: row.lastchanged.clone(),
            description: row.description.clone(),
            auctionid: row.auctionid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueContracts1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub tranche: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionResidueContracts1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueContracts1Row<'data> {
    type Row<'other> = IrauctionResidueContracts1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
            && self.tranche == row.tranche
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionResidueContracts1Row<'data> {
    type PrimaryKey = IrauctionResidueContracts1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
            && self.tranche == key.tranche
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueContracts1PrimaryKey {
    type Row<'other> = IrauctionResidueContracts1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
            && self.tranche == row.tranche
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResidueContracts1PrimaryKey {
    type PrimaryKey = IrauctionResidueContracts1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
            && self.tranche == key.tranche
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResidueContracts1 {
    type Builder = IrauctionResidueContracts1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tranche",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
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
                    "notifydate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "auctiondate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calcmethod",
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
                    "notifypostdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "notifyby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "postdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "postedby",
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
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionResidueContracts1Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            tranche_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            notifydate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            auctiondate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            calcmethod_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            notifypostdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            notifyby_array: arrow::array::builder::StringBuilder::new(),
            postdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            postedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            auctionid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .tranche_array
            .append_value({
                let mut val = row.tranche;
                val.rescale(0);
                val.mantissa()
            });
        builder.contractid_array.append_option(row.contractid());
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .notifydate_array
            .append_option(row.notifydate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .auctiondate_array
            .append_option(row.auctiondate.map(|val| val.and_utc().timestamp_millis()));
        builder.calcmethod_array.append_option(row.calcmethod());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .notifypostdate_array
            .append_option(
                row.notifypostdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.notifyby_array.append_option(row.notifyby());
        builder
            .postdate_array
            .append_option(row.postdate.map(|val| val.and_utc().timestamp_millis()));
        builder.postedby_array.append_option(row.postedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.description_array.append_option(row.description());
        builder.auctionid_array.append_option(row.auctionid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tranche_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.notifydate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctiondate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calcmethod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.notifypostdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.notifyby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.postdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.postedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResidueContracts1Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    tranche_array: arrow::array::builder::Decimal128Builder,
    contractid_array: arrow::array::builder::StringBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    notifydate_array: arrow::array::builder::TimestampMillisecondBuilder,
    auctiondate_array: arrow::array::builder::TimestampMillisecondBuilder,
    calcmethod_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    notifypostdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    notifyby_array: arrow::array::builder::StringBuilder,
    postdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    postedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    description_array: arrow::array::builder::StringBuilder,
    auctionid_array: arrow::array::builder::StringBuilder,
}
pub struct IrauctionResidueConData2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResidueConData2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResidueConData2 {
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
pub struct IrauctionResidueConData2Mapping([usize; 9]);
/// # Summary
///
/// ## RESIDUE_CON_DATA
///  _RESIDUE_CON_DATA supports the Settlement Residue Auction, by holding for each participant the confidential data from the auction. RESIDUE_CON_DATA joins to RESIDUE_PUBLIC_DATA and RESIDUE_TRK._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Data
/// * Data Version: 2
///
/// # Description
///  Source RESIDUE_CON_DATA refreshes whenever a Settlement Residue Auction is run (i.e. quarterly). RESIDUE_CON_DATA data is confidential to the relevant participant. RESIDUE_CON_DATA excludes contracts and versions without a valid publication date (i.e invalid bids are ignored). Volume RESIDUE_CON_DATA shows a maximum of 6000 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResidueConData2Row<'data> {
    /// SRA Contract unique identifier
    pub contractid: core::ops::Range<usize>,
    /// Contract run version
    pub versionno: rust_decimal::Decimal,
    /// Identifier of Contracted Participant
    pub participantid: core::ops::Range<usize>,
    /// Identifier of Contracted Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Units purchased on the directional interconnector (i.e. Contracted quantity)
    pub unitspurchased: Option<rust_decimal::Decimal>,
    /// Payment due (i.e. total purchase price)
    pub linkpayment: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The number of cancelled Units for all Auction Participants.
    pub secondary_units_sold: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResidueConData2Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResidueConData2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_CON_DATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResidueConData2Mapping([
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
        "CONTRACTID",
        "VERSIONNO",
        "PARTICIPANTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "UNITSPURCHASED",
        "LINKPAYMENT",
        "LASTCHANGED",
        "SECONDARY_UNITS_SOLD",
    ];
    type Row<'row> = IrauctionResidueConData2Row<'row>;
    type FieldMapping = IrauctionResidueConData2Mapping;
    type PrimaryKey = IrauctionResidueConData2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResidueConData2Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[3])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[4])?,
            unitspurchased: row
                .get_opt_custom_parsed_at_idx(
                    "unitspurchased",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            linkpayment: row
                .get_opt_custom_parsed_at_idx(
                    "linkpayment",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            secondary_units_sold: row
                .get_opt_custom_parsed_at_idx(
                    "secondary_units_sold",
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
        Ok(IrauctionResidueConData2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResidueConData2PrimaryKey {
        IrauctionResidueConData2PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_residue_con_data_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResidueConData2Row {
            contractid: row.contractid.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            unitspurchased: row.unitspurchased.clone(),
            linkpayment: row.linkpayment.clone(),
            lastchanged: row.lastchanged.clone(),
            secondary_units_sold: row.secondary_units_sold.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueConData2PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionResidueConData2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueConData2Row<'data> {
    type Row<'other> = IrauctionResidueConData2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionResidueConData2Row<'data> {
    type PrimaryKey = IrauctionResidueConData2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueConData2PrimaryKey {
    type Row<'other> = IrauctionResidueConData2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResidueConData2PrimaryKey {
    type PrimaryKey = IrauctionResidueConData2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResidueConData2 {
    type Builder = IrauctionResidueConData2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
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
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "unitspurchased",
                    arrow::datatypes::DataType::Decimal128(17, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "linkpayment",
                    arrow::datatypes::DataType::Decimal128(17, 5),
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
                    "secondary_units_sold",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionResidueConData2Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            unitspurchased_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            linkpayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            secondary_units_sold_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .unitspurchased_array
            .append_option({
                row.unitspurchased
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .linkpayment_array
            .append_option({
                row.linkpayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .secondary_units_sold_array
            .append_option({
                row.secondary_units_sold
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitspurchased_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.linkpayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.secondary_units_sold_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResidueConData2Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    unitspurchased_array: arrow::array::builder::Decimal128Builder,
    linkpayment_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    secondary_units_sold_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionResidueConEstimatesTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResidueConEstimatesTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResidueConEstimatesTrk1 {
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
pub struct IrauctionResidueConEstimatesTrk1Mapping([usize; 6]);
/// # Summary
///
/// ## RESIDUE_CON_ESTIMATES_TRK
///  _RESIDUE_CON_ESTIMATES_TRK supports the Settlement Residue Auction, by holding the tracking details of the estimates used to generate the reserve price for each contract._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Estimates Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_CON_ESTIMATES_TRK updates are quarterly by SRA team. Volume Assuming monthly contracts, RESIDUE_CON_ESTIMATES_TRK shows a maximum of 50 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * CONTRACTYEAR
/// * QUARTER
/// * VALUATIONID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResidueConEstimatesTrk1Row<'data> {
    /// SRA Contract unique identifier
    pub contractid: core::ops::Range<usize>,
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Contract Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: core::ops::Range<usize>,
    /// Version of a record, as nominated by the participant
    pub versionno: Option<rust_decimal::Decimal>,
    /// Date and time this record was changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResidueConEstimatesTrk1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn valuationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.valuationid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResidueConEstimatesTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_CON_ESTIMATES_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResidueConEstimatesTrk1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "CONTRACTYEAR",
        "QUARTER",
        "VALUATIONID",
        "VERSIONNO",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionResidueConEstimatesTrk1Row<'row>;
    type FieldMapping = IrauctionResidueConEstimatesTrk1Mapping;
    type PrimaryKey = IrauctionResidueConEstimatesTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResidueConEstimatesTrk1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            quarter: row
                .get_custom_parsed_at_idx(
                    "quarter",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            valuationid: row.get_range("valuationid", field_mapping.0[3])?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
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
        Ok(IrauctionResidueConEstimatesTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResidueConEstimatesTrk1PrimaryKey {
        IrauctionResidueConEstimatesTrk1PrimaryKey {
            contractid: row.contractid().to_string(),
            contractyear: row.contractyear,
            quarter: row.quarter,
            valuationid: row.valuationid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_residue_con_estimates_trk_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResidueConEstimatesTrk1Row {
            contractid: row.contractid.clone(),
            contractyear: row.contractyear.clone(),
            quarter: row.quarter.clone(),
            valuationid: row.valuationid.clone(),
            versionno: row.versionno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueConEstimatesTrk1PrimaryKey {
    pub contractid: alloc::string::String,
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionResidueConEstimatesTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueConEstimatesTrk1Row<'data> {
    type Row<'other> = IrauctionResidueConEstimatesTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid() && self.contractyear == row.contractyear
            && self.quarter == row.quarter && self.valuationid() == row.valuationid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionResidueConEstimatesTrk1Row<'data> {
    type PrimaryKey = IrauctionResidueConEstimatesTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.contractyear == key.contractyear
            && self.quarter == key.quarter && self.valuationid() == key.valuationid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueConEstimatesTrk1PrimaryKey {
    type Row<'other> = IrauctionResidueConEstimatesTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.contractyear == row.contractyear
            && self.quarter == row.quarter && self.valuationid == row.valuationid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResidueConEstimatesTrk1PrimaryKey {
    type PrimaryKey = IrauctionResidueConEstimatesTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.contractyear == key.contractyear
            && self.quarter == key.quarter && self.valuationid == key.valuationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResidueConEstimatesTrk1 {
    type Builder = IrauctionResidueConEstimatesTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "quarter",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "valuationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
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
        IrauctionResidueConEstimatesTrk1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            quarter_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            valuationid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
        builder
            .contractyear_array
            .append_value({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .quarter_array
            .append_value({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
        builder.valuationid_array.append_value(row.valuationid());
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
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.valuationid_array.finish())
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
pub struct IrauctionResidueConEstimatesTrk1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    contractyear_array: arrow::array::builder::Decimal128Builder,
    quarter_array: arrow::array::builder::Decimal128Builder,
    valuationid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionResidueConFunds1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResidueConFunds1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResidueConFunds1 {
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
pub struct IrauctionResidueConFunds1Mapping([usize; 11]);
/// # Summary
///
/// ## RESIDUE_CON_FUNDS
///  _RESIDUE_CON_FUNDS supports the Settlement Residue Auction, by holding the fund details for each contract._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Funds
/// * Data Version: 1
///
/// # Description
///  RESIDUE_CON_FUNDS data is public, so is available to all participants. Source RESIDUE_CON_FUNDS updates are quarterly from SRA team via SRIS interface. Volume Assuming quarterly contracts, RESIDUE_CON_FUNDS contains a maximum of 600 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResidueConFunds1Row<'data> {
    /// SRA Contract unique identifier as specified by AEMO
    pub contractid: core::ops::Range<usize>,
    /// Identifier for the Contracted Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Actual number of units allocated based on the auction default percentage for the tranche and the total number of units to be auctioned for this quarter
    pub defaultunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter
    pub rolloverunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter because they were not taken up by the participant
    pub reallocatedunits: Option<rust_decimal::Decimal>,
    /// Total units offered for Contract
    pub unitsoffered: Option<rust_decimal::Decimal>,
    /// Average reserve price calculated from the selected estimates
    pub meanreserveprice: Option<rust_decimal::Decimal>,
    /// Scaling factor for regional Frequency control Ancillary Service requirement
    pub scalefactor: Option<rust_decimal::Decimal>,
    /// Actual reserve price
    pub actualreserveprice: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResidueConFunds1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResidueConFunds1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_CON_FUNDS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResidueConFunds1Mapping([
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
        "CONTRACTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "DEFAULTUNITS",
        "ROLLOVERUNITS",
        "REALLOCATEDUNITS",
        "UNITSOFFERED",
        "MEANRESERVEPRICE",
        "SCALEFACTOR",
        "ACTUALRESERVEPRICE",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionResidueConFunds1Row<'row>;
    type FieldMapping = IrauctionResidueConFunds1Mapping;
    type PrimaryKey = IrauctionResidueConFunds1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResidueConFunds1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[1])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[2])?,
            defaultunits: row
                .get_opt_custom_parsed_at_idx(
                    "defaultunits",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rolloverunits: row
                .get_opt_custom_parsed_at_idx(
                    "rolloverunits",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            reallocatedunits: row
                .get_opt_custom_parsed_at_idx(
                    "reallocatedunits",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unitsoffered: row
                .get_opt_custom_parsed_at_idx(
                    "unitsoffered",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meanreserveprice: row
                .get_opt_custom_parsed_at_idx(
                    "meanreserveprice",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            scalefactor: row
                .get_opt_custom_parsed_at_idx(
                    "scalefactor",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            actualreserveprice: row
                .get_opt_custom_parsed_at_idx(
                    "actualreserveprice",
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
        Ok(IrauctionResidueConFunds1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResidueConFunds1PrimaryKey {
        IrauctionResidueConFunds1PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_residue_con_funds_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResidueConFunds1Row {
            contractid: row.contractid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            defaultunits: row.defaultunits.clone(),
            rolloverunits: row.rolloverunits.clone(),
            reallocatedunits: row.reallocatedunits.clone(),
            unitsoffered: row.unitsoffered.clone(),
            meanreserveprice: row.meanreserveprice.clone(),
            scalefactor: row.scalefactor.clone(),
            actualreserveprice: row.actualreserveprice.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueConFunds1PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionResidueConFunds1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueConFunds1Row<'data> {
    type Row<'other> = IrauctionResidueConFunds1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionResidueConFunds1Row<'data> {
    type PrimaryKey = IrauctionResidueConFunds1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueConFunds1PrimaryKey {
    type Row<'other> = IrauctionResidueConFunds1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResidueConFunds1PrimaryKey {
    type PrimaryKey = IrauctionResidueConFunds1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResidueConFunds1 {
    type Builder = IrauctionResidueConFunds1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "defaultunits",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rolloverunits",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reallocatedunits",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitsoffered",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meanreserveprice",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "scalefactor",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "actualreserveprice",
                    arrow::datatypes::DataType::Decimal128(9, 2),
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
        IrauctionResidueConFunds1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            defaultunits_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            rolloverunits_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            reallocatedunits_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            unitsoffered_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            meanreserveprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            scalefactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            actualreserveprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .defaultunits_array
            .append_option({
                row.defaultunits
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .rolloverunits_array
            .append_option({
                row.rolloverunits
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .reallocatedunits_array
            .append_option({
                row.reallocatedunits
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .unitsoffered_array
            .append_option({
                row.unitsoffered
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .meanreserveprice_array
            .append_option({
                row.meanreserveprice
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .scalefactor_array
            .append_option({
                row.scalefactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .actualreserveprice_array
            .append_option({
                row.actualreserveprice
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.defaultunits_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rolloverunits_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reallocatedunits_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitsoffered_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meanreserveprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.scalefactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.actualreserveprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResidueConFunds1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    defaultunits_array: arrow::array::builder::Decimal128Builder,
    rolloverunits_array: arrow::array::builder::Decimal128Builder,
    reallocatedunits_array: arrow::array::builder::Decimal128Builder,
    unitsoffered_array: arrow::array::builder::Decimal128Builder,
    meanreserveprice_array: arrow::array::builder::Decimal128Builder,
    scalefactor_array: arrow::array::builder::Decimal128Builder,
    actualreserveprice_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionBidsFundsBid1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionBidsFundsBid1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionBidsFundsBid1 {
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
pub struct IrauctionBidsFundsBid1Mapping([usize; 8]);
/// # Summary
///
/// ## RESIDUE_FUNDS_BID
///  _RESIDUE_FUNDS_BID supports the Settlement Residue Auction, by showing the fund details for each SRA bid by each Participant._
///
/// * Data Set Name: Irauction Bids
/// * File Name: Funds Bid
/// * Data Version: 1
///
/// # Description
///  Source Participant's bid file. RESIDUE_FUNDS_BID data is confidential to the relevant participant. RESIDUE_FUNDS_BID shows a maximum of 30,000 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionBidsFundsBid1Row<'data> {
    /// SRA Contract identifier
    pub contractid: core::ops::Range<usize>,
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Date and time the batcher loaded the SRA offer
    pub loaddate: chrono::NaiveDateTime,
    /// Unique option identifier (1..20)
    pub optionid: rust_decimal::Decimal,
    /// Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Quantity of units bid for
    pub units: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionBidsFundsBid1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionBidsFundsBid1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION_BIDS";
    const TABLE_NAME: &'static str = "FUNDS_BID";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionBidsFundsBid1Mapping([
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
        "CONTRACTID",
        "PARTICIPANTID",
        "LOADDATE",
        "OPTIONID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "UNITS",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionBidsFundsBid1Row<'row>;
    type FieldMapping = IrauctionBidsFundsBid1Mapping;
    type PrimaryKey = IrauctionBidsFundsBid1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionBidsFundsBid1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            loaddate: row
                .get_custom_parsed_at_idx(
                    "loaddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            optionid: row
                .get_custom_parsed_at_idx(
                    "optionid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[4])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[5])?,
            units: row
                .get_opt_custom_parsed_at_idx(
                    "units",
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
        Ok(IrauctionBidsFundsBid1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionBidsFundsBid1PrimaryKey {
        IrauctionBidsFundsBid1PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            loaddate: row.loaddate,
            optionid: row.optionid,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_bids_funds_bid_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionBidsFundsBid1Row {
            contractid: row.contractid.clone(),
            participantid: row.participantid.clone(),
            loaddate: row.loaddate.clone(),
            optionid: row.optionid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            units: row.units.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionBidsFundsBid1PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub loaddate: chrono::NaiveDateTime,
    pub optionid: rust_decimal::Decimal,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionBidsFundsBid1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionBidsFundsBid1Row<'data> {
    type Row<'other> = IrauctionBidsFundsBid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.loaddate == row.loaddate && self.optionid == row.optionid
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionBidsFundsBid1Row<'data> {
    type PrimaryKey = IrauctionBidsFundsBid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.loaddate == key.loaddate && self.optionid == key.optionid
            && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionBidsFundsBid1PrimaryKey {
    type Row<'other> = IrauctionBidsFundsBid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.loaddate == row.loaddate && self.optionid == row.optionid
            && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionBidsFundsBid1PrimaryKey {
    type PrimaryKey = IrauctionBidsFundsBid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.loaddate == key.loaddate && self.optionid == key.optionid
            && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionBidsFundsBid1 {
    type Builder = IrauctionBidsFundsBid1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "loaddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "optionid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "units",
                    arrow::datatypes::DataType::Decimal128(5, 0),
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
        IrauctionBidsFundsBid1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            loaddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            optionid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            units_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
        builder.participantid_array.append_value(row.participantid());
        builder.loaddate_array.append_value(row.loaddate.and_utc().timestamp_millis());
        builder
            .optionid_array
            .append_value({
                let mut val = row.optionid;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .units_array
            .append_option({
                row.units
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.loaddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.optionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.units_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionBidsFundsBid1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    loaddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    optionid_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    units_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionResiduePriceBid1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResiduePriceBid1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResiduePriceBid1 {
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
pub struct IrauctionResiduePriceBid1Mapping([usize; 7]);
/// # Summary
///
/// ## RESIDUE_PRICE_BID
///  _RESIDUE_PRICE_BID supports the Settlement Residue Auction, holding the unit and bid price details for each participant._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Price Bid
/// * Data Version: 1
///
/// # Description
///  Source The participant's own bid file RESIDUE_PRICE_BID data is confidential to the relevant participant. The public version of the data is available to all auction participants post the associated auction date in RESIDUE_PRICE_FUNDS_BID. Volume RESIDUE_PRICE_BID shows a maximum of 10,000 records per year.
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResiduePriceBid1Row<'data> {
    /// Not to be used. Unique id for each SRA contract (specified by AEMO)
    pub contractid: core::ops::Range<usize>,
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Date and Time the batcher loaded the bid
    pub loaddate: chrono::NaiveDateTime,
    /// Unique option (bid) identifier (1..800)
    pub optionid: rust_decimal::Decimal,
    /// Price offered for each unit
    pub bidprice: Option<rust_decimal::Decimal>,
    /// Date and time this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResiduePriceBid1Row<'data> {
    pub fn contractid(&self) -> Option<&str> {
        if self.contractid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.contractid.clone(),
                ),
            )
        }
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResiduePriceBid1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_PRICE_BID";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResiduePriceBid1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "PARTICIPANTID",
        "LOADDATE",
        "OPTIONID",
        "BIDPRICE",
        "LASTCHANGED",
        "AUCTIONID",
    ];
    type Row<'row> = IrauctionResiduePriceBid1Row<'row>;
    type FieldMapping = IrauctionResiduePriceBid1Mapping;
    type PrimaryKey = IrauctionResiduePriceBid1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResiduePriceBid1Row {
            contractid: row.get_opt_range("contractid", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            loaddate: row
                .get_custom_parsed_at_idx(
                    "loaddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            optionid: row
                .get_custom_parsed_at_idx(
                    "optionid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bidprice: row
                .get_opt_custom_parsed_at_idx(
                    "bidprice",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            auctionid: row.get_range("auctionid", field_mapping.0[6])?,
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
        Ok(IrauctionResiduePriceBid1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResiduePriceBid1PrimaryKey {
        IrauctionResiduePriceBid1PrimaryKey {
            auctionid: row.auctionid().to_string(),
            loaddate: row.loaddate,
            optionid: row.optionid,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_residue_price_bid_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResiduePriceBid1Row {
            contractid: row.contractid.clone(),
            participantid: row.participantid.clone(),
            loaddate: row.loaddate.clone(),
            optionid: row.optionid.clone(),
            bidprice: row.bidprice.clone(),
            lastchanged: row.lastchanged.clone(),
            auctionid: row.auctionid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResiduePriceBid1PrimaryKey {
    pub auctionid: alloc::string::String,
    pub loaddate: chrono::NaiveDateTime,
    pub optionid: rust_decimal::Decimal,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionResiduePriceBid1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResiduePriceBid1Row<'data> {
    type Row<'other> = IrauctionResiduePriceBid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid() && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionResiduePriceBid1Row<'data> {
    type PrimaryKey = IrauctionResiduePriceBid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid && self.loaddate == key.loaddate
            && self.optionid == key.optionid && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResiduePriceBid1PrimaryKey {
    type Row<'other> = IrauctionResiduePriceBid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid() && self.loaddate == row.loaddate
            && self.optionid == row.optionid && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResiduePriceBid1PrimaryKey {
    type PrimaryKey = IrauctionResiduePriceBid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.loaddate == key.loaddate
            && self.optionid == key.optionid && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResiduePriceBid1 {
    type Builder = IrauctionResiduePriceBid1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "loaddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "optionid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bidprice",
                    arrow::datatypes::DataType::Decimal128(17, 5),
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
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionResiduePriceBid1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            loaddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            optionid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            bidprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            auctionid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_option(row.contractid());
        builder.participantid_array.append_value(row.participantid());
        builder.loaddate_array.append_value(row.loaddate.and_utc().timestamp_millis());
        builder
            .optionid_array
            .append_value({
                let mut val = row.optionid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .bidprice_array
            .append_option({
                row.bidprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.auctionid_array.append_value(row.auctionid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.loaddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.optionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResiduePriceBid1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    loaddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    optionid_array: arrow::array::builder::Decimal128Builder,
    bidprice_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    auctionid_array: arrow::array::builder::StringBuilder,
}
pub struct IrauctionResiduePriceFundsBid1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResiduePriceFundsBid1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResiduePriceFundsBid1 {
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
pub struct IrauctionResiduePriceFundsBid1Mapping([usize; 8]);
/// # Summary
///
/// ## RESIDUE_PRICE_FUNDS_BID
///  _RESIDUE_PRICE_FUNDS_BIDshows the bids producing the auction outcome, without exposing participant-specific details. RESIDUE_PRICE_FUNDS_BID is new in March 2003 to support SRA Inter-Temporal Linking._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Price Funds Bid
/// * Data Version: 1
///
/// # Description
///  RESIDUE_PRICE_FUNDS_BID data is public. The data is available to all auction participants post the associated auction date. Volume The volume is very dependent on the number of active bids. An indication is about 250,000 per year.
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * LINKEDBIDFLAG
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResiduePriceFundsBid1Row<'data> {
    /// Unique id for each contract specified by AEMO
    pub contractid: core::ops::Range<usize>,
    /// Unique interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Unique region identifier
    pub fromregionid: core::ops::Range<usize>,
    /// Quantity of units bid
    pub units: Option<rust_decimal::Decimal>,
    /// Price bid for each unit
    pub bidprice: Option<rust_decimal::Decimal>,
    /// A unique option id, with respect to the auction, created to show which bid elements are linked.
    pub linkedbidflag: rust_decimal::Decimal,
    /// Unique id for each auction date
    pub auctionid: core::ops::Range<usize>,
    /// Date and time this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResiduePriceFundsBid1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResiduePriceFundsBid1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_PRICE_FUNDS_BID";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResiduePriceFundsBid1Mapping([
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
        "CONTRACTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "UNITS",
        "BIDPRICE",
        "LINKEDBIDFLAG",
        "AUCTIONID",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionResiduePriceFundsBid1Row<'row>;
    type FieldMapping = IrauctionResiduePriceFundsBid1Mapping;
    type PrimaryKey = IrauctionResiduePriceFundsBid1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResiduePriceFundsBid1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[1])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[2])?,
            units: row
                .get_opt_custom_parsed_at_idx(
                    "units",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bidprice: row
                .get_opt_custom_parsed_at_idx(
                    "bidprice",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            linkedbidflag: row
                .get_custom_parsed_at_idx(
                    "linkedbidflag",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            auctionid: row.get_range("auctionid", field_mapping.0[6])?,
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
        Ok(IrauctionResiduePriceFundsBid1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResiduePriceFundsBid1PrimaryKey {
        IrauctionResiduePriceFundsBid1PrimaryKey {
            auctionid: row.auctionid().to_string(),
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            linkedbidflag: row.linkedbidflag,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_residue_price_funds_bid_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResiduePriceFundsBid1Row {
            contractid: row.contractid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            units: row.units.clone(),
            bidprice: row.bidprice.clone(),
            linkedbidflag: row.linkedbidflag.clone(),
            auctionid: row.auctionid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResiduePriceFundsBid1PrimaryKey {
    pub auctionid: alloc::string::String,
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub linkedbidflag: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionResiduePriceFundsBid1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResiduePriceFundsBid1Row<'data> {
    type Row<'other> = IrauctionResiduePriceFundsBid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid() && self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.linkedbidflag == row.linkedbidflag
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionResiduePriceFundsBid1Row<'data> {
    type PrimaryKey = IrauctionResiduePriceFundsBid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid && self.contractid() == key.contractid
            && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.linkedbidflag == key.linkedbidflag
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResiduePriceFundsBid1PrimaryKey {
    type Row<'other> = IrauctionResiduePriceFundsBid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid() && self.contractid == row.contractid()
            && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.linkedbidflag == row.linkedbidflag
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResiduePriceFundsBid1PrimaryKey {
    type PrimaryKey = IrauctionResiduePriceFundsBid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.linkedbidflag == key.linkedbidflag
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResiduePriceFundsBid1 {
    type Builder = IrauctionResiduePriceFundsBid1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "units",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bidprice",
                    arrow::datatypes::DataType::Decimal128(17, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "linkedbidflag",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "auctionid",
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
        IrauctionResiduePriceFundsBid1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            units_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            bidprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            linkedbidflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            auctionid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .units_array
            .append_option({
                row.units
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bidprice_array
            .append_option({
                row.bidprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .linkedbidflag_array
            .append_value({
                let mut val = row.linkedbidflag;
                val.rescale(0);
                val.mantissa()
            });
        builder.auctionid_array.append_value(row.auctionid());
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.units_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.linkedbidflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResiduePriceFundsBid1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    units_array: arrow::array::builder::Decimal128Builder,
    bidprice_array: arrow::array::builder::Decimal128Builder,
    linkedbidflag_array: arrow::array::builder::Decimal128Builder,
    auctionid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionResiduePublicData1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResiduePublicData1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResiduePublicData1 {
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
pub struct IrauctionResiduePublicData1Mapping([usize; 9]);
/// # Summary
///
/// ## RESIDUE_PUBLIC_DATA
///  _RESIDUE_PUBLIC_DATA shows the public auction results.<br>RESIDUE_PUBLIC_DATA supports the Settlement Residue Auction, by holding the public details of the auction for a given contract. RESIDUE_PUBLIC_DATA joins to RESIDUE_CON_DATA and RESIDUE._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Public Data
/// * Data Version: 1
///
/// # Description
///  RESIDUE_PUBLIC_DATA excludes contracts and versions without a valid publication date (i.e. invalid bids are ignored).  The data is available to all auction participants post the associated auction date. Source RESIDUE_PUBLIC_DATA updates are quarterly from NEMMCO. Volume RESIDUE_PUBLIC_DATA shows a maximum of 120 records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResiduePublicData1Row<'data> {
    /// Unique id for each contract to be specified by AEMO
    pub contractid: core::ops::Range<usize>,
    /// Version Number
    pub versionno: rust_decimal::Decimal,
    /// Unique interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Total units offered for auction
    pub unitsoffered: Option<rust_decimal::Decimal>,
    /// Units Sold (modified format and usage in March 2003 to support SRA Inter-Temporal Linking)
    pub unitssold: Option<rust_decimal::Decimal>,
    /// Clearing price
    pub clearingprice: Option<rust_decimal::Decimal>,
    /// Reserve price
    pub reserveprice: Option<rust_decimal::Decimal>,
    /// Date and time this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResiduePublicData1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResiduePublicData1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_PUBLIC_DATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResiduePublicData1Mapping([
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
        "CONTRACTID",
        "VERSIONNO",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "UNITSOFFERED",
        "UNITSSOLD",
        "CLEARINGPRICE",
        "RESERVEPRICE",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionResiduePublicData1Row<'row>;
    type FieldMapping = IrauctionResiduePublicData1Mapping;
    type PrimaryKey = IrauctionResiduePublicData1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResiduePublicData1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[3])?,
            unitsoffered: row
                .get_opt_custom_parsed_at_idx(
                    "unitsoffered",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unitssold: row
                .get_opt_custom_parsed_at_idx(
                    "unitssold",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearingprice: row
                .get_opt_custom_parsed_at_idx(
                    "clearingprice",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            reserveprice: row
                .get_opt_custom_parsed_at_idx(
                    "reserveprice",
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
        Ok(IrauctionResiduePublicData1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResiduePublicData1PrimaryKey {
        IrauctionResiduePublicData1PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_residue_public_data_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResiduePublicData1Row {
            contractid: row.contractid.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            unitsoffered: row.unitsoffered.clone(),
            unitssold: row.unitssold.clone(),
            clearingprice: row.clearingprice.clone(),
            reserveprice: row.reserveprice.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResiduePublicData1PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionResiduePublicData1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResiduePublicData1Row<'data> {
    type Row<'other> = IrauctionResiduePublicData1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionResiduePublicData1Row<'data> {
    type PrimaryKey = IrauctionResiduePublicData1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResiduePublicData1PrimaryKey {
    type Row<'other> = IrauctionResiduePublicData1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResiduePublicData1PrimaryKey {
    type PrimaryKey = IrauctionResiduePublicData1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResiduePublicData1 {
    type Builder = IrauctionResiduePublicData1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
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
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "unitsoffered",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitssold",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearingprice",
                    arrow::datatypes::DataType::Decimal128(17, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reserveprice",
                    arrow::datatypes::DataType::Decimal128(17, 5),
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
        IrauctionResiduePublicData1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            unitsoffered_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            unitssold_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            clearingprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            reserveprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(17, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .unitsoffered_array
            .append_option({
                row.unitsoffered
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .unitssold_array
            .append_option({
                row.unitssold
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .clearingprice_array
            .append_option({
                row.clearingprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .reserveprice_array
            .append_option({
                row.reserveprice
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitsoffered_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitssold_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearingprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reserveprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResiduePublicData1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    unitsoffered_array: arrow::array::builder::Decimal128Builder,
    unitssold_array: arrow::array::builder::Decimal128Builder,
    clearingprice_array: arrow::array::builder::Decimal128Builder,
    reserveprice_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionResidueTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionResidueTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionResidueTrk1 {
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
pub struct IrauctionResidueTrk1Mapping([usize; 10]);
/// # Summary
///
/// ## RESIDUE_TRK
///  _RESIDUE_TRK supports the Settlement Residue Auction, by showing the tracking records for different residue auction runs. RESIDUE_TRK joins to RESIDUE_PUBLIC_DATA and RESIDUE_CON_DATA._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_TRK updates whenever Settlement Residue Auctions are run and the results published (i.e. quarterly). The RESIDUE_TRK data is available to all participants post the associated auction date. Volume Assuming quarterly contracts, RESIDUE_TRK shows a maximum of 50 records per year.
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionResidueTrk1Row<'data> {
    /// SRA Contract identifier
    pub contractid: core::ops::Range<usize>,
    /// Contract run version
    pub versionno: rust_decimal::Decimal,
    /// Date auction results determined
    pub rundate: Option<chrono::NaiveDateTime>,
    /// Date results published
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer or process
    pub authorisedby: core::ops::Range<usize>,
    /// Date the run is authorised
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    pub postedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: core::ops::Range<usize>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionResidueTrk1Row<'data> {
    pub fn contractid(&self) -> Option<&str> {
        if self.contractid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.contractid.clone(),
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
    pub fn postedby(&self) -> Option<&str> {
        if self.postedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.postedby.clone(),
                ),
            )
        }
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
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionResidueTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "RESIDUE_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionResidueTrk1Mapping([
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
        "CONTRACTID",
        "VERSIONNO",
        "RUNDATE",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "POSTDATE",
        "POSTEDBY",
        "LASTCHANGED",
        "STATUS",
        "AUCTIONID",
    ];
    type Row<'row> = IrauctionResidueTrk1Row<'row>;
    type FieldMapping = IrauctionResidueTrk1Mapping;
    type PrimaryKey = IrauctionResidueTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionResidueTrk1Row {
            contractid: row.get_opt_range("contractid", field_mapping.0[0])?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rundate: row
                .get_opt_custom_parsed_at_idx(
                    "rundate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[4])?,
            postdate: row
                .get_opt_custom_parsed_at_idx(
                    "postdate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            postedby: row.get_opt_range("postedby", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            status: row.get_opt_range("status", field_mapping.0[8])?,
            auctionid: row.get_range("auctionid", field_mapping.0[9])?,
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
        Ok(IrauctionResidueTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionResidueTrk1PrimaryKey {
        IrauctionResidueTrk1PrimaryKey {
            auctionid: row.auctionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_residue_trk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionResidueTrk1Row {
            contractid: row.contractid.clone(),
            versionno: row.versionno.clone(),
            rundate: row.rundate.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            postdate: row.postdate.clone(),
            postedby: row.postedby.clone(),
            lastchanged: row.lastchanged.clone(),
            status: row.status.clone(),
            auctionid: row.auctionid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueTrk1PrimaryKey {
    pub auctionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for IrauctionResidueTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueTrk1Row<'data> {
    type Row<'other> = IrauctionResidueTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionResidueTrk1Row<'data> {
    type PrimaryKey = IrauctionResidueTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionResidueTrk1PrimaryKey {
    type Row<'other> = IrauctionResidueTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionResidueTrk1PrimaryKey {
    type PrimaryKey = IrauctionResidueTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionResidueTrk1 {
    type Builder = IrauctionResidueTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rundate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
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
                    "postdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "postedby",
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
                    "status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionResidueTrk1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            rundate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            postdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            postedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            status_array: arrow::array::builder::StringBuilder::new(),
            auctionid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_option(row.contractid());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .rundate_array
            .append_option(row.rundate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .postdate_array
            .append_option(row.postdate.map(|val| val.and_utc().timestamp_millis()));
        builder.postedby_array.append_option(row.postedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.status_array.append_option(row.status());
        builder.auctionid_array.append_value(row.auctionid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rundate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.postdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.postedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionResidueTrk1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    rundate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    postdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    postedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    status_array: arrow::array::builder::StringBuilder,
    auctionid_array: arrow::array::builder::StringBuilder,
}
pub struct IrauctionSraCashSecurity1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraCashSecurity1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraCashSecurity1 {
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
pub struct IrauctionSraCashSecurity1Mapping([usize; 10]);
/// # Summary
///
/// ## SRA_CASH_SECURITY
///  _Records the Cash Security details provided by an SRA Auction Participant as collateral to cover their Trading Position in the SRA market_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Cash Security
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraCashSecurity1Row<'data> {
    /// Unique identifier for the cash security.
    pub cash_security_id: core::ops::Range<usize>,
    /// Unique identifier for the auction participant lodging the cash security.
    pub participantid: core::ops::Range<usize>,
    /// Date AEMO received the Cash Security deposit
    pub provision_date: Option<chrono::NaiveDateTime>,
    /// Dollar amount of the cash security.
    pub cash_amount: Option<rust_decimal::Decimal>,
    /// The interest account ID for calculating the interest payment
    pub interest_acct_id: core::ops::Range<usize>,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Date the entire Cash Security amount was returned to the Auction Participant
    pub finalreturndate: Option<chrono::NaiveDateTime>,
    /// Returned Dollar amount of the Cash Security.
    pub cash_security_returned: Option<rust_decimal::Decimal>,
    /// Cash Security deleted date. For valid records, DeletionDate will be Null.
    pub deletiondate: Option<chrono::NaiveDateTime>,
    /// The date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraCashSecurity1Row<'data> {
    pub fn cash_security_id(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.cash_security_id.clone(),
        )
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
    pub fn interest_acct_id(&self) -> Option<&str> {
        if self.interest_acct_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.interest_acct_id.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionSraCashSecurity1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_CASH_SECURITY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraCashSecurity1Mapping([
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
        "CASH_SECURITY_ID",
        "PARTICIPANTID",
        "PROVISION_DATE",
        "CASH_AMOUNT",
        "INTEREST_ACCT_ID",
        "AUTHORISEDDATE",
        "FINALRETURNDATE",
        "CASH_SECURITY_RETURNED",
        "DELETIONDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionSraCashSecurity1Row<'row>;
    type FieldMapping = IrauctionSraCashSecurity1Mapping;
    type PrimaryKey = IrauctionSraCashSecurity1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraCashSecurity1Row {
            cash_security_id: row.get_range("cash_security_id", field_mapping.0[0])?,
            participantid: row.get_opt_range("participantid", field_mapping.0[1])?,
            provision_date: row
                .get_opt_custom_parsed_at_idx(
                    "provision_date",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            cash_amount: row
                .get_opt_custom_parsed_at_idx(
                    "cash_amount",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interest_acct_id: row.get_opt_range("interest_acct_id", field_mapping.0[4])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            finalreturndate: row
                .get_opt_custom_parsed_at_idx(
                    "finalreturndate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            cash_security_returned: row
                .get_opt_custom_parsed_at_idx(
                    "cash_security_returned",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            deletiondate: row
                .get_opt_custom_parsed_at_idx(
                    "deletiondate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(IrauctionSraCashSecurity1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraCashSecurity1PrimaryKey {
        IrauctionSraCashSecurity1PrimaryKey {
            cash_security_id: row.cash_security_id().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_sra_cash_security_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraCashSecurity1Row {
            cash_security_id: row.cash_security_id.clone(),
            participantid: row.participantid.clone(),
            provision_date: row.provision_date.clone(),
            cash_amount: row.cash_amount.clone(),
            interest_acct_id: row.interest_acct_id.clone(),
            authoriseddate: row.authoriseddate.clone(),
            finalreturndate: row.finalreturndate.clone(),
            cash_security_returned: row.cash_security_returned.clone(),
            deletiondate: row.deletiondate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraCashSecurity1PrimaryKey {
    pub cash_security_id: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionSraCashSecurity1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraCashSecurity1Row<'data> {
    type Row<'other> = IrauctionSraCashSecurity1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.cash_security_id() == row.cash_security_id()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionSraCashSecurity1Row<'data> {
    type PrimaryKey = IrauctionSraCashSecurity1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id() == key.cash_security_id
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraCashSecurity1PrimaryKey {
    type Row<'other> = IrauctionSraCashSecurity1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.cash_security_id == row.cash_security_id()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraCashSecurity1PrimaryKey {
    type PrimaryKey = IrauctionSraCashSecurity1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraCashSecurity1 {
    type Builder = IrauctionSraCashSecurity1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "cash_security_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "provision_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cash_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interest_acct_id",
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
                    "finalreturndate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cash_security_returned",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "deletiondate",
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
        IrauctionSraCashSecurity1Builder {
            cash_security_id_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            provision_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            cash_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            interest_acct_id_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            finalreturndate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            cash_security_returned_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            deletiondate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.cash_security_id_array.append_value(row.cash_security_id());
        builder.participantid_array.append_option(row.participantid());
        builder
            .provision_date_array
            .append_option(
                row.provision_date.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .cash_amount_array
            .append_option({
                row.cash_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder.interest_acct_id_array.append_option(row.interest_acct_id());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .finalreturndate_array
            .append_option(
                row.finalreturndate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .cash_security_returned_array
            .append_option({
                row.cash_security_returned
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .deletiondate_array
            .append_option(row.deletiondate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.cash_security_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.provision_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cash_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interest_acct_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.finalreturndate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cash_security_returned_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deletiondate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraCashSecurity1Builder {
    cash_security_id_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    provision_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    cash_amount_array: arrow::array::builder::Decimal128Builder,
    interest_acct_id_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    finalreturndate_array: arrow::array::builder::TimestampMillisecondBuilder,
    cash_security_returned_array: arrow::array::builder::Decimal128Builder,
    deletiondate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionSraFinancialAucpayDetail1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraFinancialAucpayDetail1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraFinancialAucpayDetail1 {
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
pub struct IrauctionSraFinancialAucpayDetail1Mapping([usize; 17]);
/// # Summary
///
/// ## SRA_FINANCIAL_AUCPAY_DETAIL
///  _Records details of the SRA financial auction payment_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Aucpay Detail
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraFinancialAucpayDetail1Row<'data> {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique  participant identifier
    pub participantid: core::ops::Range<usize>,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// The SRA contract identifier
    pub contractid: core::ops::Range<usize>,
    /// The Maximum Units Available for purchase in the Auction
    pub maximum_units: Option<rust_decimal::Decimal>,
    /// The total number of Allocated Units in the Auction, including Cancelled Units by an Auction Participant
    pub units_sold: Option<rust_decimal::Decimal>,
    /// The total number of units unpaid for in the auction
    pub shortfall_units: Option<rust_decimal::Decimal>,
    /// The reserve price of the auction
    pub reserve_price: Option<rust_decimal::Decimal>,
    /// The Market Clearing Price of the Auction
    pub clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO before shortfall
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The percentage of the auction proceeds allocated to the TNSP on the directional link
    pub allocation: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO, including shortfall
    pub net_payment_amount: Option<rust_decimal::Decimal>,
    /// The date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraFinancialAucpayDetail1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionSraFinancialAucpayDetail1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_FINANCIAL_AUCPAY_DETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraFinancialAucpayDetail1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SRA_YEAR",
        "SRA_QUARTER",
        "SRA_RUNNO",
        "PARTICIPANTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "CONTRACTID",
        "MAXIMUM_UNITS",
        "UNITS_SOLD",
        "SHORTFALL_UNITS",
        "RESERVE_PRICE",
        "CLEARING_PRICE",
        "PAYMENT_AMOUNT",
        "SHORTFALL_AMOUNT",
        "ALLOCATION",
        "NET_PAYMENT_AMOUNT",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionSraFinancialAucpayDetail1Row<'row>;
    type FieldMapping = IrauctionSraFinancialAucpayDetail1Mapping;
    type PrimaryKey = IrauctionSraFinancialAucpayDetail1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraFinancialAucpayDetail1Row {
            sra_year: row.get_parsed_at_idx("sra_year", field_mapping.0[0])?,
            sra_quarter: row.get_parsed_at_idx("sra_quarter", field_mapping.0[1])?,
            sra_runno: row.get_parsed_at_idx("sra_runno", field_mapping.0[2])?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[4])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[5])?,
            contractid: row.get_range("contractid", field_mapping.0[6])?,
            maximum_units: row
                .get_opt_custom_parsed_at_idx(
                    "maximum_units",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            units_sold: row
                .get_opt_custom_parsed_at_idx(
                    "units_sold",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            shortfall_units: row
                .get_opt_custom_parsed_at_idx(
                    "shortfall_units",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            reserve_price: row
                .get_opt_custom_parsed_at_idx(
                    "reserve_price",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearing_price: row
                .get_opt_custom_parsed_at_idx(
                    "clearing_price",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            payment_amount: row
                .get_opt_custom_parsed_at_idx(
                    "payment_amount",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            shortfall_amount: row
                .get_opt_custom_parsed_at_idx(
                    "shortfall_amount",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            allocation: row
                .get_opt_custom_parsed_at_idx(
                    "allocation",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            net_payment_amount: row
                .get_opt_custom_parsed_at_idx(
                    "net_payment_amount",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
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
        Ok(IrauctionSraFinancialAucpayDetail1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraFinancialAucpayDetail1PrimaryKey {
        IrauctionSraFinancialAucpayDetail1PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            sra_quarter: row.sra_quarter,
            sra_runno: row.sra_runno,
            sra_year: row.sra_year,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_financial_aucpay_detail_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraFinancialAucpayDetail1Row {
            sra_year: row.sra_year.clone(),
            sra_quarter: row.sra_quarter.clone(),
            sra_runno: row.sra_runno.clone(),
            participantid: row.participantid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            contractid: row.contractid.clone(),
            maximum_units: row.maximum_units.clone(),
            units_sold: row.units_sold.clone(),
            shortfall_units: row.shortfall_units.clone(),
            reserve_price: row.reserve_price.clone(),
            clearing_price: row.clearing_price.clone(),
            payment_amount: row.payment_amount.clone(),
            shortfall_amount: row.shortfall_amount.clone(),
            allocation: row.allocation.clone(),
            net_payment_amount: row.net_payment_amount.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucpayDetail1PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraFinancialAucpayDetail1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucpayDetail1Row<'data> {
    type Row<'other> = IrauctionSraFinancialAucpayDetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraFinancialAucpayDetail1Row<'data> {
    type PrimaryKey = IrauctionSraFinancialAucpayDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid
            && self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucpayDetail1PrimaryKey {
    type Row<'other> = IrauctionSraFinancialAucpayDetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraFinancialAucpayDetail1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucpayDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraFinancialAucpayDetail1 {
    type Builder = IrauctionSraFinancialAucpayDetail1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "sra_year",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_quarter",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "maximum_units",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "units_sold",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "shortfall_units",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reserve_price",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearing_price",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "payment_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "shortfall_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "allocation",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "net_payment_amount",
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
        IrauctionSraFinancialAucpayDetail1Builder {
            sra_year_array: arrow::array::builder::Int64Builder::new(),
            sra_quarter_array: arrow::array::builder::Int64Builder::new(),
            sra_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            maximum_units_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            units_sold_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            shortfall_units_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            reserve_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            clearing_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            payment_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            shortfall_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            allocation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            net_payment_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.sra_year_array.append_value(row.sra_year);
        builder.sra_quarter_array.append_value(row.sra_quarter);
        builder.sra_runno_array.append_value(row.sra_runno);
        builder.participantid_array.append_value(row.participantid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder.contractid_array.append_value(row.contractid());
        builder
            .maximum_units_array
            .append_option({
                row.maximum_units
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .units_sold_array
            .append_option({
                row.units_sold
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .shortfall_units_array
            .append_option({
                row.shortfall_units
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .reserve_price_array
            .append_option({
                row.reserve_price
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .clearing_price_array
            .append_option({
                row.clearing_price
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .payment_amount_array
            .append_option({
                row.payment_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .shortfall_amount_array
            .append_option({
                row.shortfall_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .allocation_array
            .append_option({
                row.allocation
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .net_payment_amount_array
            .append_option({
                row.net_payment_amount
                    .map(|mut val| {
                        val.rescale(8);
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
                    alloc::sync::Arc::new(builder.sra_year_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maximum_units_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.units_sold_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.shortfall_units_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reserve_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearing_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.payment_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.shortfall_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.allocation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.net_payment_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraFinancialAucpayDetail1Builder {
    sra_year_array: arrow::array::builder::Int64Builder,
    sra_quarter_array: arrow::array::builder::Int64Builder,
    sra_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    maximum_units_array: arrow::array::builder::Decimal128Builder,
    units_sold_array: arrow::array::builder::Decimal128Builder,
    shortfall_units_array: arrow::array::builder::Decimal128Builder,
    reserve_price_array: arrow::array::builder::Decimal128Builder,
    clearing_price_array: arrow::array::builder::Decimal128Builder,
    payment_amount_array: arrow::array::builder::Decimal128Builder,
    shortfall_amount_array: arrow::array::builder::Decimal128Builder,
    allocation_array: arrow::array::builder::Decimal128Builder,
    net_payment_amount_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionSraFinancialAucpaySum1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraFinancialAucpaySum1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraFinancialAucpaySum1 {
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
pub struct IrauctionSraFinancialAucpaySum1Mapping([usize; 10]);
/// # Summary
///
/// ## SRA_FINANCIAL_AUCPAY_SUM
///  _Records a summary of the Auction payment amount_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Aucpay Sum
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraFinancialAucpaySum1Row<'data> {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// The total auction proceeds allocated to the TNSP
    pub gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The total auction proceeds allocated to all TNSPs in the SRA quarter
    pub total_gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount for in the SRA Quarter for the TNSP
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The total shortfall amount for in the SRA Quarter for all TNSPs
    pub total_shortfall_amount: Option<rust_decimal::Decimal>,
    /// The net payment amount owed by AEMO to the TNSP
    pub net_payment_amount: Option<rust_decimal::Decimal>,
    /// The date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraFinancialAucpaySum1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionSraFinancialAucpaySum1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_FINANCIAL_AUCPAY_SUM";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraFinancialAucpaySum1Mapping([
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
        "SRA_YEAR",
        "SRA_QUARTER",
        "SRA_RUNNO",
        "PARTICIPANTID",
        "GROSS_PROCEEDS_AMOUNT",
        "TOTAL_GROSS_PROCEEDS_AMOUNT",
        "SHORTFALL_AMOUNT",
        "TOTAL_SHORTFALL_AMOUNT",
        "NET_PAYMENT_AMOUNT",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionSraFinancialAucpaySum1Row<'row>;
    type FieldMapping = IrauctionSraFinancialAucpaySum1Mapping;
    type PrimaryKey = IrauctionSraFinancialAucpaySum1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraFinancialAucpaySum1Row {
            sra_year: row.get_parsed_at_idx("sra_year", field_mapping.0[0])?,
            sra_quarter: row.get_parsed_at_idx("sra_quarter", field_mapping.0[1])?,
            sra_runno: row.get_parsed_at_idx("sra_runno", field_mapping.0[2])?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            gross_proceeds_amount: row
                .get_opt_custom_parsed_at_idx(
                    "gross_proceeds_amount",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total_gross_proceeds_amount: row
                .get_opt_custom_parsed_at_idx(
                    "total_gross_proceeds_amount",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            shortfall_amount: row
                .get_opt_custom_parsed_at_idx(
                    "shortfall_amount",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total_shortfall_amount: row
                .get_opt_custom_parsed_at_idx(
                    "total_shortfall_amount",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            net_payment_amount: row
                .get_opt_custom_parsed_at_idx(
                    "net_payment_amount",
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
        Ok(IrauctionSraFinancialAucpaySum1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraFinancialAucpaySum1PrimaryKey {
        IrauctionSraFinancialAucpaySum1PrimaryKey {
            participantid: row.participantid().to_string(),
            sra_quarter: row.sra_quarter,
            sra_runno: row.sra_runno,
            sra_year: row.sra_year,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_financial_aucpay_sum_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraFinancialAucpaySum1Row {
            sra_year: row.sra_year.clone(),
            sra_quarter: row.sra_quarter.clone(),
            sra_runno: row.sra_runno.clone(),
            participantid: row.participantid.clone(),
            gross_proceeds_amount: row.gross_proceeds_amount.clone(),
            total_gross_proceeds_amount: row.total_gross_proceeds_amount.clone(),
            shortfall_amount: row.shortfall_amount.clone(),
            total_shortfall_amount: row.total_shortfall_amount.clone(),
            net_payment_amount: row.net_payment_amount.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucpaySum1PrimaryKey {
    pub participantid: alloc::string::String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraFinancialAucpaySum1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucpaySum1Row<'data> {
    type Row<'other> = IrauctionSraFinancialAucpaySum1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraFinancialAucpaySum1Row<'data> {
    type PrimaryKey = IrauctionSraFinancialAucpaySum1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno && self.sra_year == key.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucpaySum1PrimaryKey {
    type Row<'other> = IrauctionSraFinancialAucpaySum1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid() && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno && self.sra_year == row.sra_year
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraFinancialAucpaySum1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucpaySum1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno && self.sra_year == key.sra_year
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraFinancialAucpaySum1 {
    type Builder = IrauctionSraFinancialAucpaySum1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "sra_year",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_quarter",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "gross_proceeds_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "total_gross_proceeds_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "shortfall_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "total_shortfall_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "net_payment_amount",
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
        IrauctionSraFinancialAucpaySum1Builder {
            sra_year_array: arrow::array::builder::Int64Builder::new(),
            sra_quarter_array: arrow::array::builder::Int64Builder::new(),
            sra_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            gross_proceeds_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            total_gross_proceeds_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            shortfall_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            total_shortfall_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            net_payment_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.sra_year_array.append_value(row.sra_year);
        builder.sra_quarter_array.append_value(row.sra_quarter);
        builder.sra_runno_array.append_value(row.sra_runno);
        builder.participantid_array.append_value(row.participantid());
        builder
            .gross_proceeds_amount_array
            .append_option({
                row.gross_proceeds_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .total_gross_proceeds_amount_array
            .append_option({
                row.total_gross_proceeds_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .shortfall_amount_array
            .append_option({
                row.shortfall_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .total_shortfall_amount_array
            .append_option({
                row.total_shortfall_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .net_payment_amount_array
            .append_option({
                row.net_payment_amount
                    .map(|mut val| {
                        val.rescale(8);
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
                    alloc::sync::Arc::new(builder.sra_year_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gross_proceeds_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.total_gross_proceeds_amount_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.shortfall_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.total_shortfall_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.net_payment_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraFinancialAucpaySum1Builder {
    sra_year_array: arrow::array::builder::Int64Builder,
    sra_quarter_array: arrow::array::builder::Int64Builder,
    sra_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    gross_proceeds_amount_array: arrow::array::builder::Decimal128Builder,
    total_gross_proceeds_amount_array: arrow::array::builder::Decimal128Builder,
    shortfall_amount_array: arrow::array::builder::Decimal128Builder,
    total_shortfall_amount_array: arrow::array::builder::Decimal128Builder,
    net_payment_amount_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionSraFinancialAucMardetail1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraFinancialAucMardetail1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraFinancialAucMardetail1 {
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
pub struct IrauctionSraFinancialAucMardetail1Mapping([usize; 7]);
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_MARDETAIL
///  _This table stores details of the margins returned to the participants._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Mardetail
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraFinancialAucMardetail1Row<'data> {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// The participant identifier.
    pub participantid: core::ops::Range<usize>,
    /// Unique identifier for the cash security.
    pub cash_security_id: core::ops::Range<usize>,
    /// The amount returned to the Auction participant from this cash security.
    pub returned_amount: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to the returned amount.
    pub returned_interest: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraFinancialAucMardetail1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn cash_security_id(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.cash_security_id.clone(),
        )
    }
}
impl mmsdm_core::GetTable for IrauctionSraFinancialAucMardetail1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_FINANCIAL_AUC_MARDETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraFinancialAucMardetail1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SRA_YEAR",
        "SRA_QUARTER",
        "SRA_RUNNO",
        "PARTICIPANTID",
        "CASH_SECURITY_ID",
        "RETURNED_AMOUNT",
        "RETURNED_INTEREST",
    ];
    type Row<'row> = IrauctionSraFinancialAucMardetail1Row<'row>;
    type FieldMapping = IrauctionSraFinancialAucMardetail1Mapping;
    type PrimaryKey = IrauctionSraFinancialAucMardetail1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraFinancialAucMardetail1Row {
            sra_year: row.get_parsed_at_idx("sra_year", field_mapping.0[0])?,
            sra_quarter: row.get_parsed_at_idx("sra_quarter", field_mapping.0[1])?,
            sra_runno: row.get_parsed_at_idx("sra_runno", field_mapping.0[2])?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            cash_security_id: row.get_range("cash_security_id", field_mapping.0[4])?,
            returned_amount: row
                .get_opt_custom_parsed_at_idx(
                    "returned_amount",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            returned_interest: row
                .get_opt_custom_parsed_at_idx(
                    "returned_interest",
                    field_mapping.0[6],
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
        Ok(IrauctionSraFinancialAucMardetail1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraFinancialAucMardetail1PrimaryKey {
        IrauctionSraFinancialAucMardetail1PrimaryKey {
            cash_security_id: row.cash_security_id().to_string(),
            participantid: row.participantid().to_string(),
            sra_quarter: row.sra_quarter,
            sra_runno: row.sra_runno,
            sra_year: row.sra_year,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_financial_auc_mardetail_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraFinancialAucMardetail1Row {
            sra_year: row.sra_year.clone(),
            sra_quarter: row.sra_quarter.clone(),
            sra_runno: row.sra_runno.clone(),
            participantid: row.participantid.clone(),
            cash_security_id: row.cash_security_id.clone(),
            returned_amount: row.returned_amount.clone(),
            returned_interest: row.returned_interest.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucMardetail1PrimaryKey {
    pub cash_security_id: alloc::string::String,
    pub participantid: alloc::string::String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraFinancialAucMardetail1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucMardetail1Row<'data> {
    type Row<'other> = IrauctionSraFinancialAucMardetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.cash_security_id() == row.cash_security_id()
            && self.participantid() == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraFinancialAucMardetail1Row<'data> {
    type PrimaryKey = IrauctionSraFinancialAucMardetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id() == key.cash_security_id
            && self.participantid() == key.participantid
            && self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucMardetail1PrimaryKey {
    type Row<'other> = IrauctionSraFinancialAucMardetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.cash_security_id == row.cash_security_id()
            && self.participantid == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraFinancialAucMardetail1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucMardetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraFinancialAucMardetail1 {
    type Builder = IrauctionSraFinancialAucMardetail1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "sra_year",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_quarter",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "cash_security_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "returned_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "returned_interest",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionSraFinancialAucMardetail1Builder {
            sra_year_array: arrow::array::builder::Int64Builder::new(),
            sra_quarter_array: arrow::array::builder::Int64Builder::new(),
            sra_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            cash_security_id_array: arrow::array::builder::StringBuilder::new(),
            returned_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            returned_interest_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.sra_year_array.append_value(row.sra_year);
        builder.sra_quarter_array.append_value(row.sra_quarter);
        builder.sra_runno_array.append_value(row.sra_runno);
        builder.participantid_array.append_value(row.participantid());
        builder.cash_security_id_array.append_value(row.cash_security_id());
        builder
            .returned_amount_array
            .append_option({
                row.returned_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .returned_interest_array
            .append_option({
                row.returned_interest
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
                    alloc::sync::Arc::new(builder.sra_year_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cash_security_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.returned_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.returned_interest_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraFinancialAucMardetail1Builder {
    sra_year_array: arrow::array::builder::Int64Builder,
    sra_quarter_array: arrow::array::builder::Int64Builder,
    sra_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    cash_security_id_array: arrow::array::builder::StringBuilder,
    returned_amount_array: arrow::array::builder::Decimal128Builder,
    returned_interest_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionSraFinancialAucMargin1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraFinancialAucMargin1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraFinancialAucMargin1 {
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
pub struct IrauctionSraFinancialAucMargin1Mapping([usize; 8]);
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_MARGIN
///  _Records the amount of Cash Security required to be held by an Auction Participant after settlement_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Margin
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraFinancialAucMargin1Row<'data> {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique  participant identifier.
    pub participantid: core::ops::Range<usize>,
    /// Total cash security held by the participant.
    pub total_cash_security: Option<rust_decimal::Decimal>,
    /// The amount of trading  cash security required to be held by the participant after settlement.
    pub required_margin: Option<rust_decimal::Decimal>,
    /// The amount of cash security returned to the participant.
    pub returned_margin: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to returned cash security amounts.
    pub returned_margin_interest: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraFinancialAucMargin1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionSraFinancialAucMargin1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_FINANCIAL_AUC_MARGIN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraFinancialAucMargin1Mapping([
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
        "SRA_YEAR",
        "SRA_QUARTER",
        "SRA_RUNNO",
        "PARTICIPANTID",
        "TOTAL_CASH_SECURITY",
        "REQUIRED_MARGIN",
        "RETURNED_MARGIN",
        "RETURNED_MARGIN_INTEREST",
    ];
    type Row<'row> = IrauctionSraFinancialAucMargin1Row<'row>;
    type FieldMapping = IrauctionSraFinancialAucMargin1Mapping;
    type PrimaryKey = IrauctionSraFinancialAucMargin1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraFinancialAucMargin1Row {
            sra_year: row.get_parsed_at_idx("sra_year", field_mapping.0[0])?,
            sra_quarter: row.get_parsed_at_idx("sra_quarter", field_mapping.0[1])?,
            sra_runno: row.get_parsed_at_idx("sra_runno", field_mapping.0[2])?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            total_cash_security: row
                .get_opt_custom_parsed_at_idx(
                    "total_cash_security",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            required_margin: row
                .get_opt_custom_parsed_at_idx(
                    "required_margin",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            returned_margin: row
                .get_opt_custom_parsed_at_idx(
                    "returned_margin",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            returned_margin_interest: row
                .get_opt_custom_parsed_at_idx(
                    "returned_margin_interest",
                    field_mapping.0[7],
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
        Ok(IrauctionSraFinancialAucMargin1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraFinancialAucMargin1PrimaryKey {
        IrauctionSraFinancialAucMargin1PrimaryKey {
            participantid: row.participantid().to_string(),
            sra_quarter: row.sra_quarter,
            sra_runno: row.sra_runno,
            sra_year: row.sra_year,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_financial_auc_margin_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraFinancialAucMargin1Row {
            sra_year: row.sra_year.clone(),
            sra_quarter: row.sra_quarter.clone(),
            sra_runno: row.sra_runno.clone(),
            participantid: row.participantid.clone(),
            total_cash_security: row.total_cash_security.clone(),
            required_margin: row.required_margin.clone(),
            returned_margin: row.returned_margin.clone(),
            returned_margin_interest: row.returned_margin_interest.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucMargin1PrimaryKey {
    pub participantid: alloc::string::String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraFinancialAucMargin1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucMargin1Row<'data> {
    type Row<'other> = IrauctionSraFinancialAucMargin1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraFinancialAucMargin1Row<'data> {
    type PrimaryKey = IrauctionSraFinancialAucMargin1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno && self.sra_year == key.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucMargin1PrimaryKey {
    type Row<'other> = IrauctionSraFinancialAucMargin1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid() && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno && self.sra_year == row.sra_year
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraFinancialAucMargin1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucMargin1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno && self.sra_year == key.sra_year
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraFinancialAucMargin1 {
    type Builder = IrauctionSraFinancialAucMargin1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "sra_year",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_quarter",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "total_cash_security",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "required_margin",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "returned_margin",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "returned_margin_interest",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionSraFinancialAucMargin1Builder {
            sra_year_array: arrow::array::builder::Int64Builder::new(),
            sra_quarter_array: arrow::array::builder::Int64Builder::new(),
            sra_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            total_cash_security_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            required_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            returned_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            returned_margin_interest_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.sra_year_array.append_value(row.sra_year);
        builder.sra_quarter_array.append_value(row.sra_quarter);
        builder.sra_runno_array.append_value(row.sra_runno);
        builder.participantid_array.append_value(row.participantid());
        builder
            .total_cash_security_array
            .append_option({
                row.total_cash_security
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .required_margin_array
            .append_option({
                row.required_margin
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .returned_margin_array
            .append_option({
                row.returned_margin
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .returned_margin_interest_array
            .append_option({
                row.returned_margin_interest
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
                    alloc::sync::Arc::new(builder.sra_year_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.total_cash_security_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.required_margin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.returned_margin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.returned_margin_interest_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraFinancialAucMargin1Builder {
    sra_year_array: arrow::array::builder::Int64Builder,
    sra_quarter_array: arrow::array::builder::Int64Builder,
    sra_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    total_cash_security_array: arrow::array::builder::Decimal128Builder,
    required_margin_array: arrow::array::builder::Decimal128Builder,
    returned_margin_array: arrow::array::builder::Decimal128Builder,
    returned_margin_interest_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionSraFinancialAucReceipts1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraFinancialAucReceipts1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraFinancialAucReceipts1 {
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
pub struct IrauctionSraFinancialAucReceipts1Mapping([usize; 13]);
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_RECEIPTS
///  _Records details of the Cancelled Units and their value for the Auction Participant_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Receipts
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraFinancialAucReceipts1Row<'data> {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// The source region identifier for the Directional Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// The SRA contract identifier
    pub contractid: core::ops::Range<usize>,
    /// The number of units purchased
    pub units_purchased: Option<rust_decimal::Decimal>,
    /// The clearing price of the auction
    pub clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed to AEMO
    pub receipt_amount: Option<rust_decimal::Decimal>,
    /// The last changed date for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Dollar value of Cancelled Units in the Auction for the Auction Participant
    pub proceeds_amount: Option<rust_decimal::Decimal>,
    /// Units cancelled in the auction by the Auction  participant.
    pub units_sold: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraFinancialAucReceipts1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionSraFinancialAucReceipts1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_FINANCIAL_AUC_RECEIPTS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraFinancialAucReceipts1Mapping([
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
        "SRA_YEAR",
        "SRA_QUARTER",
        "SRA_RUNNO",
        "PARTICIPANTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "CONTRACTID",
        "UNITS_PURCHASED",
        "CLEARING_PRICE",
        "RECEIPT_AMOUNT",
        "LASTCHANGED",
        "PROCEEDS_AMOUNT",
        "UNITS_SOLD",
    ];
    type Row<'row> = IrauctionSraFinancialAucReceipts1Row<'row>;
    type FieldMapping = IrauctionSraFinancialAucReceipts1Mapping;
    type PrimaryKey = IrauctionSraFinancialAucReceipts1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraFinancialAucReceipts1Row {
            sra_year: row.get_parsed_at_idx("sra_year", field_mapping.0[0])?,
            sra_quarter: row.get_parsed_at_idx("sra_quarter", field_mapping.0[1])?,
            sra_runno: row.get_parsed_at_idx("sra_runno", field_mapping.0[2])?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[4])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[5])?,
            contractid: row.get_range("contractid", field_mapping.0[6])?,
            units_purchased: row
                .get_opt_custom_parsed_at_idx(
                    "units_purchased",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearing_price: row
                .get_opt_custom_parsed_at_idx(
                    "clearing_price",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            receipt_amount: row
                .get_opt_custom_parsed_at_idx(
                    "receipt_amount",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            proceeds_amount: row
                .get_opt_custom_parsed_at_idx(
                    "proceeds_amount",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            units_sold: row
                .get_opt_custom_parsed_at_idx(
                    "units_sold",
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
        Ok(IrauctionSraFinancialAucReceipts1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraFinancialAucReceipts1PrimaryKey {
        IrauctionSraFinancialAucReceipts1PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            sra_quarter: row.sra_quarter,
            sra_runno: row.sra_runno,
            sra_year: row.sra_year,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_financial_auc_receipts_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraFinancialAucReceipts1Row {
            sra_year: row.sra_year.clone(),
            sra_quarter: row.sra_quarter.clone(),
            sra_runno: row.sra_runno.clone(),
            participantid: row.participantid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            contractid: row.contractid.clone(),
            units_purchased: row.units_purchased.clone(),
            clearing_price: row.clearing_price.clone(),
            receipt_amount: row.receipt_amount.clone(),
            lastchanged: row.lastchanged.clone(),
            proceeds_amount: row.proceeds_amount.clone(),
            units_sold: row.units_sold.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucReceipts1PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraFinancialAucReceipts1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucReceipts1Row<'data> {
    type Row<'other> = IrauctionSraFinancialAucReceipts1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraFinancialAucReceipts1Row<'data> {
    type PrimaryKey = IrauctionSraFinancialAucReceipts1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid
            && self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialAucReceipts1PrimaryKey {
    type Row<'other> = IrauctionSraFinancialAucReceipts1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid()
            && self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraFinancialAucReceipts1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucReceipts1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraFinancialAucReceipts1 {
    type Builder = IrauctionSraFinancialAucReceipts1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "sra_year",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_quarter",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "units_purchased",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearing_price",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "receipt_amount",
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
                arrow::datatypes::Field::new(
                    "proceeds_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "units_sold",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionSraFinancialAucReceipts1Builder {
            sra_year_array: arrow::array::builder::Int64Builder::new(),
            sra_quarter_array: arrow::array::builder::Int64Builder::new(),
            sra_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            units_purchased_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            clearing_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            receipt_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            proceeds_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            units_sold_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.sra_year_array.append_value(row.sra_year);
        builder.sra_quarter_array.append_value(row.sra_quarter);
        builder.sra_runno_array.append_value(row.sra_runno);
        builder.participantid_array.append_value(row.participantid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder.contractid_array.append_value(row.contractid());
        builder
            .units_purchased_array
            .append_option({
                row.units_purchased
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .clearing_price_array
            .append_option({
                row.clearing_price
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .receipt_amount_array
            .append_option({
                row.receipt_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .proceeds_amount_array
            .append_option({
                row.proceeds_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .units_sold_array
            .append_option({
                row.units_sold
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
                    alloc::sync::Arc::new(builder.sra_year_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.units_purchased_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearing_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.receipt_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.proceeds_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.units_sold_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraFinancialAucReceipts1Builder {
    sra_year_array: arrow::array::builder::Int64Builder,
    sra_quarter_array: arrow::array::builder::Int64Builder,
    sra_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    units_purchased_array: arrow::array::builder::Decimal128Builder,
    clearing_price_array: arrow::array::builder::Decimal128Builder,
    receipt_amount_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    proceeds_amount_array: arrow::array::builder::Decimal128Builder,
    units_sold_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionSraFinancialRuntrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraFinancialRuntrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraFinancialRuntrk1 {
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
pub struct IrauctionSraFinancialRuntrk1Mapping([usize; 9]);
/// # Summary
///
/// ## SRA_FINANCIAL_RUNTRK
///  _Records details of the settlement process for the cancellation and purchase of SRA Auction Units_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Runtrk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraFinancialRuntrk1Row<'data> {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// The type of SRA run
    pub runtype: core::ops::Range<usize>,
    /// The date and time the run was triggered
    pub rundate: Option<chrono::NaiveDateTime>,
    /// The date/time the run was posted
    pub posteddate: Option<chrono::NaiveDateTime>,
    /// Version number of the interest component used in the payments run
    pub interest_versionno: Option<i64>,
    /// Version number of the makeup component used in the makeup run
    pub makeup_versionno: Option<i64>,
    /// The date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraFinancialRuntrk1Row<'data> {
    pub fn runtype(&self) -> Option<&str> {
        if self.runtype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.runtype.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionSraFinancialRuntrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_FINANCIAL_RUNTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraFinancialRuntrk1Mapping([
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
        "SRA_YEAR",
        "SRA_QUARTER",
        "SRA_RUNNO",
        "RUNTYPE",
        "RUNDATE",
        "POSTEDDATE",
        "INTEREST_VERSIONNO",
        "MAKEUP_VERSIONNO",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionSraFinancialRuntrk1Row<'row>;
    type FieldMapping = IrauctionSraFinancialRuntrk1Mapping;
    type PrimaryKey = IrauctionSraFinancialRuntrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraFinancialRuntrk1Row {
            sra_year: row.get_parsed_at_idx("sra_year", field_mapping.0[0])?,
            sra_quarter: row.get_parsed_at_idx("sra_quarter", field_mapping.0[1])?,
            sra_runno: row.get_parsed_at_idx("sra_runno", field_mapping.0[2])?,
            runtype: row.get_opt_range("runtype", field_mapping.0[3])?,
            rundate: row
                .get_opt_custom_parsed_at_idx(
                    "rundate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            posteddate: row
                .get_opt_custom_parsed_at_idx(
                    "posteddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interest_versionno: row
                .get_opt_parsed_at_idx("interest_versionno", field_mapping.0[6])?,
            makeup_versionno: row
                .get_opt_parsed_at_idx("makeup_versionno", field_mapping.0[7])?,
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
        Ok(IrauctionSraFinancialRuntrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraFinancialRuntrk1PrimaryKey {
        IrauctionSraFinancialRuntrk1PrimaryKey {
            sra_quarter: row.sra_quarter,
            sra_runno: row.sra_runno,
            sra_year: row.sra_year,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_sra_financial_runtrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraFinancialRuntrk1Row {
            sra_year: row.sra_year.clone(),
            sra_quarter: row.sra_quarter.clone(),
            sra_runno: row.sra_runno.clone(),
            runtype: row.runtype.clone(),
            rundate: row.rundate.clone(),
            posteddate: row.posteddate.clone(),
            interest_versionno: row.interest_versionno.clone(),
            makeup_versionno: row.makeup_versionno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialRuntrk1PrimaryKey {
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraFinancialRuntrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialRuntrk1Row<'data> {
    type Row<'other> = IrauctionSraFinancialRuntrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraFinancialRuntrk1Row<'data> {
    type PrimaryKey = IrauctionSraFinancialRuntrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraFinancialRuntrk1PrimaryKey {
    type Row<'other> = IrauctionSraFinancialRuntrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.sra_quarter == row.sra_quarter && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraFinancialRuntrk1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialRuntrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.sra_quarter == key.sra_quarter && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraFinancialRuntrk1 {
    type Builder = IrauctionSraFinancialRuntrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "sra_year",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_quarter",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rundate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "posteddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interest_versionno",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "makeup_versionno",
                    arrow::datatypes::DataType::Int64,
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
        IrauctionSraFinancialRuntrk1Builder {
            sra_year_array: arrow::array::builder::Int64Builder::new(),
            sra_quarter_array: arrow::array::builder::Int64Builder::new(),
            sra_runno_array: arrow::array::builder::Int64Builder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            rundate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            posteddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interest_versionno_array: arrow::array::builder::Int64Builder::new(),
            makeup_versionno_array: arrow::array::builder::Int64Builder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.sra_year_array.append_value(row.sra_year);
        builder.sra_quarter_array.append_value(row.sra_quarter);
        builder.sra_runno_array.append_value(row.sra_runno);
        builder.runtype_array.append_option(row.runtype());
        builder
            .rundate_array
            .append_option(row.rundate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .posteddate_array
            .append_option(row.posteddate.map(|val| val.and_utc().timestamp_millis()));
        builder.interest_versionno_array.append_option(row.interest_versionno);
        builder.makeup_versionno_array.append_option(row.makeup_versionno);
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
                    alloc::sync::Arc::new(builder.sra_year_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rundate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.posteddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interest_versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.makeup_versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraFinancialRuntrk1Builder {
    sra_year_array: arrow::array::builder::Int64Builder,
    sra_quarter_array: arrow::array::builder::Int64Builder,
    sra_runno_array: arrow::array::builder::Int64Builder,
    runtype_array: arrow::array::builder::StringBuilder,
    rundate_array: arrow::array::builder::TimestampMillisecondBuilder,
    posteddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    interest_versionno_array: arrow::array::builder::Int64Builder,
    makeup_versionno_array: arrow::array::builder::Int64Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionSraOfferProduct1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraOfferProduct1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraOfferProduct1 {
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
pub struct IrauctionSraOfferProduct1Mapping([usize; 10]);
/// # Summary
///
/// ## SRA_OFFER_PRODUCT
///  _Holds the Product details for each Offer File submitted by each SRA Auction Participant._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Offer Product
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraOfferProduct1Row<'data> {
    /// Unique ID for each Auction date
    pub auctionid: core::ops::Range<usize>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// The date and time the system loaded the SRA Offer File
    pub loaddate: chrono::NaiveDateTime,
    /// Unique Product identifier (1 - 2000)
    pub optionid: i64,
    /// Unique Directional Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// The Offer quantity for this Product
    pub offer_quantity: Option<i64>,
    /// The Offer price for this Product
    pub offer_price: Option<rust_decimal::Decimal>,
    /// Tranche identifier
    pub trancheid: core::ops::Range<usize>,
    /// The date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraOfferProduct1Row<'data> {
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
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
    pub fn fromregionid(&self) -> Option<&str> {
        if self.fromregionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.fromregionid.clone(),
                ),
            )
        }
    }
    pub fn trancheid(&self) -> Option<&str> {
        if self.trancheid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.trancheid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionSraOfferProduct1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_OFFER_PRODUCT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraOfferProduct1Mapping([
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
        "AUCTIONID",
        "PARTICIPANTID",
        "LOADDATE",
        "OPTIONID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "OFFER_QUANTITY",
        "OFFER_PRICE",
        "TRANCHEID",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionSraOfferProduct1Row<'row>;
    type FieldMapping = IrauctionSraOfferProduct1Mapping;
    type PrimaryKey = IrauctionSraOfferProduct1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraOfferProduct1Row {
            auctionid: row.get_range("auctionid", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            loaddate: row
                .get_custom_parsed_at_idx(
                    "loaddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            optionid: row.get_parsed_at_idx("optionid", field_mapping.0[3])?,
            interconnectorid: row.get_opt_range("interconnectorid", field_mapping.0[4])?,
            fromregionid: row.get_opt_range("fromregionid", field_mapping.0[5])?,
            offer_quantity: row
                .get_opt_parsed_at_idx("offer_quantity", field_mapping.0[6])?,
            offer_price: row
                .get_opt_custom_parsed_at_idx(
                    "offer_price",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            trancheid: row.get_opt_range("trancheid", field_mapping.0[8])?,
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
        Ok(IrauctionSraOfferProduct1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraOfferProduct1PrimaryKey {
        IrauctionSraOfferProduct1PrimaryKey {
            auctionid: row.auctionid().to_string(),
            loaddate: row.loaddate,
            optionid: row.optionid,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_sra_offer_product_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraOfferProduct1Row {
            auctionid: row.auctionid.clone(),
            participantid: row.participantid.clone(),
            loaddate: row.loaddate.clone(),
            optionid: row.optionid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            offer_quantity: row.offer_quantity.clone(),
            offer_price: row.offer_price.clone(),
            trancheid: row.trancheid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraOfferProduct1PrimaryKey {
    pub auctionid: alloc::string::String,
    pub loaddate: chrono::NaiveDateTime,
    pub optionid: i64,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionSraOfferProduct1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraOfferProduct1Row<'data> {
    type Row<'other> = IrauctionSraOfferProduct1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid() && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionSraOfferProduct1Row<'data> {
    type PrimaryKey = IrauctionSraOfferProduct1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid && self.loaddate == key.loaddate
            && self.optionid == key.optionid && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraOfferProduct1PrimaryKey {
    type Row<'other> = IrauctionSraOfferProduct1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid() && self.loaddate == row.loaddate
            && self.optionid == row.optionid && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraOfferProduct1PrimaryKey {
    type PrimaryKey = IrauctionSraOfferProduct1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.loaddate == key.loaddate
            && self.optionid == key.optionid && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraOfferProduct1 {
    type Builder = IrauctionSraOfferProduct1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "loaddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "optionid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offer_quantity",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offer_price",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "trancheid",
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
        IrauctionSraOfferProduct1Builder {
            auctionid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            loaddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            optionid_array: arrow::array::builder::Int64Builder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            offer_quantity_array: arrow::array::builder::Int64Builder::new(),
            offer_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            trancheid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.auctionid_array.append_value(row.auctionid());
        builder.participantid_array.append_value(row.participantid());
        builder.loaddate_array.append_value(row.loaddate.and_utc().timestamp_millis());
        builder.optionid_array.append_value(row.optionid);
        builder.interconnectorid_array.append_option(row.interconnectorid());
        builder.fromregionid_array.append_option(row.fromregionid());
        builder.offer_quantity_array.append_option(row.offer_quantity);
        builder
            .offer_price_array
            .append_option({
                row.offer_price
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder.trancheid_array.append_option(row.trancheid());
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
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.loaddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.optionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_quantity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.trancheid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraOfferProduct1Builder {
    auctionid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    loaddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    optionid_array: arrow::array::builder::Int64Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    offer_quantity_array: arrow::array::builder::Int64Builder,
    offer_price_array: arrow::array::builder::Decimal128Builder,
    trancheid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionSraOfferProfile1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraOfferProfile1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraOfferProfile1 {
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
pub struct IrauctionSraOfferProfile1Mapping([usize; 7]);
/// # Summary
///
/// ## SRA_OFFER_PROFILE
///  _Holds the data of an SRA Auction Participant Offer Submission._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Offer Profile
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraOfferProfile1Row<'data> {
    /// Unique ID for each Auction date
    pub auctionid: core::ops::Range<usize>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// The date and time the system loaded the SRA Offer File
    pub loaddate: chrono::NaiveDateTime,
    /// SRA Offer File name
    pub filename: core::ops::Range<usize>,
    /// SRA acknowledgment file name
    pub ackfilename: core::ops::Range<usize>,
    /// Transaction ID used for tracking
    pub transactionid: core::ops::Range<usize>,
    /// The date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraOfferProfile1Row<'data> {
    pub fn auctionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.auctionid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
    }
    pub fn ackfilename(&self) -> Option<&str> {
        if self.ackfilename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.ackfilename.clone(),
                ),
            )
        }
    }
    pub fn transactionid(&self) -> Option<&str> {
        if self.transactionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.transactionid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionSraOfferProfile1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_OFFER_PROFILE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraOfferProfile1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "AUCTIONID",
        "PARTICIPANTID",
        "LOADDATE",
        "FILENAME",
        "ACKFILENAME",
        "TRANSACTIONID",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionSraOfferProfile1Row<'row>;
    type FieldMapping = IrauctionSraOfferProfile1Mapping;
    type PrimaryKey = IrauctionSraOfferProfile1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraOfferProfile1Row {
            auctionid: row.get_range("auctionid", field_mapping.0[0])?,
            participantid: row.get_range("participantid", field_mapping.0[1])?,
            loaddate: row
                .get_custom_parsed_at_idx(
                    "loaddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            filename: row.get_opt_range("filename", field_mapping.0[3])?,
            ackfilename: row.get_opt_range("ackfilename", field_mapping.0[4])?,
            transactionid: row.get_opt_range("transactionid", field_mapping.0[5])?,
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
        Ok(IrauctionSraOfferProfile1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraOfferProfile1PrimaryKey {
        IrauctionSraOfferProfile1PrimaryKey {
            auctionid: row.auctionid().to_string(),
            loaddate: row.loaddate,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_sra_offer_profile_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraOfferProfile1Row {
            auctionid: row.auctionid.clone(),
            participantid: row.participantid.clone(),
            loaddate: row.loaddate.clone(),
            filename: row.filename.clone(),
            ackfilename: row.ackfilename.clone(),
            transactionid: row.transactionid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraOfferProfile1PrimaryKey {
    pub auctionid: alloc::string::String,
    pub loaddate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionSraOfferProfile1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraOfferProfile1Row<'data> {
    type Row<'other> = IrauctionSraOfferProfile1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid() == row.auctionid() && self.loaddate == row.loaddate
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionSraOfferProfile1Row<'data> {
    type PrimaryKey = IrauctionSraOfferProfile1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid() == key.auctionid && self.loaddate == key.loaddate
            && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraOfferProfile1PrimaryKey {
    type Row<'other> = IrauctionSraOfferProfile1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.auctionid == row.auctionid() && self.loaddate == row.loaddate
            && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraOfferProfile1PrimaryKey {
    type PrimaryKey = IrauctionSraOfferProfile1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.loaddate == key.loaddate
            && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraOfferProfile1 {
    type Builder = IrauctionSraOfferProfile1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "loaddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "filename",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ackfilename",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "transactionid",
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
        IrauctionSraOfferProfile1Builder {
            auctionid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            loaddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            ackfilename_array: arrow::array::builder::StringBuilder::new(),
            transactionid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.auctionid_array.append_value(row.auctionid());
        builder.participantid_array.append_value(row.participantid());
        builder.loaddate_array.append_value(row.loaddate.and_utc().timestamp_millis());
        builder.filename_array.append_option(row.filename());
        builder.ackfilename_array.append_option(row.ackfilename());
        builder.transactionid_array.append_option(row.transactionid());
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
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.loaddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ackfilename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transactionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraOfferProfile1Builder {
    auctionid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    loaddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    ackfilename_array: arrow::array::builder::StringBuilder,
    transactionid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct IrauctionSraPrudentialCashSecurity1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraPrudentialCashSecurity1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraPrudentialCashSecurity1 {
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
pub struct IrauctionSraPrudentialCashSecurity1Mapping([usize; 5]);
/// # Summary
///
/// ## SRA_PRUDENTIAL_CASH_SECURITY
///  _Records the Cash Security details provided by an SRA Auction Participant as collateral to cover their Trading Position in the SRA market_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Cash Security
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraPrudentialCashSecurity1Row<'data> {
    /// The prudential date of the run.
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub prudential_runno: i64,
    /// Unique participant identifier for the Auction Participant lodging the Cash Security
    pub participantid: core::ops::Range<usize>,
    /// Unique identifier for the cash security.
    pub cash_security_id: core::ops::Range<usize>,
    /// Remaining Cash Security deposit available
    pub cash_security_amount: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraPrudentialCashSecurity1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn cash_security_id(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.cash_security_id.clone(),
        )
    }
}
impl mmsdm_core::GetTable for IrauctionSraPrudentialCashSecurity1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_PRUDENTIAL_CASH_SECURITY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraPrudentialCashSecurity1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PRUDENTIAL_DATE",
        "PRUDENTIAL_RUNNO",
        "PARTICIPANTID",
        "CASH_SECURITY_ID",
        "CASH_SECURITY_AMOUNT",
    ];
    type Row<'row> = IrauctionSraPrudentialCashSecurity1Row<'row>;
    type FieldMapping = IrauctionSraPrudentialCashSecurity1Mapping;
    type PrimaryKey = IrauctionSraPrudentialCashSecurity1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraPrudentialCashSecurity1Row {
            prudential_date: row
                .get_custom_parsed_at_idx(
                    "prudential_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            prudential_runno: row
                .get_parsed_at_idx("prudential_runno", field_mapping.0[1])?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            cash_security_id: row.get_range("cash_security_id", field_mapping.0[3])?,
            cash_security_amount: row
                .get_opt_custom_parsed_at_idx(
                    "cash_security_amount",
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
        Ok(IrauctionSraPrudentialCashSecurity1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> IrauctionSraPrudentialCashSecurity1PrimaryKey {
        IrauctionSraPrudentialCashSecurity1PrimaryKey {
            cash_security_id: row.cash_security_id().to_string(),
            participantid: row.participantid().to_string(),
            prudential_date: row.prudential_date,
            prudential_runno: row.prudential_runno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_prudential_cash_security_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraPrudentialCashSecurity1Row {
            prudential_date: row.prudential_date.clone(),
            prudential_runno: row.prudential_runno.clone(),
            participantid: row.participantid.clone(),
            cash_security_id: row.cash_security_id.clone(),
            cash_security_amount: row.cash_security_amount.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialCashSecurity1PrimaryKey {
    pub cash_security_id: alloc::string::String,
    pub participantid: alloc::string::String,
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraPrudentialCashSecurity1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionSraPrudentialCashSecurity1Row<'data> {
    type Row<'other> = IrauctionSraPrudentialCashSecurity1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.cash_security_id() == row.cash_security_id()
            && self.participantid() == row.participantid()
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraPrudentialCashSecurity1Row<'data> {
    type PrimaryKey = IrauctionSraPrudentialCashSecurity1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id() == key.cash_security_id
            && self.participantid() == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionSraPrudentialCashSecurity1PrimaryKey {
    type Row<'other> = IrauctionSraPrudentialCashSecurity1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.cash_security_id == row.cash_security_id()
            && self.participantid == row.participantid()
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for IrauctionSraPrudentialCashSecurity1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialCashSecurity1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
            && self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraPrudentialCashSecurity1 {
    type Builder = IrauctionSraPrudentialCashSecurity1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "prudential_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "prudential_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "cash_security_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "cash_security_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionSraPrudentialCashSecurity1Builder {
            prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            prudential_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            cash_security_id_array: arrow::array::builder::StringBuilder::new(),
            cash_security_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .prudential_date_array
            .append_value(row.prudential_date.and_utc().timestamp_millis());
        builder.prudential_runno_array.append_value(row.prudential_runno);
        builder.participantid_array.append_value(row.participantid());
        builder.cash_security_id_array.append_value(row.cash_security_id());
        builder
            .cash_security_amount_array
            .append_option({
                row.cash_security_amount
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
                    alloc::sync::Arc::new(builder.prudential_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.prudential_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cash_security_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cash_security_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraPrudentialCashSecurity1Builder {
    prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    prudential_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    cash_security_id_array: arrow::array::builder::StringBuilder,
    cash_security_amount_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionSraPrudentialCompPosition1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraPrudentialCompPosition1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraPrudentialCompPosition1 {
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
pub struct IrauctionSraPrudentialCompPosition1Mapping([usize; 6]);
/// # Summary
///
/// ## SRA_PRUDENTIAL_COMP_POSITION
///  _The prudential position of each company at the date and time of a specific prudential run_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Comp Position
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraPrudentialCompPosition1Row<'data> {
    /// The prudential date of the run.
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub prudential_runno: i64,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// The Trading Limit of the company at the time of the prudential run
    pub trading_limit: Option<rust_decimal::Decimal>,
    /// Current Prudential Exposure of the Auction Participant including Offers
    pub prudential_exposure_amount: Option<rust_decimal::Decimal>,
    /// The amount of Trading Margin available to the Auction Participant to trade (including Offered Units and Cancelled Units). Equal to TRADING_LIMIT minus PRUDENTIAL_EXPOSURE_AMOUNT.
    pub trading_margin: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraPrudentialCompPosition1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for IrauctionSraPrudentialCompPosition1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_PRUDENTIAL_COMP_POSITION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraPrudentialCompPosition1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PRUDENTIAL_DATE",
        "PRUDENTIAL_RUNNO",
        "PARTICIPANTID",
        "TRADING_LIMIT",
        "PRUDENTIAL_EXPOSURE_AMOUNT",
        "TRADING_MARGIN",
    ];
    type Row<'row> = IrauctionSraPrudentialCompPosition1Row<'row>;
    type FieldMapping = IrauctionSraPrudentialCompPosition1Mapping;
    type PrimaryKey = IrauctionSraPrudentialCompPosition1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraPrudentialCompPosition1Row {
            prudential_date: row
                .get_custom_parsed_at_idx(
                    "prudential_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            prudential_runno: row
                .get_parsed_at_idx("prudential_runno", field_mapping.0[1])?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            trading_limit: row
                .get_opt_custom_parsed_at_idx(
                    "trading_limit",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            prudential_exposure_amount: row
                .get_opt_custom_parsed_at_idx(
                    "prudential_exposure_amount",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            trading_margin: row
                .get_opt_custom_parsed_at_idx(
                    "trading_margin",
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
        Ok(IrauctionSraPrudentialCompPosition1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> IrauctionSraPrudentialCompPosition1PrimaryKey {
        IrauctionSraPrudentialCompPosition1PrimaryKey {
            participantid: row.participantid().to_string(),
            prudential_date: row.prudential_date,
            prudential_runno: row.prudential_runno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_prudential_comp_position_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraPrudentialCompPosition1Row {
            prudential_date: row.prudential_date.clone(),
            prudential_runno: row.prudential_runno.clone(),
            participantid: row.participantid.clone(),
            trading_limit: row.trading_limit.clone(),
            prudential_exposure_amount: row.prudential_exposure_amount.clone(),
            trading_margin: row.trading_margin.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialCompPosition1PrimaryKey {
    pub participantid: alloc::string::String,
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraPrudentialCompPosition1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionSraPrudentialCompPosition1Row<'data> {
    type Row<'other> = IrauctionSraPrudentialCompPosition1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid()
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraPrudentialCompPosition1Row<'data> {
    type PrimaryKey = IrauctionSraPrudentialCompPosition1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for IrauctionSraPrudentialCompPosition1PrimaryKey {
    type Row<'other> = IrauctionSraPrudentialCompPosition1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid()
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for IrauctionSraPrudentialCompPosition1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialCompPosition1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraPrudentialCompPosition1 {
    type Builder = IrauctionSraPrudentialCompPosition1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "prudential_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "prudential_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "trading_limit",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "prudential_exposure_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "trading_margin",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionSraPrudentialCompPosition1Builder {
            prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            prudential_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            trading_limit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            prudential_exposure_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            trading_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .prudential_date_array
            .append_value(row.prudential_date.and_utc().timestamp_millis());
        builder.prudential_runno_array.append_value(row.prudential_runno);
        builder.participantid_array.append_value(row.participantid());
        builder
            .trading_limit_array
            .append_option({
                row.trading_limit
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .prudential_exposure_amount_array
            .append_option({
                row.prudential_exposure_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .trading_margin_array
            .append_option({
                row.trading_margin
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
                    alloc::sync::Arc::new(builder.prudential_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.prudential_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.trading_limit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.prudential_exposure_amount_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.trading_margin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraPrudentialCompPosition1Builder {
    prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    prudential_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    trading_limit_array: arrow::array::builder::Decimal128Builder,
    prudential_exposure_amount_array: arrow::array::builder::Decimal128Builder,
    trading_margin_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionSraPrudentialExposure1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraPrudentialExposure1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraPrudentialExposure1 {
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
pub struct IrauctionSraPrudentialExposure1Mapping([usize; 14]);
/// # Summary
///
/// ## SRA_PRUDENTIAL_EXPOSURE
///  _Records details of the Prudential Exposure of an SRA Auction Participant_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Exposure
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
/// * SRA_QUARTER
/// * SRA_YEAR
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraPrudentialExposure1Row<'data> {
    /// The prudential date of the run.
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date.
    pub prudential_runno: i64,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// AEMO Contract Year number starting the week beginning 1 January
    pub sra_year: i64,
    /// Contract Relevant Quarter
    pub sra_quarter: i64,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// The largest Tranche where the Unit was either sold or offered. Used in the calculation of the Average Purchase Price for the Trading Position of the Product. The most recent Tranche where Units were cancelled or offered (if the Offer is below the Average Purchase Price)
    pub max_tranche: Option<i64>,
    /// Unique identifier for the Auction having the Offer. Has a null value when no Offer is made for the Relevant Quarter
    pub auctionid: core::ops::Range<usize>,
    /// Timestamp of the Offer File submitted by the Auction Participant. Has a null value when no Offer is made for the Relevant Quarter
    pub offer_submissiontime: Option<chrono::NaiveDateTime>,
    /// Calculated Average Purchase Price for the Product
    pub average_purchase_price: Option<rust_decimal::Decimal>,
    /// Calculated average cancellation price for product.
    pub average_cancellation_price: Option<rust_decimal::Decimal>,
    /// Calculated cancellation volume for product.
    pub cancellation_volume: Option<rust_decimal::Decimal>,
    /// Calculated trading position for product.
    pub trading_position: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionSraPrudentialExposure1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
    pub fn auctionid(&self) -> Option<&str> {
        if self.auctionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.auctionid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for IrauctionSraPrudentialExposure1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_PRUDENTIAL_EXPOSURE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraPrudentialExposure1Mapping([
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
        "PRUDENTIAL_DATE",
        "PRUDENTIAL_RUNNO",
        "PARTICIPANTID",
        "SRA_YEAR",
        "SRA_QUARTER",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "MAX_TRANCHE",
        "AUCTIONID",
        "OFFER_SUBMISSIONTIME",
        "AVERAGE_PURCHASE_PRICE",
        "AVERAGE_CANCELLATION_PRICE",
        "CANCELLATION_VOLUME",
        "TRADING_POSITION",
    ];
    type Row<'row> = IrauctionSraPrudentialExposure1Row<'row>;
    type FieldMapping = IrauctionSraPrudentialExposure1Mapping;
    type PrimaryKey = IrauctionSraPrudentialExposure1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraPrudentialExposure1Row {
            prudential_date: row
                .get_custom_parsed_at_idx(
                    "prudential_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            prudential_runno: row
                .get_parsed_at_idx("prudential_runno", field_mapping.0[1])?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            sra_year: row.get_parsed_at_idx("sra_year", field_mapping.0[3])?,
            sra_quarter: row.get_parsed_at_idx("sra_quarter", field_mapping.0[4])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[5])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[6])?,
            max_tranche: row.get_opt_parsed_at_idx("max_tranche", field_mapping.0[7])?,
            auctionid: row.get_opt_range("auctionid", field_mapping.0[8])?,
            offer_submissiontime: row
                .get_opt_custom_parsed_at_idx(
                    "offer_submissiontime",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            average_purchase_price: row
                .get_opt_custom_parsed_at_idx(
                    "average_purchase_price",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            average_cancellation_price: row
                .get_opt_custom_parsed_at_idx(
                    "average_cancellation_price",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cancellation_volume: row
                .get_opt_custom_parsed_at_idx(
                    "cancellation_volume",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            trading_position: row
                .get_opt_custom_parsed_at_idx(
                    "trading_position",
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
        Ok(IrauctionSraPrudentialExposure1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraPrudentialExposure1PrimaryKey {
        IrauctionSraPrudentialExposure1PrimaryKey {
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            prudential_date: row.prudential_date,
            prudential_runno: row.prudential_runno,
            sra_quarter: row.sra_quarter,
            sra_year: row.sra_year,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "irauction_sra_prudential_exposure_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraPrudentialExposure1Row {
            prudential_date: row.prudential_date.clone(),
            prudential_runno: row.prudential_runno.clone(),
            participantid: row.participantid.clone(),
            sra_year: row.sra_year.clone(),
            sra_quarter: row.sra_quarter.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            max_tranche: row.max_tranche.clone(),
            auctionid: row.auctionid.clone(),
            offer_submissiontime: row.offer_submissiontime.clone(),
            average_purchase_price: row.average_purchase_price.clone(),
            average_cancellation_price: row.average_cancellation_price.clone(),
            cancellation_volume: row.cancellation_volume.clone(),
            trading_position: row.trading_position.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialExposure1PrimaryKey {
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
    pub sra_quarter: i64,
    pub sra_year: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraPrudentialExposure1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraPrudentialExposure1Row<'data> {
    type Row<'other> = IrauctionSraPrudentialExposure1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
            && self.sra_quarter == row.sra_quarter && self.sra_year == row.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for IrauctionSraPrudentialExposure1Row<'data> {
    type PrimaryKey = IrauctionSraPrudentialExposure1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
            && self.sra_quarter == key.sra_quarter && self.sra_year == key.sra_year
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraPrudentialExposure1PrimaryKey {
    type Row<'other> = IrauctionSraPrudentialExposure1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid()
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
            && self.sra_quarter == row.sra_quarter && self.sra_year == row.sra_year
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraPrudentialExposure1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialExposure1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
            && self.sra_quarter == key.sra_quarter && self.sra_year == key.sra_year
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraPrudentialExposure1 {
    type Builder = IrauctionSraPrudentialExposure1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "prudential_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "prudential_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_year",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sra_quarter",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "max_tranche",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "auctionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offer_submissiontime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "average_purchase_price",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "average_cancellation_price",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cancellation_volume",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "trading_position",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionSraPrudentialExposure1Builder {
            prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            prudential_runno_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            sra_year_array: arrow::array::builder::Int64Builder::new(),
            sra_quarter_array: arrow::array::builder::Int64Builder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            max_tranche_array: arrow::array::builder::Int64Builder::new(),
            auctionid_array: arrow::array::builder::StringBuilder::new(),
            offer_submissiontime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            average_purchase_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            average_cancellation_price_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            cancellation_volume_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            trading_position_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .prudential_date_array
            .append_value(row.prudential_date.and_utc().timestamp_millis());
        builder.prudential_runno_array.append_value(row.prudential_runno);
        builder.participantid_array.append_value(row.participantid());
        builder.sra_year_array.append_value(row.sra_year);
        builder.sra_quarter_array.append_value(row.sra_quarter);
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder.max_tranche_array.append_option(row.max_tranche);
        builder.auctionid_array.append_option(row.auctionid());
        builder
            .offer_submissiontime_array
            .append_option(
                row.offer_submissiontime.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .average_purchase_price_array
            .append_option({
                row.average_purchase_price
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .average_cancellation_price_array
            .append_option({
                row.average_cancellation_price
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .cancellation_volume_array
            .append_option({
                row.cancellation_volume
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .trading_position_array
            .append_option({
                row.trading_position
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
                    alloc::sync::Arc::new(builder.prudential_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.prudential_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_year_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sra_quarter_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.max_tranche_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.auctionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_submissiontime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.average_purchase_price_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.average_cancellation_price_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cancellation_volume_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.trading_position_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraPrudentialExposure1Builder {
    prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    prudential_runno_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    sra_year_array: arrow::array::builder::Int64Builder,
    sra_quarter_array: arrow::array::builder::Int64Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    max_tranche_array: arrow::array::builder::Int64Builder,
    auctionid_array: arrow::array::builder::StringBuilder,
    offer_submissiontime_array: arrow::array::builder::TimestampMillisecondBuilder,
    average_purchase_price_array: arrow::array::builder::Decimal128Builder,
    average_cancellation_price_array: arrow::array::builder::Decimal128Builder,
    cancellation_volume_array: arrow::array::builder::Decimal128Builder,
    trading_position_array: arrow::array::builder::Decimal128Builder,
}
pub struct IrauctionSraPrudentialRun1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionSraPrudentialRun1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionSraPrudentialRun1 {
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
pub struct IrauctionSraPrudentialRun1Mapping([usize; 2]);
/// # Summary
///
/// ## SRA_PRUDENTIAL_RUN
///  _Records the prudential run details for each prudential date_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Run
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionSraPrudentialRun1Row<'data> {
    /// The prudential date of the run.
    pub prudential_date: chrono::NaiveDateTime,
    /// The prudential run number for the run
    pub prudential_runno: i64,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> IrauctionSraPrudentialRun1Row<'data> {}
impl mmsdm_core::GetTable for IrauctionSraPrudentialRun1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "SRA_PRUDENTIAL_RUN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionSraPrudentialRun1Mapping([
        4,
        5,
    ]);
    const COLUMNS: &'static [&'static str] = &["PRUDENTIAL_DATE", "PRUDENTIAL_RUNNO"];
    type Row<'row> = IrauctionSraPrudentialRun1Row<'row>;
    type FieldMapping = IrauctionSraPrudentialRun1Mapping;
    type PrimaryKey = IrauctionSraPrudentialRun1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionSraPrudentialRun1Row {
            prudential_date: row
                .get_custom_parsed_at_idx(
                    "prudential_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            prudential_runno: row
                .get_parsed_at_idx("prudential_runno", field_mapping.0[1])?,
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
        Ok(IrauctionSraPrudentialRun1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionSraPrudentialRun1PrimaryKey {
        IrauctionSraPrudentialRun1PrimaryKey {
            prudential_date: row.prudential_date,
            prudential_runno: row.prudential_runno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_sra_prudential_run_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionSraPrudentialRun1Row {
            prudential_date: row.prudential_date.clone(),
            prudential_runno: row.prudential_runno.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialRun1PrimaryKey {
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
}
impl mmsdm_core::PrimaryKey for IrauctionSraPrudentialRun1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraPrudentialRun1Row<'data> {
    type Row<'other> = IrauctionSraPrudentialRun1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionSraPrudentialRun1Row<'data> {
    type PrimaryKey = IrauctionSraPrudentialRun1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionSraPrudentialRun1PrimaryKey {
    type Row<'other> = IrauctionSraPrudentialRun1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionSraPrudentialRun1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialRun1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionSraPrudentialRun1 {
    type Builder = IrauctionSraPrudentialRun1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "prudential_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "prudential_runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        IrauctionSraPrudentialRun1Builder {
            prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            prudential_runno_array: arrow::array::builder::Int64Builder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .prudential_date_array
            .append_value(row.prudential_date.and_utc().timestamp_millis());
        builder.prudential_runno_array.append_value(row.prudential_runno);
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.prudential_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.prudential_runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct IrauctionSraPrudentialRun1Builder {
    prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    prudential_runno_array: arrow::array::builder::Int64Builder,
}
pub struct IrauctionValuationid1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &IrauctionValuationid1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl IrauctionValuationid1 {
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
pub struct IrauctionValuationid1Mapping([usize; 3]);
/// # Summary
///
/// ## VALUATIONID
///  _VALUATIONID shows the identifiers and descriptions of the valuers submitting estimates of upcoming settlement residues. VALUATIONID supports the Settlement Residue Auction._
///
/// * Data Set Name: Irauction
/// * File Name: Valuationid
/// * Data Version: 1
///
/// # Description
///  VALUATIONID is public data, and is available to all participants. Source VALUATIONID updates are quarterly from the Settlement Residues Information System [SRIS]. Volume VALUATIONID shows up to five (5) records. Updates are rare.
///
///
///
/// # Primary Key Columns
///
/// * VALUATIONID
#[derive(Debug, PartialEq, Eq)]
pub struct IrauctionValuationid1Row<'data> {
    /// Identifier of the estimator
    pub valuationid: core::ops::Range<usize>,
    /// Full name of estimator
    pub description: core::ops::Range<usize>,
    /// Timestamp of record creation or modification
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> IrauctionValuationid1Row<'data> {
    pub fn valuationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.valuationid.clone())
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
impl mmsdm_core::GetTable for IrauctionValuationid1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "IRAUCTION";
    const TABLE_NAME: &'static str = "VALUATIONID";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = IrauctionValuationid1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "VALUATIONID",
        "DESCRIPTION",
        "LASTCHANGED",
    ];
    type Row<'row> = IrauctionValuationid1Row<'row>;
    type FieldMapping = IrauctionValuationid1Mapping;
    type PrimaryKey = IrauctionValuationid1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(IrauctionValuationid1Row {
            valuationid: row.get_range("valuationid", field_mapping.0[0])?,
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
        Ok(IrauctionValuationid1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> IrauctionValuationid1PrimaryKey {
        IrauctionValuationid1PrimaryKey {
            valuationid: row.valuationid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("irauction_valuationid_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        IrauctionValuationid1Row {
            valuationid: row.valuationid.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionValuationid1PrimaryKey {
    pub valuationid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for IrauctionValuationid1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for IrauctionValuationid1Row<'data> {
    type Row<'other> = IrauctionValuationid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.valuationid() == row.valuationid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for IrauctionValuationid1Row<'data> {
    type PrimaryKey = IrauctionValuationid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.valuationid() == key.valuationid
    }
}
impl<'data> mmsdm_core::CompareWithRow for IrauctionValuationid1PrimaryKey {
    type Row<'other> = IrauctionValuationid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.valuationid == row.valuationid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for IrauctionValuationid1PrimaryKey {
    type PrimaryKey = IrauctionValuationid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.valuationid == key.valuationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for IrauctionValuationid1 {
    type Builder = IrauctionValuationid1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "valuationid",
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
        IrauctionValuationid1Builder {
            valuationid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.valuationid_array.append_value(row.valuationid());
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
                    alloc::sync::Arc::new(builder.valuationid_array.finish())
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
pub struct IrauctionValuationid1Builder {
    valuationid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
