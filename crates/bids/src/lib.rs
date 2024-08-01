#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct OfferBiddayoffer3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&OfferBiddayoffer3Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl OfferBiddayoffer3 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct OfferBiddayoffer3Mapping([usize; 33]);
/// # Summary
///
/// ## BIDDAYOFFER
///  _BIDDAYOFFER shows the Energy and Ancillary Service bid data for each Market Day. BIDDAYOFFER is the parent table to BIDOFFERPERIOD. BIDDAYOFFER is a child table to BIDOFFERFILETRK_
///
/// * Data Set Name: Offer
/// * File Name: Biddayoffer
/// * Data Version: 3
///
/// # Description
///  The ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDOFFERPERIOD) facilitate ancillary service bidding and include energy bidding.  BIDDAYOFFER data is confidential to the submitting participant until made public after 4am the next day. Source BIDDAYOFFER updates as ancillary service bids are processed. BIDDAYOFFER includes all accepted energy and ancillary service bids. Volume Approximately 1,500,000 records per year
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DIRECTION
/// * DUID
/// * OFFERDATE
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct OfferBiddayoffer3Row<'data> {
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// Market date for applying the bid
    pub settlementdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    pub offerdate: chrono::NaiveDateTime,
    /// The power flow direction to which this offer applies: GEN, LOAD or BIDIRECTIONAL
    pub direction: core::ops::Range<usize>,
    /// Version No. for given offer date
    pub versionno: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Maximum energy available from Energy Constrained Plant. (Energy Bids Only)
    pub dailyenergyconstraint: Option<rust_decimal::Decimal>,
    /// Explanation for all rebids and inflexibilities
    pub rebidexplanation: core::ops::Range<usize>,
    /// Price for Availability Band 1
    pub priceband1: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 2
    pub priceband2: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 3
    pub priceband3: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 4
    pub priceband4: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 5
    pub priceband5: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    pub priceband6: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    pub priceband7: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 8
    pub priceband8: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 9
    pub priceband9: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 10
    pub priceband10: Option<rust_decimal::Decimal>,
    /// Minimum MW load fast start plant
    pub minimumload: Option<rust_decimal::Decimal>,
    /// Time to synchronise in minutes (Energy Bids Only)
    pub t1: Option<rust_decimal::Decimal>,
    /// Time to minimum load in minutes (Energy Bids Only)
    pub t2: Option<rust_decimal::Decimal>,
    /// Time at minimum load in minutes (Energy Bids Only)
    pub t3: Option<rust_decimal::Decimal>,
    /// Time to shutdown in minutes (Energy Bids Only)
    pub t4: Option<rust_decimal::Decimal>,
    /// not used; was ON/OFF for loads (Energy Bids Only)
    pub normalstatus: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Offer Factor
    pub mr_factor: Option<rust_decimal::Decimal>,
    /// Daily if processed before BidCutOff of previous day, otherwise REBID
    pub entrytype: core::ops::Range<usize>,
    /// The time of the event(s) or other occurrence(s) cited/adduced as the reason for the rebid. Required for rebids, not required for fixed load or low ramp rates. Expected in the format: HH:MM:SS e.g. 20:11:00
    pub rebid_event_time: core::ops::Range<usize>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant became aware of the event(s) / occurrence(s) that prompted the rebid.Not validated by AEMO
    pub rebid_aware_time: core::ops::Range<usize>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant made the decision to rebid. Not validated by AEMO
    pub rebid_decision_time: core::ops::Range<usize>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. A provided rebid category. Not validated by AEMO
    pub rebid_category: core::ops::Range<usize>,
    /// A participants unique Reference Id
    pub reference_id: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> OfferBiddayoffer3Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
    pub fn direction(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.direction.clone())
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
    pub fn rebidexplanation(&self) -> Option<&str> {
        if self.rebidexplanation.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebidexplanation.clone(),
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
    pub fn entrytype(&self) -> Option<&str> {
        if self.entrytype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.entrytype.clone(),
                ),
            )
        }
    }
    pub fn rebid_event_time(&self) -> Option<&str> {
        if self.rebid_event_time.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_event_time.clone(),
                ),
            )
        }
    }
    pub fn rebid_aware_time(&self) -> Option<&str> {
        if self.rebid_aware_time.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_aware_time.clone(),
                ),
            )
        }
    }
    pub fn rebid_decision_time(&self) -> Option<&str> {
        if self.rebid_decision_time.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_decision_time.clone(),
                ),
            )
        }
    }
    pub fn rebid_category(&self) -> Option<&str> {
        if self.rebid_category.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_category.clone(),
                ),
            )
        }
    }
    pub fn reference_id(&self) -> Option<&str> {
        if self.reference_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reference_id.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for OfferBiddayoffer3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "OFFER";
    const TABLE_NAME: &'static str = "BIDDAYOFFER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = OfferBiddayoffer3Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "BIDTYPE",
        "SETTLEMENTDATE",
        "OFFERDATE",
        "DIRECTION",
        "VERSIONNO",
        "PARTICIPANTID",
        "DAILYENERGYCONSTRAINT",
        "REBIDEXPLANATION",
        "PRICEBAND1",
        "PRICEBAND2",
        "PRICEBAND3",
        "PRICEBAND4",
        "PRICEBAND5",
        "PRICEBAND6",
        "PRICEBAND7",
        "PRICEBAND8",
        "PRICEBAND9",
        "PRICEBAND10",
        "MINIMUMLOAD",
        "T1",
        "T2",
        "T3",
        "T4",
        "NORMALSTATUS",
        "LASTCHANGED",
        "MR_FACTOR",
        "ENTRYTYPE",
        "REBID_EVENT_TIME",
        "REBID_AWARE_TIME",
        "REBID_DECISION_TIME",
        "REBID_CATEGORY",
        "REFERENCE_ID",
    ];
    type Row<'row> = OfferBiddayoffer3Row<'row>;
    type FieldMapping = OfferBiddayoffer3Mapping;
    type PrimaryKey = OfferBiddayoffer3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(OfferBiddayoffer3Row {
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
            direction: row.get_range("direction", field_mapping.0[4])?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_opt_range("participantid", field_mapping.0[6])?,
            dailyenergyconstraint: row
                .get_opt_custom_parsed_at_idx(
                    "dailyenergyconstraint",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rebidexplanation: row.get_opt_range("rebidexplanation", field_mapping.0[8])?,
            priceband1: row
                .get_opt_custom_parsed_at_idx(
                    "priceband1",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband2: row
                .get_opt_custom_parsed_at_idx(
                    "priceband2",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband3: row
                .get_opt_custom_parsed_at_idx(
                    "priceband3",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband4: row
                .get_opt_custom_parsed_at_idx(
                    "priceband4",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband5: row
                .get_opt_custom_parsed_at_idx(
                    "priceband5",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband6: row
                .get_opt_custom_parsed_at_idx(
                    "priceband6",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband7: row
                .get_opt_custom_parsed_at_idx(
                    "priceband7",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband8: row
                .get_opt_custom_parsed_at_idx(
                    "priceband8",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband9: row
                .get_opt_custom_parsed_at_idx(
                    "priceband9",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband10: row
                .get_opt_custom_parsed_at_idx(
                    "priceband10",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            minimumload: row
                .get_opt_custom_parsed_at_idx(
                    "minimumload",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t1: row
                .get_opt_custom_parsed_at_idx(
                    "t1",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t2: row
                .get_opt_custom_parsed_at_idx(
                    "t2",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t3: row
                .get_opt_custom_parsed_at_idx(
                    "t3",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t4: row
                .get_opt_custom_parsed_at_idx(
                    "t4",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            normalstatus: row.get_opt_range("normalstatus", field_mapping.0[24])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[25],
                    mmsdm_core::mms_datetime::parse,
                )?,
            mr_factor: row
                .get_opt_custom_parsed_at_idx(
                    "mr_factor",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            entrytype: row.get_opt_range("entrytype", field_mapping.0[27])?,
            rebid_event_time: row
                .get_opt_range("rebid_event_time", field_mapping.0[28])?,
            rebid_aware_time: row
                .get_opt_range("rebid_aware_time", field_mapping.0[29])?,
            rebid_decision_time: row
                .get_opt_range("rebid_decision_time", field_mapping.0[30])?,
            rebid_category: row.get_opt_range("rebid_category", field_mapping.0[31])?,
            reference_id: row.get_opt_range("reference_id", field_mapping.0[32])?,
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
        Ok(OfferBiddayoffer3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> OfferBiddayoffer3PrimaryKey {
        OfferBiddayoffer3PrimaryKey {
            bidtype: row.bidtype().to_string(),
            direction: row.direction().to_string(),
            duid: row.duid().to_string(),
            offerdate: row.offerdate,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("offer_biddayoffer_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        OfferBiddayoffer3Row {
            duid: row.duid.clone(),
            bidtype: row.bidtype.clone(),
            settlementdate: row.settlementdate.clone(),
            offerdate: row.offerdate.clone(),
            direction: row.direction.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            dailyenergyconstraint: row.dailyenergyconstraint.clone(),
            rebidexplanation: row.rebidexplanation.clone(),
            priceband1: row.priceband1.clone(),
            priceband2: row.priceband2.clone(),
            priceband3: row.priceband3.clone(),
            priceband4: row.priceband4.clone(),
            priceband5: row.priceband5.clone(),
            priceband6: row.priceband6.clone(),
            priceband7: row.priceband7.clone(),
            priceband8: row.priceband8.clone(),
            priceband9: row.priceband9.clone(),
            priceband10: row.priceband10.clone(),
            minimumload: row.minimumload.clone(),
            t1: row.t1.clone(),
            t2: row.t2.clone(),
            t3: row.t3.clone(),
            t4: row.t4.clone(),
            normalstatus: row.normalstatus.clone(),
            lastchanged: row.lastchanged.clone(),
            mr_factor: row.mr_factor.clone(),
            entrytype: row.entrytype.clone(),
            rebid_event_time: row.rebid_event_time.clone(),
            rebid_aware_time: row.rebid_aware_time.clone(),
            rebid_decision_time: row.rebid_decision_time.clone(),
            rebid_category: row.rebid_category.clone(),
            reference_id: row.reference_id.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OfferBiddayoffer3PrimaryKey {
    pub bidtype: alloc::string::String,
    pub direction: alloc::string::String,
    pub duid: alloc::string::String,
    pub offerdate: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for OfferBiddayoffer3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for OfferBiddayoffer3Row<'data> {
    type Row<'other> = OfferBiddayoffer3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.direction() == row.direction()
            && self.duid() == row.duid() && self.offerdate == row.offerdate
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for OfferBiddayoffer3Row<'data> {
    type PrimaryKey = OfferBiddayoffer3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.direction() == key.direction
            && self.duid() == key.duid && self.offerdate == key.offerdate
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for OfferBiddayoffer3PrimaryKey {
    type Row<'other> = OfferBiddayoffer3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.direction == row.direction()
            && self.duid == row.duid() && self.offerdate == row.offerdate
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for OfferBiddayoffer3PrimaryKey {
    type PrimaryKey = OfferBiddayoffer3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.direction == key.direction
            && self.duid == key.duid && self.offerdate == key.offerdate
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for OfferBiddayoffer3 {
    type Builder = OfferBiddayoffer3Builder;
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
                    "direction",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dailyenergyconstraint",
                    arrow::datatypes::DataType::Decimal128(12, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebidexplanation",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband1",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband2",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband3",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband4",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband5",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband6",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband7",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband8",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband9",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband10",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "minimumload",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t1",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t2",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t3",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t4",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "normalstatus",
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
                    "mr_factor",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "entrytype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_event_time",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_aware_time",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_decision_time",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_category",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reference_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        OfferBiddayoffer3Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            direction_array: arrow::array::builder::StringBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            dailyenergyconstraint_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 6)),
            rebidexplanation_array: arrow::array::builder::StringBuilder::new(),
            priceband1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            minimumload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            normalstatus_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            mr_factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            entrytype_array: arrow::array::builder::StringBuilder::new(),
            rebid_event_time_array: arrow::array::builder::StringBuilder::new(),
            rebid_aware_time_array: arrow::array::builder::StringBuilder::new(),
            rebid_decision_time_array: arrow::array::builder::StringBuilder::new(),
            rebid_category_array: arrow::array::builder::StringBuilder::new(),
            reference_id_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder.bidtype_array.append_value(row.bidtype());
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.offerdate_array.append_value(row.offerdate.timestamp_millis());
        builder.direction_array.append_value(row.direction());
        builder
            .versionno_array
            .append_option({
                row.versionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.participantid_array.append_option(row.participantid());
        builder
            .dailyenergyconstraint_array
            .append_option({
                row.dailyenergyconstraint
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.rebidexplanation_array.append_option(row.rebidexplanation());
        builder
            .priceband1_array
            .append_option({
                row.priceband1
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband2_array
            .append_option({
                row.priceband2
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband3_array
            .append_option({
                row.priceband3
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband4_array
            .append_option({
                row.priceband4
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband5_array
            .append_option({
                row.priceband5
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband6_array
            .append_option({
                row.priceband6
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband7_array
            .append_option({
                row.priceband7
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband8_array
            .append_option({
                row.priceband8
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband9_array
            .append_option({
                row.priceband9
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband10_array
            .append_option({
                row.priceband10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .minimumload_array
            .append_option({
                row.minimumload
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t1_array
            .append_option({
                row.t1
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t2_array
            .append_option({
                row.t2
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t3_array
            .append_option({
                row.t3
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t4_array
            .append_option({
                row.t4
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.normalstatus_array.append_option(row.normalstatus());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .mr_factor_array
            .append_option({
                row.mr_factor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.entrytype_array.append_option(row.entrytype());
        builder.rebid_event_time_array.append_option(row.rebid_event_time());
        builder.rebid_aware_time_array.append_option(row.rebid_aware_time());
        builder.rebid_decision_time_array.append_option(row.rebid_decision_time());
        builder.rebid_category_array.append_option(row.rebid_category());
        builder.reference_id_array.append_option(row.reference_id());
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
                    alloc::sync::Arc::new(builder.direction_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dailyenergyconstraint_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebidexplanation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.minimumload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.normalstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mr_factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.entrytype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_event_time_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_aware_time_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_decision_time_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_category_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reference_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct OfferBiddayoffer3Builder {
    duid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    direction_array: arrow::array::builder::StringBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    dailyenergyconstraint_array: arrow::array::builder::Decimal128Builder,
    rebidexplanation_array: arrow::array::builder::StringBuilder,
    priceband1_array: arrow::array::builder::Decimal128Builder,
    priceband2_array: arrow::array::builder::Decimal128Builder,
    priceband3_array: arrow::array::builder::Decimal128Builder,
    priceband4_array: arrow::array::builder::Decimal128Builder,
    priceband5_array: arrow::array::builder::Decimal128Builder,
    priceband6_array: arrow::array::builder::Decimal128Builder,
    priceband7_array: arrow::array::builder::Decimal128Builder,
    priceband8_array: arrow::array::builder::Decimal128Builder,
    priceband9_array: arrow::array::builder::Decimal128Builder,
    priceband10_array: arrow::array::builder::Decimal128Builder,
    minimumload_array: arrow::array::builder::Decimal128Builder,
    t1_array: arrow::array::builder::Decimal128Builder,
    t2_array: arrow::array::builder::Decimal128Builder,
    t3_array: arrow::array::builder::Decimal128Builder,
    t4_array: arrow::array::builder::Decimal128Builder,
    normalstatus_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    mr_factor_array: arrow::array::builder::Decimal128Builder,
    entrytype_array: arrow::array::builder::StringBuilder,
    rebid_event_time_array: arrow::array::builder::StringBuilder,
    rebid_aware_time_array: arrow::array::builder::StringBuilder,
    rebid_decision_time_array: arrow::array::builder::StringBuilder,
    rebid_category_array: arrow::array::builder::StringBuilder,
    reference_id_array: arrow::array::builder::StringBuilder,
}
pub struct BidBiddayofferD3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&BidBiddayofferD3Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidBiddayofferD3 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BidBiddayofferD3Mapping([usize; 29]);
/// # Summary
///
/// ## BIDDAYOFFER_D
///  _BIDDAYOFFER_D shows the public summary of the energy and FCAS offers applicable in the Dispatch for the<br>intervals identified. BIDDAYOFFER_D is the parent table to BIDPEROFFER_D._
///
/// * Data Set Name: Bid
/// * File Name: Biddayoffer D
/// * Data Version: 3
///
/// # Description
///  BIDDAYOFFER_D data is made public after 4am the next day. Source BIDDAYOFFER_D updates as ancillary service bids are processed. BIDDAYOFFER_D shows latest accepted energy and ancillary service bids. Volume Summary - approximately 1,000 rows per day
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DIRECTION
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct BidBiddayofferD3Row<'data> {
    /// Market date for which the bid applied
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// The power flow direction to which this offer applies: GEN, LOAD or BIDIRECTIONAL
    pub direction: core::ops::Range<usize>,
    /// Market date for which the bid was submitted.
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Offer date and time
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Version No. for given offer date
    pub versionno: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Maximum energy available from Energy Constrained Plant. (Energy Bids Only)
    pub dailyenergyconstraint: Option<rust_decimal::Decimal>,
    /// Explanation for all rebids and inflexibilities
    pub rebidexplanation: core::ops::Range<usize>,
    /// Price for Availability Band 1
    pub priceband1: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 2
    pub priceband2: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 3
    pub priceband3: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 4
    pub priceband4: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 5
    pub priceband5: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    pub priceband6: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 7
    pub priceband7: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 8
    pub priceband8: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 9
    pub priceband9: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 10
    pub priceband10: Option<rust_decimal::Decimal>,
    /// Minimum MW load fast start plant
    pub minimumload: Option<rust_decimal::Decimal>,
    /// Time to synchronise in minutes (Energy Bids Only)
    pub t1: Option<rust_decimal::Decimal>,
    /// Time to minimum load in minutes (Energy Bids Only)
    pub t2: Option<rust_decimal::Decimal>,
    /// Time at minimum load in minutes (Energy Bids Only)
    pub t3: Option<rust_decimal::Decimal>,
    /// Time to shutdown in minutes (Energy Bids Only)
    pub t4: Option<rust_decimal::Decimal>,
    /// ON/OFF for loads (Energy Bids Only)
    pub normalstatus: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Scaling Factor
    pub mr_factor: Option<rust_decimal::Decimal>,
    /// Daily if processed before BidCutOff of previous day, otherwise REBID
    pub entrytype: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidBiddayofferD3Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
    pub fn direction(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.direction.clone())
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
    pub fn rebidexplanation(&self) -> Option<&str> {
        if self.rebidexplanation.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebidexplanation.clone(),
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
    pub fn entrytype(&self) -> Option<&str> {
        if self.entrytype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.entrytype.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for BidBiddayofferD3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "BID";
    const TABLE_NAME: &'static str = "BIDDAYOFFER_D";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidBiddayofferD3Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "DUID",
        "BIDTYPE",
        "DIRECTION",
        "BIDSETTLEMENTDATE",
        "OFFERDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "DAILYENERGYCONSTRAINT",
        "REBIDEXPLANATION",
        "PRICEBAND1",
        "PRICEBAND2",
        "PRICEBAND3",
        "PRICEBAND4",
        "PRICEBAND5",
        "PRICEBAND6",
        "PRICEBAND7",
        "PRICEBAND8",
        "PRICEBAND9",
        "PRICEBAND10",
        "MINIMUMLOAD",
        "T1",
        "T2",
        "T3",
        "T4",
        "NORMALSTATUS",
        "LASTCHANGED",
        "MR_FACTOR",
        "ENTRYTYPE",
    ];
    type Row<'row> = BidBiddayofferD3Row<'row>;
    type FieldMapping = BidBiddayofferD3Mapping;
    type PrimaryKey = BidBiddayofferD3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidBiddayofferD3Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            bidtype: row.get_range("bidtype", field_mapping.0[2])?,
            direction: row.get_range("direction", field_mapping.0[3])?,
            bidsettlementdate: row
                .get_opt_custom_parsed_at_idx(
                    "bidsettlementdate",
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
            participantid: row.get_opt_range("participantid", field_mapping.0[7])?,
            dailyenergyconstraint: row
                .get_opt_custom_parsed_at_idx(
                    "dailyenergyconstraint",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rebidexplanation: row.get_opt_range("rebidexplanation", field_mapping.0[9])?,
            priceband1: row
                .get_opt_custom_parsed_at_idx(
                    "priceband1",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband2: row
                .get_opt_custom_parsed_at_idx(
                    "priceband2",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband3: row
                .get_opt_custom_parsed_at_idx(
                    "priceband3",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband4: row
                .get_opt_custom_parsed_at_idx(
                    "priceband4",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband5: row
                .get_opt_custom_parsed_at_idx(
                    "priceband5",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband6: row
                .get_opt_custom_parsed_at_idx(
                    "priceband6",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband7: row
                .get_opt_custom_parsed_at_idx(
                    "priceband7",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband8: row
                .get_opt_custom_parsed_at_idx(
                    "priceband8",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband9: row
                .get_opt_custom_parsed_at_idx(
                    "priceband9",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband10: row
                .get_opt_custom_parsed_at_idx(
                    "priceband10",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            minimumload: row
                .get_opt_custom_parsed_at_idx(
                    "minimumload",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t1: row
                .get_opt_custom_parsed_at_idx(
                    "t1",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t2: row
                .get_opt_custom_parsed_at_idx(
                    "t2",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t3: row
                .get_opt_custom_parsed_at_idx(
                    "t3",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            t4: row
                .get_opt_custom_parsed_at_idx(
                    "t4",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            normalstatus: row.get_opt_range("normalstatus", field_mapping.0[25])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[26],
                    mmsdm_core::mms_datetime::parse,
                )?,
            mr_factor: row
                .get_opt_custom_parsed_at_idx(
                    "mr_factor",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            entrytype: row.get_opt_range("entrytype", field_mapping.0[28])?,
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
        Ok(BidBiddayofferD3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidBiddayofferD3PrimaryKey {
        BidBiddayofferD3PrimaryKey {
            bidtype: row.bidtype().to_string(),
            direction: row.direction().to_string(),
            duid: row.duid().to_string(),
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bid_biddayoffer_d_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidBiddayofferD3Row {
            settlementdate: row.settlementdate.clone(),
            duid: row.duid.clone(),
            bidtype: row.bidtype.clone(),
            direction: row.direction.clone(),
            bidsettlementdate: row.bidsettlementdate.clone(),
            offerdate: row.offerdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            dailyenergyconstraint: row.dailyenergyconstraint.clone(),
            rebidexplanation: row.rebidexplanation.clone(),
            priceband1: row.priceband1.clone(),
            priceband2: row.priceband2.clone(),
            priceband3: row.priceband3.clone(),
            priceband4: row.priceband4.clone(),
            priceband5: row.priceband5.clone(),
            priceband6: row.priceband6.clone(),
            priceband7: row.priceband7.clone(),
            priceband8: row.priceband8.clone(),
            priceband9: row.priceband9.clone(),
            priceband10: row.priceband10.clone(),
            minimumload: row.minimumload.clone(),
            t1: row.t1.clone(),
            t2: row.t2.clone(),
            t3: row.t3.clone(),
            t4: row.t4.clone(),
            normalstatus: row.normalstatus.clone(),
            lastchanged: row.lastchanged.clone(),
            mr_factor: row.mr_factor.clone(),
            entrytype: row.entrytype.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidBiddayofferD3PrimaryKey {
    pub bidtype: alloc::string::String,
    pub direction: alloc::string::String,
    pub duid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BidBiddayofferD3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidBiddayofferD3Row<'data> {
    type Row<'other> = BidBiddayofferD3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.direction() == row.direction()
            && self.duid() == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidBiddayofferD3Row<'data> {
    type PrimaryKey = BidBiddayofferD3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.direction() == key.direction
            && self.duid() == key.duid && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidBiddayofferD3PrimaryKey {
    type Row<'other> = BidBiddayofferD3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.direction == row.direction()
            && self.duid == row.duid() && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidBiddayofferD3PrimaryKey {
    type PrimaryKey = BidBiddayofferD3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.direction == key.direction
            && self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidBiddayofferD3 {
    type Builder = BidBiddayofferD3Builder;
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
                    "direction",
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
                    "offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dailyenergyconstraint",
                    arrow::datatypes::DataType::Decimal128(12, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebidexplanation",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband1",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband2",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband3",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband4",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband5",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband6",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband7",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband8",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband9",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband10",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "minimumload",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t1",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t2",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t3",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "t4",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "normalstatus",
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
                    "mr_factor",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "entrytype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BidBiddayofferD3Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            direction_array: arrow::array::builder::StringBuilder::new(),
            bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            dailyenergyconstraint_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 6)),
            rebidexplanation_array: arrow::array::builder::StringBuilder::new(),
            priceband1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            minimumload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            t4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            normalstatus_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            mr_factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            entrytype_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.bidtype_array.append_value(row.bidtype());
        builder.direction_array.append_value(row.direction());
        builder
            .bidsettlementdate_array
            .append_option(row.bidsettlementdate.map(|val| val.timestamp_millis()));
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
        builder.participantid_array.append_option(row.participantid());
        builder
            .dailyenergyconstraint_array
            .append_option({
                row.dailyenergyconstraint
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.rebidexplanation_array.append_option(row.rebidexplanation());
        builder
            .priceband1_array
            .append_option({
                row.priceband1
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband2_array
            .append_option({
                row.priceband2
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband3_array
            .append_option({
                row.priceband3
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband4_array
            .append_option({
                row.priceband4
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband5_array
            .append_option({
                row.priceband5
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband6_array
            .append_option({
                row.priceband6
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband7_array
            .append_option({
                row.priceband7
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband8_array
            .append_option({
                row.priceband8
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband9_array
            .append_option({
                row.priceband9
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband10_array
            .append_option({
                row.priceband10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .minimumload_array
            .append_option({
                row.minimumload
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t1_array
            .append_option({
                row.t1
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t2_array
            .append_option({
                row.t2
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t3_array
            .append_option({
                row.t3
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .t4_array
            .append_option({
                row.t4
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.normalstatus_array.append_option(row.normalstatus());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .mr_factor_array
            .append_option({
                row.mr_factor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.entrytype_array.append_option(row.entrytype());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.direction_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidsettlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dailyenergyconstraint_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebidexplanation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.minimumload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.t4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.normalstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mr_factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.entrytype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidBiddayofferD3Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    direction_array: arrow::array::builder::StringBuilder,
    bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    dailyenergyconstraint_array: arrow::array::builder::Decimal128Builder,
    rebidexplanation_array: arrow::array::builder::StringBuilder,
    priceband1_array: arrow::array::builder::Decimal128Builder,
    priceband2_array: arrow::array::builder::Decimal128Builder,
    priceband3_array: arrow::array::builder::Decimal128Builder,
    priceband4_array: arrow::array::builder::Decimal128Builder,
    priceband5_array: arrow::array::builder::Decimal128Builder,
    priceband6_array: arrow::array::builder::Decimal128Builder,
    priceband7_array: arrow::array::builder::Decimal128Builder,
    priceband8_array: arrow::array::builder::Decimal128Builder,
    priceband9_array: arrow::array::builder::Decimal128Builder,
    priceband10_array: arrow::array::builder::Decimal128Builder,
    minimumload_array: arrow::array::builder::Decimal128Builder,
    t1_array: arrow::array::builder::Decimal128Builder,
    t2_array: arrow::array::builder::Decimal128Builder,
    t3_array: arrow::array::builder::Decimal128Builder,
    t4_array: arrow::array::builder::Decimal128Builder,
    normalstatus_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    mr_factor_array: arrow::array::builder::Decimal128Builder,
    entrytype_array: arrow::array::builder::StringBuilder,
}
pub struct BidsBidofferfiletrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&BidsBidofferfiletrk1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidsBidofferfiletrk1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BidsBidofferfiletrk1Mapping([usize; 12]);
/// # Summary
///
/// ## BIDOFFERFILETRK
///  _BIDOFFERFILETRK shows an audit trail of all files submitted containing ENERGY/FCAS/MNSP bid, including corrupt bids and rebids._
///
/// * Data Set Name: Bids
/// * File Name: Bidofferfiletrk
/// * Data Version: 1
///
/// # Description
///  BIDOFFERFILETRK data is confidential to the submitting participant. The new ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three new tables facilitate ancillary service bidding. The new tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDOFFERPERIOD) are similar in structure to energy bidding tables (OFFERFILETRK, DAYOFFER and PEROFFER). The significant differences with the new tables are. ·  The OFFERDATE field reflects the time the bid was loaded and this field alone provides the key for versioning of bids. The VERSIONNO field is retained for participant use as information only. ·  The new tables support bids for multiple services. The BIDTYPE field defines the service to which the bid applies. ·  There are no default bids. In the absence of a bid for a specific settlement date, the latest bid submitted for a previous settlement date applies. Source This data is updated as bids are processed. It includes all bids submitted including corrupt bids. Volume Approximately 100,000 records per year Note Confirmation is via CSV bid acknowledgement file
///
///
///
/// # Primary Key Columns
///
/// * OFFERDATE
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct BidsBidofferfiletrk1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Time this bid was processed and loaded
    pub offerdate: chrono::NaiveDateTime,
    /// Submitted file name
    pub filename: core::ops::Range<usize>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant agent who created the Offer
    pub authorisedby: core::ops::Range<usize>,
    /// When the Offer was processed - synonymous with LastChanged
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// A GUID used to identify the submission transaction in AEMOs systems
    pub transaction_id: core::ops::Range<usize>,
    /// A participant provided reference, which is required to be unique per submission (for a PARTICIPANTID)
    pub reference_id: core::ops::Range<usize>,
    /// The participant provided date/time for the submission
    pub submission_timestamp: Option<chrono::NaiveDateTime>,
    /// A participant provided comment
    pub comments: core::ops::Range<usize>,
    /// Method by which this submission was made typically FTP, API, WEB
    pub submission_method: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidsBidofferfiletrk1Row<'data> {
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
    pub fn transaction_id(&self) -> Option<&str> {
        if self.transaction_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.transaction_id.clone(),
                ),
            )
        }
    }
    pub fn reference_id(&self) -> Option<&str> {
        if self.reference_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reference_id.clone(),
                ),
            )
        }
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
    pub fn submission_method(&self) -> Option<&str> {
        if self.submission_method.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.submission_method.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for BidsBidofferfiletrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BIDS";
    const TABLE_NAME: &'static str = "BIDOFFERFILETRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidsBidofferfiletrk1Mapping([
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
        "PARTICIPANTID",
        "OFFERDATE",
        "FILENAME",
        "STATUS",
        "LASTCHANGED",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "TRANSACTION_ID",
        "REFERENCE_ID",
        "SUBMISSION_TIMESTAMP",
        "COMMENTS",
        "SUBMISSION_METHOD",
    ];
    type Row<'row> = BidsBidofferfiletrk1Row<'row>;
    type FieldMapping = BidsBidofferfiletrk1Mapping;
    type PrimaryKey = BidsBidofferfiletrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidsBidofferfiletrk1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            filename: row.get_range("filename", field_mapping.0[2])?,
            status: row.get_opt_range("status", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[5])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            transaction_id: row.get_opt_range("transaction_id", field_mapping.0[7])?,
            reference_id: row.get_opt_range("reference_id", field_mapping.0[8])?,
            submission_timestamp: row
                .get_opt_custom_parsed_at_idx(
                    "submission_timestamp",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            comments: row.get_opt_range("comments", field_mapping.0[10])?,
            submission_method: row
                .get_opt_range("submission_method", field_mapping.0[11])?,
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
        Ok(BidsBidofferfiletrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidsBidofferfiletrk1PrimaryKey {
        BidsBidofferfiletrk1PrimaryKey {
            offerdate: row.offerdate,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bids_bidofferfiletrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidsBidofferfiletrk1Row {
            participantid: row.participantid.clone(),
            offerdate: row.offerdate.clone(),
            filename: row.filename.clone(),
            status: row.status.clone(),
            lastchanged: row.lastchanged.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            transaction_id: row.transaction_id.clone(),
            reference_id: row.reference_id.clone(),
            submission_timestamp: row.submission_timestamp.clone(),
            comments: row.comments.clone(),
            submission_method: row.submission_method.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidsBidofferfiletrk1PrimaryKey {
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for BidsBidofferfiletrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidsBidofferfiletrk1Row<'data> {
    type Row<'other> = BidsBidofferfiletrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.offerdate == row.offerdate && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidsBidofferfiletrk1Row<'data> {
    type PrimaryKey = BidsBidofferfiletrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidsBidofferfiletrk1PrimaryKey {
    type Row<'other> = BidsBidofferfiletrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.offerdate == row.offerdate && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidsBidofferfiletrk1PrimaryKey {
    type PrimaryKey = BidsBidofferfiletrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidsBidofferfiletrk1 {
    type Builder = BidsBidofferfiletrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
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
                    "lastchanged",
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
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "transaction_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reference_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "submission_timestamp",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "comments",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "submission_method",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BidsBidofferfiletrk1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            status_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            transaction_id_array: arrow::array::builder::StringBuilder::new(),
            reference_id_array: arrow::array::builder::StringBuilder::new(),
            submission_timestamp_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            comments_array: arrow::array::builder::StringBuilder::new(),
            submission_method_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
        builder.offerdate_array.append_value(row.offerdate.timestamp_millis());
        builder.filename_array.append_value(row.filename());
        builder.status_array.append_option(row.status());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(row.authoriseddate.map(|val| val.timestamp_millis()));
        builder.transaction_id_array.append_option(row.transaction_id());
        builder.reference_id_array.append_option(row.reference_id());
        builder
            .submission_timestamp_array
            .append_option(row.submission_timestamp.map(|val| val.timestamp_millis()));
        builder.comments_array.append_option(row.comments());
        builder.submission_method_array.append_option(row.submission_method());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transaction_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reference_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.submission_timestamp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.comments_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.submission_method_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidsBidofferfiletrk1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    status_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    transaction_id_array: arrow::array::builder::StringBuilder,
    reference_id_array: arrow::array::builder::StringBuilder,
    submission_timestamp_array: arrow::array::builder::TimestampMillisecondBuilder,
    comments_array: arrow::array::builder::StringBuilder,
    submission_method_array: arrow::array::builder::StringBuilder,
}
pub struct BidsBidofferperiod2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&BidsBidofferperiod2Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidsBidofferperiod2 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BidsBidofferperiod2Mapping([usize; 27]);
/// # Summary
///
/// ## BIDOFFERPERIOD
///  _BIDOFFERPERIOD shows 5-minute period-based Energy and Ancillary Service bid data.BIDOFFERPERIOD is a child table of BIDDAYOFFER_
///
/// * Data Set Name: Bids
/// * File Name: Bidofferperiod
/// * Data Version: 2
///
///
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DIRECTION
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct BidsBidofferperiod2Row<'data> {
    /// Dispatchable Unit ID
    pub duid: core::ops::Range<usize>,
    /// The type of bid, one-of ENERGY, RAISE6SEC, RAISE60SEC, RAISE5MIN, RAISEREG, LOWER6SEC, LOWER60SEC, LOWER5MIN, LOWERREG
    pub bidtype: core::ops::Range<usize>,
    /// The trading date this bid is for
    pub tradingdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// The power flow direction to which this offer applies: GEN, LOAD or BIDIRECTIONAL
    pub direction: core::ops::Range<usize>,
    /// Period ID 1 to 288
    pub periodid: rust_decimal::Decimal,
    /// Maximum availability for this BidType in this period
    pub maxavail: Option<rust_decimal::Decimal>,
    /// Fixed unit output MW (Energy bids only) A null value means no fixed load so the unit is dispatched according to bid and market
    pub fixedload: Option<rust_decimal::Decimal>,
    /// MW/Min for raise (Energy bids only)
    pub rampuprate: Option<i64>,
    /// MW/Min for lower (Energy bids only)
    pub rampdownrate: Option<i64>,
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
    /// Allows for future use for Energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// The Energy limit applying at the end of this dispatch interval in MWh. For GEN this is a lower energy limit. For LOAD this is an upper energy limit
    pub energylimit: Option<rust_decimal::Decimal>,
    /// Period ID Ending
    pub periodidto: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidsBidofferperiod2Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
    pub fn direction(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.direction.clone())
    }
}
impl mmsdm_core::GetTable for BidsBidofferperiod2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "BIDS";
    const TABLE_NAME: &'static str = "BIDOFFERPERIOD";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidsBidofferperiod2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "DUID",
        "BIDTYPE",
        "TRADINGDATE",
        "OFFERDATETIME",
        "DIRECTION",
        "PERIODID",
        "MAXAVAIL",
        "FIXEDLOAD",
        "RAMPUPRATE",
        "RAMPDOWNRATE",
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
        "PASAAVAILABILITY",
        "ENERGYLIMIT",
        "PERIODIDTO",
    ];
    type Row<'row> = BidsBidofferperiod2Row<'row>;
    type FieldMapping = BidsBidofferperiod2Mapping;
    type PrimaryKey = BidsBidofferperiod2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidsBidofferperiod2Row {
            duid: row.get_range("duid", field_mapping.0[0])?,
            bidtype: row.get_range("bidtype", field_mapping.0[1])?,
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            direction: row.get_range("direction", field_mapping.0[4])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
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
            rampuprate: row.get_opt_parsed_at_idx("rampuprate", field_mapping.0[8])?,
            rampdownrate: row.get_opt_parsed_at_idx("rampdownrate", field_mapping.0[9])?,
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
            pasaavailability: row
                .get_opt_custom_parsed_at_idx(
                    "pasaavailability",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energylimit: row
                .get_opt_custom_parsed_at_idx(
                    "energylimit",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodidto: row
                .get_opt_custom_parsed_at_idx(
                    "periodidto",
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
        Ok(BidsBidofferperiod2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidsBidofferperiod2PrimaryKey {
        BidsBidofferperiod2PrimaryKey {
            bidtype: row.bidtype().to_string(),
            direction: row.direction().to_string(),
            duid: row.duid().to_string(),
            offerdatetime: row.offerdatetime,
            periodid: row.periodid,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bids_bidofferperiod_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidsBidofferperiod2Row {
            duid: row.duid.clone(),
            bidtype: row.bidtype.clone(),
            tradingdate: row.tradingdate.clone(),
            offerdatetime: row.offerdatetime.clone(),
            direction: row.direction.clone(),
            periodid: row.periodid.clone(),
            maxavail: row.maxavail.clone(),
            fixedload: row.fixedload.clone(),
            rampuprate: row.rampuprate.clone(),
            rampdownrate: row.rampdownrate.clone(),
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
            pasaavailability: row.pasaavailability.clone(),
            energylimit: row.energylimit.clone(),
            periodidto: row.periodidto.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidsBidofferperiod2PrimaryKey {
    pub bidtype: alloc::string::String,
    pub direction: alloc::string::String,
    pub duid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BidsBidofferperiod2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidsBidofferperiod2Row<'data> {
    type Row<'other> = BidsBidofferperiod2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.direction() == row.direction()
            && self.duid() == row.duid() && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidsBidofferperiod2Row<'data> {
    type PrimaryKey = BidsBidofferperiod2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.direction() == key.direction
            && self.duid() == key.duid && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidsBidofferperiod2PrimaryKey {
    type Row<'other> = BidsBidofferperiod2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.direction == row.direction()
            && self.duid == row.duid() && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidsBidofferperiod2PrimaryKey {
    type PrimaryKey = BidsBidofferperiod2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.direction == key.direction
            && self.duid == key.duid && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidsBidofferperiod2 {
    type Builder = BidsBidofferperiod2Builder;
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
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "direction",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "maxavail",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fixedload",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rampuprate",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rampdownrate",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablementmin",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablementmax",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowbreakpoint",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "highbreakpoint",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail1",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail2",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail3",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail4",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail5",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail6",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail7",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail8",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail9",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail10",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pasaavailability",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energylimit",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "periodidto",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BidsBidofferperiod2Builder {
            duid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            direction_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            maxavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            fixedload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            rampuprate_array: arrow::array::builder::Int64Builder::new(),
            rampdownrate_array: arrow::array::builder::Int64Builder::new(),
            enablementmin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            enablementmax_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            lowbreakpoint_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            highbreakpoint_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            pasaavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            energylimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            periodidto_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.duid_array.append_value(row.duid());
        builder.bidtype_array.append_value(row.bidtype());
        builder.tradingdate_array.append_value(row.tradingdate.timestamp_millis());
        builder.offerdatetime_array.append_value(row.offerdatetime.timestamp_millis());
        builder.direction_array.append_value(row.direction());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .maxavail_array
            .append_option({
                row.maxavail
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .fixedload_array
            .append_option({
                row.fixedload
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder.rampuprate_array.append_option(row.rampuprate);
        builder.rampdownrate_array.append_option(row.rampdownrate);
        builder
            .enablementmin_array
            .append_option({
                row.enablementmin
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .enablementmax_array
            .append_option({
                row.enablementmax
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .lowbreakpoint_array
            .append_option({
                row.lowbreakpoint
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .highbreakpoint_array
            .append_option({
                row.highbreakpoint
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail1_array
            .append_option({
                row.bandavail1
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail2_array
            .append_option({
                row.bandavail2
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail3_array
            .append_option({
                row.bandavail3
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail4_array
            .append_option({
                row.bandavail4
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail5_array
            .append_option({
                row.bandavail5
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail6_array
            .append_option({
                row.bandavail6
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail7_array
            .append_option({
                row.bandavail7
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail8_array
            .append_option({
                row.bandavail8
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail9_array
            .append_option({
                row.bandavail9
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail10_array
            .append_option({
                row.bandavail10
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .pasaavailability_array
            .append_option({
                row.pasaavailability
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .energylimit_array
            .append_option({
                row.energylimit
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .periodidto_array
            .append_option({
                row.periodidto
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
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.direction_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxavail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fixedload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rampuprate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rampdownrate_array.finish())
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
                    alloc::sync::Arc::new(builder.pasaavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energylimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodidto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidsBidofferperiod2Builder {
    duid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    direction_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    maxavail_array: arrow::array::builder::Decimal128Builder,
    fixedload_array: arrow::array::builder::Decimal128Builder,
    rampuprate_array: arrow::array::builder::Int64Builder,
    rampdownrate_array: arrow::array::builder::Int64Builder,
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
    pasaavailability_array: arrow::array::builder::Decimal128Builder,
    energylimit_array: arrow::array::builder::Decimal128Builder,
    periodidto_array: arrow::array::builder::Decimal128Builder,
}
pub struct BidBidperofferD3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&BidBidperofferD3Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidBidperofferD3 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BidBidperofferD3Mapping([usize; 31]);
/// # Summary
///
/// ## BIDPEROFFER_D
///  _BIDPEROFFER_D shows the public summary of the energy and FCAS offers applicable in the Dispatch for the<br>intervals identified. BIDPEROFFER_D is the child to BIDDAYOFFER_D._
///
/// * Data Set Name: Bid
/// * File Name: Bidperoffer D
/// * Data Version: 3
///
/// # Description
///  BIDPEROFFER_D is public data, so is available to all participants. Source BIDPEROFFER_D updates daily shortly after 4am.  See also BIDPEROFFER.
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DIRECTION
/// * DUID
/// * INTERVAL_DATETIME
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct BidBidperofferD3Row<'data> {
    /// Market date for which the bid applied
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable Unit identifier
    pub duid: core::ops::Range<usize>,
    /// Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// The power flow direction to which this offer applies: GEN, LOAD or BIDIRECTIONAL
    pub direction: core::ops::Range<usize>,
    /// Date and Time of the dispatch interval to which the offer applied
    pub interval_datetime: chrono::NaiveDateTime,
    /// Market date for which the bid was submitted
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Offer date and time
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// The trading interval period identifier (1-288)
    pub periodid: Option<rust_decimal::Decimal>,
    /// Version number of offer
    pub versionno: Option<rust_decimal::Decimal>,
    /// Maximum availability for this BidType in this period
    pub maxavail: Option<rust_decimal::Decimal>,
    /// Fixed unit output MW (Energy Bids Only).  A value of zero means no fixed load so the unit is dispatched according to bid and market (rather than zero fixed load)
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
    /// The Energy limit applying at the end of this dispatch interval in MWh. For GEN this is a lower energy limit. For LOAD this is an upper energy limit
    pub energylimit: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidBidperofferD3Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
    pub fn direction(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.direction.clone())
    }
}
impl mmsdm_core::GetTable for BidBidperofferD3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "BID";
    const TABLE_NAME: &'static str = "BIDPEROFFER_D";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidBidperofferD3Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "DUID",
        "BIDTYPE",
        "DIRECTION",
        "INTERVAL_DATETIME",
        "BIDSETTLEMENTDATE",
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
        "ENERGYLIMIT",
    ];
    type Row<'row> = BidBidperofferD3Row<'row>;
    type FieldMapping = BidBidperofferD3Mapping;
    type PrimaryKey = BidBidperofferD3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidBidperofferD3Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[1])?,
            bidtype: row.get_range("bidtype", field_mapping.0[2])?,
            direction: row.get_range("direction", field_mapping.0[3])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            bidsettlementdate: row
                .get_opt_custom_parsed_at_idx(
                    "bidsettlementdate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdate: row
                .get_opt_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_opt_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            versionno: row
                .get_opt_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxavail: row
                .get_opt_custom_parsed_at_idx(
                    "maxavail",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fixedload: row
                .get_opt_custom_parsed_at_idx(
                    "fixedload",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rocup: row
                .get_opt_custom_parsed_at_idx(
                    "rocup",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rocdown: row
                .get_opt_custom_parsed_at_idx(
                    "rocdown",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablementmin: row
                .get_opt_custom_parsed_at_idx(
                    "enablementmin",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablementmax: row
                .get_opt_custom_parsed_at_idx(
                    "enablementmax",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowbreakpoint: row
                .get_opt_custom_parsed_at_idx(
                    "lowbreakpoint",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            highbreakpoint: row
                .get_opt_custom_parsed_at_idx(
                    "highbreakpoint",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail1: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail1",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail2: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail2",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail3: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail3",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail4: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail4",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail5: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail5",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail6: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail6",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail7: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail7",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail8: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail8",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail9: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail9",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bandavail10: row
                .get_opt_custom_parsed_at_idx(
                    "bandavail10",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[27],
                    mmsdm_core::mms_datetime::parse,
                )?,
            pasaavailability: row
                .get_opt_custom_parsed_at_idx(
                    "pasaavailability",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mr_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "mr_capacity",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energylimit: row
                .get_opt_custom_parsed_at_idx(
                    "energylimit",
                    field_mapping.0[30],
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
        Ok(BidBidperofferD3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidBidperofferD3PrimaryKey {
        BidBidperofferD3PrimaryKey {
            bidtype: row.bidtype().to_string(),
            direction: row.direction().to_string(),
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bid_bidperoffer_d_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidBidperofferD3Row {
            settlementdate: row.settlementdate.clone(),
            duid: row.duid.clone(),
            bidtype: row.bidtype.clone(),
            direction: row.direction.clone(),
            interval_datetime: row.interval_datetime.clone(),
            bidsettlementdate: row.bidsettlementdate.clone(),
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
            energylimit: row.energylimit.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidBidperofferD3PrimaryKey {
    pub bidtype: alloc::string::String,
    pub direction: alloc::string::String,
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BidBidperofferD3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidBidperofferD3Row<'data> {
    type Row<'other> = BidBidperofferD3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.direction() == row.direction()
            && self.duid() == row.duid()
            && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidBidperofferD3Row<'data> {
    type PrimaryKey = BidBidperofferD3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.direction() == key.direction
            && self.duid() == key.duid && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidBidperofferD3PrimaryKey {
    type Row<'other> = BidBidperofferD3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.direction == row.direction()
            && self.duid == row.duid() && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidBidperofferD3PrimaryKey {
    type PrimaryKey = BidBidperofferD3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.direction == key.direction
            && self.duid == key.duid && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidBidperofferD3 {
    type Builder = BidBidperofferD3Builder;
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
                    "direction",
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
                    "bidsettlementdate",
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
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
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
                arrow::datatypes::Field::new(
                    "energylimit",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BidBidperofferD3Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            direction_array: arrow::array::builder::StringBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
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
            energylimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder.bidtype_array.append_value(row.bidtype());
        builder.direction_array.append_value(row.direction());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
        builder
            .bidsettlementdate_array
            .append_option(row.bidsettlementdate.map(|val| val.timestamp_millis()));
        builder
            .offerdate_array
            .append_option(row.offerdate.map(|val| val.timestamp_millis()));
        builder
            .periodid_array
            .append_option({
                row.periodid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
        builder
            .energylimit_array
            .append_option({
                row.energylimit
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
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.direction_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidsettlementdate_array.finish())
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
                    alloc::sync::Arc::new(builder.energylimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidBidperofferD3Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    direction_array: arrow::array::builder::StringBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    bidsettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
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
    energylimit_array: arrow::array::builder::Decimal128Builder,
}
pub struct BidsMnspBidofferperiod1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&BidsMnspBidofferperiod1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidsMnspBidofferperiod1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BidsMnspBidofferperiod1Mapping([usize; 18]);
/// # Summary
///
/// ## MNSP_BIDOFFERPERIOD
///  _MNSP_BIDOFFERPERIOD shows availability for 5-minute periods for a specific Bid and LinkID for the given Trading Date and period. MNSP_BIDOFFERPERIOD is a child to MNSP_DAYOFFER and links to BIDOFFERFILETRK for 5MS Bids._
///
/// * Data Set Name: Bids
/// * File Name: Mnsp Bidofferperiod
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * LINKID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, PartialEq, Eq)]
pub struct BidsMnspBidofferperiod1Row<'data> {
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: core::ops::Range<usize>,
    /// The trading date this bid is for
    pub tradingdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    pub offerdatetime: chrono::NaiveDateTime,
    /// Period ID, 1 to 288
    pub periodid: rust_decimal::Decimal,
    /// Maximum planned availability MW
    pub maxavail: Option<rust_decimal::Decimal>,
    /// Fixed unit output, in MW. A value of NULL means no fixed load so the unit is dispatched according to bid and the market.
    pub fixedload: Option<rust_decimal::Decimal>,
    /// Ramp rate (MW / min) in the positive direction of flow for this MNSP link for this half-hour period
    pub rampuprate: Option<i64>,
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
    /// Allows for future use for Energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidsMnspBidofferperiod1Row<'data> {
    pub fn linkid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.linkid.clone())
    }
}
impl mmsdm_core::GetTable for BidsMnspBidofferperiod1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "BIDS";
    const TABLE_NAME: &'static str = "MNSP_BIDOFFERPERIOD";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidsMnspBidofferperiod1Mapping([
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
        "LINKID",
        "TRADINGDATE",
        "OFFERDATETIME",
        "PERIODID",
        "MAXAVAIL",
        "FIXEDLOAD",
        "RAMPUPRATE",
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
        "PASAAVAILABILITY",
    ];
    type Row<'row> = BidsMnspBidofferperiod1Row<'row>;
    type FieldMapping = BidsMnspBidofferperiod1Mapping;
    type PrimaryKey = BidsMnspBidofferperiod1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidsMnspBidofferperiod1Row {
            linkid: row.get_range("linkid", field_mapping.0[0])?,
            tradingdate: row
                .get_custom_parsed_at_idx(
                    "tradingdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxavail: row
                .get_opt_custom_parsed_at_idx(
                    "maxavail",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fixedload: row
                .get_opt_custom_parsed_at_idx(
                    "fixedload",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rampuprate: row.get_opt_parsed_at_idx("rampuprate", field_mapping.0[6])?,
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
            pasaavailability: row
                .get_opt_custom_parsed_at_idx(
                    "pasaavailability",
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
        Ok(BidsMnspBidofferperiod1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidsMnspBidofferperiod1PrimaryKey {
        BidsMnspBidofferperiod1PrimaryKey {
            linkid: row.linkid().to_string(),
            offerdatetime: row.offerdatetime,
            periodid: row.periodid,
            tradingdate: row.tradingdate,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("bids_mnsp_bidofferperiod_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidsMnspBidofferperiod1Row {
            linkid: row.linkid.clone(),
            tradingdate: row.tradingdate.clone(),
            offerdatetime: row.offerdatetime.clone(),
            periodid: row.periodid.clone(),
            maxavail: row.maxavail.clone(),
            fixedload: row.fixedload.clone(),
            rampuprate: row.rampuprate.clone(),
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
            pasaavailability: row.pasaavailability.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidsMnspBidofferperiod1PrimaryKey {
    pub linkid: alloc::string::String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BidsMnspBidofferperiod1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidsMnspBidofferperiod1Row<'data> {
    type Row<'other> = BidsMnspBidofferperiod1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid() == row.linkid() && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid && self.tradingdate == row.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidsMnspBidofferperiod1Row<'data> {
    type PrimaryKey = BidsMnspBidofferperiod1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid() == key.linkid && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid && self.tradingdate == key.tradingdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidsMnspBidofferperiod1PrimaryKey {
    type Row<'other> = BidsMnspBidofferperiod1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid == row.linkid() && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid && self.tradingdate == row.tradingdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidsMnspBidofferperiod1PrimaryKey {
    type PrimaryKey = BidsMnspBidofferperiod1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid && self.tradingdate == key.tradingdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidsMnspBidofferperiod1 {
    type Builder = BidsMnspBidofferperiod1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "linkid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "maxavail",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fixedload",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rampuprate",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail1",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail2",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail3",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail4",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail5",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail6",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail7",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail8",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail9",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bandavail10",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pasaavailability",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BidsMnspBidofferperiod1Builder {
            linkid_array: arrow::array::builder::StringBuilder::new(),
            tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            maxavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            fixedload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            rampuprate_array: arrow::array::builder::Int64Builder::new(),
            bandavail1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            bandavail10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            pasaavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.linkid_array.append_value(row.linkid());
        builder.tradingdate_array.append_value(row.tradingdate.timestamp_millis());
        builder.offerdatetime_array.append_value(row.offerdatetime.timestamp_millis());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .maxavail_array
            .append_option({
                row.maxavail
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .fixedload_array
            .append_option({
                row.fixedload
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder.rampuprate_array.append_option(row.rampuprate);
        builder
            .bandavail1_array
            .append_option({
                row.bandavail1
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail2_array
            .append_option({
                row.bandavail2
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail3_array
            .append_option({
                row.bandavail3
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail4_array
            .append_option({
                row.bandavail4
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail5_array
            .append_option({
                row.bandavail5
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail6_array
            .append_option({
                row.bandavail6
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail7_array
            .append_option({
                row.bandavail7
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail8_array
            .append_option({
                row.bandavail8
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail9_array
            .append_option({
                row.bandavail9
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .bandavail10_array
            .append_option({
                row.bandavail10
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .pasaavailability_array
            .append_option({
                row.pasaavailability
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
                    alloc::sync::Arc::new(builder.linkid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxavail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fixedload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rampuprate_array.finish())
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
                    alloc::sync::Arc::new(builder.pasaavailability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidsMnspBidofferperiod1Builder {
    linkid_array: arrow::array::builder::StringBuilder,
    tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    maxavail_array: arrow::array::builder::Decimal128Builder,
    fixedload_array: arrow::array::builder::Decimal128Builder,
    rampuprate_array: arrow::array::builder::Int64Builder,
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
    pasaavailability_array: arrow::array::builder::Decimal128Builder,
}
pub struct BidMnspDayoffer2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&BidMnspDayoffer2Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl BidMnspDayoffer2 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct BidMnspDayoffer2Mapping([usize; 24]);
/// # Summary
///
/// ## MNSP_DAYOFFER
///  _MNSP_DAYOFFER updates as bids are processed. All bids are available as part of next day market data. MNSP_DAYOFFER is the parent table to MNSP_BIDOFFERPERIOD, and joins to BIDOFFERFILETRK for 5MS Bids._
///
/// * Data Set Name: Bid
/// * File Name: Mnsp Dayoffer
/// * Data Version: 2
///
/// # Description
///  MNSP_DAYOFFER shows own (confidential) data updates as bids are processed. All bids are available as part of next day market data. Volume 4, 000 per year
///
///
///
/// # Primary Key Columns
///
/// * LINKID
/// * OFFERDATE
/// * PARTICIPANTID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct BidMnspDayoffer2Row<'data> {
    /// Market Date from which bid is active
    pub settlementdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    pub offerdate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: core::ops::Range<usize>,
    /// Bid type. Either Rebid or Daily
    pub entrytype: core::ops::Range<usize>,
    /// Explanation for all rebids and inflexibilities
    pub rebidexplanation: core::ops::Range<usize>,
    /// Price for Availability Band 1
    pub priceband1: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 2
    pub priceband2: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 3
    pub priceband3: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 4
    pub priceband4: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 5
    pub priceband5: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    pub priceband6: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 7
    pub priceband7: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 8
    pub priceband8: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 9
    pub priceband9: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 10
    pub priceband10: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Offer Factor
    pub mr_factor: Option<rust_decimal::Decimal>,
    /// The time of the event(s) or other occurrence(s) cited/adduced as the reason for the rebid. Required for rebids, not required for fixed load or low ramp rates. Expected in the format: HH:MM:SS e.g. 20:11:00
    pub rebid_event_time: core::ops::Range<usize>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant became aware of the event(s) / occurrence(s) that prompted the rebid. Not validated by AEMO
    pub rebid_aware_time: core::ops::Range<usize>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant made the decision to rebid. Not validated by AEMO
    pub rebid_decision_time: core::ops::Range<usize>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. A provided rebid category. Not validated by AEMO
    pub rebid_category: core::ops::Range<usize>,
    /// A participants unique Reference Id
    pub reference_id: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> BidMnspDayoffer2Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn linkid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.linkid.clone())
    }
    pub fn entrytype(&self) -> Option<&str> {
        if self.entrytype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.entrytype.clone(),
                ),
            )
        }
    }
    pub fn rebidexplanation(&self) -> Option<&str> {
        if self.rebidexplanation.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebidexplanation.clone(),
                ),
            )
        }
    }
    pub fn rebid_event_time(&self) -> Option<&str> {
        if self.rebid_event_time.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_event_time.clone(),
                ),
            )
        }
    }
    pub fn rebid_aware_time(&self) -> Option<&str> {
        if self.rebid_aware_time.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_aware_time.clone(),
                ),
            )
        }
    }
    pub fn rebid_decision_time(&self) -> Option<&str> {
        if self.rebid_decision_time.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_decision_time.clone(),
                ),
            )
        }
    }
    pub fn rebid_category(&self) -> Option<&str> {
        if self.rebid_category.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.rebid_category.clone(),
                ),
            )
        }
    }
    pub fn reference_id(&self) -> Option<&str> {
        if self.reference_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reference_id.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for BidMnspDayoffer2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "BID";
    const TABLE_NAME: &'static str = "MNSP_DAYOFFER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = BidMnspDayoffer2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "OFFERDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "LINKID",
        "ENTRYTYPE",
        "REBIDEXPLANATION",
        "PRICEBAND1",
        "PRICEBAND2",
        "PRICEBAND3",
        "PRICEBAND4",
        "PRICEBAND5",
        "PRICEBAND6",
        "PRICEBAND7",
        "PRICEBAND8",
        "PRICEBAND9",
        "PRICEBAND10",
        "LASTCHANGED",
        "MR_FACTOR",
        "REBID_EVENT_TIME",
        "REBID_AWARE_TIME",
        "REBID_DECISION_TIME",
        "REBID_CATEGORY",
        "REFERENCE_ID",
    ];
    type Row<'row> = BidMnspDayoffer2Row<'row>;
    type FieldMapping = BidMnspDayoffer2Mapping;
    type PrimaryKey = BidMnspDayoffer2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(BidMnspDayoffer2Row {
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
            entrytype: row.get_opt_range("entrytype", field_mapping.0[5])?,
            rebidexplanation: row.get_opt_range("rebidexplanation", field_mapping.0[6])?,
            priceband1: row
                .get_opt_custom_parsed_at_idx(
                    "priceband1",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband2: row
                .get_opt_custom_parsed_at_idx(
                    "priceband2",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband3: row
                .get_opt_custom_parsed_at_idx(
                    "priceband3",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband4: row
                .get_opt_custom_parsed_at_idx(
                    "priceband4",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband5: row
                .get_opt_custom_parsed_at_idx(
                    "priceband5",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband6: row
                .get_opt_custom_parsed_at_idx(
                    "priceband6",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband7: row
                .get_opt_custom_parsed_at_idx(
                    "priceband7",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband8: row
                .get_opt_custom_parsed_at_idx(
                    "priceband8",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband9: row
                .get_opt_custom_parsed_at_idx(
                    "priceband9",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            priceband10: row
                .get_opt_custom_parsed_at_idx(
                    "priceband10",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[17],
                    mmsdm_core::mms_datetime::parse,
                )?,
            mr_factor: row
                .get_opt_custom_parsed_at_idx(
                    "mr_factor",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rebid_event_time: row
                .get_opt_range("rebid_event_time", field_mapping.0[19])?,
            rebid_aware_time: row
                .get_opt_range("rebid_aware_time", field_mapping.0[20])?,
            rebid_decision_time: row
                .get_opt_range("rebid_decision_time", field_mapping.0[21])?,
            rebid_category: row.get_opt_range("rebid_category", field_mapping.0[22])?,
            reference_id: row.get_opt_range("reference_id", field_mapping.0[23])?,
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
        Ok(BidMnspDayoffer2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> BidMnspDayoffer2PrimaryKey {
        BidMnspDayoffer2PrimaryKey {
            linkid: row.linkid().to_string(),
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
        alloc::format!("bid_mnsp_dayoffer_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        BidMnspDayoffer2Row {
            settlementdate: row.settlementdate.clone(),
            offerdate: row.offerdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            linkid: row.linkid.clone(),
            entrytype: row.entrytype.clone(),
            rebidexplanation: row.rebidexplanation.clone(),
            priceband1: row.priceband1.clone(),
            priceband2: row.priceband2.clone(),
            priceband3: row.priceband3.clone(),
            priceband4: row.priceband4.clone(),
            priceband5: row.priceband5.clone(),
            priceband6: row.priceband6.clone(),
            priceband7: row.priceband7.clone(),
            priceband8: row.priceband8.clone(),
            priceband9: row.priceband9.clone(),
            priceband10: row.priceband10.clone(),
            lastchanged: row.lastchanged.clone(),
            mr_factor: row.mr_factor.clone(),
            rebid_event_time: row.rebid_event_time.clone(),
            rebid_aware_time: row.rebid_aware_time.clone(),
            rebid_decision_time: row.rebid_decision_time.clone(),
            rebid_category: row.rebid_category.clone(),
            reference_id: row.reference_id.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BidMnspDayoffer2PrimaryKey {
    pub linkid: alloc::string::String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BidMnspDayoffer2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for BidMnspDayoffer2Row<'data> {
    type Row<'other> = BidMnspDayoffer2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid() == row.linkid() && self.offerdate == row.offerdate
            && self.participantid() == row.participantid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for BidMnspDayoffer2Row<'data> {
    type PrimaryKey = BidMnspDayoffer2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid() == key.linkid && self.offerdate == key.offerdate
            && self.participantid() == key.participantid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for BidMnspDayoffer2PrimaryKey {
    type Row<'other> = BidMnspDayoffer2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.linkid == row.linkid() && self.offerdate == row.offerdate
            && self.participantid == row.participantid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidMnspDayoffer2PrimaryKey {
    type PrimaryKey = BidMnspDayoffer2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid && self.offerdate == key.offerdate
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BidMnspDayoffer2 {
    type Builder = BidMnspDayoffer2Builder;
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
                    "entrytype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebidexplanation",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband1",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband2",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband3",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband4",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband5",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband6",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband7",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband8",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband9",
                    arrow::datatypes::DataType::Decimal128(9, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "priceband10",
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
                arrow::datatypes::Field::new(
                    "mr_factor",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_event_time",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_aware_time",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_decision_time",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rebid_category",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reference_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        BidMnspDayoffer2Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            linkid_array: arrow::array::builder::StringBuilder::new(),
            entrytype_array: arrow::array::builder::StringBuilder::new(),
            rebidexplanation_array: arrow::array::builder::StringBuilder::new(),
            priceband1_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband3_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband4_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband5_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband6_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband7_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband8_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband9_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            priceband10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(9, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            mr_factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            rebid_event_time_array: arrow::array::builder::StringBuilder::new(),
            rebid_aware_time_array: arrow::array::builder::StringBuilder::new(),
            rebid_decision_time_array: arrow::array::builder::StringBuilder::new(),
            rebid_category_array: arrow::array::builder::StringBuilder::new(),
            reference_id_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.offerdate_array.append_value(row.offerdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.linkid_array.append_value(row.linkid());
        builder.entrytype_array.append_option(row.entrytype());
        builder.rebidexplanation_array.append_option(row.rebidexplanation());
        builder
            .priceband1_array
            .append_option({
                row.priceband1
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband2_array
            .append_option({
                row.priceband2
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband3_array
            .append_option({
                row.priceband3
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband4_array
            .append_option({
                row.priceband4
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband5_array
            .append_option({
                row.priceband5
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband6_array
            .append_option({
                row.priceband6
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband7_array
            .append_option({
                row.priceband7
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband8_array
            .append_option({
                row.priceband8
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband9_array
            .append_option({
                row.priceband9
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .priceband10_array
            .append_option({
                row.priceband10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .mr_factor_array
            .append_option({
                row.mr_factor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.rebid_event_time_array.append_option(row.rebid_event_time());
        builder.rebid_aware_time_array.append_option(row.rebid_aware_time());
        builder.rebid_decision_time_array.append_option(row.rebid_decision_time());
        builder.rebid_category_array.append_option(row.rebid_category());
        builder.reference_id_array.append_option(row.reference_id());
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
                    alloc::sync::Arc::new(builder.entrytype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebidexplanation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband8_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband9_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.priceband10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mr_factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_event_time_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_aware_time_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_decision_time_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rebid_category_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reference_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct BidMnspDayoffer2Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    linkid_array: arrow::array::builder::StringBuilder,
    entrytype_array: arrow::array::builder::StringBuilder,
    rebidexplanation_array: arrow::array::builder::StringBuilder,
    priceband1_array: arrow::array::builder::Decimal128Builder,
    priceband2_array: arrow::array::builder::Decimal128Builder,
    priceband3_array: arrow::array::builder::Decimal128Builder,
    priceband4_array: arrow::array::builder::Decimal128Builder,
    priceband5_array: arrow::array::builder::Decimal128Builder,
    priceband6_array: arrow::array::builder::Decimal128Builder,
    priceband7_array: arrow::array::builder::Decimal128Builder,
    priceband8_array: arrow::array::builder::Decimal128Builder,
    priceband9_array: arrow::array::builder::Decimal128Builder,
    priceband10_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    mr_factor_array: arrow::array::builder::Decimal128Builder,
    rebid_event_time_array: arrow::array::builder::StringBuilder,
    rebid_aware_time_array: arrow::array::builder::StringBuilder,
    rebid_decision_time_array: arrow::array::builder::StringBuilder,
    rebid_category_array: arrow::array::builder::StringBuilder,
    reference_id_array: arrow::array::builder::StringBuilder,
}
pub struct OfferMtpasaOfferdata2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&OfferMtpasaOfferdata2Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl OfferMtpasaOfferdata2 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct OfferMtpasaOfferdata2Mapping([usize; 27]);
/// # Summary
///
/// ## MTPASA_OFFERDATA
///  _Participant submitted Offers for MTPASA process_
///
/// * Data Set Name: Offer
/// * File Name: Mtpasa Offerdata
/// * Data Version: 2
///
///
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * OFFERDATETIME
/// * PARTICIPANTID
/// * UNITID
#[derive(Debug, PartialEq, Eq)]
pub struct OfferMtpasaOfferdata2Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Date time file processed
    pub offerdatetime: chrono::NaiveDateTime,
    /// either duid or mnsp linkid
    pub unitid: core::ops::Range<usize>,
    /// trade date when the offer becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// weekly energy constraint value
    pub energy: Option<i64>,
    /// capacity value day 1 (sunday)
    pub capacity1: Option<i64>,
    /// capacity value day 2 (monday)
    pub capacity2: Option<i64>,
    /// capacity value day 3 (tuesday)
    pub capacity3: Option<i64>,
    /// capacity value day 4 (wednesday)
    pub capacity4: Option<i64>,
    /// capacity value day 5 (thursday)
    pub capacity5: Option<i64>,
    /// capacity value day 6 (friday)
    pub capacity6: Option<i64>,
    /// capacity value day 7 (saturday)
    pub capacity7: Option<i64>,
    /// timestamp when record last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The unit state value for day 1 Sunday
    pub unitstate1: core::ops::Range<usize>,
    /// The unit state value for day 2 Monday
    pub unitstate2: core::ops::Range<usize>,
    /// The unit state value for day 3 Tuesday
    pub unitstate3: core::ops::Range<usize>,
    /// The unit state value for 4 Wednesday
    pub unitstate4: core::ops::Range<usize>,
    /// The unit state value for day 5 Thursday
    pub unitstate5: core::ops::Range<usize>,
    /// The unit state value for day 6 Friday
    pub unitstate6: core::ops::Range<usize>,
    /// The unit state value for day 7 Saturday
    pub unitstate7: core::ops::Range<usize>,
    /// The recall time associated with the unit state for day 1 Sunday
    pub recalltime1: Option<i64>,
    /// The recall time associated with the unit state for day 2 Monday
    pub recalltime2: Option<i64>,
    /// The recall time associated with the unit state for day 3 Tuesday
    pub recalltime3: Option<i64>,
    /// The recall time associated with the unit state for day 4 Wednesday
    pub recalltime4: Option<i64>,
    /// The recall time associated with the unit state for day 5 Thursday
    pub recalltime5: Option<i64>,
    /// The recall time associated with the unit state for day 6 Friday
    pub recalltime6: Option<i64>,
    /// The recall time associated with the unit state for day 7 Saturday
    pub recalltime7: Option<i64>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> OfferMtpasaOfferdata2Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn unitid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.unitid.clone())
    }
    pub fn unitstate1(&self) -> Option<&str> {
        if self.unitstate1.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unitstate1.clone(),
                ),
            )
        }
    }
    pub fn unitstate2(&self) -> Option<&str> {
        if self.unitstate2.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unitstate2.clone(),
                ),
            )
        }
    }
    pub fn unitstate3(&self) -> Option<&str> {
        if self.unitstate3.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unitstate3.clone(),
                ),
            )
        }
    }
    pub fn unitstate4(&self) -> Option<&str> {
        if self.unitstate4.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unitstate4.clone(),
                ),
            )
        }
    }
    pub fn unitstate5(&self) -> Option<&str> {
        if self.unitstate5.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unitstate5.clone(),
                ),
            )
        }
    }
    pub fn unitstate6(&self) -> Option<&str> {
        if self.unitstate6.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unitstate6.clone(),
                ),
            )
        }
    }
    pub fn unitstate7(&self) -> Option<&str> {
        if self.unitstate7.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.unitstate7.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for OfferMtpasaOfferdata2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "OFFER";
    const TABLE_NAME: &'static str = "MTPASA_OFFERDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = OfferMtpasaOfferdata2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "OFFERDATETIME",
        "UNITID",
        "EFFECTIVEDATE",
        "ENERGY",
        "CAPACITY1",
        "CAPACITY2",
        "CAPACITY3",
        "CAPACITY4",
        "CAPACITY5",
        "CAPACITY6",
        "CAPACITY7",
        "LASTCHANGED",
        "UNITSTATE1",
        "UNITSTATE2",
        "UNITSTATE3",
        "UNITSTATE4",
        "UNITSTATE5",
        "UNITSTATE6",
        "UNITSTATE7",
        "RECALLTIME1",
        "RECALLTIME2",
        "RECALLTIME3",
        "RECALLTIME4",
        "RECALLTIME5",
        "RECALLTIME6",
        "RECALLTIME7",
    ];
    type Row<'row> = OfferMtpasaOfferdata2Row<'row>;
    type FieldMapping = OfferMtpasaOfferdata2Mapping;
    type PrimaryKey = OfferMtpasaOfferdata2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(OfferMtpasaOfferdata2Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            unitid: row.get_range("unitid", field_mapping.0[2])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            energy: row.get_opt_parsed_at_idx("energy", field_mapping.0[4])?,
            capacity1: row.get_opt_parsed_at_idx("capacity1", field_mapping.0[5])?,
            capacity2: row.get_opt_parsed_at_idx("capacity2", field_mapping.0[6])?,
            capacity3: row.get_opt_parsed_at_idx("capacity3", field_mapping.0[7])?,
            capacity4: row.get_opt_parsed_at_idx("capacity4", field_mapping.0[8])?,
            capacity5: row.get_opt_parsed_at_idx("capacity5", field_mapping.0[9])?,
            capacity6: row.get_opt_parsed_at_idx("capacity6", field_mapping.0[10])?,
            capacity7: row.get_opt_parsed_at_idx("capacity7", field_mapping.0[11])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            unitstate1: row.get_opt_range("unitstate1", field_mapping.0[13])?,
            unitstate2: row.get_opt_range("unitstate2", field_mapping.0[14])?,
            unitstate3: row.get_opt_range("unitstate3", field_mapping.0[15])?,
            unitstate4: row.get_opt_range("unitstate4", field_mapping.0[16])?,
            unitstate5: row.get_opt_range("unitstate5", field_mapping.0[17])?,
            unitstate6: row.get_opt_range("unitstate6", field_mapping.0[18])?,
            unitstate7: row.get_opt_range("unitstate7", field_mapping.0[19])?,
            recalltime1: row.get_opt_parsed_at_idx("recalltime1", field_mapping.0[20])?,
            recalltime2: row.get_opt_parsed_at_idx("recalltime2", field_mapping.0[21])?,
            recalltime3: row.get_opt_parsed_at_idx("recalltime3", field_mapping.0[22])?,
            recalltime4: row.get_opt_parsed_at_idx("recalltime4", field_mapping.0[23])?,
            recalltime5: row.get_opt_parsed_at_idx("recalltime5", field_mapping.0[24])?,
            recalltime6: row.get_opt_parsed_at_idx("recalltime6", field_mapping.0[25])?,
            recalltime7: row.get_opt_parsed_at_idx("recalltime7", field_mapping.0[26])?,
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
        Ok(OfferMtpasaOfferdata2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> OfferMtpasaOfferdata2PrimaryKey {
        OfferMtpasaOfferdata2PrimaryKey {
            effectivedate: row.effectivedate,
            offerdatetime: row.offerdatetime,
            participantid: row.participantid().to_string(),
            unitid: row.unitid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("offer_mtpasa_offerdata_v2_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        OfferMtpasaOfferdata2Row {
            participantid: row.participantid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            unitid: row.unitid.clone(),
            effectivedate: row.effectivedate.clone(),
            energy: row.energy.clone(),
            capacity1: row.capacity1.clone(),
            capacity2: row.capacity2.clone(),
            capacity3: row.capacity3.clone(),
            capacity4: row.capacity4.clone(),
            capacity5: row.capacity5.clone(),
            capacity6: row.capacity6.clone(),
            capacity7: row.capacity7.clone(),
            lastchanged: row.lastchanged.clone(),
            unitstate1: row.unitstate1.clone(),
            unitstate2: row.unitstate2.clone(),
            unitstate3: row.unitstate3.clone(),
            unitstate4: row.unitstate4.clone(),
            unitstate5: row.unitstate5.clone(),
            unitstate6: row.unitstate6.clone(),
            unitstate7: row.unitstate7.clone(),
            recalltime1: row.recalltime1.clone(),
            recalltime2: row.recalltime2.clone(),
            recalltime3: row.recalltime3.clone(),
            recalltime4: row.recalltime4.clone(),
            recalltime5: row.recalltime5.clone(),
            recalltime6: row.recalltime6.clone(),
            recalltime7: row.recalltime7.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OfferMtpasaOfferdata2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub offerdatetime: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub unitid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for OfferMtpasaOfferdata2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for OfferMtpasaOfferdata2Row<'data> {
    type Row<'other> = OfferMtpasaOfferdata2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.offerdatetime == row.offerdatetime
            && self.participantid() == row.participantid()
            && self.unitid() == row.unitid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for OfferMtpasaOfferdata2Row<'data> {
    type PrimaryKey = OfferMtpasaOfferdata2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.offerdatetime == key.offerdatetime
            && self.participantid() == key.participantid && self.unitid() == key.unitid
    }
}
impl<'data> mmsdm_core::CompareWithRow for OfferMtpasaOfferdata2PrimaryKey {
    type Row<'other> = OfferMtpasaOfferdata2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.offerdatetime == row.offerdatetime
            && self.participantid == row.participantid() && self.unitid == row.unitid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for OfferMtpasaOfferdata2PrimaryKey {
    type PrimaryKey = OfferMtpasaOfferdata2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.offerdatetime == key.offerdatetime
            && self.participantid == key.participantid && self.unitid == key.unitid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for OfferMtpasaOfferdata2 {
    type Builder = OfferMtpasaOfferdata2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "unitid",
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
                    "energy",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacity1",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacity2",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacity3",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacity4",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacity5",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacity6",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacity7",
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
                arrow::datatypes::Field::new(
                    "unitstate1",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitstate2",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitstate3",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitstate4",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitstate5",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitstate6",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unitstate7",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltime1",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltime2",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltime3",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltime4",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltime5",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltime6",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltime7",
                    arrow::datatypes::DataType::Int64,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        OfferMtpasaOfferdata2Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            unitid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            energy_array: arrow::array::builder::Int64Builder::new(),
            capacity1_array: arrow::array::builder::Int64Builder::new(),
            capacity2_array: arrow::array::builder::Int64Builder::new(),
            capacity3_array: arrow::array::builder::Int64Builder::new(),
            capacity4_array: arrow::array::builder::Int64Builder::new(),
            capacity5_array: arrow::array::builder::Int64Builder::new(),
            capacity6_array: arrow::array::builder::Int64Builder::new(),
            capacity7_array: arrow::array::builder::Int64Builder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            unitstate1_array: arrow::array::builder::StringBuilder::new(),
            unitstate2_array: arrow::array::builder::StringBuilder::new(),
            unitstate3_array: arrow::array::builder::StringBuilder::new(),
            unitstate4_array: arrow::array::builder::StringBuilder::new(),
            unitstate5_array: arrow::array::builder::StringBuilder::new(),
            unitstate6_array: arrow::array::builder::StringBuilder::new(),
            unitstate7_array: arrow::array::builder::StringBuilder::new(),
            recalltime1_array: arrow::array::builder::Int64Builder::new(),
            recalltime2_array: arrow::array::builder::Int64Builder::new(),
            recalltime3_array: arrow::array::builder::Int64Builder::new(),
            recalltime4_array: arrow::array::builder::Int64Builder::new(),
            recalltime5_array: arrow::array::builder::Int64Builder::new(),
            recalltime6_array: arrow::array::builder::Int64Builder::new(),
            recalltime7_array: arrow::array::builder::Int64Builder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
        builder.offerdatetime_array.append_value(row.offerdatetime.timestamp_millis());
        builder.unitid_array.append_value(row.unitid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder.energy_array.append_option(row.energy);
        builder.capacity1_array.append_option(row.capacity1);
        builder.capacity2_array.append_option(row.capacity2);
        builder.capacity3_array.append_option(row.capacity3);
        builder.capacity4_array.append_option(row.capacity4);
        builder.capacity5_array.append_option(row.capacity5);
        builder.capacity6_array.append_option(row.capacity6);
        builder.capacity7_array.append_option(row.capacity7);
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.unitstate1_array.append_option(row.unitstate1());
        builder.unitstate2_array.append_option(row.unitstate2());
        builder.unitstate3_array.append_option(row.unitstate3());
        builder.unitstate4_array.append_option(row.unitstate4());
        builder.unitstate5_array.append_option(row.unitstate5());
        builder.unitstate6_array.append_option(row.unitstate6());
        builder.unitstate7_array.append_option(row.unitstate7());
        builder.recalltime1_array.append_option(row.recalltime1);
        builder.recalltime2_array.append_option(row.recalltime2);
        builder.recalltime3_array.append_option(row.recalltime3);
        builder.recalltime4_array.append_option(row.recalltime4);
        builder.recalltime5_array.append_option(row.recalltime5);
        builder.recalltime6_array.append_option(row.recalltime6);
        builder.recalltime7_array.append_option(row.recalltime7);
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacity1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacity2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacity3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacity4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacity5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacity6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacity7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitstate1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitstate2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitstate3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitstate4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitstate5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitstate6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unitstate7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltime1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltime2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltime3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltime4_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltime5_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltime6_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltime7_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct OfferMtpasaOfferdata2Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    unitid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    energy_array: arrow::array::builder::Int64Builder,
    capacity1_array: arrow::array::builder::Int64Builder,
    capacity2_array: arrow::array::builder::Int64Builder,
    capacity3_array: arrow::array::builder::Int64Builder,
    capacity4_array: arrow::array::builder::Int64Builder,
    capacity5_array: arrow::array::builder::Int64Builder,
    capacity6_array: arrow::array::builder::Int64Builder,
    capacity7_array: arrow::array::builder::Int64Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    unitstate1_array: arrow::array::builder::StringBuilder,
    unitstate2_array: arrow::array::builder::StringBuilder,
    unitstate3_array: arrow::array::builder::StringBuilder,
    unitstate4_array: arrow::array::builder::StringBuilder,
    unitstate5_array: arrow::array::builder::StringBuilder,
    unitstate6_array: arrow::array::builder::StringBuilder,
    unitstate7_array: arrow::array::builder::StringBuilder,
    recalltime1_array: arrow::array::builder::Int64Builder,
    recalltime2_array: arrow::array::builder::Int64Builder,
    recalltime3_array: arrow::array::builder::Int64Builder,
    recalltime4_array: arrow::array::builder::Int64Builder,
    recalltime5_array: arrow::array::builder::Int64Builder,
    recalltime6_array: arrow::array::builder::Int64Builder,
    recalltime7_array: arrow::array::builder::Int64Builder,
}
pub struct OfferMtpasaOfferfiletrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&OfferMtpasaOfferfiletrk1Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl OfferMtpasaOfferfiletrk1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct OfferMtpasaOfferfiletrk1Mapping([usize; 3]);
/// # Summary
///
/// ## MTPASA_OFFERFILETRK
///  _Participant submitted Offers for MTPASA process_
///
/// * Data Set Name: Offer
/// * File Name: Mtpasa Offerfiletrk
/// * Data Version: 1
///
/// # Description
///  MTPASA_OFFERFILETRK is confidential to the relevant participant. Source MTPASA_OFFERFILETRK updates for every submitted MTPASA bid. Volume 4000 per year, being one per bid containing an MTPASA bid
///
///
///
/// # Primary Key Columns
///
/// * OFFERDATETIME
/// * PARTICIPANTID
#[derive(Debug, PartialEq, Eq)]
pub struct OfferMtpasaOfferfiletrk1Row<'data> {
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Date time file processed
    pub offerdatetime: chrono::NaiveDateTime,
    /// Submitted file name
    pub filename: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> OfferMtpasaOfferfiletrk1Row<'data> {
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
}
impl mmsdm_core::GetTable for OfferMtpasaOfferfiletrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "OFFER";
    const TABLE_NAME: &'static str = "MTPASA_OFFERFILETRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = OfferMtpasaOfferfiletrk1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PARTICIPANTID",
        "OFFERDATETIME",
        "FILENAME",
    ];
    type Row<'row> = OfferMtpasaOfferfiletrk1Row<'row>;
    type FieldMapping = OfferMtpasaOfferfiletrk1Mapping;
    type PrimaryKey = OfferMtpasaOfferfiletrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(OfferMtpasaOfferfiletrk1Row {
            participantid: row.get_range("participantid", field_mapping.0[0])?,
            offerdatetime: row
                .get_custom_parsed_at_idx(
                    "offerdatetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            filename: row.get_opt_range("filename", field_mapping.0[2])?,
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
        Ok(OfferMtpasaOfferfiletrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> OfferMtpasaOfferfiletrk1PrimaryKey {
        OfferMtpasaOfferfiletrk1PrimaryKey {
            offerdatetime: row.offerdatetime,
            participantid: row.participantid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("offer_mtpasa_offerfiletrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        OfferMtpasaOfferfiletrk1Row {
            participantid: row.participantid.clone(),
            offerdatetime: row.offerdatetime.clone(),
            filename: row.filename.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OfferMtpasaOfferfiletrk1PrimaryKey {
    pub offerdatetime: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for OfferMtpasaOfferfiletrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for OfferMtpasaOfferfiletrk1Row<'data> {
    type Row<'other> = OfferMtpasaOfferfiletrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.offerdatetime == row.offerdatetime
            && self.participantid() == row.participantid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for OfferMtpasaOfferfiletrk1Row<'data> {
    type PrimaryKey = OfferMtpasaOfferfiletrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdatetime == key.offerdatetime
            && self.participantid() == key.participantid
    }
}
impl<'data> mmsdm_core::CompareWithRow for OfferMtpasaOfferfiletrk1PrimaryKey {
    type Row<'other> = OfferMtpasaOfferfiletrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.offerdatetime == row.offerdatetime
            && self.participantid == row.participantid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for OfferMtpasaOfferfiletrk1PrimaryKey {
    type PrimaryKey = OfferMtpasaOfferfiletrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdatetime == key.offerdatetime
            && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for OfferMtpasaOfferfiletrk1 {
    type Builder = OfferMtpasaOfferfiletrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdatetime",
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
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        OfferMtpasaOfferfiletrk1Builder {
            participantid_array: arrow::array::builder::StringBuilder::new(),
            offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.participantid_array.append_value(row.participantid());
        builder.offerdatetime_array.append_value(row.offerdatetime.timestamp_millis());
        builder.filename_array.append_option(row.filename());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct OfferMtpasaOfferfiletrk1Builder {
    participantid_array: arrow::array::builder::StringBuilder,
    offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    filename_array: arrow::array::builder::StringBuilder,
}
