#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct OfferBidperoffer1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &OfferBidperoffer1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl OfferBidperoffer1 {
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
pub struct OfferBidperoffer1Mapping([usize; 27]);
/// # Summary
///
/// ## BIDPEROFFER
///
/// BIDPEROFFER shows period-based Energy and Ancillary Service bid data. BIDPEROFFER is a child table of BIDDAYOFFER.
///
/// * Data Set Name: Offer
/// * File Name: Bidperoffer
/// * Data Version: 1
///
/// # Description
/// The new ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three new tables facilitate ancillary service bidding. The new tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDPEROFFER) are similar in structure to energy bidding tables (OFFERFILETRK, DAYOFFER and PEROFFER). The significant differences with the new tables are:·The OFFERDATE field reflects the time the bid was loaded and this field alone provides the key for versioning of bids. The VERSIONNO field is retained for participant use as information only.·The new tables support bids for multiple services. The BIDTYPE field defines the service to which the bid applies.·There are no default bids. In the absence of a bid for a specific settlement date, the latest bid submitted for a previous settlement date applies.BIDPEROFFER data is confidential to the submitting participant until made public after 4am the next day.SourceBIDPEROFFER updates as energy and ancillary service bids are processed. BIDPEROFFER includes all accepted energy and ancillary service bids.VolumeApproximately 72,000,000  records per year
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * OFFERDATE
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct OfferBidperoffer1Row<'data> {
    /// Dispatchable Unit identifier
    pub duid: core::ops::Range<usize>,
    /// Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// Market date starting at 04:05
    pub settlementdate: chrono::NaiveDateTime,
    /// Offer date
    pub offerdate: chrono::NaiveDateTime,
    /// Period ID
    pub periodid: rust_decimal::Decimal,
    /// Version number of offer
    pub versionno: Option<rust_decimal::Decimal>,
    /// Maximum availability for this BidType in this period
    pub maxavail: Option<rust_decimal::Decimal>,
    /// Fixed unit output MW (Energy Bids Only)  A value of zero means no fixed load so the unit is dispatched according to bid and market (rather than zero fixed load)
    pub fixedload: Option<rust_decimal::Decimal>,
    /// MW/min for raise (Energy Bids Only)
    pub rocup: Option<rust_decimal::Decimal>,
    /// MW/Min for lower (Energy Bids Only)
    pub rocdown: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which this ancillary service becomes available (AS Only)
    pub enablementmin: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which this ancillary service can be supplied (AS Only)
    pub enablementmax: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which the unit can provide the full availability (MAXAVAIL) for this ancillary service (AS Only)
    pub lowbreakpoint: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which the unit can provide the full availability (MAXAVAIL) for this ancillary service (AS Only)
    pub highbreakpoint: Option<rust_decimal::Decimal>,
    /// Availability at price band 1
    pub bandavail1: Option<rust_decimal::Decimal>,
    /// Availability at price band 2
    pub bandavail2: Option<rust_decimal::Decimal>,
    /// Availability at price band 3
    pub bandavail3: Option<rust_decimal::Decimal>,
    /// Availability at price band 4
    pub bandavail4: Option<rust_decimal::Decimal>,
    /// Availability at price band 5
    pub bandavail5: Option<rust_decimal::Decimal>,
    /// Availability at price band 6
    pub bandavail6: Option<rust_decimal::Decimal>,
    /// Availability at price band 7
    pub bandavail7: Option<rust_decimal::Decimal>,
    /// Availability at price band 8
    pub bandavail8: Option<rust_decimal::Decimal>,
    /// Availability at price band 9
    pub bandavail9: Option<rust_decimal::Decimal>,
    /// Availability at price band 10
    pub bandavail10: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Mandatory Restriction Offer amount
    pub mr_capacity: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> OfferBidperoffer1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for OfferBidperoffer1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "OFFER";
    const TABLE_NAME: &'static str = "BIDPEROFFER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = OfferBidperoffer1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "BIDTYPE",
        "SETTLEMENTDATE",
        "OFFERDATE",
        "PERIODID",
        "VERSIONNO",
        "MAXAVAIL",
        "FIXEDLOAD",
        "ROCUP",
        "ROCDOWN",
        "ENABLEMENTMIN",
        "ENABLEMENTMAX",
        "LOWBREAKPOINT",
        "HIGHBREAKPOINT",
        "BANDAVAIL1",
        "BANDAVAIL2",
        "BANDAVAIL3",
        "BANDAVAIL4",
        "BANDAVAIL5",
        "BANDAVAIL6",
        "BANDAVAIL7",
        "BANDAVAIL8",
        "BANDAVAIL9",
        "BANDAVAIL10",
        "LASTCHANGED",
        "PASAAVAILABILITY",
        "MR_CAPACITY",
    ];
    type Row<'row> = OfferBidperoffer1Row<'row>;
    type FieldMapping = OfferBidperoffer1Mapping;
    type PrimaryKey = OfferBidperoffer1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(OfferBidperoffer1Row {
            duid: row.get_range("duid", field_mapping.0[0])?,
            bidtype: row.get_range("bidtype", field_mapping.0[1])?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxavail: row
                .get_opt_custom_parsed_at_idx(
                    "maxavail",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fixedload: row
                .get_opt_custom_parsed_at_idx(
                    "fixedload",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rocup: row
                .get_opt_custom_parsed_at_idx(
                    "rocup",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rocdown: row
                .get_opt_custom_parsed_at_idx(
                    "rocdown",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablementmin: row
                .get_opt_custom_parsed_at_idx(
                    "enablementmin",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablementmax: row
                .get_opt_custom_parsed_at_idx(
                    "enablementmax",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowbreakpoint: row
                .get_opt_custom_parsed_at_idx(
                    "lowbreakpoint",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            highbreakpoint: row
                .get_opt_custom_parsed_at_idx(
                    "highbreakpoint",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail1: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail1",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail2: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail2",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail3: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail3",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail4: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail4",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail5: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail5",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail6: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail6",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail7: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail7",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail8: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail8",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail9: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail9",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail10: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail10",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[24],
                    mmsdm_core::mms_datetime::parse,
                )?,
            pasaavailability: row
                .get_opt_custom_parsed_at_idx(
                    "pasaavailability",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mr_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "mr_capacity",
                    field_mapping.0[26],
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
        Ok(OfferBidperoffer1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> OfferBidperoffer1PrimaryKey {
        OfferBidperoffer1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            duid: row.duid().to_string(),
            offerdate: row.offerdate,
            periodid: row.periodid,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("offer_bidperoffer_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        OfferBidperoffer1Row {
            duid: row.duid.clone(),
            bidtype: row.bidtype.clone(),
            settlementdate: row.settlementdate.clone(),
            offerdate: row.offerdate.clone(),
            periodid: row.periodid.clone(),
            versionno: row.versionno.clone(),
            maxavail: row.maxavail.clone(),
            fixedload: row.fixedload.clone(),
            rocup: row.rocup.clone(),
            rocdown: row.rocdown.clone(),
            enablementmin: row.enablementmin.clone(),
            enablementmax: row.enablementmax.clone(),
            lowbreakpoint: row.lowbreakpoint.clone(),
            highbreakpoint: row.highbreakpoint.clone(),
            bandavail1: row.bandavail1.clone(),
            bandavail2: row.bandavail2.clone(),
            bandavail3: row.bandavail3.clone(),
            bandavail4: row.bandavail4.clone(),
            bandavail5: row.bandavail5.clone(),
            bandavail6: row.bandavail6.clone(),
            bandavail7: row.bandavail7.clone(),
            bandavail8: row.bandavail8.clone(),
            bandavail9: row.bandavail9.clone(),
            bandavail10: row.bandavail10.clone(),
            lastchanged: row.lastchanged.clone(),
            pasaavailability: row.pasaavailability.clone(),
            mr_capacity: row.mr_capacity.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OfferBidperoffer1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub duid: alloc::string::String,
    pub offerdate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for OfferBidperoffer1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for OfferBidperoffer1Row<'data> {
    type Row<'other> = OfferBidperoffer1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.duid() == row.duid()
            && self.offerdate == row.offerdate && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for OfferBidperoffer1Row<'data> {
    type PrimaryKey = OfferBidperoffer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.duid() == key.duid
            && self.offerdate == key.offerdate && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for OfferBidperoffer1PrimaryKey {
    type Row<'other> = OfferBidperoffer1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.duid == row.duid()
            && self.offerdate == row.offerdate && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for OfferBidperoffer1PrimaryKey {
    type PrimaryKey = OfferBidperoffer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.duid == key.duid
            && self.offerdate == key.offerdate && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for OfferBidperoffer1 {
    type Builder = OfferBidperoffer1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
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
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxavail",
                    arrow::datatypes::DataType::Decimal128(12, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fixedload",
                    arrow::datatypes::DataType::Decimal128(12, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rocup",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rocdown",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablementmin",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablementmax",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowbreakpoint",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "highbreakpoint",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail1",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail2",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail3",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail4",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail5",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail6",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail7",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail8",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail9",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail10",
                    arrow::datatypes::DataType::Decimal128(22, 0),
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
                    "pasaavailability",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mr_capacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        OfferBidperoffer1Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            maxavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 6)),
            fixedload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 6)),
            rocup_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            rocdown_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            enablementmin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            enablementmax_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            lowbreakpoint_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            highbreakpoint_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            bandavail10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            pasaavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            mr_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.offerdate_array.append_value(row.offerdate.and_utc().timestamp_millis());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
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
            .maxavail_array
            .append_option({
                row.maxavail
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .fixedload_array
            .append_option({
                row.fixedload
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .rocup_array
            .append_option({
                row.rocup
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .rocdown_array
            .append_option({
                row.rocdown
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .enablementmin_array
            .append_option({
                row.enablementmin
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .enablementmax_array
            .append_option({
                row.enablementmax
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lowbreakpoint_array
            .append_option({
                row.lowbreakpoint
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .highbreakpoint_array
            .append_option({
                row.highbreakpoint
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail1_array
            .append_option({
                row.bandavail1
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail2_array
            .append_option({
                row.bandavail2
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail3_array
            .append_option({
                row.bandavail3
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail4_array
            .append_option({
                row.bandavail4
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail5_array
            .append_option({
                row.bandavail5
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail6_array
            .append_option({
                row.bandavail6
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail7_array
            .append_option({
                row.bandavail7
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail8_array
            .append_option({
                row.bandavail8
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail9_array
            .append_option({
                row.bandavail9
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail10_array
            .append_option({
                row.bandavail10
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .pasaavailability_array
            .append_option({
                row.pasaavailability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .mr_capacity_array
            .append_option({
                row.mr_capacity
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxavail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fixedload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rocup_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rocdown_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enablementmin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enablementmax_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowbreakpoint_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.highbreakpoint_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pasaavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mr_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct OfferBidperoffer1Builder {
    duid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    maxavail_array: arrow::array::builder::Decimal128Builder,
    fixedload_array: arrow::array::builder::Decimal128Builder,
    rocup_array: arrow::array::builder::Decimal128Builder,
    rocdown_array: arrow::array::builder::Decimal128Builder,
    enablementmin_array: arrow::array::builder::Decimal128Builder,
    enablementmax_array: arrow::array::builder::Decimal128Builder,
    lowbreakpoint_array: arrow::array::builder::Decimal128Builder,
    highbreakpoint_array: arrow::array::builder::Decimal128Builder,
    bandavail1_array: arrow::array::builder::Decimal128Builder,
    bandavail2_array: arrow::array::builder::Decimal128Builder,
    bandavail3_array: arrow::array::builder::Decimal128Builder,
    bandavail4_array: arrow::array::builder::Decimal128Builder,
    bandavail5_array: arrow::array::builder::Decimal128Builder,
    bandavail6_array: arrow::array::builder::Decimal128Builder,
    bandavail7_array: arrow::array::builder::Decimal128Builder,
    bandavail8_array: arrow::array::builder::Decimal128Builder,
    bandavail9_array: arrow::array::builder::Decimal128Builder,
    bandavail10_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    pasaavailability_array: arrow::array::builder::Decimal128Builder,
    mr_capacity_array: arrow::array::builder::Decimal128Builder,
}
pub struct BillingCpdata7 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingCpdata7Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingCpdata7 {
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
pub struct BillingCpdata7Mapping([usize; 16]);
/// # Summary
///
/// ## BILLINGCPDATA
///
/// BILLINGCPDATA shows energy quantity and $ value purchased per participant connection point.
///
/// * Data Set Name: Billing
/// * File Name: Cpdata
/// * Data Version: 7
///
/// # Description
/// BILLINGCPDATA data is confidential to relevant participant.SourcePopulated by the posting of a billing run, being several times each week.VolumeThe number of records depends on  the number of Transmission ConnectionPointIDs a participant may use to purchase energy. An indicative maximum is approximately 150 records per billrunno, or about 1,500 records inserted per week.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * MDA
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, PartialEq, Eq)]
pub struct BillingCpdata7Row<'data> {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Unique connection point identifier
    pub connectionpointid: core::ops::Range<usize>,
    /// Aggregate energy purchased/sold by customer, in MWh, plus UFEA. When GS commences, this includes the UFEA amount in the settlement runs.
    pub aggregateenergy: Option<rust_decimal::Decimal>,
    /// The Purchase column has the dollar value of the Energy Purchased rather than Aggregate Energy Dollar
    pub purchases: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// relevant MDA for this connection point.
    pub mda: core::ops::Range<usize>,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Billing run, excluding any UFEA component.
    pub afe: Option<rust_decimal::Decimal>,
    /// Sum of ME- for all NMIs at this Market Customer FRMP and TNI in the Billing run.
    pub dme: Option<rust_decimal::Decimal>,
    /// Share of UFE allocated to this FRMP and TNI in the Billing run.
    pub ufea: Option<rust_decimal::Decimal>,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the trading interval. This will include the UFEA value once financial settlement of UFE commences with GS.
    pub age: Option<rust_decimal::Decimal>,
    /// Energy sold at the connection point by the participant in this billing run
    pub soldenergy: Option<rust_decimal::Decimal>,
    /// The total cost of energy sold at the connection point by the participant in this billing run
    pub sales: Option<rust_decimal::Decimal>,
    /// The energy consumed at the connection point by the participant in this billing run
    pub purchasedenergy: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingCpdata7Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn mda(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.mda.clone())
    }
}
impl mmsdm_core::GetTable for BillingCpdata7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "BILLING";
    const TABLE_NAME: &'static str = "CPDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingCpdata7Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTYEAR",
        "WEEKNO",
        "BILLRUNNO",
        "PARTICIPANTID",
        "CONNECTIONPOINTID",
        "AGGREGATEENERGY",
        "PURCHASES",
        "LASTCHANGED",
        "MDA",
        "AFE",
        "DME",
        "UFEA",
        "AGE",
        "SOLDENERGY",
        "SALES",
        "PURCHASEDENERGY",
    ];
    type Row<'row> = BillingCpdata7Row<'row>;
    type FieldMapping = BillingCpdata7Mapping;
    type PrimaryKey = BillingCpdata7PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingCpdata7Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            weekno: row
                .get_custom_parsed_at_idx(
                    "weekno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            billrunno: row
                .get_custom_parsed_at_idx(
                    "billrunno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[4])?,
            aggregateenergy: row
                .get_opt_custom_parsed_at_idx(
                    "aggregateenergy",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            purchases: row
                .get_opt_custom_parsed_at_idx(
                    "purchases",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            mda: row.get_range("mda", field_mapping.0[8])?,
            afe: row
                .get_opt_custom_parsed_at_idx(
                    "afe",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dme: row
                .get_opt_custom_parsed_at_idx(
                    "dme",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ufea: row
                .get_opt_custom_parsed_at_idx(
                    "ufea",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            age: row
                .get_opt_custom_parsed_at_idx(
                    "age",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            soldenergy: row
                .get_opt_custom_parsed_at_idx(
                    "soldenergy",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sales: row
                .get_opt_custom_parsed_at_idx(
                    "sales",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            purchasedenergy: row
                .get_opt_custom_parsed_at_idx(
                    "purchasedenergy",
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
        Ok(BillingCpdata7Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingCpdata7PrimaryKey {
        BillingCpdata7PrimaryKey {
            billrunno: row.billrunno,
            connectionpointid: row.connectionpointid().to_string(),
            contractyear: row.contractyear,
            mda: row.mda().to_string(),
            participantid: row.participantid().to_string(),
            weekno: row.weekno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("billing_cpdata_v7_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingCpdata7Row {
            contractyear: row.contractyear.clone(),
            weekno: row.weekno.clone(),
            billrunno: row.billrunno.clone(),
            participantid: row.participantid.clone(),
            connectionpointid: row.connectionpointid.clone(),
            aggregateenergy: row.aggregateenergy.clone(),
            purchases: row.purchases.clone(),
            lastchanged: row.lastchanged.clone(),
            mda: row.mda.clone(),
            afe: row.afe.clone(),
            dme: row.dme.clone(),
            ufea: row.ufea.clone(),
            age: row.age.clone(),
            soldenergy: row.soldenergy.clone(),
            sales: row.sales.clone(),
            purchasedenergy: row.purchasedenergy.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingCpdata7PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub connectionpointid: alloc::string::String,
    pub contractyear: rust_decimal::Decimal,
    pub mda: alloc::string::String,
    pub participantid: alloc::string::String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingCpdata7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingCpdata7Row<'data> {
    type Row<'other> = BillingCpdata7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid() == row.connectionpointid()
            && self.contractyear == row.contractyear && self.mda() == row.mda()
            && self.participantid() == row.participantid() && self.weekno == row.weekno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BillingCpdata7Row<'data> {
    type PrimaryKey = BillingCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid() == key.connectionpointid
            && self.contractyear == key.contractyear && self.mda() == key.mda
            && self.participantid() == key.participantid && self.weekno == key.weekno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingCpdata7PrimaryKey {
    type Row<'other> = BillingCpdata7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid()
            && self.contractyear == row.contractyear && self.mda == row.mda()
            && self.participantid == row.participantid() && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingCpdata7PrimaryKey {
    type PrimaryKey = BillingCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear && self.mda == key.mda
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingCpdata7 {
    type Builder = BillingCpdata7Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "weekno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "billrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "aggregateenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "purchases",
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
                    "mda",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "afe",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dme",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ufea",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "age",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "soldenergy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sales",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "purchasedenergy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingCpdata7Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            weekno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            billrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            aggregateenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            purchases_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            mda_array: arrow::array::builder::StringBuilder::new(),
            afe_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            dme_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            ufea_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            age_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            soldenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            sales_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            purchasedenergy_array: arrow::array::builder::Decimal128Builder::new()
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
            .weekno_array
            .append_value({
                let mut val = row.weekno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .billrunno_array
            .append_value({
                let mut val = row.billrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder
            .aggregateenergy_array
            .append_option({
                row.aggregateenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .purchases_array
            .append_option({
                row.purchases
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.mda_array.append_value(row.mda());
        builder
            .afe_array
            .append_option({
                row.afe
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .dme_array
            .append_option({
                row.dme
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .ufea_array
            .append_option({
                row.ufea
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .age_array
            .append_option({
                row.age
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .soldenergy_array
            .append_option({
                row.soldenergy
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .sales_array
            .append_option({
                row.sales
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .purchasedenergy_array
            .append_option({
                row.purchasedenergy
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
                    alloc::sync::Arc::new(builder.weekno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.billrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregateenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.purchases_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mda_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.afe_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dme_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ufea_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.age_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.soldenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sales_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.purchasedenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingCpdata7Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    weekno_array: arrow::array::builder::Decimal128Builder,
    billrunno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    aggregateenergy_array: arrow::array::builder::Decimal128Builder,
    purchases_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    mda_array: arrow::array::builder::StringBuilder,
    afe_array: arrow::array::builder::Decimal128Builder,
    dme_array: arrow::array::builder::Decimal128Builder,
    ufea_array: arrow::array::builder::Decimal128Builder,
    age_array: arrow::array::builder::Decimal128Builder,
    soldenergy_array: arrow::array::builder::Decimal128Builder,
    sales_array: arrow::array::builder::Decimal128Builder,
    purchasedenergy_array: arrow::array::builder::Decimal128Builder,
}
pub struct BillingGendata5 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BillingGendata5Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BillingGendata5 {
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
pub struct BillingGendata5Mapping([usize; 13]);
/// # Summary
///
/// ## BILLINGGENDATA
///
/// BILLINGGENDATA shows the total energy sold and purchased per participant transmission connection point for a billing period.
///
/// * Data Set Name: Billing
/// * File Name: Gendata
/// * Data Version: 5
///
/// # Description
/// BILLINGGENDATA data is confidential to the the relevant participant.SourceBILLINGGENDATA is populated by the posting of a billing run, being several times each week.VolumeThe number of records depends on the number of transmission ConnectionPointIDs a Participant may have sold energy from per billrunno.  An indicative maximum is approximately 15 records inserted per billrunno, or about 165 records inserted per week.BILLINGGENDATA is confidential to the the relevant participant.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, PartialEq, Eq)]
pub struct BillingGendata5Row<'data> {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Connection point identifier
    pub connectionpointid: core::ops::Range<usize>,
    /// not populated
    pub stationid: core::ops::Range<usize>,
    /// not populated
    pub duid: core::ops::Range<usize>,
    /// Aggregate energy sold, in MWh
    pub aggregateenergy: Option<rust_decimal::Decimal>,
    /// $ income
    pub sales: Option<rust_decimal::Decimal>,
    /// $ outgoing
    pub purchases: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Amount of energy purchased in MWh
    pub purchasedenergy: Option<rust_decimal::Decimal>,
    /// Metering Data Agent supplying data
    pub mda: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BillingGendata5Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
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
    pub fn duid(&self) -> Option<&str> {
        if self.duid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone()),
            )
        }
    }
    pub fn mda(&self) -> Option<&str> {
        if self.mda.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.mda.clone()))
        }
    }
}
impl mmsdm_core::GetTable for BillingGendata5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "BILLING";
    const TABLE_NAME: &'static str = "GENDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BillingGendata5Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTYEAR",
        "WEEKNO",
        "BILLRUNNO",
        "PARTICIPANTID",
        "CONNECTIONPOINTID",
        "STATIONID",
        "DUID",
        "AGGREGATEENERGY",
        "SALES",
        "PURCHASES",
        "LASTCHANGED",
        "PURCHASEDENERGY",
        "MDA",
    ];
    type Row<'row> = BillingGendata5Row<'row>;
    type FieldMapping = BillingGendata5Mapping;
    type PrimaryKey = BillingGendata5PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BillingGendata5Row {
            contractyear: row
                .get_custom_parsed_at_idx(
                    "contractyear",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            weekno: row
                .get_custom_parsed_at_idx(
                    "weekno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            billrunno: row
                .get_custom_parsed_at_idx(
                    "billrunno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[4])?,
            stationid: row.get_opt_range("stationid", field_mapping.0[5])?,
            duid: row.get_opt_range("duid", field_mapping.0[6])?,
            aggregateenergy: row
                .get_opt_custom_parsed_at_idx(
                    "aggregateenergy",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sales: row
                .get_opt_custom_parsed_at_idx(
                    "sales",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            purchases: row
                .get_opt_custom_parsed_at_idx(
                    "purchases",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            purchasedenergy: row
                .get_opt_custom_parsed_at_idx(
                    "purchasedenergy",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mda: row.get_opt_range("mda", field_mapping.0[12])?,
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
        Ok(BillingGendata5Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BillingGendata5PrimaryKey {
        BillingGendata5PrimaryKey {
            billrunno: row.billrunno,
            connectionpointid: row.connectionpointid().to_string(),
            contractyear: row.contractyear,
            participantid: row.participantid().to_string(),
            weekno: row.weekno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("billing_gendata_v5_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BillingGendata5Row {
            contractyear: row.contractyear.clone(),
            weekno: row.weekno.clone(),
            billrunno: row.billrunno.clone(),
            participantid: row.participantid.clone(),
            connectionpointid: row.connectionpointid.clone(),
            stationid: row.stationid.clone(),
            duid: row.duid.clone(),
            aggregateenergy: row.aggregateenergy.clone(),
            sales: row.sales.clone(),
            purchases: row.purchases.clone(),
            lastchanged: row.lastchanged.clone(),
            purchasedenergy: row.purchasedenergy.clone(),
            mda: row.mda.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BillingGendata5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub connectionpointid: alloc::string::String,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: alloc::string::String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingGendata5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BillingGendata5Row<'data> {
    type Row<'other> = BillingGendata5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid() == row.connectionpointid()
            && self.contractyear == row.contractyear
            && self.participantid() == row.participantid() && self.weekno == row.weekno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BillingGendata5Row<'data> {
    type PrimaryKey = BillingGendata5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid() == key.connectionpointid
            && self.contractyear == key.contractyear
            && self.participantid() == key.participantid && self.weekno == key.weekno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BillingGendata5PrimaryKey {
    type Row<'other> = BillingGendata5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid()
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid() && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingGendata5PrimaryKey {
    type PrimaryKey = BillingGendata5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingGendata5 {
    type Builder = BillingGendata5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractyear",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "weekno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "billrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregateenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sales",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "purchases",
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
                    "purchasedenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mda",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BillingGendata5Builder {
            contractyear_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            weekno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            billrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            aggregateenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            sales_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            purchases_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            purchasedenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            mda_array: arrow::array::builder::StringBuilder::new(),
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
            .weekno_array
            .append_value({
                let mut val = row.weekno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .billrunno_array
            .append_value({
                let mut val = row.billrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder.stationid_array.append_option(row.stationid());
        builder.duid_array.append_option(row.duid());
        builder
            .aggregateenergy_array
            .append_option({
                row.aggregateenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .sales_array
            .append_option({
                row.sales
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .purchases_array
            .append_option({
                row.purchases
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .purchasedenergy_array
            .append_option({
                row.purchasedenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.mda_array.append_option(row.mda());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractyear_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.weekno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.billrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregateenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sales_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.purchases_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.purchasedenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mda_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BillingGendata5Builder {
    contractyear_array: arrow::array::builder::Decimal128Builder,
    weekno_array: arrow::array::builder::Decimal128Builder,
    billrunno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    aggregateenergy_array: arrow::array::builder::Decimal128Builder,
    sales_array: arrow::array::builder::Decimal128Builder,
    purchases_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    purchasedenergy_array: arrow::array::builder::Decimal128Builder,
    mda_array: arrow::array::builder::StringBuilder,
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
///
/// DISPATCH_FCAS_REQ shows Dispatch Constraint tracking for Regional FCAS recovery.
///
/// * Data Set Name: Dispatch
/// * File Name: Fcas Req
/// * Data Version: 2
///
/// # Description
/// DISPATCH_FCAS_REQ is public data and is available to all participants.SourceDISPATCH_FCAS_REQ updates with each dispatch run (5 minutes).VolumeApproximately 10,000 rows per day
///
/// # Notes
/// * (Visibility)  Public
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
    pub regionid: core::ops::Range<usize>,
    /// DUID offered type
    pub bidtype: core::ops::Range<usize>,
    /// Generic Constraint EffectiveDate - Join to table GenConData
    pub genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    pub genconversionno: Option<rust_decimal::Decimal>,
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
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
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
pub struct DispatchocdCase1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchocdCase1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchocdCase1 {
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
pub struct DispatchocdCase1Mapping([usize; 3]);
/// # Summary
///
/// ## DISPATCHCASE_OCD
///
/// DISPATCHCASE_OCD shows the key data to indicate when an over-constrained dispatch (OCD) re-run actually occurred. One record per over-constrained dispatch interval.
///
/// * Data Set Name: Dispatchocd
/// * File Name: Case
/// * Data Version: 1
///
/// # Description
/// The DISPATCHCASE_OCD data is public.SourceThe occurrences of Over-constrained dispatch (OCD) or binding intra-regional network constraints (BNC)  re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Potentially updated every 5 minutes.VolumeRows per day: ~2Mb per month: <1The estimates on the number of rows are based on a 1% occurrence rate for OCD runs.NoteDue to the close dependency with the dispatch process, the OCD and BNC data models use a “CaseSolution” key table in the same manner as the DISPATCHCASESOLUTION table.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchocdCase1Row<'data> {
    /// End date and time of the dispatch interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> DispatchocdCase1Row<'data> {}
impl mmsdm_core::GetTable for DispatchocdCase1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCHOCD";
    const TABLE_NAME: &'static str = "CASE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchocdCase1Mapping([4, 5, 6]);
    const COLUMNS: &'static [&'static str] = &["SETTLEMENTDATE", "RUNNO", "LASTCHANGED"];
    type Row<'row> = DispatchocdCase1Row<'row>;
    type FieldMapping = DispatchocdCase1Mapping;
    type PrimaryKey = DispatchocdCase1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchocdCase1Row {
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
        Ok(DispatchocdCase1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchocdCase1PrimaryKey {
        DispatchocdCase1PrimaryKey {
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatchocd_case_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchocdCase1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchocdCase1PrimaryKey {
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchocdCase1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchocdCase1Row<'data> {
    type Row<'other> = DispatchocdCase1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchocdCase1Row<'data> {
    type PrimaryKey = DispatchocdCase1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchocdCase1PrimaryKey {
    type Row<'other> = DispatchocdCase1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchocdCase1PrimaryKey {
    type PrimaryKey = DispatchocdCase1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchocdCase1 {
    type Builder = DispatchocdCase1Builder;
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
        DispatchocdCase1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchocdCase1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
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
///
/// DISPATCHCASESOLUTION shows information relating to the complete dispatch run. The fields in DISPATCHCASESOLUTION provide an overview of the dispatch run results allowing immediate identification of conditions such as energy or FCAS deficiencies.
///
/// * Data Set Name: Dispatch
/// * File Name: Case Solution
/// * Data Version: 2
///
/// # Description
/// The DISPATCHCASESOLUTION data is public.SourceDISPATCHCASESOLUTION updates every 5 minutes.VolumeApproximately 288 records per day.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchCaseSolution2Row<'data> {
    /// Date and time of the dispatch interval (e.g. five minute dispatch interval ending 28/09/2000 16:35)
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention flag - refer to package documentation for definition and practical query examples
    pub intervention: rust_decimal::Decimal,
    /// Overconstrained dispatch indicator: * OCD = detecting over-constrained dispatch* null = no special condition
    pub casesubtype: core::ops::Range<usize>,
    /// If non-zero indicated one of the following conditions:* 1 = Supply Scarcity, Excess generation or constraint violations* X = Model failure
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
    /// Flag indicating the SCADA status for FCAS Interconnector dead-band. "0"if SCADA Status or requesting Constraint not invoked. "1"if SCADA Status AND requesting Constraint is invoked
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
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27,
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
pub struct DispatchbncCasesolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchbncCasesolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchbncCasesolution1 {
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
pub struct DispatchbncCasesolution1Mapping([usize; 23]);
/// # Summary
///
/// ## DISPATCHCASESOLUTION_BNC
///
/// DISPATCHCASESOLUTION_BNC was discontinued on 30 September 2009. Prior: DISPATCHCASESOLUTION_BNC is the key data to indicate when a binding intra-regional network constraints (BNC) re-run actually occurred.
///
/// * Data Set Name: Dispatchbnc
/// * File Name: Casesolution
/// * Data Version: 1
///
/// # Description
/// DISPATCHCASESOLUTION_BNC was discontinued on 30 September 2009.In accordance with the "Arrangements for Managing Risks Associated with Transmission Network Congestion"set of rule changes the Dispatch Binding Network Constraints re-run was discontinued on September 30, 2009.SourceThe occurrences of Over-constrained dispatch (OCD) or binding intra-regional network constraints (BNC)  re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Potentially updated every 5 minutes.VolumeRows per day: ~72Mb per month: 25% of DISPATCHCASESOLUTIONThe estimates on the number of rows are based on a 25% occurrence rate for BNC runs.NoteDue to the close dependency with the dispatch process, the OCD and BNC data models use a “CaseSolution” key table in the same manner as DISPATCHCASESOLUTION.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchbncCasesolution1Row<'data> {
    /// End date and time of the dispatch interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Manual intervention flag
    pub intervention: rust_decimal::Decimal,
    /// always BNC
    pub casesubtype: core::ops::Range<usize>,
    /// If non-zero indicated one of the following conditions:* 1 = Supply Scarcity, Excess generation or constraint violations* X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: Option<rust_decimal::Decimal>,
    /// Period identifier of first interval of the case (yyyymmddppp)
    pub startperiod: mmsdm_core::DispatchPeriod,
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
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Total of ancillary service trader profile violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Total of fast start trader profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchbncCasesolution1Row<'data> {
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
}
impl mmsdm_core::GetTable for DispatchbncCasesolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCHBNC";
    const TABLE_NAME: &'static str = "CASESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchbncCasesolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "INTERVENTION",
        "CASESUBTYPE",
        "SOLUTIONSTATUS",
        "SPDVERSION",
        "STARTPERIOD",
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
        "TOTALENERGYCONSTRVIOLATION",
        "TOTALENERGYOFFERVIOLATION",
        "TOTALASPROFILEVIOLATION",
        "TOTALFASTSTARTVIOLATION",
        "LASTCHANGED",
    ];
    type Row<'row> = DispatchbncCasesolution1Row<'row>;
    type FieldMapping = DispatchbncCasesolution1Mapping;
    type PrimaryKey = DispatchbncCasesolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchbncCasesolution1Row {
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
            spdversion: row
                .get_opt_custom_parsed_at_idx(
                    "spdversion",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            startperiod: row.get_parsed_at_idx("startperiod", field_mapping.0[6])?,
            nonphysicallosses: row
                .get_opt_custom_parsed_at_idx(
                    "nonphysicallosses",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalobjective: row
                .get_opt_custom_parsed_at_idx(
                    "totalobjective",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalareagenviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalareagenviolation",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalinterconnectorviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalinterconnectorviolation",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalgenericviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalgenericviolation",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalramprateviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalramprateviolation",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalunitmwcapacityviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalunitmwcapacityviolation",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total5minviolation",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalregviolation",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total6secviolation",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            total60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "total60secviolation",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalenergyconstrviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalenergyconstrviolation",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalenergyofferviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalenergyofferviolation",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalasprofileviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalasprofileviolation",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalfaststartviolation: row
                .get_opt_custom_parsed_at_idx(
                    "totalfaststartviolation",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[22],
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
        Ok(DispatchbncCasesolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchbncCasesolution1PrimaryKey {
        DispatchbncCasesolution1PrimaryKey {
            intervention: row.intervention,
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("dispatchbnc_casesolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchbncCasesolution1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            intervention: row.intervention.clone(),
            casesubtype: row.casesubtype.clone(),
            solutionstatus: row.solutionstatus.clone(),
            spdversion: row.spdversion.clone(),
            startperiod: row.startperiod.clone(),
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
            totalenergyconstrviolation: row.totalenergyconstrviolation.clone(),
            totalenergyofferviolation: row.totalenergyofferviolation.clone(),
            totalasprofileviolation: row.totalasprofileviolation.clone(),
            totalfaststartviolation: row.totalfaststartviolation.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchbncCasesolution1PrimaryKey {
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchbncCasesolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchbncCasesolution1Row<'data> {
    type Row<'other> = DispatchbncCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchbncCasesolution1Row<'data> {
    type PrimaryKey = DispatchbncCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchbncCasesolution1PrimaryKey {
    type Row<'other> = DispatchbncCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchbncCasesolution1PrimaryKey {
    type PrimaryKey = DispatchbncCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchbncCasesolution1 {
    type Builder = DispatchbncCasesolution1Builder;
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
                    arrow::datatypes::DataType::Decimal128(10, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "startperiod",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        DispatchbncCasesolution1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            casesubtype_array: arrow::array::builder::StringBuilder::new(),
            solutionstatus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            spdversion_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 3)),
            startperiod_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            totalenergyconstrviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalasprofileviolation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            totalfaststartviolation_array: arrow::array::builder::Decimal128Builder::new()
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
        builder
            .spdversion_array
            .append_option({
                row.spdversion
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .startperiod_array
            .append_value(row.startperiod.start().and_utc().timestamp_millis());
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
                    alloc::sync::Arc::new(builder.casesubtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.solutionstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spdversion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startperiod_array.finish())
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
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchbncCasesolution1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    casesubtype_array: arrow::array::builder::StringBuilder,
    solutionstatus_array: arrow::array::builder::Decimal128Builder,
    spdversion_array: arrow::array::builder::Decimal128Builder,
    startperiod_array: arrow::array::builder::TimestampMillisecondBuilder,
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
    totalenergyconstrviolation_array: arrow::array::builder::Decimal128Builder,
    totalenergyofferviolation_array: arrow::array::builder::Decimal128Builder,
    totalasprofileviolation_array: arrow::array::builder::Decimal128Builder,
    totalfaststartviolation_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct DispatchbncUnitsolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &DispatchbncUnitsolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl DispatchbncUnitsolution1 {
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
pub struct DispatchbncUnitsolution1Mapping([usize; 24]);
/// # Summary
///
/// ## DISPATCHLOAD_BNC
///
/// DISPATCHLOAD_BNC was discontinued on 30 September 2009. Prior: DISPATCHLOAD_BNC gives binding intra-regional network constraints (BNC) re-run dispatch results for all scheduled generating units. DISPATCHLOAD_BNC has a similar structure to DISPATCHLOAD but does not repeat input type data (e.g. InitialMW, AGCStatus) since these values are available from DISPATCHLOAD.
///
/// * Data Set Name: Dispatchbnc
/// * File Name: Unitsolution
/// * Data Version: 1
///
/// # Description
/// DISPATCHLOAD_BNC was discontinued on 30 September 2009.In accordance with the "Arrangements for Managing Risks Associated with Transmission Network Congestion"set of rule changes the Dispatch Binding Network Constraints re-run was discontinued on September 30, 2009.SourceThe occurrences of Over-constrained dispatch (OCD) or binding intra-regional network constraints (BNC)  re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Potentially updated every 5 minutes.DISPATCHLOAD_BNC shows data produced every 5 minutes for all units when they have over-constrained dispatch, with own data being confidential until the next day.VolumeRows per day: ~14000Mb per month: 25% of DISPATCHLOADThe basis of estimation on the number of rows is on a 25% occurrence rate for BNC runsNoteA flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined as follows:Flag NameBitDescriptionField valueEnabled0The unit is enabled to provide this ancillary service type.>1Trapped1The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market.3Stranded2The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service.4
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct DispatchbncUnitsolution1Row<'data> {
    /// End date and time of the dispatch interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Intervention flag if intervention run
    pub intervention: rust_decimal::Decimal,
    /// Connection point identifier for DUID
    pub connectionpointid: core::ops::Range<usize>,
    /// Dispatch mode for fast start plant (0 to 4).
    pub dispatchmode: Option<rust_decimal::Decimal>,
    /// Target MW for end of period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Raise 5 min reserve target
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec reserve target
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec reserve target
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Lower 5 min reserve target
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec reserve target
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec reserve target
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag
    pub lowerregflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> DispatchbncUnitsolution1Row<'data> {
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
impl mmsdm_core::GetTable for DispatchbncUnitsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "DISPATCHBNC";
    const TABLE_NAME: &'static str = "UNITSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = DispatchbncUnitsolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "DUID",
        "INTERVENTION",
        "CONNECTIONPOINTID",
        "DISPATCHMODE",
        "TOTALCLEARED",
        "RAISEREG",
        "RAISE5MIN",
        "RAISE60SEC",
        "RAISE6SEC",
        "LOWERREG",
        "LOWER5MIN",
        "LOWER60SEC",
        "LOWER6SEC",
        "RAISEREGFLAGS",
        "RAISE5MINFLAGS",
        "RAISE60SECFLAGS",
        "RAISE6SECFLAGS",
        "LOWERREGFLAGS",
        "LOWER5MINFLAGS",
        "LOWER60SECFLAGS",
        "LOWER6SECFLAGS",
        "LASTCHANGED",
    ];
    type Row<'row> = DispatchbncUnitsolution1Row<'row>;
    type FieldMapping = DispatchbncUnitsolution1Mapping;
    type PrimaryKey = DispatchbncUnitsolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(DispatchbncUnitsolution1Row {
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
            intervention: row
                .get_custom_parsed_at_idx(
                    "intervention",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row
                .get_opt_range("connectionpointid", field_mapping.0[4])?,
            dispatchmode: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchmode",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalcleared: row
                .get_opt_custom_parsed_at_idx(
                    "totalcleared",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreg: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5min: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregflags: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregflags",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minflags",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secflags",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secflags",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregflags: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregflags",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minflags",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secflags",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secflags: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secflags",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[23],
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
        Ok(DispatchbncUnitsolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> DispatchbncUnitsolution1PrimaryKey {
        DispatchbncUnitsolution1PrimaryKey {
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
        alloc::format!("dispatchbnc_unitsolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        DispatchbncUnitsolution1Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            duid: row.duid.clone(),
            intervention: row.intervention.clone(),
            connectionpointid: row.connectionpointid.clone(),
            dispatchmode: row.dispatchmode.clone(),
            totalcleared: row.totalcleared.clone(),
            raisereg: row.raisereg.clone(),
            raise5min: row.raise5min.clone(),
            raise60sec: row.raise60sec.clone(),
            raise6sec: row.raise6sec.clone(),
            lowerreg: row.lowerreg.clone(),
            lower5min: row.lower5min.clone(),
            lower60sec: row.lower60sec.clone(),
            lower6sec: row.lower6sec.clone(),
            raiseregflags: row.raiseregflags.clone(),
            raise5minflags: row.raise5minflags.clone(),
            raise60secflags: row.raise60secflags.clone(),
            raise6secflags: row.raise6secflags.clone(),
            lowerregflags: row.lowerregflags.clone(),
            lower5minflags: row.lower5minflags.clone(),
            lower60secflags: row.lower60secflags.clone(),
            lower6secflags: row.lower6secflags.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchbncUnitsolution1PrimaryKey {
    pub duid: alloc::string::String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchbncUnitsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for DispatchbncUnitsolution1Row<'data> {
    type Row<'other> = DispatchbncUnitsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.intervention == row.intervention
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for DispatchbncUnitsolution1Row<'data> {
    type PrimaryKey = DispatchbncUnitsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for DispatchbncUnitsolution1PrimaryKey {
    type Row<'other> = DispatchbncUnitsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.intervention == row.intervention
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchbncUnitsolution1PrimaryKey {
    type PrimaryKey = DispatchbncUnitsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for DispatchbncUnitsolution1 {
    type Builder = DispatchbncUnitsolution1Builder;
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
                    "totalcleared",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereg",
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
                    "raiseregflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secflags",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secflags",
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
        DispatchbncUnitsolution1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            intervention_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            dispatchmode_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            totalcleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raisereg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise5min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise60sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raise6sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lowerreg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower5min_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower60sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lower6sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raiseregflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise5minflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise60secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            raise6secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lowerregflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower5minflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower60secflags_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower6secflags_array: arrow::array::builder::Decimal128Builder::new()
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
        builder.duid_array.append_value(row.duid());
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
            .totalcleared_array
            .append_option({
                row.totalcleared
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
            .raiseregflags_array
            .append_option({
                row.raiseregflags
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
            .raise60secflags_array
            .append_option({
                row.raise60secflags
                    .map(|mut val| {
                        val.rescale(0);
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
            .lowerregflags_array
            .append_option({
                row.lowerregflags
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
            .lower60secflags_array
            .append_option({
                row.lower60secflags
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatchmode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalcleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5min_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secflags_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct DispatchbncUnitsolution1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    dispatchmode_array: arrow::array::builder::Decimal128Builder,
    totalcleared_array: arrow::array::builder::Decimal128Builder,
    raisereg_array: arrow::array::builder::Decimal128Builder,
    raise5min_array: arrow::array::builder::Decimal128Builder,
    raise60sec_array: arrow::array::builder::Decimal128Builder,
    raise6sec_array: arrow::array::builder::Decimal128Builder,
    lowerreg_array: arrow::array::builder::Decimal128Builder,
    lower5min_array: arrow::array::builder::Decimal128Builder,
    lower60sec_array: arrow::array::builder::Decimal128Builder,
    lower6sec_array: arrow::array::builder::Decimal128Builder,
    raiseregflags_array: arrow::array::builder::Decimal128Builder,
    raise5minflags_array: arrow::array::builder::Decimal128Builder,
    raise60secflags_array: arrow::array::builder::Decimal128Builder,
    raise6secflags_array: arrow::array::builder::Decimal128Builder,
    lowerregflags_array: arrow::array::builder::Decimal128Builder,
    lower5minflags_array: arrow::array::builder::Decimal128Builder,
    lower60secflags_array: arrow::array::builder::Decimal128Builder,
    lower6secflags_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MeterDataCustomer1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MeterDataCustomer1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterDataCustomer1 {
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
pub struct MeterDataCustomer1Mapping([usize; 12]);
/// # Summary
///
/// ## METERDATA
///
/// METERDATA sets out a meter data for each customer connection point. METERDATA covers market load. Use the field METERRUNNO to match the meter data version for each settlement run.
///
/// * Data Set Name: Meter Data
/// * File Name: Customer
/// * Data Version: 1
///
/// # Description
/// METERDATA data is confidential to the relevant participant.SourceMETERDATA updates whenever meter files are processed from MSATS.VolumeDepends on number of TNI, FRMP, LR combinations and number of data file loads (versions) from MSATS per settlement run.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * HOSTDISTRIBUTOR
/// * MDA
/// * METERRUNNO
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct MeterDataCustomer1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Settlement period identifier (half hour period)
    pub periodid: rust_decimal::Decimal,
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Data version no
    pub meterrunno: rust_decimal::Decimal,
    /// Transmission Node Identifier (TNI).  Identifies a Transmission NetworkConnection Point.
    pub connectionpointid: core::ops::Range<usize>,
    /// Imported energy value (MWh)
    pub importenergyvalue: Option<rust_decimal::Decimal>,
    /// Exported energy value (MWh)
    pub exportenergyvalue: Option<rust_decimal::Decimal>,
    /// Not used
    pub importreactivevalue: Option<rust_decimal::Decimal>,
    /// Not used
    pub exportreactivevalue: Option<rust_decimal::Decimal>,
    /// Local Retailer participant identifier
    pub hostdistributor: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Defaults to MSATS
    pub mda: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterDataCustomer1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn hostdistributor(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.hostdistributor.clone(),
        )
    }
    pub fn mda(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.mda.clone())
    }
}
impl mmsdm_core::GetTable for MeterDataCustomer1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METER_DATA";
    const TABLE_NAME: &'static str = "CUSTOMER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterDataCustomer1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "PERIODID",
        "SETTLEMENTDATE",
        "METERRUNNO",
        "CONNECTIONPOINTID",
        "IMPORTENERGYVALUE",
        "EXPORTENERGYVALUE",
        "IMPORTREACTIVEVALUE",
        "EXPORTREACTIVEVALUE",
        "HOSTDISTRIBUTOR",
        "LASTCHANGED",
        "MDA",
    ];
    type Row<'row> = MeterDataCustomer1Row<'row>;
    type FieldMapping = MeterDataCustomer1Mapping;
    type PrimaryKey = MeterDataCustomer1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterDataCustomer1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            meterrunno: row
                .get_custom_parsed_at_idx(
                    "meterrunno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[4])?,
            importenergyvalue: row
                .get_opt_custom_parsed_at_idx(
                    "importenergyvalue",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportenergyvalue: row
                .get_opt_custom_parsed_at_idx(
                    "exportenergyvalue",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importreactivevalue: row
                .get_opt_custom_parsed_at_idx(
                    "importreactivevalue",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportreactivevalue: row
                .get_opt_custom_parsed_at_idx(
                    "exportreactivevalue",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            hostdistributor: row.get_range("hostdistributor", field_mapping.0[9])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            mda: row.get_range("mda", field_mapping.0[11])?,
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
        Ok(MeterDataCustomer1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterDataCustomer1PrimaryKey {
        MeterDataCustomer1PrimaryKey {
            connectionpointid: row.connectionpointid().to_string(),
            hostdistributor: row.hostdistributor().to_string(),
            mda: row.mda().to_string(),
            meterrunno: row.meterrunno,
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meter_data_customer_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterDataCustomer1Row {
            participantid: row.participantid.clone(),
            periodid: row.periodid.clone(),
            settlementdate: row.settlementdate.clone(),
            meterrunno: row.meterrunno.clone(),
            connectionpointid: row.connectionpointid.clone(),
            importenergyvalue: row.importenergyvalue.clone(),
            exportenergyvalue: row.exportenergyvalue.clone(),
            importreactivevalue: row.importreactivevalue.clone(),
            exportreactivevalue: row.exportreactivevalue.clone(),
            hostdistributor: row.hostdistributor.clone(),
            lastchanged: row.lastchanged.clone(),
            mda: row.mda.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterDataCustomer1PrimaryKey {
    pub connectionpointid: alloc::string::String,
    pub hostdistributor: alloc::string::String,
    pub mda: alloc::string::String,
    pub meterrunno: rust_decimal::Decimal,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterDataCustomer1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterDataCustomer1Row<'data> {
    type Row<'other> = MeterDataCustomer1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid() == row.connectionpointid()
            && self.hostdistributor() == row.hostdistributor() && self.mda() == row.mda()
            && self.meterrunno == row.meterrunno
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterDataCustomer1Row<'data> {
    type PrimaryKey = MeterDataCustomer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid() == key.connectionpointid
            && self.hostdistributor() == key.hostdistributor && self.mda() == key.mda
            && self.meterrunno == key.meterrunno
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterDataCustomer1PrimaryKey {
    type Row<'other> = MeterDataCustomer1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid == row.connectionpointid()
            && self.hostdistributor == row.hostdistributor() && self.mda == row.mda()
            && self.meterrunno == row.meterrunno
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterDataCustomer1PrimaryKey {
    type PrimaryKey = MeterDataCustomer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.hostdistributor == key.hostdistributor && self.mda == key.mda
            && self.meterrunno == key.meterrunno
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterDataCustomer1 {
    type Builder = MeterDataCustomer1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "meterrunno",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "importenergyvalue",
                    arrow::datatypes::DataType::Decimal128(9, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportenergyvalue",
                    arrow::datatypes::DataType::Decimal128(9, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importreactivevalue",
                    arrow::datatypes::DataType::Decimal128(9, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportreactivevalue",
                    arrow::datatypes::DataType::Decimal128(9, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "hostdistributor",
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
                arrow::datatypes::Field::new(
                    "mda",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MeterDataCustomer1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            meterrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            importenergyvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 6)),
            exportenergyvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 6)),
            importreactivevalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 6)),
            exportreactivevalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 6)),
            hostdistributor_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            mda_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .meterrunno_array
            .append_value({
                let mut val = row.meterrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder
            .importenergyvalue_array
            .append_option({
                row.importenergyvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .exportenergyvalue_array
            .append_option({
                row.exportenergyvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .importreactivevalue_array
            .append_option({
                row.importreactivevalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .exportreactivevalue_array
            .append_option({
                row.exportreactivevalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.hostdistributor_array.append_value(row.hostdistributor());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.mda_array.append_value(row.mda());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meterrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importenergyvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportenergyvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importreactivevalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportreactivevalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.hostdistributor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mda_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterDataCustomer1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    meterrunno_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    importenergyvalue_array: arrow::array::builder::Decimal128Builder,
    exportenergyvalue_array: arrow::array::builder::Decimal128Builder,
    importreactivevalue_array: arrow::array::builder::Decimal128Builder,
    exportreactivevalue_array: arrow::array::builder::Decimal128Builder,
    hostdistributor_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    mda_array: arrow::array::builder::StringBuilder,
}
pub struct MeterDataGenDuid1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MeterDataGenDuid1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterDataGenDuid1 {
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
pub struct MeterDataGenDuid1Mapping([usize; 4]);
/// # Summary
///
/// ## METERDATA_GEN_DUID
///
/// Recorded actual generation of non-scheduled units where SCADA data is available.
///
/// * Data Set Name: Meter Data
/// * File Name: Gen Duid
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
/// * DUID
/// * INTERVAL_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MeterDataGenDuid1Row<'data> {
    /// Timestamp of the recorded interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Unit ID
    pub duid: core::ops::Range<usize>,
    /// MW reading
    pub mwh_reading: Option<rust_decimal::Decimal>,
    /// Timestamp of last record change
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterDataGenDuid1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for MeterDataGenDuid1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METER_DATA";
    const TABLE_NAME: &'static str = "GEN_DUID";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterDataGenDuid1Mapping([
        4, 5, 6, 7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERVAL_DATETIME",
        "DUID",
        "MWH_READING",
        "LASTCHANGED",
    ];
    type Row<'row> = MeterDataGenDuid1Row<'row>;
    type FieldMapping = MeterDataGenDuid1Mapping;
    type PrimaryKey = MeterDataGenDuid1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterDataGenDuid1Row {
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            mwh_reading: row
                .get_opt_custom_parsed_at_idx(
                    "mwh_reading",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
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
        Ok(MeterDataGenDuid1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterDataGenDuid1PrimaryKey {
        MeterDataGenDuid1PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meter_data_gen_duid_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterDataGenDuid1Row {
            interval_datetime: row.interval_datetime.clone(),
            duid: row.duid.clone(),
            mwh_reading: row.mwh_reading.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterDataGenDuid1PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterDataGenDuid1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterDataGenDuid1Row<'data> {
    type Row<'other> = MeterDataGenDuid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterDataGenDuid1Row<'data> {
    type PrimaryKey = MeterDataGenDuid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterDataGenDuid1PrimaryKey {
    type Row<'other> = MeterDataGenDuid1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterDataGenDuid1PrimaryKey {
    type PrimaryKey = MeterDataGenDuid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterDataGenDuid1 {
    type Builder = MeterDataGenDuid1Builder;
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
                    "mwh_reading",
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
        MeterDataGenDuid1Builder {
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            mwh_reading_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .mwh_reading_array
            .append_option({
                row.mwh_reading
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
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwh_reading_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterDataGenDuid1Builder {
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    mwh_reading_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MeterdataTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MeterdataTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterdataTrk1 {
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
pub struct MeterdataTrk1Mapping([usize; 6]);
/// # Summary
///
/// ## METERDATA_TRK
///
/// Tracking table for the publication of wholesale settlement data associated with BILLING run
///
/// * Data Set Name: Meterdata
/// * File Name: Trk
/// * Data Version: 1
///
/// # Description
/// METERDATATRK data is confidential to the relevant participant.SourceMETERDATATRK updates whenever meter files are processed.VolumeDepends on the number of TNI, FRMP and LR combinations plus the number of data file loads (versions) from MSATS per settlement run.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * CASE_ID
#[derive(Debug, PartialEq, Eq)]
pub struct MeterdataTrk1Row<'data> {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    /// Timestamp of the aggregated reads being loaded for this case
    pub aggregate_reads_load_datetime: Option<chrono::NaiveDateTime>,
    /// Timestamp of the non aggregated reads being loaded for this case
    pub individual_reads_load_datetime: Option<chrono::NaiveDateTime>,
    /// The start date of data associated with the CASE_ID
    pub startdate: Option<chrono::NaiveDateTime>,
    /// The end date of data associated with the Case_ID
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Last changed date for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> MeterdataTrk1Row<'data> {}
impl mmsdm_core::GetTable for MeterdataTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METERDATA";
    const TABLE_NAME: &'static str = "TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterdataTrk1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CASE_ID",
        "AGGREGATE_READS_LOAD_DATETIME",
        "INDIVIDUAL_READS_LOAD_DATETIME",
        "STARTDATE",
        "ENDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = MeterdataTrk1Row<'row>;
    type FieldMapping = MeterdataTrk1Mapping;
    type PrimaryKey = MeterdataTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterdataTrk1Row {
            case_id: row
                .get_custom_parsed_at_idx(
                    "case_id",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregate_reads_load_datetime: row
                .get_opt_custom_parsed_at_idx(
                    "aggregate_reads_load_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            individual_reads_load_datetime: row
                .get_opt_custom_parsed_at_idx(
                    "individual_reads_load_datetime",
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
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
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
        Ok(MeterdataTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterdataTrk1PrimaryKey {
        MeterdataTrk1PrimaryKey {
            case_id: row.case_id,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meterdata_trk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterdataTrk1Row {
            case_id: row.case_id.clone(),
            aggregate_reads_load_datetime: row.aggregate_reads_load_datetime.clone(),
            individual_reads_load_datetime: row.individual_reads_load_datetime.clone(),
            startdate: row.startdate.clone(),
            enddate: row.enddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataTrk1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MeterdataTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterdataTrk1Row<'data> {
    type Row<'other> = MeterdataTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterdataTrk1Row<'data> {
    type PrimaryKey = MeterdataTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterdataTrk1PrimaryKey {
    type Row<'other> = MeterdataTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.case_id == row.case_id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataTrk1PrimaryKey {
    type PrimaryKey = MeterdataTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataTrk1 {
    type Builder = MeterdataTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "case_id",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "aggregate_reads_load_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "individual_reads_load_datetime",
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
        MeterdataTrk1Builder {
            case_id_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            aggregate_reads_load_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            individual_reads_load_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .case_id_array
            .append_value({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .aggregate_reads_load_datetime_array
            .append_option(
                row
                    .aggregate_reads_load_datetime
                    .map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .individual_reads_load_datetime_array
            .append_option(
                row
                    .individual_reads_load_datetime
                    .map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .startdate_array
            .append_option(row.startdate.map(|val| val.and_utc().timestamp_millis()));
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.and_utc().timestamp_millis()));
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
                    alloc::sync::Arc::new(builder.case_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.aggregate_reads_load_datetime_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.individual_reads_load_datetime_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterdataTrk1Builder {
    case_id_array: arrow::array::builder::Decimal128Builder,
    aggregate_reads_load_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    individual_reads_load_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MeterDataCustomerTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MeterDataCustomerTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MeterDataCustomerTrk1 {
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
pub struct MeterDataCustomerTrk1Mapping([usize; 11]);
/// # Summary
///
/// ## METERDATATRK
///
/// METERDATATRK records meter data files submitted for each connection point on a daily basis. The same data is provided in METERDATA period by period (i.e. 48 records), whereas METERDATATRK shows one record per day for each file submitted for a connection point.
///
/// * Data Set Name: Meter Data
/// * File Name: Customer Trk
/// * Data Version: 1
///
/// # Description
/// METERDATATRK data is confidential to the relevant participant.SourceMETERDATATRK updates whenever meter files are processed.VolumeDepends on the number of TNI, FRMP and LR combinations plus the number of data file loads (versions) from MSATS per settlement run.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * HOSTDISTRIBUTOR
/// * METERINGDATAAGENT
/// * METERRUNNO
/// * PARTICIPANTID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct MeterDataCustomerTrk1Row<'data> {
    /// Settlement calendar date
    pub settlementdate: chrono::NaiveDateTime,
    /// Meter data version number
    pub meterrunno: rust_decimal::Decimal,
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Meter file name (MSATS file name)
    pub filename: core::ops::Range<usize>,
    /// Not used
    pub ackfilename: core::ops::Range<usize>,
    /// Transmission Node Identifier (TNI)
    pub connectionpointid: core::ops::Range<usize>,
    /// Date processed
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub authorisedby: core::ops::Range<usize>,
    /// Defaults to MSATS
    pub meteringdataagent: core::ops::Range<usize>,
    /// Local retailer participant identifier
    pub hostdistributor: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MeterDataCustomerTrk1Row<'data> {
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
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
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
    pub fn meteringdataagent(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.meteringdataagent.clone(),
        )
    }
    pub fn hostdistributor(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.hostdistributor.clone(),
        )
    }
}
impl mmsdm_core::GetTable for MeterDataCustomerTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "METER_DATA";
    const TABLE_NAME: &'static str = "CUSTOMER_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MeterDataCustomerTrk1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "METERRUNNO",
        "PARTICIPANTID",
        "FILENAME",
        "ACKFILENAME",
        "CONNECTIONPOINTID",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "METERINGDATAAGENT",
        "HOSTDISTRIBUTOR",
        "LASTCHANGED",
    ];
    type Row<'row> = MeterDataCustomerTrk1Row<'row>;
    type FieldMapping = MeterDataCustomerTrk1Mapping;
    type PrimaryKey = MeterDataCustomerTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MeterDataCustomerTrk1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            meterrunno: row
                .get_custom_parsed_at_idx(
                    "meterrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            filename: row.get_opt_range("filename", field_mapping.0[3])?,
            ackfilename: row.get_opt_range("ackfilename", field_mapping.0[4])?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[5])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[7])?,
            meteringdataagent: row.get_range("meteringdataagent", field_mapping.0[8])?,
            hostdistributor: row.get_range("hostdistributor", field_mapping.0[9])?,
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
        Ok(MeterDataCustomerTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MeterDataCustomerTrk1PrimaryKey {
        MeterDataCustomerTrk1PrimaryKey {
            connectionpointid: row.connectionpointid().to_string(),
            hostdistributor: row.hostdistributor().to_string(),
            meteringdataagent: row.meteringdataagent().to_string(),
            meterrunno: row.meterrunno,
            participantid: row.participantid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("meter_data_customer_trk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MeterDataCustomerTrk1Row {
            settlementdate: row.settlementdate.clone(),
            meterrunno: row.meterrunno.clone(),
            participantid: row.participantid.clone(),
            filename: row.filename.clone(),
            ackfilename: row.ackfilename.clone(),
            connectionpointid: row.connectionpointid.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            meteringdataagent: row.meteringdataagent.clone(),
            hostdistributor: row.hostdistributor.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterDataCustomerTrk1PrimaryKey {
    pub connectionpointid: alloc::string::String,
    pub hostdistributor: alloc::string::String,
    pub meteringdataagent: alloc::string::String,
    pub meterrunno: rust_decimal::Decimal,
    pub participantid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterDataCustomerTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MeterDataCustomerTrk1Row<'data> {
    type Row<'other> = MeterDataCustomerTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid() == row.connectionpointid()
            && self.hostdistributor() == row.hostdistributor()
            && self.meteringdataagent() == row.meteringdataagent()
            && self.meterrunno == row.meterrunno
            && self.participantid() == row.participantid()
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MeterDataCustomerTrk1Row<'data> {
    type PrimaryKey = MeterDataCustomerTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid() == key.connectionpointid
            && self.hostdistributor() == key.hostdistributor
            && self.meteringdataagent() == key.meteringdataagent
            && self.meterrunno == key.meterrunno
            && self.participantid() == key.participantid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for MeterDataCustomerTrk1PrimaryKey {
    type Row<'other> = MeterDataCustomerTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid == row.connectionpointid()
            && self.hostdistributor == row.hostdistributor()
            && self.meteringdataagent == row.meteringdataagent()
            && self.meterrunno == row.meterrunno
            && self.participantid == row.participantid()
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterDataCustomerTrk1PrimaryKey {
    type PrimaryKey = MeterDataCustomerTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.hostdistributor == key.hostdistributor
            && self.meteringdataagent == key.meteringdataagent
            && self.meterrunno == key.meterrunno
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterDataCustomerTrk1 {
    type Builder = MeterDataCustomerTrk1Builder;
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
                    "meterrunno",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
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
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
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
                    "meteringdataagent",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "hostdistributor",
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
        MeterDataCustomerTrk1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            meterrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            ackfilename_array: arrow::array::builder::StringBuilder::new(),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            meteringdataagent_array: arrow::array::builder::StringBuilder::new(),
            hostdistributor_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .meterrunno_array
            .append_value({
                let mut val = row.meterrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.filename_array.append_option(row.filename());
        builder.ackfilename_array.append_option(row.ackfilename());
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder.meteringdataagent_array.append_value(row.meteringdataagent());
        builder.hostdistributor_array.append_value(row.hostdistributor());
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
                    alloc::sync::Arc::new(builder.meterrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ackfilename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meteringdataagent_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.hostdistributor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MeterDataCustomerTrk1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    meterrunno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    ackfilename_array: arrow::array::builder::StringBuilder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    meteringdataagent_array: arrow::array::builder::StringBuilder,
    hostdistributor_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BidMnspFiletrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BidMnspFiletrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidMnspFiletrk1 {
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
pub struct BidMnspFiletrk1Mapping([usize; 7]);
/// # Summary
///
/// ## MNSP_FILETRK
///
/// MNSP_FILETRK shows all MNSPOFFERS transmitted to the MMS system.
///
/// * Data Set Name: Bid
/// * File Name: Mnsp Filetrk
/// * Data Version: 1
///
/// # Description
/// MNSP_FILETRK is confidential to the relevant participant.SourceMNSP_FILETRK updates for every submitted MNSP bid.Volume4000 per year, being one per bid containing an MNSP bid
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * FILENAME
/// * OFFERDATE
/// * PARTICIPANTID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct BidMnspFiletrk1Row<'data> {
    /// Market Date from which bid is active
    pub settlementdate: chrono::NaiveDateTime,
    /// The actual date and time the bid file was submitted by the participant
    pub offerdate: chrono::NaiveDateTime,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// File name for default bids, bids, rebids, re-offers or meter files, as appropriate to table
    pub filename: core::ops::Range<usize>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: core::ops::Range<usize>,
    /// Acknowledge file name for bids, rebids
    pub ackfilename: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidMnspFiletrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn filename(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.filename.clone())
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
}
impl mmsdm_core::GetTable for BidMnspFiletrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BID";
    const TABLE_NAME: &'static str = "MNSP_FILETRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidMnspFiletrk1Mapping([
        4, 5, 6, 7, 8, 9, 10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "OFFERDATE",
        "PARTICIPANTID",
        "FILENAME",
        "STATUS",
        "ACKFILENAME",
        "LASTCHANGED",
    ];
    type Row<'row> = BidMnspFiletrk1Row<'row>;
    type FieldMapping = BidMnspFiletrk1Mapping;
    type PrimaryKey = BidMnspFiletrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidMnspFiletrk1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            filename: row.get_range("filename", field_mapping.0[3])?,
            status: row.get_opt_range("status", field_mapping.0[4])?,
            ackfilename: row.get_opt_range("ackfilename", field_mapping.0[5])?,
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
        Ok(BidMnspFiletrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidMnspFiletrk1PrimaryKey {
        BidMnspFiletrk1PrimaryKey {
            filename: row.filename().to_string(),
            offerdate: row.offerdate,
            participantid: row.participantid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bid_mnsp_filetrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidMnspFiletrk1Row {
            settlementdate: row.settlementdate.clone(),
            offerdate: row.offerdate.clone(),
            participantid: row.participantid.clone(),
            filename: row.filename.clone(),
            status: row.status.clone(),
            ackfilename: row.ackfilename.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidMnspFiletrk1PrimaryKey {
    pub filename: alloc::string::String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BidMnspFiletrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidMnspFiletrk1Row<'data> {
    type Row<'other> = BidMnspFiletrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.filename() == row.filename() && self.offerdate == row.offerdate
            && self.participantid() == row.participantid()
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidMnspFiletrk1Row<'data> {
    type PrimaryKey = BidMnspFiletrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.filename() == key.filename && self.offerdate == key.offerdate
            && self.participantid() == key.participantid
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidMnspFiletrk1PrimaryKey {
    type Row<'other> = BidMnspFiletrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.filename == row.filename() && self.offerdate == row.offerdate
            && self.participantid == row.participantid()
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidMnspFiletrk1PrimaryKey {
    type PrimaryKey = BidMnspFiletrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.filename == key.filename && self.offerdate == key.offerdate
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidMnspFiletrk1 {
    type Builder = BidMnspFiletrk1Builder;
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
                    "offerdate",
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
                    "filename",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ackfilename",
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
        BidMnspFiletrk1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            status_array: arrow::array::builder::StringBuilder::new(),
            ackfilename_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.offerdate_array.append_value(row.offerdate.and_utc().timestamp_millis());
        builder.participantid_array.append_value(row.participantid());
        builder.filename_array.append_value(row.filename());
        builder.status_array.append_option(row.status());
        builder.ackfilename_array.append_option(row.ackfilename());
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
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ackfilename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidMnspFiletrk1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    status_array: arrow::array::builder::StringBuilder,
    ackfilename_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BidMnspOffertrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BidMnspOffertrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidMnspOffertrk1 {
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
pub struct BidMnspOffertrk1Mapping([usize; 8]);
/// # Summary
///
/// ## MNSP_OFFERTRK
///
/// MNSP_OFFERTRK records all valid MNSPOFFERS loaded into the MMS system. The authorised date reflects the date and time of the load. MNSP_OFFERTRK is key for tracking MNSP bid submission.
///
/// * Data Set Name: Bid
/// * File Name: Mnsp Offertrk
/// * Data Version: 1
///
/// # Description
/// MNSP_OFFERTRK shows own (confidential) data updates as bids are processed. All bids are available as part of next day market data.Volume4000 per year
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * FILENAME
/// * OFFERDATE
/// * PARTICIPANTID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct BidMnspOffertrk1Row<'data> {
    pub settlementdate: chrono::NaiveDateTime,
    pub offerdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
    pub participantid: core::ops::Range<usize>,
    pub filename: core::ops::Range<usize>,
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    pub authorisedby: core::ops::Range<usize>,
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidMnspOffertrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn filename(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.filename.clone())
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
impl mmsdm_core::GetTable for BidMnspOffertrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BID";
    const TABLE_NAME: &'static str = "MNSP_OFFERTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidMnspOffertrk1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "OFFERDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "FILENAME",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = BidMnspOffertrk1Row<'row>;
    type FieldMapping = BidMnspOffertrk1Mapping;
    type PrimaryKey = BidMnspOffertrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidMnspOffertrk1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
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
            filename: row.get_range("filename", field_mapping.0[4])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[6])?,
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
        Ok(BidMnspOffertrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidMnspOffertrk1PrimaryKey {
        BidMnspOffertrk1PrimaryKey {
            filename: row.filename().to_string(),
            offerdate: row.offerdate,
            participantid: row.participantid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bid_mnsp_offertrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidMnspOffertrk1Row {
            settlementdate: row.settlementdate.clone(),
            offerdate: row.offerdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            filename: row.filename.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidMnspOffertrk1PrimaryKey {
    pub filename: alloc::string::String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BidMnspOffertrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidMnspOffertrk1Row<'data> {
    type Row<'other> = BidMnspOffertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.filename() == row.filename() && self.offerdate == row.offerdate
            && self.participantid() == row.participantid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidMnspOffertrk1Row<'data> {
    type PrimaryKey = BidMnspOffertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.filename() == key.filename && self.offerdate == key.offerdate
            && self.participantid() == key.participantid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidMnspOffertrk1PrimaryKey {
    type Row<'other> = BidMnspOffertrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.filename == row.filename() && self.offerdate == row.offerdate
            && self.participantid == row.participantid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidMnspOffertrk1PrimaryKey {
    type PrimaryKey = BidMnspOffertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.filename == key.filename && self.offerdate == key.offerdate
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidMnspOffertrk1 {
    type Builder = BidMnspOffertrk1Builder;
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
                    "offerdate",
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
                    "filename",
                    arrow::datatypes::DataType::Utf8,
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
        BidMnspOffertrk1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.offerdate_array.append_value(row.offerdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.filename_array.append_value(row.filename());
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
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
pub struct BidMnspOffertrk1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct BidMnspPeroffer1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &BidMnspPeroffer1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidMnspPeroffer1 {
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
pub struct BidMnspPeroffer1Mapping([usize; 22]);
/// # Summary
///
/// ## MNSP_PEROFFER
///
/// MNSP_PEROFFER shows period by period availability and other period data pertaining to a specific bid and LinkID for the given Settlement Date.MNSP_PEROFFER is a child to MNSP_DAYOFFER and links to MNSP_OFFERTRK.
///
/// * Data Set Name: Bid
/// * File Name: Mnsp Peroffer
/// * Data Version: 1
///
/// # Description
/// MNSP_PEROFFER shows own (confidential) data updates as bids are processed. All bids are available as part of next day market data.Volume192, 000 per year
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * LINKID
/// * OFFERDATE
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct BidMnspPeroffer1Row<'data> {
    /// Market Date from which bid is active
    pub settlementdate: chrono::NaiveDateTime,
    /// Offer date for bid
    pub offerdate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: core::ops::Range<usize>,
    /// Trading Interval number
    pub periodid: mmsdm_core::TradingPeriod,
    /// Maximum planned availability MW
    pub maxavail: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail1: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail2: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail3: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail4: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail5: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail6: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail7: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail8: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail9: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    pub bandavail10: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Inflexibility flag and availability. Fixed unit output MW. A value of zero means no fixed load so the unit is dispatched according to bid and market (rather than zero fixed load)
    pub fixedload: Option<rust_decimal::Decimal>,
    /// Ramp rate (MW / min) in the positive direction of flow for this MNSP link for this half-hour period
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Mandatory Restriction Offer amount
    pub mr_capacity: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidMnspPeroffer1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn linkid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.linkid.clone())
    }
}
impl mmsdm_core::GetTable for BidMnspPeroffer1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BID";
    const TABLE_NAME: &'static str = "MNSP_PEROFFER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidMnspPeroffer1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "OFFERDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "LINKID",
        "PERIODID",
        "MAXAVAIL",
        "BANDAVAIL1",
        "BANDAVAIL2",
        "BANDAVAIL3",
        "BANDAVAIL4",
        "BANDAVAIL5",
        "BANDAVAIL6",
        "BANDAVAIL7",
        "BANDAVAIL8",
        "BANDAVAIL9",
        "BANDAVAIL10",
        "LASTCHANGED",
        "FIXEDLOAD",
        "RAMPUPRATE",
        "PASAAVAILABILITY",
        "MR_CAPACITY",
    ];
    type Row<'row> = BidMnspPeroffer1Row<'row>;
    type FieldMapping = BidMnspPeroffer1Mapping;
    type PrimaryKey = BidMnspPeroffer1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidMnspPeroffer1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
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
            linkid: row.get_range("linkid", field_mapping.0[4])?,
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[5])?,
            maxavail: row
                .get_opt_custom_parsed_at_idx(
                    "maxavail",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail1: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail1",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail2: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail2",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail3: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail3",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail4: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail4",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail5: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail5",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail6: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail6",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail7: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail7",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail8: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail8",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail9: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail9",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail10: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail10",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[17],
                    mmsdm_core::mms_datetime::parse,
                )?,
            fixedload: row
                .get_opt_custom_parsed_at_idx(
                    "fixedload",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampuprate: row
                .get_opt_custom_parsed_at_idx(
                    "rampuprate",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            pasaavailability: row
                .get_opt_custom_parsed_at_idx(
                    "pasaavailability",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mr_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "mr_capacity",
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
        Ok(BidMnspPeroffer1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidMnspPeroffer1PrimaryKey {
        BidMnspPeroffer1PrimaryKey {
            linkid: row.linkid().to_string(),
            offerdate: row.offerdate,
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bid_mnsp_peroffer_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidMnspPeroffer1Row {
            settlementdate: row.settlementdate.clone(),
            offerdate: row.offerdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            linkid: row.linkid.clone(),
            periodid: row.periodid.clone(),
            maxavail: row.maxavail.clone(),
            bandavail1: row.bandavail1.clone(),
            bandavail2: row.bandavail2.clone(),
            bandavail3: row.bandavail3.clone(),
            bandavail4: row.bandavail4.clone(),
            bandavail5: row.bandavail5.clone(),
            bandavail6: row.bandavail6.clone(),
            bandavail7: row.bandavail7.clone(),
            bandavail8: row.bandavail8.clone(),
            bandavail9: row.bandavail9.clone(),
            bandavail10: row.bandavail10.clone(),
            lastchanged: row.lastchanged.clone(),
            fixedload: row.fixedload.clone(),
            rampuprate: row.rampuprate.clone(),
            pasaavailability: row.pasaavailability.clone(),
            mr_capacity: row.mr_capacity.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidMnspPeroffer1PrimaryKey {
    pub linkid: alloc::string::String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub periodid: mmsdm_core::TradingPeriod,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BidMnspPeroffer1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidMnspPeroffer1Row<'data> {
    type Row<'other> = BidMnspPeroffer1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid() == row.linkid() && self.offerdate == row.offerdate
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidMnspPeroffer1Row<'data> {
    type PrimaryKey = BidMnspPeroffer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid() == key.linkid && self.offerdate == key.offerdate
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidMnspPeroffer1PrimaryKey {
    type Row<'other> = BidMnspPeroffer1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid == row.linkid() && self.offerdate == row.offerdate
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidMnspPeroffer1PrimaryKey {
    type PrimaryKey = BidMnspPeroffer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid && self.offerdate == key.offerdate
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidMnspPeroffer1 {
    type Builder = BidMnspPeroffer1Builder;
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
                    "offerdate",
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
                    "maxavail",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail1",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail2",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail3",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail4",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail5",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail6",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail7",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail8",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail9",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail10",
                    arrow::datatypes::DataType::Decimal128(6, 0),
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
                    "fixedload",
                    arrow::datatypes::DataType::Decimal128(12, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rampuprate",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pasaavailability",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mr_capacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BidMnspPeroffer1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            linkid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            maxavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            bandavail10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            fixedload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 6)),
            rampuprate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            pasaavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            mr_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder.offerdate_array.append_value(row.offerdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.linkid_array.append_value(row.linkid());
        builder
            .periodid_array
            .append_value(row.periodid.start().and_utc().timestamp_millis());
        builder
            .maxavail_array
            .append_option({
                row.maxavail
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail1_array
            .append_option({
                row.bandavail1
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail2_array
            .append_option({
                row.bandavail2
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail3_array
            .append_option({
                row.bandavail3
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail4_array
            .append_option({
                row.bandavail4
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail5_array
            .append_option({
                row.bandavail5
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail6_array
            .append_option({
                row.bandavail6
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail7_array
            .append_option({
                row.bandavail7
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail8_array
            .append_option({
                row.bandavail8
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail9_array
            .append_option({
                row.bandavail9
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .bandavail10_array
            .append_option({
                row.bandavail10
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .fixedload_array
            .append_option({
                row.fixedload
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .rampuprate_array
            .append_option({
                row.rampuprate
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .pasaavailability_array
            .append_option({
                row.pasaavailability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .mr_capacity_array
            .append_option({
                row.mr_capacity
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
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.linkid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxavail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bandavail10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fixedload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rampuprate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pasaavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mr_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidMnspPeroffer1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    linkid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::TimestampMillisecondBuilder,
    maxavail_array: arrow::array::builder::Decimal128Builder,
    bandavail1_array: arrow::array::builder::Decimal128Builder,
    bandavail2_array: arrow::array::builder::Decimal128Builder,
    bandavail3_array: arrow::array::builder::Decimal128Builder,
    bandavail4_array: arrow::array::builder::Decimal128Builder,
    bandavail5_array: arrow::array::builder::Decimal128Builder,
    bandavail6_array: arrow::array::builder::Decimal128Builder,
    bandavail7_array: arrow::array::builder::Decimal128Builder,
    bandavail8_array: arrow::array::builder::Decimal128Builder,
    bandavail9_array: arrow::array::builder::Decimal128Builder,
    bandavail10_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    fixedload_array: arrow::array::builder::Decimal128Builder,
    rampuprate_array: arrow::array::builder::Decimal128Builder,
    pasaavailability_array: arrow::array::builder::Decimal128Builder,
    mr_capacity_array: arrow::array::builder::Decimal128Builder,
}
pub struct MrDayofferStack1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MrDayofferStack1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MrDayofferStack1 {
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
pub struct MrDayofferStack1Mapping([usize; 12]);
/// # Summary
///
/// ## MR_DAYOFFER_STACK
///
/// MR_DAYOFFER_STACK defines the Stack order for each version of the Acceptance Schedule, including all units submitting MR offers for that event. MR_DAYOFFER_STACK is the child to MR_EVENT_SCHEDULE, and parent to MR_PEROFFER_STACK.
///
/// * Data Set Name: Mr
/// * File Name: Dayoffer Stack
/// * Data Version: 1
///
/// # Description
/// Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_EVENT_SCHEDULE, MR_DAYOFFER_STACK and MR_PEROFFER_STACK.MR_DAYOFFER_STACK sets includes all generators/MNSPs in the region that submitted an MR offer and a primary key reference to the Offer tables to identify the specific offer used for that unit. MR_DAYOFFER_STACK also includes a Stack Order, irrespective of whether the unit is required to meet the Schedule.MR_DAYOFFER_STACK updates are confidential on day of submission, with public exposure the next day.SourceMR_DAYOFFER_STACK updates are ad hoc.Volume100 rows per year
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MrDayofferStack1Row<'data> {
    /// Mandatory Restriction imposition date
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: core::ops::Range<usize>,
    /// Allows many Stack versions
    pub version_datetime: chrono::NaiveDateTime,
    /// Loss Adjusted Offer Factor Stack order starting at 1
    pub stack_position: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID
    pub duid: core::ops::Range<usize>,
    /// Confirms the unit is allowed to Contribute MR Capacity
    pub authorised: Option<rust_decimal::Decimal>,
    /// Foreign key reference to XXXX_DayOffer.SettlementDate
    pub offer_settlementdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to XXXX_DayOffer.OfferDate
    pub offer_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to XXXX_DayOffer.VersionNo
    pub offer_versionno: Option<rust_decimal::Decimal>,
    /// Source tables - ENERGY or MNSP
    pub offer_type: core::ops::Range<usize>,
    /// Loss Adjusted Offer Factor = TLF times MR_Factor
    pub laof: Option<rust_decimal::Decimal>,
    /// Date and time the record was last inserted/modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MrDayofferStack1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
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
    pub fn offer_type(&self) -> Option<&str> {
        if self.offer_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.offer_type.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MrDayofferStack1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MR";
    const TABLE_NAME: &'static str = "DAYOFFER_STACK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MrDayofferStack1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MR_DATE",
        "REGIONID",
        "VERSION_DATETIME",
        "STACK_POSITION",
        "DUID",
        "AUTHORISED",
        "OFFER_SETTLEMENTDATE",
        "OFFER_OFFERDATE",
        "OFFER_VERSIONNO",
        "OFFER_TYPE",
        "LAOF",
        "LASTCHANGED",
    ];
    type Row<'row> = MrDayofferStack1Row<'row>;
    type FieldMapping = MrDayofferStack1Mapping;
    type PrimaryKey = MrDayofferStack1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MrDayofferStack1Row {
            mr_date: row
                .get_custom_parsed_at_idx(
                    "mr_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            stack_position: row
                .get_custom_parsed_at_idx(
                    "stack_position",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            duid: row.get_opt_range("duid", field_mapping.0[4])?,
            authorised: row
                .get_opt_custom_parsed_at_idx(
                    "authorised",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            offer_settlementdate: row
                .get_opt_custom_parsed_at_idx(
                    "offer_settlementdate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offer_offerdate: row
                .get_opt_custom_parsed_at_idx(
                    "offer_offerdate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offer_versionno: row
                .get_opt_custom_parsed_at_idx(
                    "offer_versionno",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            offer_type: row.get_opt_range("offer_type", field_mapping.0[9])?,
            laof: row
                .get_opt_custom_parsed_at_idx(
                    "laof",
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
        Ok(MrDayofferStack1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MrDayofferStack1PrimaryKey {
        MrDayofferStack1PrimaryKey {
            mr_date: row.mr_date,
            regionid: row.regionid().to_string(),
            stack_position: row.stack_position,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mr_dayoffer_stack_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MrDayofferStack1Row {
            mr_date: row.mr_date.clone(),
            regionid: row.regionid.clone(),
            version_datetime: row.version_datetime.clone(),
            stack_position: row.stack_position.clone(),
            duid: row.duid.clone(),
            authorised: row.authorised.clone(),
            offer_settlementdate: row.offer_settlementdate.clone(),
            offer_offerdate: row.offer_offerdate.clone(),
            offer_versionno: row.offer_versionno.clone(),
            offer_type: row.offer_type.clone(),
            laof: row.laof.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrDayofferStack1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub stack_position: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MrDayofferStack1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MrDayofferStack1Row<'data> {
    type Row<'other> = MrDayofferStack1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.regionid() == row.regionid()
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MrDayofferStack1Row<'data> {
    type PrimaryKey = MrDayofferStack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid() == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MrDayofferStack1PrimaryKey {
    type Row<'other> = MrDayofferStack1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid()
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrDayofferStack1PrimaryKey {
    type PrimaryKey = MrDayofferStack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MrDayofferStack1 {
    type Builder = MrDayofferStack1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "mr_date",
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
                    "version_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "stack_position",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorised",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offer_settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offer_offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offer_versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offer_type",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "laof",
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
        MrDayofferStack1Builder {
            mr_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            stack_position_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            authorised_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            offer_settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offer_offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offer_versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            offer_type_array: arrow::array::builder::StringBuilder::new(),
            laof_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.mr_date_array.append_value(row.mr_date.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .stack_position_array
            .append_value({
                let mut val = row.stack_position;
                val.rescale(0);
                val.mantissa()
            });
        builder.duid_array.append_option(row.duid());
        builder
            .authorised_array
            .append_option({
                row.authorised
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .offer_settlementdate_array
            .append_option(
                row.offer_settlementdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .offer_offerdate_array
            .append_option(
                row.offer_offerdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .offer_versionno_array
            .append_option({
                row.offer_versionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.offer_type_array.append_option(row.offer_type());
        builder
            .laof_array
            .append_option({
                row.laof
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
                    alloc::sync::Arc::new(builder.mr_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stack_position_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorised_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.laof_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MrDayofferStack1Builder {
    mr_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    stack_position_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    authorised_array: arrow::array::builder::Decimal128Builder,
    offer_settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offer_offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offer_versionno_array: arrow::array::builder::Decimal128Builder,
    offer_type_array: arrow::array::builder::StringBuilder,
    laof_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MrEvent1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&MrEvent1Row<'_>) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MrEvent1 {
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
pub struct MrEvent1Mapping([usize; 8]);
/// # Summary
///
/// ## MR_EVENT
///
/// MR_EVENT defines an MR Event for a given region on a specific trading date.
///
/// * Data Set Name: Mr
/// * File Name: Event
/// * Data Version: 1
///
/// # Description
/// MR_EVENT defines a mandatory restriction event for a given region and trading date (04:30 to 04:00). Data within MR_EVENT includes the cut-off time for submission of MR offers for this event and a notification that the settlements figures are locked due to results from an independent expert being engaged to allocate settlement of a significant shortfall. If mandatory restrictions are defined in two regions on the same trading day, two MR events are defined.MR_EVENT data is public, so is available to all participants.SourceMR_EVENT updates are ad hoc.Volume1 Row per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct MrEvent1Row<'data> {
    /// Mandatory Restriction imposition date
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: core::ops::Range<usize>,
    /// Description of MR
    pub description: core::ops::Range<usize>,
    /// Required for MR_Event to take effect
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Ignored - Tracking purpose only
    pub authorisedby: core::ops::Range<usize>,
    /// Cut off after when new Offers and Scaling Factor changes are disallowed
    pub offer_cut_off_time: Option<chrono::NaiveDateTime>,
    /// Flag:1 = MR settlement figures locked. Do not recalculate, ·0 = MR settlements to be recalculated
    pub settlement_complete: Option<rust_decimal::Decimal>,
    /// Date/Time record inserted/modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MrEvent1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
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
impl mmsdm_core::GetTable for MrEvent1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MR";
    const TABLE_NAME: &'static str = "EVENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MrEvent1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MR_DATE",
        "REGIONID",
        "DESCRIPTION",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "OFFER_CUT_OFF_TIME",
        "SETTLEMENT_COMPLETE",
        "LASTCHANGED",
    ];
    type Row<'row> = MrEvent1Row<'row>;
    type FieldMapping = MrEvent1Mapping;
    type PrimaryKey = MrEvent1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MrEvent1Row {
            mr_date: row
                .get_custom_parsed_at_idx(
                    "mr_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            description: row.get_opt_range("description", field_mapping.0[2])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[4])?,
            offer_cut_off_time: row
                .get_opt_custom_parsed_at_idx(
                    "offer_cut_off_time",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlement_complete: row
                .get_opt_custom_parsed_at_idx(
                    "settlement_complete",
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
        Ok(MrEvent1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MrEvent1PrimaryKey {
        MrEvent1PrimaryKey {
            mr_date: row.mr_date,
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mr_event_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MrEvent1Row {
            mr_date: row.mr_date.clone(),
            regionid: row.regionid.clone(),
            description: row.description.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            offer_cut_off_time: row.offer_cut_off_time.clone(),
            settlement_complete: row.settlement_complete.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrEvent1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MrEvent1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MrEvent1Row<'data> {
    type Row<'other> = MrEvent1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MrEvent1Row<'data> {
    type PrimaryKey = MrEvent1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MrEvent1PrimaryKey {
    type Row<'other> = MrEvent1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrEvent1PrimaryKey {
    type PrimaryKey = MrEvent1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MrEvent1 {
    type Builder = MrEvent1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "mr_date",
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
                    "offer_cut_off_time",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "settlement_complete",
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
        MrEvent1Builder {
            mr_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            offer_cut_off_time_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlement_complete_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.mr_date_array.append_value(row.mr_date.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder.description_array.append_option(row.description());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .offer_cut_off_time_array
            .append_option(
                row.offer_cut_off_time.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .settlement_complete_array
            .append_option({
                row.settlement_complete
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
                    alloc::sync::Arc::new(builder.mr_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offer_cut_off_time_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlement_complete_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MrEvent1Builder {
    mr_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    offer_cut_off_time_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlement_complete_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MrEventSchedule1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MrEventSchedule1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MrEventSchedule1 {
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
pub struct MrEventSchedule1Mapping([usize; 9]);
/// # Summary
///
/// ## MR_EVENT_SCHEDULE
///
/// MR_EVENT_SCHEDULE defines the Stack version of the Acceptance Schedule and is the parent table to MR_DayOffer_Stack and MR_PerOffer_Stack.
///
/// * Data Set Name: Mr
/// * File Name: Event Schedule
/// * Data Version: 1
///
/// # Description
/// Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack table.The MR_EVENT_SCHEDULE table determines the existence of an MR offer acceptance stack for a specific MR schedule of an MR event. The MR_EVENT_SCHEDULE table also tracks the time each stack is exercised. MR_EVENT_SCHEDULE is public and notifies the market that a new offer stack has been created.SourceMR_EVENT_SCHEDULE updates are ad hoc.Volume2 Rows per year
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MrEventSchedule1Row<'data> {
    /// Mandatory Restriction imposition date
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: core::ops::Range<usize>,
    /// Effective Date/Time of Schedule; Allows many Stack versions
    pub version_datetime: chrono::NaiveDateTime,
    /// Foreign key reference to ResDemandTrk.EffectiveDate
    pub demand_effectivedate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to ResDemandTrk.OfferDate
    pub demand_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to ResDemandTrk.VersionNo
    pub demand_versionno: Option<rust_decimal::Decimal>,
    /// Authorised person confirming Offer Stack (AKA Acceptance)
    pub authorisedby: core::ops::Range<usize>,
    /// Date and time the Offer Stack confirmed
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Date and time the record was inserted/modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MrEventSchedule1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
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
impl mmsdm_core::GetTable for MrEventSchedule1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MR";
    const TABLE_NAME: &'static str = "EVENT_SCHEDULE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MrEventSchedule1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MR_DATE",
        "REGIONID",
        "VERSION_DATETIME",
        "DEMAND_EFFECTIVEDATE",
        "DEMAND_OFFERDATE",
        "DEMAND_VERSIONNO",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = MrEventSchedule1Row<'row>;
    type FieldMapping = MrEventSchedule1Mapping;
    type PrimaryKey = MrEventSchedule1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MrEventSchedule1Row {
            mr_date: row
                .get_custom_parsed_at_idx(
                    "mr_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            demand_effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "demand_effectivedate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            demand_offerdate: row
                .get_opt_custom_parsed_at_idx(
                    "demand_offerdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            demand_versionno: row
                .get_opt_custom_parsed_at_idx(
                    "demand_versionno",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[6])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
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
        Ok(MrEventSchedule1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MrEventSchedule1PrimaryKey {
        MrEventSchedule1PrimaryKey {
            mr_date: row.mr_date,
            regionid: row.regionid().to_string(),
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mr_event_schedule_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MrEventSchedule1Row {
            mr_date: row.mr_date.clone(),
            regionid: row.regionid.clone(),
            version_datetime: row.version_datetime.clone(),
            demand_effectivedate: row.demand_effectivedate.clone(),
            demand_offerdate: row.demand_offerdate.clone(),
            demand_versionno: row.demand_versionno.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrEventSchedule1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MrEventSchedule1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MrEventSchedule1Row<'data> {
    type Row<'other> = MrEventSchedule1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.regionid() == row.regionid()
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MrEventSchedule1Row<'data> {
    type PrimaryKey = MrEventSchedule1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid() == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MrEventSchedule1PrimaryKey {
    type Row<'other> = MrEventSchedule1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid()
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrEventSchedule1PrimaryKey {
    type PrimaryKey = MrEventSchedule1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MrEventSchedule1 {
    type Builder = MrEventSchedule1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "mr_date",
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
                    "version_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand_effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand_offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand_versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        MrEventSchedule1Builder {
            mr_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            demand_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            demand_offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            demand_versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.mr_date_array.append_value(row.mr_date.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .demand_effectivedate_array
            .append_option(
                row.demand_effectivedate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .demand_offerdate_array
            .append_option(
                row.demand_offerdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .demand_versionno_array
            .append_option({
                row.demand_versionno
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
                    alloc::sync::Arc::new(builder.mr_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_versionno_array.finish())
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
pub struct MrEventSchedule1Builder {
    mr_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    demand_effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    demand_offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    demand_versionno_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MrPerofferStack1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MrPerofferStack1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MrPerofferStack1 {
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
pub struct MrPerofferStack1Mapping([usize; 9]);
/// # Summary
///
/// ## MR_PEROFFER_STACK
///
/// MR_PEROFFER_STACK defines the accepted capacity on a period basis for the Acceptance Schedule, is a child table to MR_DayOffer_Stack and only includes records or units with accepted_capacity >0 for the specific period.
///
/// * Data Set Name: Mr
/// * File Name: Peroffer Stack
/// * Data Version: 1
///
/// # Description
/// Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack.MR_PEROFFER_STACK reports the accepted MR capacity (Accepted_Capacity) required from each unit for each trading interval. MR_PEROFFER_STACK is sparse so lists only units with accepted capacity >0 for that trading interval.  The Deducted_Capacity field allows the tracking and implementation of participant requested reductions to accepted MR capacity to be tracked and applied. MR_PEROFFER_STACK is reported confidentially to each participant to notify acceptance of an MR offer.SourceMR_PEROFFER_STACK updates are ad hoc.Volume4800 rows per year
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * PERIODID
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct MrPerofferStack1Row<'data> {
    /// Mandatory Restriction imposition date
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: core::ops::Range<usize>,
    /// Allows many Period Stack versions for the one Scaling Factor stack
    pub version_datetime: chrono::NaiveDateTime,
    /// LAOF Stack order
    pub stack_position: rust_decimal::Decimal,
    /// Trade Period for the MR Offer
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID. Only required here for CSV reports
    pub duid: core::ops::Range<usize>,
    /// MR Capacity to be Dispatched
    pub accepted_capacity: Option<rust_decimal::Decimal>,
    /// Requested capacity reduction amount
    pub deducted_capacity: Option<rust_decimal::Decimal>,
    /// Date and time the record was last  inserted/modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MrPerofferStack1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
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
impl mmsdm_core::GetTable for MrPerofferStack1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MR";
    const TABLE_NAME: &'static str = "PEROFFER_STACK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MrPerofferStack1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "MR_DATE",
        "REGIONID",
        "VERSION_DATETIME",
        "STACK_POSITION",
        "PERIODID",
        "DUID",
        "ACCEPTED_CAPACITY",
        "DEDUCTED_CAPACITY",
        "LASTCHANGED",
    ];
    type Row<'row> = MrPerofferStack1Row<'row>;
    type FieldMapping = MrPerofferStack1Mapping;
    type PrimaryKey = MrPerofferStack1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MrPerofferStack1Row {
            mr_date: row
                .get_custom_parsed_at_idx(
                    "mr_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            version_datetime: row
                .get_custom_parsed_at_idx(
                    "version_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            stack_position: row
                .get_custom_parsed_at_idx(
                    "stack_position",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            duid: row.get_opt_range("duid", field_mapping.0[5])?,
            accepted_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "accepted_capacity",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            deducted_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "deducted_capacity",
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
        Ok(MrPerofferStack1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MrPerofferStack1PrimaryKey {
        MrPerofferStack1PrimaryKey {
            mr_date: row.mr_date,
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            stack_position: row.stack_position,
            version_datetime: row.version_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("mr_peroffer_stack_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MrPerofferStack1Row {
            mr_date: row.mr_date.clone(),
            regionid: row.regionid.clone(),
            version_datetime: row.version_datetime.clone(),
            stack_position: row.stack_position.clone(),
            periodid: row.periodid.clone(),
            duid: row.duid.clone(),
            accepted_capacity: row.accepted_capacity.clone(),
            deducted_capacity: row.deducted_capacity.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrPerofferStack1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub stack_position: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MrPerofferStack1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MrPerofferStack1Row<'data> {
    type Row<'other> = MrPerofferStack1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.periodid == row.periodid
            && self.regionid() == row.regionid()
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MrPerofferStack1Row<'data> {
    type PrimaryKey = MrPerofferStack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for MrPerofferStack1PrimaryKey {
    type Row<'other> = MrPerofferStack1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mr_date == row.mr_date && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrPerofferStack1PrimaryKey {
    type PrimaryKey = MrPerofferStack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.periodid == key.periodid
            && self.regionid == key.regionid && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MrPerofferStack1 {
    type Builder = MrPerofferStack1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "mr_date",
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
                    "version_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "stack_position",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "accepted_capacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "deducted_capacity",
                    arrow::datatypes::DataType::Decimal128(6, 0),
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
        MrPerofferStack1Builder {
            mr_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            stack_position_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            accepted_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            deducted_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.mr_date_array.append_value(row.mr_date.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .version_datetime_array
            .append_value(row.version_datetime.and_utc().timestamp_millis());
        builder
            .stack_position_array
            .append_value({
                let mut val = row.stack_position;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.duid_array.append_option(row.duid());
        builder
            .accepted_capacity_array
            .append_option({
                row.accepted_capacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .deducted_capacity_array
            .append_option({
                row.deducted_capacity
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
                    alloc::sync::Arc::new(builder.mr_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.version_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stack_position_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.accepted_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deducted_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MrPerofferStack1Builder {
    mr_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    version_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    stack_position_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    accepted_capacity_array: arrow::array::builder::Decimal128Builder,
    deducted_capacity_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct P5minFcasRequirment1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &P5minFcasRequirment1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl P5minFcasRequirment1 {
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
pub struct P5minFcasRequirment1Mapping([usize; 16]);
/// # Summary
///
/// ## P5MIN_FCAS_REQUIREMENT
///
/// 5-minute Predispatch constraint tracking for Regional FCAS recovery
///
/// * Data Set Name: P5min
/// * File Name: Fcas Requirment
/// * Data Version: 1
///
/// # Description
/// PASACASESOLUTION is obsolete on 27 March 2002PASACASESOLUTION is public data, so is available to all participants.SourcePASACASESOLUTION is not used; was updated every 2 hours.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
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
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
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
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("p5min_fcas_requirment_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
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
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
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
                row.constraint_effectivedate.map(|val| val.and_utc().timestamp_millis()),
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
pub struct PredispatchRegionfcasrequirement2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PredispatchRegionfcasrequirement2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PredispatchRegionfcasrequirement2 {
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
pub struct PredispatchRegionfcasrequirement2Mapping([usize; 18]);
/// # Summary
///
/// ## PREDISPATCH_FCAS_REQ
///
/// PREDISPATCH_FCAS_REQ shows Predispatch Constraint tracking for Regional FCAS Requirements.
///
/// * Data Set Name: Predispatch
/// * File Name: Regionfcasrequirement
/// * Data Version: 2
///
/// # Description
/// SourcePREDISPATCH_FCAS_REQ updates with each pre-dispatch run (half hourly)VolumeApproximately 2,000 rows per day.NoteThe PERIODID columns in tables PREDISPATCHCONSTRAINT and PREDISPATCH_FCAS_REQ have no consistent relationship with the other PERIODID values in the other tables in the PRE-DISPATCH package (such as PREDISPATCHPRICE). AEMO and many Participants appreciate the data model is inconsistent, but the cost of changing existing systems has been judged as being unjustifiable. An additional field DATETIME was added to allow joins between these data sets.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DATETIME
/// * GENCONID
/// * REGIONID
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
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
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
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "predispatch_regionfcasrequirement_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
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
            .append_value(row.predispatchseqno.start().and_utc().timestamp_millis());
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
        builder
            .periodid_array
            .append_value(row.periodid.start().and_utc().timestamp_millis());
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
        builder.datetime_array.append_value(row.datetime.and_utc().timestamp_millis());
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
pub struct SettlementsCpdata7 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementsCpdata7Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementsCpdata7 {
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
pub struct SettlementsCpdata7Mapping([usize; 32]);
/// # Summary
///
/// ## SETCPDATA
///
/// SETCPDATA shows meter settlement data for each connection point. This is the key view for retailers to verify energy charges. A regional summary view is also provided. As the view has values for each connection point by period, for each meter data file, it is a very large view.
///
/// * Data Set Name: Settlements
/// * File Name: Cpdata
/// * Data Version: 7
///
/// # Description
/// The Connection point details (in SETCPDATA) are confidential to the participant and host retailer that the connection points relate to. By comparison, the regional data (SETCPDATAREGION) is publically available.SourceSETCPDATA updates with each Settlement run.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * MDA
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * TCPID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsCpdata7Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Connection point identifier
    pub tcpid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Import Gross energy into the pool - MWh
    pub igenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub xgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh, plus UFEA if the UFEA amount is positive. When GS commences, this includes the UFEA amount in the settlement runs.
    pub inenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh, plus (UFEA * -1) if the UFEA amount is negative. When GS commences, this includes the UFEA amount in the settlement runs.
    pub xnenergy: Option<rust_decimal::Decimal>,
    /// Import reactive power
    pub ipower: Option<rust_decimal::Decimal>,
    /// Export reactive power
    pub xpower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Export - Import of Net energy (MWh)
    pub ta: Option<rust_decimal::Decimal>,
    /// settlement amount in $ for trading period
    pub ep: Option<rust_decimal::Decimal>,
    /// Not used
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    /// Meter Run Number = version number of the meter file
    pub meterrunno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used
    pub hostdistributor: core::ops::Range<usize>,
    /// Metering Data Agent
    pub mda: core::ops::Range<usize>,
    /// Accounted For Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval, excluding any UFEA component
    pub afe: Option<rust_decimal::Decimal>,
    /// Sum of ME- for all NMIs at this Market Customer FRMP and TNI in the Settlements Trading Interval.
    pub dme: Option<rust_decimal::Decimal>,
    /// Share of UFE allocated to this FRMP and TNI in the Settlements Trading Interval.
    pub ufea: Option<rust_decimal::Decimal>,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval. When GS commences, this includes the UFEA amount in the settlement runs.
    pub age: Option<rust_decimal::Decimal>,
    /// The total cost of energy sold at the connection point by the participant in this settlement interval
    pub importenergycost: Option<rust_decimal::Decimal>,
    /// The total cost of energy purchased at the connection point by the participant in this settlement interval
    pub exportenergycost: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsCpdata7Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn tcpid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.tcpid.clone())
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
    pub fn hostdistributor(&self) -> Option<&str> {
        if self.hostdistributor.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.hostdistributor.clone(),
                ),
            )
        }
    }
    pub fn mda(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.mda.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsCpdata7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "CPDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsCpdata7Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "PARTICIPANTID",
        "TCPID",
        "REGIONID",
        "IGENERGY",
        "XGENERGY",
        "INENERGY",
        "XNENERGY",
        "IPOWER",
        "XPOWER",
        "RRP",
        "EEP",
        "TLF",
        "CPRRP",
        "CPEEP",
        "TA",
        "EP",
        "APC",
        "RESC",
        "RESP",
        "METERRUNNO",
        "LASTCHANGED",
        "HOSTDISTRIBUTOR",
        "MDA",
        "AFE",
        "DME",
        "UFEA",
        "AGE",
        "IMPORTENERGYCOST",
        "EXPORTENERGYCOST",
    ];
    type Row<'row> = SettlementsCpdata7Row<'row>;
    type FieldMapping = SettlementsCpdata7Mapping;
    type PrimaryKey = SettlementsCpdata7PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsCpdata7Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            tcpid: row.get_range("tcpid", field_mapping.0[4])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[5])?,
            igenergy: row
                .get_opt_custom_parsed_at_idx(
                    "igenergy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            xgenergy: row
                .get_opt_custom_parsed_at_idx(
                    "xgenergy",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            inenergy: row
                .get_opt_custom_parsed_at_idx(
                    "inenergy",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            xnenergy: row
                .get_opt_custom_parsed_at_idx(
                    "xnenergy",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ipower: row
                .get_opt_custom_parsed_at_idx(
                    "ipower",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            xpower: row
                .get_opt_custom_parsed_at_idx(
                    "xpower",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep: row
                .get_opt_custom_parsed_at_idx(
                    "eep",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cprrp: row
                .get_opt_custom_parsed_at_idx(
                    "cprrp",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cpeep: row
                .get_opt_custom_parsed_at_idx(
                    "cpeep",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ta: row
                .get_opt_custom_parsed_at_idx(
                    "ta",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ep: row
                .get_opt_custom_parsed_at_idx(
                    "ep",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apc: row
                .get_opt_custom_parsed_at_idx(
                    "apc",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resc: row
                .get_opt_custom_parsed_at_idx(
                    "resc",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resp: row
                .get_opt_custom_parsed_at_idx(
                    "resp",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meterrunno: row
                .get_opt_custom_parsed_at_idx(
                    "meterrunno",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[23],
                    mmsdm_core::mms_datetime::parse,
                )?,
            hostdistributor: row.get_opt_range("hostdistributor", field_mapping.0[24])?,
            mda: row.get_range("mda", field_mapping.0[25])?,
            afe: row
                .get_opt_custom_parsed_at_idx(
                    "afe",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dme: row
                .get_opt_custom_parsed_at_idx(
                    "dme",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ufea: row
                .get_opt_custom_parsed_at_idx(
                    "ufea",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            age: row
                .get_opt_custom_parsed_at_idx(
                    "age",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "importenergycost",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "exportenergycost",
                    field_mapping.0[31],
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
        Ok(SettlementsCpdata7Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsCpdata7PrimaryKey {
        SettlementsCpdata7PrimaryKey {
            mda: row.mda().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            tcpid: row.tcpid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("settlements_cpdata_v7_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsCpdata7Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            tcpid: row.tcpid.clone(),
            regionid: row.regionid.clone(),
            igenergy: row.igenergy.clone(),
            xgenergy: row.xgenergy.clone(),
            inenergy: row.inenergy.clone(),
            xnenergy: row.xnenergy.clone(),
            ipower: row.ipower.clone(),
            xpower: row.xpower.clone(),
            rrp: row.rrp.clone(),
            eep: row.eep.clone(),
            tlf: row.tlf.clone(),
            cprrp: row.cprrp.clone(),
            cpeep: row.cpeep.clone(),
            ta: row.ta.clone(),
            ep: row.ep.clone(),
            apc: row.apc.clone(),
            resc: row.resc.clone(),
            resp: row.resp.clone(),
            meterrunno: row.meterrunno.clone(),
            lastchanged: row.lastchanged.clone(),
            hostdistributor: row.hostdistributor.clone(),
            mda: row.mda.clone(),
            afe: row.afe.clone(),
            dme: row.dme.clone(),
            ufea: row.ufea.clone(),
            age: row.age.clone(),
            importenergycost: row.importenergycost.clone(),
            exportenergycost: row.exportenergycost.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsCpdata7PrimaryKey {
    pub mda: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub tcpid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsCpdata7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdata7Row<'data> {
    type Row<'other> = SettlementsCpdata7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mda() == row.mda() && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.tcpid() == row.tcpid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsCpdata7Row<'data> {
    type PrimaryKey = SettlementsCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mda() == key.mda && self.participantid() == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.tcpid() == key.tcpid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdata7PrimaryKey {
    type Row<'other> = SettlementsCpdata7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mda == row.mda() && self.participantid == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.tcpid == row.tcpid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdata7PrimaryKey {
    type PrimaryKey = SettlementsCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mda == key.mda && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.tcpid == key.tcpid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsCpdata7 {
    type Builder = SettlementsCpdata7Builder;
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tcpid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "igenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "xgenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "inenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "xnenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ipower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "xpower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(20, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cprrp",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cpeep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ta",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resp",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meterrunno",
                    arrow::datatypes::DataType::Decimal128(10, 0),
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
                    "hostdistributor",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mda",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "afe",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dme",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ufea",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "age",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importenergycost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportenergycost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsCpdata7Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            tcpid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            igenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            xgenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            inenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            xnenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ipower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            xpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 5)),
            eep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
            cprrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            cpeep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ta_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            apc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            meterrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            hostdistributor_array: arrow::array::builder::StringBuilder::new(),
            mda_array: arrow::array::builder::StringBuilder::new(),
            afe_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            dme_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            ufea_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            age_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            importenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.tcpid_array.append_value(row.tcpid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .igenergy_array
            .append_option({
                row.igenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .xgenergy_array
            .append_option({
                row.xgenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .inenergy_array
            .append_option({
                row.inenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .xnenergy_array
            .append_option({
                row.xnenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ipower_array
            .append_option({
                row.ipower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .xpower_array
            .append_option({
                row.xpower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
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
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cprrp_array
            .append_option({
                row.cprrp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .cpeep_array
            .append_option({
                row.cpeep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ta_array
            .append_option({
                row.ta
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ep_array
            .append_option({
                row.ep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .apc_array
            .append_option({
                row.apc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resc_array
            .append_option({
                row.resc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resp_array
            .append_option({
                row.resp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .meterrunno_array
            .append_option({
                row.meterrunno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.hostdistributor_array.append_option(row.hostdistributor());
        builder.mda_array.append_value(row.mda());
        builder
            .afe_array
            .append_option({
                row.afe
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .dme_array
            .append_option({
                row.dme
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .ufea_array
            .append_option({
                row.ufea
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .age_array
            .append_option({
                row.age
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .importenergycost_array
            .append_option({
                row.importenergycost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .exportenergycost_array
            .append_option({
                row.exportenergycost
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
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tcpid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.igenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.xgenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.inenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.xnenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ipower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.xpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cprrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cpeep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ta_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meterrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.hostdistributor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mda_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.afe_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dme_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ufea_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.age_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsCpdata7Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    tcpid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    igenergy_array: arrow::array::builder::Decimal128Builder,
    xgenergy_array: arrow::array::builder::Decimal128Builder,
    inenergy_array: arrow::array::builder::Decimal128Builder,
    xnenergy_array: arrow::array::builder::Decimal128Builder,
    ipower_array: arrow::array::builder::Decimal128Builder,
    xpower_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    eep_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    cprrp_array: arrow::array::builder::Decimal128Builder,
    cpeep_array: arrow::array::builder::Decimal128Builder,
    ta_array: arrow::array::builder::Decimal128Builder,
    ep_array: arrow::array::builder::Decimal128Builder,
    apc_array: arrow::array::builder::Decimal128Builder,
    resc_array: arrow::array::builder::Decimal128Builder,
    resp_array: arrow::array::builder::Decimal128Builder,
    meterrunno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    hostdistributor_array: arrow::array::builder::StringBuilder,
    mda_array: arrow::array::builder::StringBuilder,
    afe_array: arrow::array::builder::Decimal128Builder,
    dme_array: arrow::array::builder::Decimal128Builder,
    ufea_array: arrow::array::builder::Decimal128Builder,
    age_array: arrow::array::builder::Decimal128Builder,
    importenergycost_array: arrow::array::builder::Decimal128Builder,
    exportenergycost_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsCpdataregion5 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementsCpdataregion5Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementsCpdataregion5 {
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
pub struct SettlementsCpdataregion5Mapping([usize; 12]);
/// # Summary
///
/// ## SETCPDATAREGION
///
/// SETCPDATAREGION sets out summary meter settlement data for each region.
///
/// * Data Set Name: Settlements
/// * File Name: Cpdataregion
/// * Data Version: 5
///
/// # Description
/// SETCPDATAREGION data is public, so is available to all participants.SourceSETCPDATAREGION is a summary based on grouping on SETCPDATA and is updated with each settlement run.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsCpdataregion5Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Import Gross energy into the pool - MWh
    pub sumigenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub sumxgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh
    pub suminenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh
    pub sumxnenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumipower: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumxpower: Option<rust_decimal::Decimal>,
    /// current system date, to enable automatic replication
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of energy price across the region
    pub sumep: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsCpdataregion5Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsCpdataregion5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "CPDATAREGION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsCpdataregion5Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "REGIONID",
        "SUMIGENERGY",
        "SUMXGENERGY",
        "SUMINENERGY",
        "SUMXNENERGY",
        "SUMIPOWER",
        "SUMXPOWER",
        "LASTCHANGED",
        "SUMEP",
    ];
    type Row<'row> = SettlementsCpdataregion5Row<'row>;
    type FieldMapping = SettlementsCpdataregion5Mapping;
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsCpdataregion5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            sumigenergy: row
                .get_opt_custom_parsed_at_idx(
                    "sumigenergy",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumxgenergy: row
                .get_opt_custom_parsed_at_idx(
                    "sumxgenergy",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            suminenergy: row
                .get_opt_custom_parsed_at_idx(
                    "suminenergy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumxnenergy: row
                .get_opt_custom_parsed_at_idx(
                    "sumxnenergy",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumipower: row
                .get_opt_custom_parsed_at_idx(
                    "sumipower",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumxpower: row
                .get_opt_custom_parsed_at_idx(
                    "sumxpower",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            sumep: row
                .get_opt_custom_parsed_at_idx(
                    "sumep",
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
        Ok(SettlementsCpdataregion5Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsCpdataregion5PrimaryKey {
        SettlementsCpdataregion5PrimaryKey {
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("settlements_cpdataregion_v5_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsCpdataregion5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            sumigenergy: row.sumigenergy.clone(),
            sumxgenergy: row.sumxgenergy.clone(),
            suminenergy: row.suminenergy.clone(),
            sumxnenergy: row.sumxnenergy.clone(),
            sumipower: row.sumipower.clone(),
            sumxpower: row.sumxpower.clone(),
            lastchanged: row.lastchanged.clone(),
            sumep: row.sumep.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsCpdataregion5PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsCpdataregion5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdataregion5Row<'data> {
    type Row<'other> = SettlementsCpdataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsCpdataregion5Row<'data> {
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdataregion5PrimaryKey {
    type Row<'other> = SettlementsCpdataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdataregion5PrimaryKey {
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsCpdataregion5 {
    type Builder = SettlementsCpdataregion5Builder;
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sumigenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumxgenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "suminenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumxnenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumipower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumxpower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
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
                    "sumep",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsCpdataregion5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            sumigenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            sumxgenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            suminenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            sumxnenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            sumipower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            sumxpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            sumep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(10);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(10);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .sumigenergy_array
            .append_option({
                row.sumigenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .sumxgenergy_array
            .append_option({
                row.sumxgenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .suminenergy_array
            .append_option({
                row.suminenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .sumxnenergy_array
            .append_option({
                row.sumxnenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .sumipower_array
            .append_option({
                row.sumipower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .sumxpower_array
            .append_option({
                row.sumxpower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .sumep_array
            .append_option({
                row.sumep
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
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumigenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumxgenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.suminenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumxnenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumipower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumxpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsCpdataregion5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    sumigenergy_array: arrow::array::builder::Decimal128Builder,
    sumxgenergy_array: arrow::array::builder::Decimal128Builder,
    suminenergy_array: arrow::array::builder::Decimal128Builder,
    sumxnenergy_array: arrow::array::builder::Decimal128Builder,
    sumipower_array: arrow::array::builder::Decimal128Builder,
    sumxpower_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    sumep_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsGendata6 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementsGendata6Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementsGendata6 {
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
pub struct SettlementsGendata6Mapping([usize; 29]);
/// # Summary
///
/// ## SETGENDATA
///
/// SETGENDATA shows meter settlement data for each generation meter point. A regional summary is also provided.
///
/// * Data Set Name: Settlements
/// * File Name: Gendata
/// * Data Version: 6
///
/// # Description
/// SETGENDATA shows generator meter details, and SETGENDATA data is confidential to the participant.By comparison, the regional summary (SETGENDATAREGION) is public data.SourceSETGENDATA updates with each Settlement run.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * DUID
/// * GENSETID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsGendata6Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// Dispatchable Unit identifier
    pub duid: core::ops::Range<usize>,
    /// Physical unit identifier
    pub gensetid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Generated energy
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used
    pub apower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Generated energy
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Administered Price Compensation
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Export Energy (Generator Purchases) (MWh)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// Identifier of the meter run used in this settlement calculation
    pub meterrunno: Option<rust_decimal::Decimal>,
    /// Metering Data Agent
    pub mda: core::ops::Range<usize>,
    /// Secondary Transmission Loss Factor
    pub secondary_tlf: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsGendata6Row<'data> {
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
    pub fn stationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.stationid.clone())
    }
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn gensetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.gensetid.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn mda(&self) -> Option<&str> {
        if self.mda.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.mda.clone()))
        }
    }
}
impl mmsdm_core::GetTable for SettlementsGendata6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "GENDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsGendata6Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "PARTICIPANTID",
        "STATIONID",
        "DUID",
        "GENSETID",
        "REGIONID",
        "GENERGY",
        "AENERGY",
        "GPOWER",
        "APOWER",
        "RRP",
        "EEP",
        "TLF",
        "CPRRP",
        "CPEEP",
        "NETENERGY",
        "ENERGYCOST",
        "EXCESSENERGYCOST",
        "APC",
        "RESC",
        "RESP",
        "LASTCHANGED",
        "EXPENERGY",
        "EXPENERGYCOST",
        "METERRUNNO",
        "MDA",
        "SECONDARY_TLF",
    ];
    type Row<'row> = SettlementsGendata6Row<'row>;
    type FieldMapping = SettlementsGendata6Mapping;
    type PrimaryKey = SettlementsGendata6PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsGendata6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_opt_range("participantid", field_mapping.0[3])?,
            stationid: row.get_range("stationid", field_mapping.0[4])?,
            duid: row.get_range("duid", field_mapping.0[5])?,
            gensetid: row.get_range("gensetid", field_mapping.0[6])?,
            regionid: row.get_range("regionid", field_mapping.0[7])?,
            genergy: row
                .get_opt_custom_parsed_at_idx(
                    "genergy",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aenergy: row
                .get_opt_custom_parsed_at_idx(
                    "aenergy",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            gpower: row
                .get_opt_custom_parsed_at_idx(
                    "gpower",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apower: row
                .get_opt_custom_parsed_at_idx(
                    "apower",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep: row
                .get_opt_custom_parsed_at_idx(
                    "eep",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cprrp: row
                .get_opt_custom_parsed_at_idx(
                    "cprrp",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cpeep: row
                .get_opt_custom_parsed_at_idx(
                    "cpeep",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netenergy: row
                .get_opt_custom_parsed_at_idx(
                    "netenergy",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energycost: row
                .get_opt_custom_parsed_at_idx(
                    "energycost",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            excessenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "excessenergycost",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apc: row
                .get_opt_custom_parsed_at_idx(
                    "apc",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resc: row
                .get_opt_custom_parsed_at_idx(
                    "resc",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resp: row
                .get_opt_custom_parsed_at_idx(
                    "resp",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[23],
                    mmsdm_core::mms_datetime::parse,
                )?,
            expenergy: row
                .get_opt_custom_parsed_at_idx(
                    "expenergy",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "expenergycost",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meterrunno: row
                .get_opt_custom_parsed_at_idx(
                    "meterrunno",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mda: row.get_opt_range("mda", field_mapping.0[27])?,
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
        Ok(SettlementsGendata6Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsGendata6PrimaryKey {
        SettlementsGendata6PrimaryKey {
            duid: row.duid().to_string(),
            gensetid: row.gensetid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            stationid: row.stationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("settlements_gendata_v6_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsGendata6Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            stationid: row.stationid.clone(),
            duid: row.duid.clone(),
            gensetid: row.gensetid.clone(),
            regionid: row.regionid.clone(),
            genergy: row.genergy.clone(),
            aenergy: row.aenergy.clone(),
            gpower: row.gpower.clone(),
            apower: row.apower.clone(),
            rrp: row.rrp.clone(),
            eep: row.eep.clone(),
            tlf: row.tlf.clone(),
            cprrp: row.cprrp.clone(),
            cpeep: row.cpeep.clone(),
            netenergy: row.netenergy.clone(),
            energycost: row.energycost.clone(),
            excessenergycost: row.excessenergycost.clone(),
            apc: row.apc.clone(),
            resc: row.resc.clone(),
            resp: row.resp.clone(),
            lastchanged: row.lastchanged.clone(),
            expenergy: row.expenergy.clone(),
            expenergycost: row.expenergycost.clone(),
            meterrunno: row.meterrunno.clone(),
            mda: row.mda.clone(),
            secondary_tlf: row.secondary_tlf.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsGendata6PrimaryKey {
    pub duid: alloc::string::String,
    pub gensetid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub stationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsGendata6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendata6Row<'data> {
    type Row<'other> = SettlementsGendata6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.gensetid() == row.gensetid()
            && self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.stationid() == row.stationid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsGendata6Row<'data> {
    type PrimaryKey = SettlementsGendata6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.gensetid() == key.gensetid
            && self.periodid == key.periodid && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.stationid() == key.stationid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendata6PrimaryKey {
    type Row<'other> = SettlementsGendata6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.gensetid == row.gensetid()
            && self.periodid == row.periodid && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.stationid == row.stationid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendata6PrimaryKey {
    type PrimaryKey = SettlementsGendata6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.gensetid == key.gensetid
            && self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.stationid == key.stationid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsGendata6 {
    type Builder = SettlementsGendata6Builder;
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
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
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gpower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(20, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cprrp",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cpeep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "netenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energycost",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "excessenergycost",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resp",
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
                    "expenergy",
                    arrow::datatypes::DataType::Decimal128(15, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergycost",
                    arrow::datatypes::DataType::Decimal128(15, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meterrunno",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mda",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "secondary_tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsGendata6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            gensetid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            genergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            aenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            gpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            apower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 5)),
            eep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
            cprrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            cpeep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            netenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            energycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            excessenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            apc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            expenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 6)),
            expenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 6)),
            meterrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            mda_array: arrow::array::builder::StringBuilder::new(),
            secondary_tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_option(row.participantid());
        builder.stationid_array.append_value(row.stationid());
        builder.duid_array.append_value(row.duid());
        builder.gensetid_array.append_value(row.gensetid());
        builder.regionid_array.append_value(row.regionid());
        builder
            .genergy_array
            .append_option({
                row.genergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .aenergy_array
            .append_option({
                row.aenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .gpower_array
            .append_option({
                row.gpower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .apower_array
            .append_option({
                row.apower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
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
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cprrp_array
            .append_option({
                row.cprrp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .cpeep_array
            .append_option({
                row.cpeep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .netenergy_array
            .append_option({
                row.netenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .energycost_array
            .append_option({
                row.energycost
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .excessenergycost_array
            .append_option({
                row.excessenergycost
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .apc_array
            .append_option({
                row.apc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resc_array
            .append_option({
                row.resc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resp_array
            .append_option({
                row.resp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .expenergy_array
            .append_option({
                row.expenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .expenergycost_array
            .append_option({
                row.expenergycost
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .meterrunno_array
            .append_option({
                row.meterrunno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.mda_array.append_option(row.mda());
        builder
            .secondary_tlf_array
            .append_option({
                row.secondary_tlf
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
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gensetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cprrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cpeep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.netenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.excessenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meterrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mda_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.secondary_tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsGendata6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    gensetid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    genergy_array: arrow::array::builder::Decimal128Builder,
    aenergy_array: arrow::array::builder::Decimal128Builder,
    gpower_array: arrow::array::builder::Decimal128Builder,
    apower_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    eep_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    cprrp_array: arrow::array::builder::Decimal128Builder,
    cpeep_array: arrow::array::builder::Decimal128Builder,
    netenergy_array: arrow::array::builder::Decimal128Builder,
    energycost_array: arrow::array::builder::Decimal128Builder,
    excessenergycost_array: arrow::array::builder::Decimal128Builder,
    apc_array: arrow::array::builder::Decimal128Builder,
    resc_array: arrow::array::builder::Decimal128Builder,
    resp_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    expenergy_array: arrow::array::builder::Decimal128Builder,
    expenergycost_array: arrow::array::builder::Decimal128Builder,
    meterrunno_array: arrow::array::builder::Decimal128Builder,
    mda_array: arrow::array::builder::StringBuilder,
    secondary_tlf_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsGendataregion5 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementsGendataregion5Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementsGendataregion5 {
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
pub struct SettlementsGendataregion5Mapping([usize; 14]);
/// # Summary
///
/// ## SETGENDATAREGION
///
/// SETGENDATAREGION sets out summary settlement data for generation within the specified region.
///
/// * Data Set Name: Settlements
/// * File Name: Gendataregion
/// * Data Version: 5
///
/// # Description
/// SETGENDATAREGION shows the regional summary. SETGENDATAREGION is public data.SourceSETGENDATAREGION updates with each Settlement run.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsGendataregion5Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Generated energy - Not used in MMS Data Model
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy - Not used in MMS Data Model
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub apower: Option<rust_decimal::Decimal>,
    /// Net energy MW/hours
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy (Generator Purchases)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// current system date, to enable automatic replication
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsGendataregion5Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsGendataregion5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "GENDATAREGION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsGendataregion5Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "REGIONID",
        "GENERGY",
        "AENERGY",
        "GPOWER",
        "APOWER",
        "NETENERGY",
        "ENERGYCOST",
        "EXCESSENERGYCOST",
        "EXPENERGY",
        "EXPENERGYCOST",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsGendataregion5Row<'row>;
    type FieldMapping = SettlementsGendataregion5Mapping;
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsGendataregion5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            genergy: row
                .get_opt_custom_parsed_at_idx(
                    "genergy",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aenergy: row
                .get_opt_custom_parsed_at_idx(
                    "aenergy",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            gpower: row
                .get_opt_custom_parsed_at_idx(
                    "gpower",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apower: row
                .get_opt_custom_parsed_at_idx(
                    "apower",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netenergy: row
                .get_opt_custom_parsed_at_idx(
                    "netenergy",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energycost: row
                .get_opt_custom_parsed_at_idx(
                    "energycost",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            excessenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "excessenergycost",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergy: row
                .get_opt_custom_parsed_at_idx(
                    "expenergy",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "expenergycost",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[13],
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
        Ok(SettlementsGendataregion5Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsGendataregion5PrimaryKey {
        SettlementsGendataregion5PrimaryKey {
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("settlements_gendataregion_v5_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsGendataregion5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            genergy: row.genergy.clone(),
            aenergy: row.aenergy.clone(),
            gpower: row.gpower.clone(),
            apower: row.apower.clone(),
            netenergy: row.netenergy.clone(),
            energycost: row.energycost.clone(),
            excessenergycost: row.excessenergycost.clone(),
            expenergy: row.expenergy.clone(),
            expenergycost: row.expenergycost.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsGendataregion5PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsGendataregion5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendataregion5Row<'data> {
    type Row<'other> = SettlementsGendataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsGendataregion5Row<'data> {
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendataregion5PrimaryKey {
    type Row<'other> = SettlementsGendataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendataregion5PrimaryKey {
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsGendataregion5 {
    type Builder = SettlementsGendataregion5Builder;
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genergy",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aenergy",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gpower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "netenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energycost",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "excessenergycost",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergy",
                    arrow::datatypes::DataType::Decimal128(27, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergycost",
                    arrow::datatypes::DataType::Decimal128(27, 6),
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
        SettlementsGendataregion5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            genergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            aenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            gpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            apower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            netenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            energycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            excessenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            expenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 6)),
            expenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(10);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(10);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .genergy_array
            .append_option({
                row.genergy
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .aenergy_array
            .append_option({
                row.aenergy
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .gpower_array
            .append_option({
                row.gpower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .apower_array
            .append_option({
                row.apower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .netenergy_array
            .append_option({
                row.netenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .energycost_array
            .append_option({
                row.energycost
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .excessenergycost_array
            .append_option({
                row.excessenergycost
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .expenergy_array
            .append_option({
                row.expenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .expenergycost_array
            .append_option({
                row.expenergycost
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
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.netenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.excessenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsGendataregion5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    genergy_array: arrow::array::builder::Decimal128Builder,
    aenergy_array: arrow::array::builder::Decimal128Builder,
    gpower_array: arrow::array::builder::Decimal128Builder,
    apower_array: arrow::array::builder::Decimal128Builder,
    netenergy_array: arrow::array::builder::Decimal128Builder,
    energycost_array: arrow::array::builder::Decimal128Builder,
    excessenergycost_array: arrow::array::builder::Decimal128Builder,
    expenergy_array: arrow::array::builder::Decimal128Builder,
    expenergycost_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsSmallgendata1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &SettlementsSmallgendata1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl SettlementsSmallgendata1 {
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
pub struct SettlementsSmallgendata1Mapping([usize; 13]);
/// # Summary
///
/// ## SETSMALLGENDATA
///
/// Publishes metering data and associated settlement values for with a registered Small Generator Aggregator participants connection points.
///
/// * Data Set Name: Settlements
/// * File Name: Smallgendata
/// * Data Version: 1
///
/// # Description
/// SETVICBOUNDARYENERGY data is confidential to the relevant participants.SourceSETVICBOUNDARYENERGY is populated by the posting of a billing run.VolumeGenerally there are approximately 550 records inserted per week.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsSmallgendata1Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number of the record for the settlement date
    pub versionno: rust_decimal::Decimal,
    /// Transmission Node Identifier (TNI)
    pub connectionpointid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// The import direction value for the meter read (MWh)
    pub importenergy: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportenergy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Import Energy Cost ($)
    pub impenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// Last date and time the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsSmallgendata1Row<'data> {
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
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
impl mmsdm_core::GetTable for SettlementsSmallgendata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "SMALLGENDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsSmallgendata1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "CONNECTIONPOINTID",
        "PERIODID",
        "PARTICIPANTID",
        "REGIONID",
        "IMPORTENERGY",
        "EXPORTENERGY",
        "RRP",
        "TLF",
        "IMPENERGYCOST",
        "EXPENERGYCOST",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsSmallgendata1Row<'row>;
    type FieldMapping = SettlementsSmallgendata1Mapping;
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsSmallgendata1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[5])?,
            importenergy: row
                .get_opt_custom_parsed_at_idx(
                    "importenergy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportenergy: row
                .get_opt_custom_parsed_at_idx(
                    "exportenergy",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            impenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "impenergycost",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "expenergycost",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[12],
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
        Ok(SettlementsSmallgendata1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsSmallgendata1PrimaryKey {
        SettlementsSmallgendata1PrimaryKey {
            connectionpointid: row.connectionpointid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("settlements_smallgendata_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsSmallgendata1Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            connectionpointid: row.connectionpointid.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            regionid: row.regionid.clone(),
            importenergy: row.importenergy.clone(),
            exportenergy: row.exportenergy.clone(),
            rrp: row.rrp.clone(),
            tlf: row.tlf.clone(),
            impenergycost: row.impenergycost.clone(),
            expenergycost: row.expenergycost.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsSmallgendata1PrimaryKey {
    pub connectionpointid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsSmallgendata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSmallgendata1Row<'data> {
    type Row<'other> = SettlementsSmallgendata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid() == row.connectionpointid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsSmallgendata1Row<'data> {
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid() == key.connectionpointid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSmallgendata1PrimaryKey {
    type Row<'other> = SettlementsSmallgendata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid == row.connectionpointid()
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSmallgendata1PrimaryKey {
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSmallgendata1 {
    type Builder = SettlementsSmallgendata1Builder;
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
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importenergy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportenergy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "impenergycost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergycost",
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
        SettlementsSmallgendata1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            importenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            impenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            expenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .settlementdate_array
            .append_value(row.settlementdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .importenergy_array
            .append_option({
                row.importenergy
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .exportenergy_array
            .append_option({
                row.exportenergy
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .impenergycost_array
            .append_option({
                row.impenergycost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .expenergycost_array
            .append_option({
                row.expenergycost
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.impenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsSmallgendata1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    importenergy_array: arrow::array::builder::Decimal128Builder,
    exportenergy_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    impenergycost_array: arrow::array::builder::Decimal128Builder,
    expenergycost_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct TradingUnitSolution2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &TradingUnitSolution2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl TradingUnitSolution2 {
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
pub struct TradingUnitSolution2Mapping([usize; 20]);
/// # Summary
///
/// ## TRADINGLOAD
///
/// TRADINGLOAD shows half-hourly average dispatch levels, including fields to handle the Ancillary Services functionality.
///
/// * Data Set Name: Trading
/// * File Name: Unit Solution
/// * Data Version: 2
///
/// # Description
/// SourceOwn (confidential) TRADINGLOAD data updates half hourly, with public availability of all data on next day.
///
/// # Notes
/// * (Visibility)  Private &Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
/// * TRADETYPE
#[derive(Debug, PartialEq, Eq)]
pub struct TradingUnitSolution2Row<'data> {
    /// Date that this data applies to
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Not used
    pub tradetype: rust_decimal::Decimal,
    /// Period Identifier
    pub periodid: rust_decimal::Decimal,
    /// Average Initial MW at start of each period
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Average total MW dispatched over period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Average ramp down rate
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Average ramp up rate
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Average 5 min lower dispatch
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Average 60 sec lower dispatch
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Average60 sec lower dispatch
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Average 5 min raise dispatch
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Average 60 sec raise dispatch
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Average 6 sec raise dispatch
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    pub availability: Option<rust_decimal::Decimal>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> TradingUnitSolution2Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for TradingUnitSolution2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "TRADING";
    const TABLE_NAME: &'static str = "UNIT_SOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = TradingUnitSolution2Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "DUID",
        "TRADETYPE",
        "PERIODID",
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
        "LASTCHANGED",
        "LOWERREG",
        "RAISEREG",
        "AVAILABILITY",
        "SEMIDISPATCHCAP",
    ];
    type Row<'row> = TradingUnitSolution2Row<'row>;
    type FieldMapping = TradingUnitSolution2Mapping;
    type PrimaryKey = TradingUnitSolution2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(TradingUnitSolution2Row {
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
                .get_custom_parsed_at_idx(
                    "tradetype",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initialmw: row
                .get_opt_custom_parsed_at_idx(
                    "initialmw",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalcleared: row
                .get_opt_custom_parsed_at_idx(
                    "totalcleared",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampdownrate: row
                .get_opt_custom_parsed_at_idx(
                    "rampdownrate",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampuprate: row
                .get_opt_custom_parsed_at_idx(
                    "rampuprate",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5min: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6sec: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[15],
                    mmsdm_core::mms_datetime::parse,
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
            semidispatchcap: row
                .get_opt_custom_parsed_at_idx(
                    "semidispatchcap",
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
        Ok(TradingUnitSolution2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> TradingUnitSolution2PrimaryKey {
        TradingUnitSolution2PrimaryKey {
            duid: row.duid().to_string(),
            periodid: row.periodid,
            runno: row.runno,
            settlementdate: row.settlementdate,
            tradetype: row.tradetype,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("trading_unit_solution_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        TradingUnitSolution2Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            duid: row.duid.clone(),
            tradetype: row.tradetype.clone(),
            periodid: row.periodid.clone(),
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
            lastchanged: row.lastchanged.clone(),
            lowerreg: row.lowerreg.clone(),
            raisereg: row.raisereg.clone(),
            availability: row.availability.clone(),
            semidispatchcap: row.semidispatchcap.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradingUnitSolution2PrimaryKey {
    pub duid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub tradetype: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for TradingUnitSolution2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for TradingUnitSolution2Row<'data> {
    type Row<'other> = TradingUnitSolution2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.periodid == row.periodid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
            && self.tradetype == row.tradetype
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for TradingUnitSolution2Row<'data> {
    type PrimaryKey = TradingUnitSolution2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.periodid == key.periodid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
            && self.tradetype == key.tradetype
    }
}
impl<'data> mmsdm_core::CompareWithRow for TradingUnitSolution2PrimaryKey {
    type Row<'other> = TradingUnitSolution2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.periodid == row.periodid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
            && self.tradetype == row.tradetype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingUnitSolution2PrimaryKey {
    type PrimaryKey = TradingUnitSolution2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.tradetype == key.tradetype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for TradingUnitSolution2 {
    type Builder = TradingUnitSolution2Builder;
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
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
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
                    "semidispatchcap",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        TradingUnitSolution2Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            tradetype_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
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
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lowerreg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            raisereg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            semidispatchcap_array: arrow::array::builder::Decimal128Builder::new()
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
        builder.duid_array.append_value(row.duid());
        builder
            .tradetype_array
            .append_value({
                let mut val = row.tradetype;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
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
            .semidispatchcap_array
            .append_option({
                row.semidispatchcap
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tradetype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
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
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.semidispatchcap_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct TradingUnitSolution2Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    tradetype_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
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
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    lowerreg_array: arrow::array::builder::Decimal128Builder,
    raisereg_array: arrow::array::builder::Decimal128Builder,
    availability_array: arrow::array::builder::Decimal128Builder,
    semidispatchcap_array: arrow::array::builder::Decimal128Builder,
}
pub struct TradingRegionsum4 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &TradingRegionsum4Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl TradingRegionsum4 {
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
pub struct TradingRegionsum4Mapping([usize; 90]);
/// # Summary
///
/// ## TRADINGREGIONSUM
///
/// TRADINGREGIONSUM sets out the half-hourly average regional demand and frequency control services. TRADINGREGIONSUM includes fields for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations.
///
/// * Data Set Name: Trading
/// * File Name: Regionsum
/// * Data Version: 4
///
/// # Description
/// TRADINGREGIONSUM is public data, and is available to all participants.SourceTRADINGREGIONSUM is updated every 30 minutes.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct TradingRegionsum4Row<'data> {
    /// Date that this data applies to
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Trading interval identifier within settlement day.
    pub periodid: rust_decimal::Decimal,
    /// Total demand for region
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// The available generation in the Region for the interval
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Not used
    pub availableload: Option<rust_decimal::Decimal>,
    /// Forecast demand for region
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Averaged generation dispatched in region
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Averaged load dispatched in region
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Average energy transferred over interconnector
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Average excess generation in region
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
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> TradingRegionsum4Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for TradingRegionsum4 {
    const VERSION: i32 = 4;
    const DATA_SET_NAME: &'static str = "TRADING";
    const TABLE_NAME: &'static str = "REGIONSUM";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = TradingRegionsum4Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
        46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
        66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
        86, 87, 88, 89, 90, 91, 92, 93,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "REGIONID",
        "PERIODID",
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
        "TOTALINTERMITTENTGENERATION",
        "DEMAND_AND_NONSCHEDGEN",
        "UIGF",
    ];
    type Row<'row> = TradingRegionsum4Row<'row>;
    type FieldMapping = TradingRegionsum4Mapping;
    type PrimaryKey = TradingRegionsum4PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(TradingRegionsum4Row {
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
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totaldemand: row
                .get_opt_custom_parsed_at_idx(
                    "totaldemand",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availablegeneration: row
                .get_opt_custom_parsed_at_idx(
                    "availablegeneration",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availableload: row
                .get_opt_custom_parsed_at_idx(
                    "availableload",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demandforecast: row
                .get_opt_custom_parsed_at_idx(
                    "demandforecast",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchablegeneration: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchablegeneration",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dispatchableload: row
                .get_opt_custom_parsed_at_idx(
                    "dispatchableload",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netinterchange: row
                .get_opt_custom_parsed_at_idx(
                    "netinterchange",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            excessgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "excessgeneration",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5mindispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower5mindispatch",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minimport",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocaldispatch",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalprice",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalreq",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minprice",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minreq",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minsupplyprice",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secdispatch",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secimport",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocaldispatch",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalprice",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalreq",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secprice",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secreq",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secsupplyprice",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secdispatch",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secimport: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secimport",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocaldispatch",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalprice",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalreq",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secprice",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secreq: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secreq",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secsupplyprice",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5mindispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise5mindispatch",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minimport",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocaldispatch",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalprice",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalreq",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minprice",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minreq",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minsupplyprice",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secdispatch",
                    field_mapping.0[44],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secimport",
                    field_mapping.0[45],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocaldispatch",
                    field_mapping.0[46],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalprice",
                    field_mapping.0[47],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalreq",
                    field_mapping.0[48],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secprice",
                    field_mapping.0[49],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secreq",
                    field_mapping.0[50],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secsupplyprice",
                    field_mapping.0[51],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secdispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secdispatch",
                    field_mapping.0[52],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secimport: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secimport",
                    field_mapping.0[53],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocaldispatch",
                    field_mapping.0[54],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalprice",
                    field_mapping.0[55],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalreq",
                    field_mapping.0[56],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secprice",
                    field_mapping.0[57],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secreq: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secreq",
                    field_mapping.0[58],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secsupplyprice: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secsupplyprice",
                    field_mapping.0[59],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[60],
                    mmsdm_core::mms_datetime::parse,
                )?,
            initialsupply: row
                .get_opt_custom_parsed_at_idx(
                    "initialsupply",
                    field_mapping.0[61],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedsupply: row
                .get_opt_custom_parsed_at_idx(
                    "clearedsupply",
                    field_mapping.0[62],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregimport: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregimport",
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
            raisereglocaldispatch: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocaldispatch",
                    field_mapping.0[68],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalreq: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalreq",
                    field_mapping.0[69],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregreq: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregreq",
                    field_mapping.0[70],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minlocalviolation",
                    field_mapping.0[71],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raisereglocalviolation",
                    field_mapping.0[72],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60seclocalviolation",
                    field_mapping.0[73],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6seclocalviolation",
                    field_mapping.0[74],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minlocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minlocalviolation",
                    field_mapping.0[75],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreglocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreglocalviolation",
                    field_mapping.0[76],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60seclocalviolation",
                    field_mapping.0[77],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6seclocalviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6seclocalviolation",
                    field_mapping.0[78],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minviolation",
                    field_mapping.0[79],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregviolation",
                    field_mapping.0[80],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secviolation",
                    field_mapping.0[81],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secviolation",
                    field_mapping.0[82],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minviolation",
                    field_mapping.0[83],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregviolation",
                    field_mapping.0[84],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secviolation",
                    field_mapping.0[85],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secviolation: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secviolation",
                    field_mapping.0[86],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalintermittentgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "totalintermittentgeneration",
                    field_mapping.0[87],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand_and_nonschedgen: row
                .get_opt_custom_parsed_at_idx(
                    "demand_and_nonschedgen",
                    field_mapping.0[88],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uigf: row
                .get_opt_custom_parsed_at_idx(
                    "uigf",
                    field_mapping.0[89],
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
        Ok(TradingRegionsum4Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> TradingRegionsum4PrimaryKey {
        TradingRegionsum4PrimaryKey {
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("trading_regionsum_v4_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        TradingRegionsum4Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
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
            totalintermittentgeneration: row.totalintermittentgeneration.clone(),
            demand_and_nonschedgen: row.demand_and_nonschedgen.clone(),
            uigf: row.uigf.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradingRegionsum4PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for TradingRegionsum4PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for TradingRegionsum4Row<'data> {
    type Row<'other> = TradingRegionsum4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for TradingRegionsum4Row<'data> {
    type PrimaryKey = TradingRegionsum4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid() == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for TradingRegionsum4PrimaryKey {
    type Row<'other> = TradingRegionsum4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingRegionsum4PrimaryKey {
    type PrimaryKey = TradingRegionsum4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for TradingRegionsum4 {
    type Builder = TradingRegionsum4Builder;
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
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        TradingRegionsum4Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
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
            totalintermittentgeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            demand_and_nonschedgen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            uigf_array: arrow::array::builder::Decimal128Builder::new()
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
            .periodid_array
            .append_value({
                let mut val = row.periodid;
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
                    alloc::sync::Arc::new(builder.periodid_array.finish())
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
                        builder.totalintermittentgeneration_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_and_nonschedgen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct TradingRegionsum4Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
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
    totalintermittentgeneration_array: arrow::array::builder::Decimal128Builder,
    demand_and_nonschedgen_array: arrow::array::builder::Decimal128Builder,
    uigf_array: arrow::array::builder::Decimal128Builder,
}
