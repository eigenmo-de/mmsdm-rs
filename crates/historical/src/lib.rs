#[allow(unused_imports)]
use chrono::Datelike as _;
/// # Summary
///
/// ## BIDPEROFFER
///  _BIDPEROFFER shows period-based Energy and Ancillary Service bid data. BIDPEROFFER is a child table of BIDDAYOFFER._
///
/// * Data Set Name: Offer
/// * File Name: Bidperoffer
/// * Data Version: 1
///
/// # Description
///  The new ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three new tables facilitate ancillary service bidding. The new tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDPEROFFER) are similar in structure to energy bidding tables (OFFERFILETRK, DAYOFFER and PEROFFER). The significant differences with the new tables are: · The OFFERDATE field reflects the time the bid was loaded and this field alone provides the key for versioning of bids. The VERSIONNO field is retained for participant use as information only. · The new tables support bids for multiple services. The BIDTYPE field defines the service to which the bid applies. · There are no default bids. In the absence of a bid for a specific settlement date, the latest bid submitted for a previous settlement date applies. BIDPEROFFER data is confidential to the submitting participant until made public after 4am the next day. Source BIDPEROFFER updates as energy and ancillary service bids are processed. BIDPEROFFER includes all accepted energy and ancillary service bids. Volume Approximately 72,000,000  records per year
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * OFFERDATE
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferBidperoffer1 {
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Bid Type Identifier
    pub bidtype: String,
    /// Market date starting at 04:05
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Offer date
    #[serde(with = "mmsdm_core::mms_datetime")]
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Mandatory Restriction Offer amount
    pub mr_capacity: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for OfferBidperoffer1 {
    type PrimaryKey = OfferBidperoffer1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "OFFER".into(),
            table_name: Some("BIDPEROFFER".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> OfferBidperoffer1PrimaryKey {
        OfferBidperoffer1PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            offerdate: self.offerdate,
            periodid: self.periodid,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "offer_bidperoffer_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct OfferBidperoffer1PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub offerdate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for OfferBidperoffer1PrimaryKey {}
impl mmsdm_core::CompareWithRow for OfferBidperoffer1 {
    type Row = OfferBidperoffer1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype && self.duid == row.duid
            && self.offerdate == row.offerdate && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for OfferBidperoffer1 {
    type PrimaryKey = OfferBidperoffer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.duid == key.duid
            && self.offerdate == key.offerdate && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for OfferBidperoffer1PrimaryKey {
    type Row = OfferBidperoffer1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype && self.duid == row.duid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("bidtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(22, 0), false),
                arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("maxavail",
                arrow2::datatypes::DataType::Decimal(12, 6), true),
                arrow2::datatypes::Field::new("fixedload",
                arrow2::datatypes::DataType::Decimal(12, 6), true),
                arrow2::datatypes::Field::new("rocup",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("rocdown",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("enablementmin",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("enablementmax",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lowbreakpoint",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("highbreakpoint",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail1",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail2",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail3",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail4",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail5",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail6",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail7",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail8",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail9",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("bandavail10",
                arrow2::datatypes::DataType::Decimal(22, 0), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("pasaavailability",
                arrow2::datatypes::DataType::Decimal(12, 0), true),
                arrow2::datatypes::Field::new("mr_capacity",
                arrow2::datatypes::DataType::Decimal(6, 0), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut duid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut maxavail_array = Vec::new();
        let mut fixedload_array = Vec::new();
        let mut rocup_array = Vec::new();
        let mut rocdown_array = Vec::new();
        let mut enablementmin_array = Vec::new();
        let mut enablementmax_array = Vec::new();
        let mut lowbreakpoint_array = Vec::new();
        let mut highbreakpoint_array = Vec::new();
        let mut bandavail1_array = Vec::new();
        let mut bandavail2_array = Vec::new();
        let mut bandavail3_array = Vec::new();
        let mut bandavail4_array = Vec::new();
        let mut bandavail5_array = Vec::new();
        let mut bandavail6_array = Vec::new();
        let mut bandavail7_array = Vec::new();
        let mut bandavail8_array = Vec::new();
        let mut bandavail9_array = Vec::new();
        let mut bandavail10_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut pasaavailability_array = Vec::new();
        let mut mr_capacity_array = Vec::new();
        for row in partition {
            duid_array.push(row.duid);
            bidtype_array.push(row.bidtype);
            settlementdate_array.push(row.settlementdate.timestamp());
            offerdate_array.push(row.offerdate.timestamp());
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            versionno_array
                .push({
                    row.versionno
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            maxavail_array
                .push({
                    row.maxavail
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            fixedload_array
                .push({
                    row.fixedload
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            rocup_array
                .push({
                    row.rocup
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            rocdown_array
                .push({
                    row.rocdown
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            enablementmin_array
                .push({
                    row.enablementmin
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            enablementmax_array
                .push({
                    row.enablementmax
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lowbreakpoint_array
                .push({
                    row.lowbreakpoint
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            highbreakpoint_array
                .push({
                    row.highbreakpoint
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail1_array
                .push({
                    row.bandavail1
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail2_array
                .push({
                    row.bandavail2
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail3_array
                .push({
                    row.bandavail3
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail4_array
                .push({
                    row.bandavail4
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail5_array
                .push({
                    row.bandavail5
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail6_array
                .push({
                    row.bandavail6
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail7_array
                .push({
                    row.bandavail7
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail8_array
                .push({
                    row.bandavail8
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail9_array
                .push({
                    row.bandavail9
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail10_array
                .push({
                    row.bandavail10
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            pasaavailability_array
                .push({
                    row.pasaavailability
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            mr_capacity_array
                .push({
                    row.mr_capacity
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(bidtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(offerdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxavail_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fixedload_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rocup_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rocdown_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enablementmin_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enablementmax_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowbreakpoint_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(highbreakpoint_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail1_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail2_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail3_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail4_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail5_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail6_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail7_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail8_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail9_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail10_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(pasaavailability_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mr_capacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHCASESOLUTION_BNC
///  _DISPATCHCASESOLUTION_BNC was discontinued on 30 September 2009. Prior: DISPATCHCASESOLUTION_BNC is the key data to indicate when a binding intra-regional network constraints (BNC) re-run actually occurred._
///
/// * Data Set Name: Dispatchbnc
/// * File Name: Casesolution
/// * Data Version: 1
///
/// # Description
///  DISPATCHCASESOLUTION_BNC was discontinued on 30 September 2009. In accordance with the "Arrangements for Managing Risks Associated with Transmission Network Congestion" set of rule changes the Dispatch Binding Network Constraints re-run was discontinued on September 30, 2009. Source The occurrences of Over-constrained dispatch (OCD) or binding intra-regional network constraints (BNC)  re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Potentially updated every 5 minutes. Volume Rows per day: ~72 Mb per month: 25% of DISPATCHCASESOLUTION The estimates on the number of rows are based on a 25% occurrence rate for BNC runs. Note Due to the close dependency with the dispatch process, the OCD and BNC data models use a “CaseSolution” key table in the same manner as DISPATCHCASESOLUTION.
///
///
///
/// # Primary Key Columns
///
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchbncCasesolution1 {
    /// End date and time of the dispatch interval
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Manual intervention flag
    pub intervention: rust_decimal::Decimal,
    /// always BNC
    pub casesubtype: Option<String>,
    /// If non-zero indicated one of the following conditions:<br>* 1 = Supply Scarcity, Excess generation or constraint violations<br>* X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: Option<rust_decimal::Decimal>,
    /// Period identifier of first interval of the case (yyyymmddppp)
    pub startperiod: Option<String>,
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
    /// &nbsp;
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Total of ancillary service trader profile violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Total of fast start trader profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for DispatchbncCasesolution1 {
    type PrimaryKey = DispatchbncCasesolution1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "DISPATCHBNC".into(),
            table_name: Some("CASESOLUTION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> DispatchbncCasesolution1PrimaryKey {
        DispatchbncCasesolution1PrimaryKey {
            intervention: self.intervention,
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "dispatchbnc_casesolution_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DispatchbncCasesolution1PrimaryKey {
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchbncCasesolution1PrimaryKey {}
impl mmsdm_core::CompareWithRow for DispatchbncCasesolution1 {
    type Row = DispatchbncCasesolution1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.intervention == row.intervention && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchbncCasesolution1 {
    type PrimaryKey = DispatchbncCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.intervention == key.intervention && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for DispatchbncCasesolution1PrimaryKey {
    type Row = DispatchbncCasesolution1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("intervention",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("casesubtype",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("solutionstatus",
                arrow2::datatypes::DataType::Decimal(2, 0), true),
                arrow2::datatypes::Field::new("spdversion",
                arrow2::datatypes::DataType::Decimal(10, 3), true),
                arrow2::datatypes::Field::new("startperiod",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("nonphysicallosses",
                arrow2::datatypes::DataType::Decimal(1, 0), true),
                arrow2::datatypes::Field::new("totalobjective",
                arrow2::datatypes::DataType::Decimal(27, 10), true),
                arrow2::datatypes::Field::new("totalareagenviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalinterconnectorviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalgenericviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalramprateviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalunitmwcapacityviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("total5minviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalregviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("total6secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("total60secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalenergyconstrviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalenergyofferviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalasprofileviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalfaststartviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut casesubtype_array = Vec::new();
        let mut solutionstatus_array = Vec::new();
        let mut spdversion_array = Vec::new();
        let mut startperiod_array = Vec::new();
        let mut nonphysicallosses_array = Vec::new();
        let mut totalobjective_array = Vec::new();
        let mut totalareagenviolation_array = Vec::new();
        let mut totalinterconnectorviolation_array = Vec::new();
        let mut totalgenericviolation_array = Vec::new();
        let mut totalramprateviolation_array = Vec::new();
        let mut totalunitmwcapacityviolation_array = Vec::new();
        let mut total5minviolation_array = Vec::new();
        let mut totalregviolation_array = Vec::new();
        let mut total6secviolation_array = Vec::new();
        let mut total60secviolation_array = Vec::new();
        let mut totalenergyconstrviolation_array = Vec::new();
        let mut totalenergyofferviolation_array = Vec::new();
        let mut totalasprofileviolation_array = Vec::new();
        let mut totalfaststartviolation_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    let mut val = row.runno;
                    val.rescale(0);
                    val.mantissa()
                });
            intervention_array
                .push({
                    let mut val = row.intervention;
                    val.rescale(0);
                    val.mantissa()
                });
            casesubtype_array.push(row.casesubtype);
            solutionstatus_array
                .push({
                    row.solutionstatus
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            spdversion_array
                .push({
                    row.spdversion
                        .map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                });
            startperiod_array.push(row.startperiod);
            nonphysicallosses_array
                .push({
                    row.nonphysicallosses
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            totalobjective_array
                .push({
                    row.totalobjective
                        .map(|mut val| {
                            val.rescale(10);
                            val.mantissa()
                        })
                });
            totalareagenviolation_array
                .push({
                    row.totalareagenviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalinterconnectorviolation_array
                .push({
                    row.totalinterconnectorviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalgenericviolation_array
                .push({
                    row.totalgenericviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalramprateviolation_array
                .push({
                    row.totalramprateviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalunitmwcapacityviolation_array
                .push({
                    row.totalunitmwcapacityviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            total5minviolation_array
                .push({
                    row.total5minviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalregviolation_array
                .push({
                    row.totalregviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            total6secviolation_array
                .push({
                    row.total6secviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            total60secviolation_array
                .push({
                    row.total60secviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalenergyconstrviolation_array
                .push({
                    row.totalenergyconstrviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalenergyofferviolation_array
                .push({
                    row.totalenergyofferviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalasprofileviolation_array
                .push({
                    row.totalasprofileviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalfaststartviolation_array
                .push({
                    row.totalfaststartviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(intervention_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(casesubtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(solutionstatus_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(spdversion_array)
                    .to(arrow2::datatypes::DataType::Decimal(10, 3))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(startperiod_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nonphysicallosses_array)
                    .to(arrow2::datatypes::DataType::Decimal(1, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalobjective_array)
                    .to(arrow2::datatypes::DataType::Decimal(27, 10))) as std::sync::Arc
                    < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalareagenviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalinterconnectorviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalgenericviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalramprateviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalunitmwcapacityviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(total5minviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalregviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(total6secviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(total60secviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalenergyconstrviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalenergyofferviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalasprofileviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalfaststartviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHCASE_OCD
///  _DISPATCHCASE_OCD shows the key data to indicate when an over-constrained dispatch (OCD) re-run actually occurred. One record per over-constrained dispatch interval._
///
/// * Data Set Name: Dispatchocd
/// * File Name: Case
/// * Data Version: 1
///
/// # Description
///  The DISPATCHCASE_OCD data is public. Source The occurrences of Over-constrained dispatch (OCD) or binding intra-regional network constraints (BNC)  re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Potentially updated every 5 minutes. Volume Rows per day: ~2 Mb per month: &lt;1 The estimates on the number of rows are based on a 1% occurrence rate for OCD runs. Note Due to the close dependency with the dispatch process, the OCD and BNC data models use a “CaseSolution” key table in the same manner as the DISPATCHCASESOLUTION table.
///
///
///
/// # Primary Key Columns
///
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchocdCase1 {
    /// End date and time of the dispatch interval
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for DispatchocdCase1 {
    type PrimaryKey = DispatchocdCase1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "DISPATCHOCD".into(),
            table_name: Some("CASE".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> DispatchocdCase1PrimaryKey {
        DispatchocdCase1PrimaryKey {
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "dispatchocd_case_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DispatchocdCase1PrimaryKey {
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchocdCase1PrimaryKey {}
impl mmsdm_core::CompareWithRow for DispatchocdCase1 {
    type Row = DispatchocdCase1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchocdCase1 {
    type PrimaryKey = DispatchocdCase1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for DispatchocdCase1PrimaryKey {
    type Row = DispatchocdCase1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    let mut val = row.runno;
                    val.rescale(0);
                    val.mantissa()
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHLOAD_BNC
///  _DISPATCHLOAD_BNC was discontinued on 30 September 2009. Prior: DISPATCHLOAD_BNC gives binding intra-regional network constraints (BNC) re-run dispatch results for all scheduled generating units. DISPATCHLOAD_BNC has a similar structure to DISPATCHLOAD but does not repeat input type data (e.g. InitialMW, AGCStatus) since these values are available from DISPATCHLOAD._
///
/// * Data Set Name: Dispatchbnc
/// * File Name: Unitsolution
/// * Data Version: 1
///
/// # Description
///  DISPATCHLOAD_BNC was discontinued on 30 September 2009. In accordance with the "Arrangements for Managing Risks Associated with Transmission Network Congestion" set of rule changes the Dispatch Binding Network Constraints re-run was discontinued on September 30, 2009. Source The occurrences of Over-constrained dispatch (OCD) or binding intra-regional network constraints (BNC)  re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Potentially updated every 5 minutes. DISPATCHLOAD_BNC shows data produced every 5 minutes for all units when they have over-constrained dispatch, with own data being confidential until the next day. Volume Rows per day: ~14000 Mb per month: 25% of DISPATCHLOAD The basis of estimation on the number of rows is on a 25% occurrence rate for BNC runs Note A flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined as follows: Flag Name Bit Description Field value Enabled 0 The unit is enabled to provide this ancillary service type. &gt;1 Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. 3 Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. 4
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchbncUnitsolution1 {
    /// End date and time of the dispatch interval
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Intervention flag if intervention run
    pub intervention: rust_decimal::Decimal,
    /// Connection point identifier for DUID
    pub connectionpointid: Option<String>,
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for DispatchbncUnitsolution1 {
    type PrimaryKey = DispatchbncUnitsolution1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "DISPATCHBNC".into(),
            table_name: Some("UNITSOLUTION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> DispatchbncUnitsolution1PrimaryKey {
        DispatchbncUnitsolution1PrimaryKey {
            duid: self.duid.clone(),
            intervention: self.intervention,
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "dispatchbnc_unitsolution_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DispatchbncUnitsolution1PrimaryKey {
    pub duid: String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for DispatchbncUnitsolution1PrimaryKey {}
impl mmsdm_core::CompareWithRow for DispatchbncUnitsolution1 {
    type Row = DispatchbncUnitsolution1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.intervention == row.intervention
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for DispatchbncUnitsolution1 {
    type PrimaryKey = DispatchbncUnitsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.intervention == key.intervention
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for DispatchbncUnitsolution1PrimaryKey {
    type Row = DispatchbncUnitsolution1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.intervention == row.intervention
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("intervention",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("dispatchmode",
                arrow2::datatypes::DataType::Decimal(2, 0), true),
                arrow2::datatypes::Field::new("totalcleared",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raisereg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerreg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raiseregflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("raise5minflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("raise60secflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("raise6secflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("lowerregflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("lower5minflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("lower60secflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("lower6secflags",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut dispatchmode_array = Vec::new();
        let mut totalcleared_array = Vec::new();
        let mut raisereg_array = Vec::new();
        let mut raise5min_array = Vec::new();
        let mut raise60sec_array = Vec::new();
        let mut raise6sec_array = Vec::new();
        let mut lowerreg_array = Vec::new();
        let mut lower5min_array = Vec::new();
        let mut lower60sec_array = Vec::new();
        let mut lower6sec_array = Vec::new();
        let mut raiseregflags_array = Vec::new();
        let mut raise5minflags_array = Vec::new();
        let mut raise60secflags_array = Vec::new();
        let mut raise6secflags_array = Vec::new();
        let mut lowerregflags_array = Vec::new();
        let mut lower5minflags_array = Vec::new();
        let mut lower60secflags_array = Vec::new();
        let mut lower6secflags_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    let mut val = row.runno;
                    val.rescale(0);
                    val.mantissa()
                });
            duid_array.push(row.duid);
            intervention_array
                .push({
                    let mut val = row.intervention;
                    val.rescale(0);
                    val.mantissa()
                });
            connectionpointid_array.push(row.connectionpointid);
            dispatchmode_array
                .push({
                    row.dispatchmode
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            totalcleared_array
                .push({
                    row.totalcleared
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raisereg_array
                .push({
                    row.raisereg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5min_array
                .push({
                    row.raise5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60sec_array
                .push({
                    row.raise60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6sec_array
                .push({
                    row.raise6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerreg_array
                .push({
                    row.lowerreg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5min_array
                .push({
                    row.lower5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60sec_array
                .push({
                    row.lower60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6sec_array
                .push({
                    row.lower6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raiseregflags_array
                .push({
                    row.raiseregflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            raise5minflags_array
                .push({
                    row.raise5minflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            raise60secflags_array
                .push({
                    row.raise60secflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            raise6secflags_array
                .push({
                    row.raise6secflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lowerregflags_array
                .push({
                    row.lowerregflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lower5minflags_array
                .push({
                    row.lower5minflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lower60secflags_array
                .push({
                    row.lower60secflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lower6secflags_array
                .push({
                    row.lower6secflags
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(intervention_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(dispatchmode_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalcleared_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raiseregflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerregflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secflags_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## METERDATA
///  _METERDATA sets out a meter data for each customer connection point. METERDATA covers market load. Use the field METERRUNNO to match the meter data version for each settlement run._
///
/// * Data Set Name: Meter Data
/// * File Name: Customer
/// * Data Version: 1
///
/// # Description
///  METERDATA data is confidential to the relevant participant. Source METERDATA updates whenever meter files are processed from MSATS. Volume Depends on number of TNI, FRMP, LR combinations and number of data file loads (versions) from MSATS per settlement run.
///
///
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterDataCustomer1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Settlement period identifier (half hour period)
    pub periodid: rust_decimal::Decimal,
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Data version no
    pub meterrunno: rust_decimal::Decimal,
    /// Transmission Node Identifier (TNI).  Identifies a Transmission NetworkConnection Point.
    pub connectionpointid: String,
    /// Imported energy value (MWh)
    pub importenergyvalue: Option<rust_decimal::Decimal>,
    /// Exported energy value (MWh)
    pub exportenergyvalue: Option<rust_decimal::Decimal>,
    /// Not used
    pub importreactivevalue: Option<rust_decimal::Decimal>,
    /// Not used
    pub exportreactivevalue: Option<rust_decimal::Decimal>,
    /// Local Retailer participant identifier
    pub hostdistributor: String,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Defaults to MSATS
    pub mda: String,
}
impl mmsdm_core::GetTable for MeterDataCustomer1 {
    type PrimaryKey = MeterDataCustomer1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METER_DATA".into(),
            table_name: Some("CUSTOMER".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterDataCustomer1PrimaryKey {
        MeterDataCustomer1PrimaryKey {
            connectionpointid: self.connectionpointid.clone(),
            hostdistributor: self.hostdistributor.clone(),
            mda: self.mda.clone(),
            meterrunno: self.meterrunno,
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "meter_data_customer_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterDataCustomer1PrimaryKey {
    pub connectionpointid: String,
    pub hostdistributor: String,
    pub mda: String,
    pub meterrunno: rust_decimal::Decimal,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterDataCustomer1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterDataCustomer1 {
    type Row = MeterDataCustomer1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.hostdistributor == row.hostdistributor && self.mda == row.mda
            && self.meterrunno == row.meterrunno
            && self.participantid == row.participantid && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterDataCustomer1 {
    type PrimaryKey = MeterDataCustomer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.hostdistributor == key.hostdistributor && self.mda == key.mda
            && self.meterrunno == key.meterrunno
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for MeterDataCustomer1PrimaryKey {
    type Row = MeterDataCustomer1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.hostdistributor == row.hostdistributor && self.mda == row.mda
            && self.meterrunno == row.meterrunno
            && self.participantid == row.participantid && self.periodid == row.periodid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("meterrunno",
                arrow2::datatypes::DataType::Decimal(6, 0), false),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("importenergyvalue",
                arrow2::datatypes::DataType::Decimal(9, 6), true),
                arrow2::datatypes::Field::new("exportenergyvalue",
                arrow2::datatypes::DataType::Decimal(9, 6), true),
                arrow2::datatypes::Field::new("importreactivevalue",
                arrow2::datatypes::DataType::Decimal(9, 6), true),
                arrow2::datatypes::Field::new("exportreactivevalue",
                arrow2::datatypes::DataType::Decimal(9, 6), true),
                arrow2::datatypes::Field::new("hostdistributor",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("mda",
                arrow2::datatypes::DataType::LargeUtf8, false)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut participantid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut meterrunno_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut importenergyvalue_array = Vec::new();
        let mut exportenergyvalue_array = Vec::new();
        let mut importreactivevalue_array = Vec::new();
        let mut exportreactivevalue_array = Vec::new();
        let mut hostdistributor_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut mda_array = Vec::new();
        for row in partition {
            participantid_array.push(row.participantid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            meterrunno_array
                .push({
                    let mut val = row.meterrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            connectionpointid_array.push(row.connectionpointid);
            importenergyvalue_array
                .push({
                    row.importenergyvalue
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            exportenergyvalue_array
                .push({
                    row.exportenergyvalue
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            importreactivevalue_array
                .push({
                    row.importreactivevalue
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            exportreactivevalue_array
                .push({
                    row.exportreactivevalue
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            hostdistributor_array.push(row.hostdistributor);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            mda_array.push(row.mda);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(meterrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(importenergyvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(9, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(exportenergyvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(9, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(importreactivevalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(9, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(exportreactivevalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(9, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(hostdistributor_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(mda_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## METERDATATRK
///  _METERDATATRK records meter data files submitted for each connection point on a daily basis. The same data is provided in METERDATA period by period (i.e. 48 records), whereas METERDATATRK shows one record per day for each file submitted for a connection point._
///
/// * Data Set Name: Meter Data
/// * File Name: Customer Trk
/// * Data Version: 1
///
/// # Description
///  METERDATATRK data is confidential to the relevant participant. Source METERDATATRK updates whenever meter files are processed. Volume Depends on the number of TNI, FRMP and LR combinations plus the number of data file loads (versions) from MSATS per settlement run.
///
///
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * HOSTDISTRIBUTOR
/// * METERINGDATAAGENT
/// * METERRUNNO
/// * PARTICIPANTID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterDataCustomerTrk1 {
    /// Settlement calendar date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Meter data version number
    pub meterrunno: rust_decimal::Decimal,
    /// Participant identifier
    pub participantid: String,
    /// Meter file name (MSATS file name)
    pub filename: Option<String>,
    /// Not used
    pub ackfilename: Option<String>,
    /// Transmission Node Identifier (TNI)
    pub connectionpointid: String,
    /// Date processed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub authorisedby: Option<String>,
    /// Defaults to MSATS
    pub meteringdataagent: String,
    /// Local retailer participant identifier
    pub hostdistributor: String,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MeterDataCustomerTrk1 {
    type PrimaryKey = MeterDataCustomerTrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METER_DATA".into(),
            table_name: Some("CUSTOMER_TRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterDataCustomerTrk1PrimaryKey {
        MeterDataCustomerTrk1PrimaryKey {
            connectionpointid: self.connectionpointid.clone(),
            hostdistributor: self.hostdistributor.clone(),
            meteringdataagent: self.meteringdataagent.clone(),
            meterrunno: self.meterrunno,
            participantid: self.participantid.clone(),
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "meter_data_customer_trk_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterDataCustomerTrk1PrimaryKey {
    pub connectionpointid: String,
    pub hostdistributor: String,
    pub meteringdataagent: String,
    pub meterrunno: rust_decimal::Decimal,
    pub participantid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterDataCustomerTrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterDataCustomerTrk1 {
    type Row = MeterDataCustomerTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.hostdistributor == row.hostdistributor
            && self.meteringdataagent == row.meteringdataagent
            && self.meterrunno == row.meterrunno
            && self.participantid == row.participantid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterDataCustomerTrk1 {
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
impl mmsdm_core::CompareWithRow for MeterDataCustomerTrk1PrimaryKey {
    type Row = MeterDataCustomerTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.hostdistributor == row.hostdistributor
            && self.meteringdataagent == row.meteringdataagent
            && self.meterrunno == row.meterrunno
            && self.participantid == row.participantid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("meterrunno",
                arrow2::datatypes::DataType::Decimal(6, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("filename",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("ackfilename",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("meteringdataagent",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("hostdistributor",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut meterrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut ackfilename_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut meteringdataagent_array = Vec::new();
        let mut hostdistributor_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            meterrunno_array
                .push({
                    let mut val = row.meterrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            filename_array.push(row.filename);
            ackfilename_array.push(row.ackfilename);
            connectionpointid_array.push(row.connectionpointid);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            meteringdataagent_array.push(row.meteringdataagent);
            hostdistributor_array.push(row.hostdistributor);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(meterrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(filename_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >, std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(ackfilename_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(meteringdataagent_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(hostdistributor_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## METERDATA_GEN_DUID
///  _Recorded actual generation of non-scheduled units where SCADA data is available._
///
/// * Data Set Name: Meter Data
/// * File Name: Gen Duid
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterDataGenDuid1 {
    /// Timestamp of the recorded interval
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Unit ID
    pub duid: String,
    /// MW reading
    pub mwh_reading: Option<rust_decimal::Decimal>,
    /// Timestamp of last record change
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MeterDataGenDuid1 {
    type PrimaryKey = MeterDataGenDuid1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METER_DATA".into(),
            table_name: Some("GEN_DUID".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterDataGenDuid1PrimaryKey {
        MeterDataGenDuid1PrimaryKey {
            duid: self.duid.clone(),
            interval_datetime: self.interval_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.interval_datetime.year(),
            month: num_traits::FromPrimitive::from_u32(self.interval_datetime.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "meter_data_gen_duid_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterDataGenDuid1PrimaryKey {
    pub duid: String,
    pub interval_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterDataGenDuid1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterDataGenDuid1 {
    type Row = MeterDataGenDuid1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.interval_datetime == row.interval_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterDataGenDuid1 {
    type PrimaryKey = MeterDataGenDuid1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
    }
}
impl mmsdm_core::CompareWithRow for MeterDataGenDuid1PrimaryKey {
    type Row = MeterDataGenDuid1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.interval_datetime == row.interval_datetime
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("interval_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("mwh_reading",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut interval_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut mwh_reading_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            interval_datetime_array.push(row.interval_datetime.timestamp());
            duid_array.push(row.duid);
            mwh_reading_array
                .push({
                    row.mwh_reading
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(interval_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mwh_reading_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## METERDATA_TRK
///  _Tracking table for the publication of wholesale settlement data associated with BILLING run_
///
/// * Data Set Name: Meterdata
/// * File Name: Trk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CASE_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataTrk1 {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    /// Timestamp of the aggregated reads being loaded for this case
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub aggregate_reads_load_datetime: Option<chrono::NaiveDateTime>,
    /// Timestamp of the non aggregated reads being loaded for this case
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub individual_reads_load_datetime: Option<chrono::NaiveDateTime>,
    /// The start date of data associated with the CASE_ID
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// The end date of data associated with the Case_ID
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Last changed date for the record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MeterdataTrk1 {
    type PrimaryKey = MeterdataTrk1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("TRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterdataTrk1PrimaryKey {
        MeterdataTrk1PrimaryKey {
            case_id: self.case_id,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "meterdata_trk_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterdataTrk1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MeterdataTrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterdataTrk1 {
    type Row = MeterdataTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataTrk1 {
    type PrimaryKey = MeterdataTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
    }
}
impl mmsdm_core::CompareWithRow for MeterdataTrk1PrimaryKey {
    type Row = MeterdataTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("case_id",
                arrow2::datatypes::DataType::Decimal(15, 0), false),
                arrow2::datatypes::Field::new("aggregate_reads_load_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true),
                arrow2::datatypes::Field::new("individual_reads_load_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("startdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("enddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut case_id_array = Vec::new();
        let mut aggregate_reads_load_datetime_array = Vec::new();
        let mut individual_reads_load_datetime_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            case_id_array
                .push({
                    let mut val = row.case_id;
                    val.rescale(0);
                    val.mantissa()
                });
            aggregate_reads_load_datetime_array
                .push(row.aggregate_reads_load_datetime.map(|val| val.timestamp()));
            individual_reads_load_datetime_array
                .push(row.individual_reads_load_datetime.map(|val| val.timestamp()));
            startdate_array.push(row.startdate.map(|val| val.timestamp()));
            enddate_array.push(row.enddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(case_id_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregate_reads_load_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(individual_reads_load_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(startdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MNSP_FILETRK
///  _MNSP_FILETRK shows all MNSPOFFERS transmitted to the MMS system._
///
/// * Data Set Name: Bid
/// * File Name: Mnsp Filetrk
/// * Data Version: 1
///
/// # Description
///  MNSP_FILETRK is confidential to the relevant participant. Source MNSP_FILETRK updates for every submitted MNSP bid. Volume 4000 per year, being one per bid containing an MNSP bid
///
///
///
/// # Primary Key Columns
///
/// * FILENAME
/// * OFFERDATE
/// * PARTICIPANTID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidMnspFiletrk1 {
    /// Market Date from which bid is active
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// The actual date and time the bid file was submitted by the participant
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Participant Identifier
    pub participantid: String,
    /// File name for default bids, bids, rebids, re-offers or meter files, as appropriate to table
    pub filename: String,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    /// Acknowledge file name for bids, rebids
    pub ackfilename: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BidMnspFiletrk1 {
    type PrimaryKey = BidMnspFiletrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("MNSP_FILETRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BidMnspFiletrk1PrimaryKey {
        BidMnspFiletrk1PrimaryKey {
            filename: self.filename.clone(),
            offerdate: self.offerdate,
            participantid: self.participantid.clone(),
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "bid_mnsp_filetrk_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidMnspFiletrk1PrimaryKey {
    pub filename: String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for BidMnspFiletrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BidMnspFiletrk1 {
    type Row = BidMnspFiletrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.filename == row.filename && self.offerdate == row.offerdate
            && self.participantid == row.participantid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidMnspFiletrk1 {
    type PrimaryKey = BidMnspFiletrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.filename == key.filename && self.offerdate == key.offerdate
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for BidMnspFiletrk1PrimaryKey {
    type Row = BidMnspFiletrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.filename == row.filename && self.offerdate == row.offerdate
            && self.participantid == row.participantid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("filename",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("status",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("ackfilename",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut status_array = Vec::new();
        let mut ackfilename_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            offerdate_array.push(row.offerdate.timestamp());
            participantid_array.push(row.participantid);
            filename_array.push(row.filename);
            status_array.push(row.status);
            ackfilename_array.push(row.ackfilename);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(offerdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(filename_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(status_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >, std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(ackfilename_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MNSP_OFFERTRK
///  _MNSP_OFFERTRK records all valid MNSPOFFERS loaded into the MMS system. The authorised date reflects the date and time of the load. MNSP_OFFERTRK is key for tracking MNSP bid submission._
///
/// * Data Set Name: Bid
/// * File Name: Mnsp Offertrk
/// * Data Version: 1
///
/// # Description
///  MNSP_OFFERTRK shows own (confidential) data updates as bids are processed. All bids are available as part of next day market data. Volume 4000 per year
///
///
///
/// # Primary Key Columns
///
/// * FILENAME
/// * OFFERDATE
/// * PARTICIPANTID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidMnspOffertrk1 {
    /// &nbsp;
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// &nbsp;
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// &nbsp;
    pub versionno: rust_decimal::Decimal,
    /// &nbsp;
    pub participantid: String,
    /// &nbsp;
    pub filename: String,
    /// &nbsp;
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub authorisedby: Option<String>,
    /// &nbsp;
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BidMnspOffertrk1 {
    type PrimaryKey = BidMnspOffertrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("MNSP_OFFERTRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BidMnspOffertrk1PrimaryKey {
        BidMnspOffertrk1PrimaryKey {
            filename: self.filename.clone(),
            offerdate: self.offerdate,
            participantid: self.participantid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "bid_mnsp_offertrk_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidMnspOffertrk1PrimaryKey {
    pub filename: String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BidMnspOffertrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BidMnspOffertrk1 {
    type Row = BidMnspOffertrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.filename == row.filename && self.offerdate == row.offerdate
            && self.participantid == row.participantid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidMnspOffertrk1 {
    type PrimaryKey = BidMnspOffertrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.filename == key.filename && self.offerdate == key.offerdate
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for BidMnspOffertrk1PrimaryKey {
    type Row = BidMnspOffertrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.filename == row.filename && self.offerdate == row.offerdate
            && self.participantid == row.participantid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("filename",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            offerdate_array.push(row.offerdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            filename_array.push(row.filename);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(offerdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(filename_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MNSP_PEROFFER
///  _MNSP_PEROFFER shows period by period availability and other period data pertaining to a specific bid and LinkID for the given Settlement Date.<br>MNSP_PEROFFER is a child to MNSP_DAYOFFER and links to MNSP_OFFERTRK.<br>_
///
/// * Data Set Name: Bid
/// * File Name: Mnsp Peroffer
/// * Data Version: 1
///
/// # Description
///  MNSP_PEROFFER shows own (confidential) data updates as bids are processed. All bids are available as part of next day market data. Volume 192, 000 per year
///
///
///
/// # Primary Key Columns
///
/// * LINKID
/// * OFFERDATE
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidMnspPeroffer1 {
    /// Market Date from which bid is active
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Offer date for bid
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    /// Trading Interval number
    pub periodid: rust_decimal::Decimal,
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Inflexibility flag and availability. Fixed unit output MW. A value of zero means no fixed load so the unit is dispatched according to bid and market (rather than zero fixed load)
    pub fixedload: Option<rust_decimal::Decimal>,
    /// Ramp rate (MW / min) in the positive direction of flow for this MNSP link for this half-hour period
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Mandatory Restriction Offer amount
    pub mr_capacity: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BidMnspPeroffer1 {
    type PrimaryKey = BidMnspPeroffer1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("MNSP_PEROFFER".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BidMnspPeroffer1PrimaryKey {
        BidMnspPeroffer1PrimaryKey {
            linkid: self.linkid.clone(),
            offerdate: self.offerdate,
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "bid_mnsp_peroffer_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidMnspPeroffer1PrimaryKey {
    pub linkid: String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BidMnspPeroffer1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BidMnspPeroffer1 {
    type Row = BidMnspPeroffer1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid && self.offerdate == row.offerdate
            && self.participantid == row.participantid && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BidMnspPeroffer1 {
    type PrimaryKey = BidMnspPeroffer1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid && self.offerdate == key.offerdate
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for BidMnspPeroffer1PrimaryKey {
    type Row = BidMnspPeroffer1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid && self.offerdate == row.offerdate
            && self.participantid == row.participantid && self.periodid == row.periodid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("linkid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(22, 0), false),
                arrow2::datatypes::Field::new("maxavail",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail1",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail2",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail3",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail4",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail5",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail6",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail7",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail8",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail9",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("bandavail10",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("fixedload",
                arrow2::datatypes::DataType::Decimal(12, 6), true),
                arrow2::datatypes::Field::new("rampuprate",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("pasaavailability",
                arrow2::datatypes::DataType::Decimal(12, 0), true),
                arrow2::datatypes::Field::new("mr_capacity",
                arrow2::datatypes::DataType::Decimal(6, 0), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut linkid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut maxavail_array = Vec::new();
        let mut bandavail1_array = Vec::new();
        let mut bandavail2_array = Vec::new();
        let mut bandavail3_array = Vec::new();
        let mut bandavail4_array = Vec::new();
        let mut bandavail5_array = Vec::new();
        let mut bandavail6_array = Vec::new();
        let mut bandavail7_array = Vec::new();
        let mut bandavail8_array = Vec::new();
        let mut bandavail9_array = Vec::new();
        let mut bandavail10_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut fixedload_array = Vec::new();
        let mut rampuprate_array = Vec::new();
        let mut pasaavailability_array = Vec::new();
        let mut mr_capacity_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            offerdate_array.push(row.offerdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            linkid_array.push(row.linkid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            maxavail_array
                .push({
                    row.maxavail
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail1_array
                .push({
                    row.bandavail1
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail2_array
                .push({
                    row.bandavail2
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail3_array
                .push({
                    row.bandavail3
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail4_array
                .push({
                    row.bandavail4
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail5_array
                .push({
                    row.bandavail5
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail6_array
                .push({
                    row.bandavail6
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail7_array
                .push({
                    row.bandavail7
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail8_array
                .push({
                    row.bandavail8
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail9_array
                .push({
                    row.bandavail9
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bandavail10_array
                .push({
                    row.bandavail10
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            fixedload_array
                .push({
                    row.fixedload
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            rampuprate_array
                .push({
                    row.rampuprate
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            pasaavailability_array
                .push({
                    row.pasaavailability
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            mr_capacity_array
                .push({
                    row.mr_capacity
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(offerdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(linkid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxavail_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail1_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail2_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail3_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail4_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail5_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail6_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail7_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail8_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail9_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(bandavail10_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fixedload_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rampuprate_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(pasaavailability_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mr_capacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MR_DAYOFFER_STACK
///  _MR_DAYOFFER_STACK defines the Stack order for each version of the Acceptance Schedule, including all units submitting MR offers for that event. MR_DAYOFFER_STACK is the child to MR_EVENT_SCHEDULE, and parent to MR_PEROFFER_STACK._
///
/// * Data Set Name: Mr
/// * File Name: Dayoffer Stack
/// * Data Version: 1
///
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_EVENT_SCHEDULE, MR_DAYOFFER_STACK and MR_PEROFFER_STACK. MR_DAYOFFER_STACK sets includes all generators/MNSPs in the region that submitted an MR offer and a primary key reference to the Offer tables to identify the specific offer used for that unit. MR_DAYOFFER_STACK also includes a Stack Order, irrespective of whether the unit is required to meet the Schedule. MR_DAYOFFER_STACK updates are confidential on day of submission, with public exposure the next day. Source MR_DAYOFFER_STACK updates are ad hoc. Volume 100 rows per year
///
///
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrDayofferStack1 {
    /// Mandatory Restriction imposition date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    /// Allows many Stack versions
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Loss Adjusted Offer Factor Stack order starting at 1
    pub stack_position: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID
    pub duid: Option<String>,
    /// Confirms the unit is allowed to Contribute MR Capacity
    pub authorised: Option<rust_decimal::Decimal>,
    /// Foreign key reference to XXXX_DayOffer.SettlementDate
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub offer_settlementdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to XXXX_DayOffer.OfferDate
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub offer_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to XXXX_DayOffer.VersionNo
    pub offer_versionno: Option<rust_decimal::Decimal>,
    /// Source tables - ENERGY or MNSP
    pub offer_type: Option<String>,
    /// Loss Adjusted Offer Factor = TLF times MR_Factor
    pub laof: Option<rust_decimal::Decimal>,
    /// Date and time the record was last inserted/modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MrDayofferStack1 {
    type PrimaryKey = MrDayofferStack1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("DAYOFFER_STACK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MrDayofferStack1PrimaryKey {
        MrDayofferStack1PrimaryKey {
            mr_date: self.mr_date,
            regionid: self.regionid.clone(),
            stack_position: self.stack_position,
            version_datetime: self.version_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mr_dayoffer_stack_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MrDayofferStack1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: String,
    pub stack_position: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MrDayofferStack1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MrDayofferStack1 {
    type Row = MrDayofferStack1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrDayofferStack1 {
    type PrimaryKey = MrDayofferStack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
impl mmsdm_core::CompareWithRow for MrDayofferStack1PrimaryKey {
    type Row = MrDayofferStack1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("mr_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("version_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("stack_position",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authorised",
                arrow2::datatypes::DataType::Decimal(1, 0), true),
                arrow2::datatypes::Field::new("offer_settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("offer_offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("offer_versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("offer_type",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("laof",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut stack_position_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut authorised_array = Vec::new();
        let mut offer_settlementdate_array = Vec::new();
        let mut offer_offerdate_array = Vec::new();
        let mut offer_versionno_array = Vec::new();
        let mut offer_type_array = Vec::new();
        let mut laof_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            mr_date_array.push(row.mr_date.timestamp());
            regionid_array.push(row.regionid);
            version_datetime_array.push(row.version_datetime.timestamp());
            stack_position_array
                .push({
                    let mut val = row.stack_position;
                    val.rescale(0);
                    val.mantissa()
                });
            duid_array.push(row.duid);
            authorised_array
                .push({
                    row.authorised
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            offer_settlementdate_array
                .push(row.offer_settlementdate.map(|val| val.timestamp()));
            offer_offerdate_array.push(row.offer_offerdate.map(|val| val.timestamp()));
            offer_versionno_array
                .push({
                    row.offer_versionno
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            offer_type_array.push(row.offer_type);
            laof_array
                .push({
                    row.laof
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(mr_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(stack_position_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(duid_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authorised_array)
                    .to(arrow2::datatypes::DataType::Decimal(1, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offer_settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offer_offerdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offer_versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(offer_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(laof_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MR_EVENT
///  _MR_EVENT defines an MR Event for a given region on a specific trading date.<br>_
///
/// * Data Set Name: Mr
/// * File Name: Event
/// * Data Version: 1
///
/// # Description
///  MR_EVENT defines a mandatory restriction event for a given region and trading date (04:30 to 04:00). Data within MR_EVENT includes the cut-off time for submission of MR offers for this event and a notification that the settlements figures are locked due to results from an independent expert being engaged to allocate settlement of a significant shortfall. If mandatory restrictions are defined in two regions on the same trading day, two MR events are defined. MR_EVENT data is public, so is available to all participants. Source MR_EVENT updates are ad hoc. Volume 1 Row per year
///
///
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEvent1 {
    /// Mandatory Restriction imposition date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    /// Description of MR
    pub description: Option<String>,
    /// Required for MR_Event to take effect
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Ignored - Tracking purpose only
    pub authorisedby: Option<String>,
    /// Cut off after when new Offers and Scaling Factor changes are disallowed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub offer_cut_off_time: Option<chrono::NaiveDateTime>,
    /// Flag:1 = MR settlement figures locked. Do not recalculate, ·&nbsp;&nbsp;&nbsp;0 = MR settlements to be recalculated
    pub settlement_complete: Option<rust_decimal::Decimal>,
    /// Date/Time record inserted/modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MrEvent1 {
    type PrimaryKey = MrEvent1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("EVENT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MrEvent1PrimaryKey {
        MrEvent1PrimaryKey {
            mr_date: self.mr_date,
            regionid: self.regionid.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mr_event_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MrEvent1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: String,
}
impl mmsdm_core::PrimaryKey for MrEvent1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MrEvent1 {
    type Row = MrEvent1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrEvent1 {
    type PrimaryKey = MrEvent1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
    }
}
impl mmsdm_core::CompareWithRow for MrEvent1PrimaryKey {
    type Row = MrEvent1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("mr_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("description",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("offer_cut_off_time",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("settlement_complete",
                arrow2::datatypes::DataType::Decimal(1, 0), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut offer_cut_off_time_array = Vec::new();
        let mut settlement_complete_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            mr_date_array.push(row.mr_date.timestamp());
            regionid_array.push(row.regionid);
            description_array.push(row.description);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            offer_cut_off_time_array
                .push(row.offer_cut_off_time.map(|val| val.timestamp()));
            settlement_complete_array
                .push({
                    row.settlement_complete
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(mr_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(description_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offer_cut_off_time_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(settlement_complete_array)
                    .to(arrow2::datatypes::DataType::Decimal(1, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MR_EVENT_SCHEDULE
///  _MR_EVENT_SCHEDULE defines the Stack version of the Acceptance Schedule and is the parent table to MR_DayOffer_Stack and MR_PerOffer_Stack.<br>_
///
/// * Data Set Name: Mr
/// * File Name: Event Schedule
/// * Data Version: 1
///
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack table. The MR_EVENT_SCHEDULE table determines the existence of an MR offer acceptance stack for a specific MR schedule of an MR event. The MR_EVENT_SCHEDULE table also tracks the time each stack is exercised. MR_EVENT_SCHEDULE is public and notifies the market that a new offer stack has been created. Source MR_EVENT_SCHEDULE updates are ad hoc. Volume 2 Rows per year
///
///
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEventSchedule1 {
    /// Mandatory Restriction imposition date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    /// Effective Date/Time of Schedule; Allows many Stack versions
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Foreign key reference to ResDemandTrk.EffectiveDate
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub demand_effectivedate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to ResDemandTrk.OfferDate
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub demand_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to ResDemandTrk.VersionNo
    pub demand_versionno: Option<rust_decimal::Decimal>,
    /// Authorised person confirming Offer Stack (AKA Acceptance)
    pub authorisedby: Option<String>,
    /// Date and time the Offer Stack confirmed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Date and time the record was inserted/modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MrEventSchedule1 {
    type PrimaryKey = MrEventSchedule1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("EVENT_SCHEDULE".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MrEventSchedule1PrimaryKey {
        MrEventSchedule1PrimaryKey {
            mr_date: self.mr_date,
            regionid: self.regionid.clone(),
            version_datetime: self.version_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mr_event_schedule_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MrEventSchedule1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MrEventSchedule1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MrEventSchedule1 {
    type Row = MrEventSchedule1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrEventSchedule1 {
    type PrimaryKey = MrEventSchedule1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
impl mmsdm_core::CompareWithRow for MrEventSchedule1PrimaryKey {
    type Row = MrEventSchedule1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("mr_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("version_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("demand_effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("demand_offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("demand_versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut demand_effectivedate_array = Vec::new();
        let mut demand_offerdate_array = Vec::new();
        let mut demand_versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            mr_date_array.push(row.mr_date.timestamp());
            regionid_array.push(row.regionid);
            version_datetime_array.push(row.version_datetime.timestamp());
            demand_effectivedate_array
                .push(row.demand_effectivedate.map(|val| val.timestamp()));
            demand_offerdate_array.push(row.demand_offerdate.map(|val| val.timestamp()));
            demand_versionno_array
                .push({
                    row.demand_versionno
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(mr_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand_effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand_offerdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand_versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MR_PEROFFER_STACK
///  _MR_PEROFFER_STACK defines the accepted capacity on a period basis for the Acceptance Schedule, is a child table to MR_DayOffer_Stack and only includes records or units with accepted_capacity &gt; 0 for the specific period.<br>_
///
/// * Data Set Name: Mr
/// * File Name: Peroffer Stack
/// * Data Version: 1
///
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack. MR_PEROFFER_STACK reports the accepted MR capacity (Accepted_Capacity) required from each unit for each trading interval. MR_PEROFFER_STACK is sparse so lists only units with accepted capacity &gt; 0 for that trading interval.  The Deducted_Capacity field allows the tracking and implementation of participant requested reductions to accepted MR capacity to be tracked and applied. MR_PEROFFER_STACK is reported confidentially to each participant to notify acceptance of an MR offer. Source MR_PEROFFER_STACK updates are ad hoc. Volume 4800 rows per year
///
///
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * PERIODID
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrPerofferStack1 {
    /// Mandatory Restriction imposition date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    /// Allows many Period Stack versions for the one Scaling Factor stack
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// LAOF Stack order
    pub stack_position: rust_decimal::Decimal,
    /// Trade Period for the MR Offer
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID. Only required here for CSV reports
    pub duid: Option<String>,
    /// MR Capacity to be Dispatched
    pub accepted_capacity: Option<rust_decimal::Decimal>,
    /// Requested capacity reduction amount
    pub deducted_capacity: Option<rust_decimal::Decimal>,
    /// Date and time the record was last  inserted/modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MrPerofferStack1 {
    type PrimaryKey = MrPerofferStack1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("PEROFFER_STACK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MrPerofferStack1PrimaryKey {
        MrPerofferStack1PrimaryKey {
            mr_date: self.mr_date,
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            stack_position: self.stack_position,
            version_datetime: self.version_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mr_peroffer_stack_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MrPerofferStack1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub stack_position: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MrPerofferStack1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MrPerofferStack1 {
    type Row = MrPerofferStack1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.periodid == row.periodid
            && self.regionid == row.regionid && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MrPerofferStack1 {
    type PrimaryKey = MrPerofferStack1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.periodid == key.periodid
            && self.regionid == key.regionid && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
impl mmsdm_core::CompareWithRow for MrPerofferStack1PrimaryKey {
    type Row = MrPerofferStack1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.periodid == row.periodid
            && self.regionid == row.regionid && self.stack_position == row.stack_position
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("mr_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("version_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("stack_position",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("accepted_capacity",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("deducted_capacity",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut stack_position_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut accepted_capacity_array = Vec::new();
        let mut deducted_capacity_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            mr_date_array.push(row.mr_date.timestamp());
            regionid_array.push(row.regionid);
            version_datetime_array.push(row.version_datetime.timestamp());
            stack_position_array
                .push({
                    let mut val = row.stack_position;
                    val.rescale(0);
                    val.mantissa()
                });
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            duid_array.push(row.duid);
            accepted_capacity_array
                .push({
                    row.accepted_capacity
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            deducted_capacity_array
                .push({
                    row.deducted_capacity
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(mr_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(stack_position_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(duid_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(accepted_capacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(deducted_capacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETFCASCOMP
///  _SETFCASCOMP shows the compensation details for Frequency Controlled Ancillary Services (FCAS). These compensation values are calculated by a separate “what if” run of the LP Solver and entered as an unconstrained MW value into settlements._
///
/// * Data Set Name: Settlements
/// * File Name: Fcascomp
/// * Data Version: 5
///
/// # Description
///  SETFCASCOMP data is confidential to the relevant participant Source SETFCASCOMP updates with each Settlement run, if required.
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcascomp5 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Dispatchable Unit ID
    pub duid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Period Identifier
    pub periodid: rust_decimal::Decimal,
    /// Compensation Cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Cleared MW of Unit in First Dispatch period in Trading Interval
    pub clearedmw: Option<rust_decimal::Decimal>,
    /// Initial MW of Unit in First Dispatch period in Trading Interval
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    pub ebp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor of Unit
    pub tlf: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Generation Payment in trading interval
    pub excessgen: Option<rust_decimal::Decimal>,
    /// Frequency Control AS Compensation payment to Generator
    pub fcascomp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsFcascomp5 {
    type PrimaryKey = SettlementsFcascomp5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("FCASCOMP".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> SettlementsFcascomp5PrimaryKey {
        SettlementsFcascomp5PrimaryKey {
            duid: self.duid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "settlements_fcascomp_v5_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsFcascomp5PrimaryKey {
    pub duid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsFcascomp5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsFcascomp5 {
    type Row = SettlementsFcascomp5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.participantid == row.participantid
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcascomp5 {
    type PrimaryKey = SettlementsFcascomp5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsFcascomp5PrimaryKey {
    type Row = SettlementsFcascomp5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.participantid == row.participantid
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcascomp5PrimaryKey {
    type PrimaryKey = SettlementsFcascomp5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsFcascomp5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("ccprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("clearedmw",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("unconstrainedmw",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("ebp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("tlf",
                arrow2::datatypes::DataType::Decimal(7, 5), true),
                arrow2::datatypes::Field::new("rrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("excessgen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("fcascomp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut ccprice_array = Vec::new();
        let mut clearedmw_array = Vec::new();
        let mut unconstrainedmw_array = Vec::new();
        let mut ebp_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut excessgen_array = Vec::new();
        let mut fcascomp_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            duid_array.push(row.duid);
            regionid_array.push(row.regionid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            ccprice_array
                .push({
                    row.ccprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            clearedmw_array
                .push({
                    row.clearedmw
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            unconstrainedmw_array
                .push({
                    row.unconstrainedmw
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            ebp_array
                .push({
                    row.ebp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            tlf_array
                .push({
                    row.tlf
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rrp_array
                .push({
                    row.rrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            excessgen_array
                .push({
                    row.excessgen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            fcascomp_array
                .push({
                    row.fcascomp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ccprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(clearedmw_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unconstrainedmw_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ebp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(tlf_array)
                    .to(arrow2::datatypes::DataType::Decimal(7, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(excessgen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fcascomp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETINTERVENTION
///  _SETINTERVENTION shows intervention settlement payment details by unit._
///
/// * Data Set Name: Settlements
/// * File Name: Intervention
/// * Data Version: 5
///
/// # Description
///  SETINTERVENTION became unused when Ancillary Services Review was implemented. For more details, see Change Notice 126. SETINTERVENTION data is confidential to each participant. Source SETINTERVENTION is unused; was updating when intervention occurred in a billing run.
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIntervention5 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    pub periodid: rust_decimal::Decimal,
    /// Intervention Contract Identifier
    pub contractid: Option<String>,
    /// Intervention Contract Version
    pub contractversion: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: String,
    /// Regional Recovery Flag
    pub rcf: Option<char>,
    /// Payment to Generator for Intervention
    pub interventionpayment: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsIntervention5 {
    type PrimaryKey = SettlementsIntervention5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("INTERVENTION".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> SettlementsIntervention5PrimaryKey {
        SettlementsIntervention5PrimaryKey {
            duid: self.duid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "settlements_intervention_v5_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsIntervention5PrimaryKey {
    pub duid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIntervention5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsIntervention5 {
    type Row = SettlementsIntervention5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIntervention5 {
    type PrimaryKey = SettlementsIntervention5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsIntervention5PrimaryKey {
    type Row = SettlementsIntervention5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIntervention5PrimaryKey {
    type PrimaryKey = SettlementsIntervention5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIntervention5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("contractversion",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("rcf",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("interventionpayment",
                arrow2::datatypes::DataType::Decimal(12, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut contractversion_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut rcf_array = Vec::new();
        let mut interventionpayment_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            contractid_array.push(row.contractid);
            contractversion_array
                .push({
                    row.contractversion
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            duid_array.push(row.duid);
            rcf_array.push(row.rcf.map(|val| val.to_string()));
            interventionpayment_array
                .push({
                    row.interventionpayment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(contractversion_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >, std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(rcf_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interventionpayment_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETINTERVENTIONRECOVERY
///  _SETINTERVENTIONRECOVERY shows intervention recovery details by participant._
///
/// * Data Set Name: Settlements
/// * File Name: Interventionrecovery
/// * Data Version: 5
///
/// # Description
///  Status SETINTERVENTIONRECOVERY became unused when Ancillary Services Review was implemented. For more details, see Change Notice 126. Confidential to participant Source Unused; was updating when intervention occurred in a billing run.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsInterventionrecovery5 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    pub periodid: rust_decimal::Decimal,
    /// Intervention Contract Identifier
    pub contractid: String,
    /// Regional Recovery Flag
    pub rcf: Option<char>,
    /// Unique participant identifier
    pub participantid: String,
    /// Demand of Participant in Region/Market
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total Demand of Region/Market
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Payment to Generator for Intervention
    pub interventionpayment: Option<rust_decimal::Decimal>,
    /// Retailer Payment to Pool for Intervention
    pub interventionamount: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Region Identifier
    pub regionid: Option<String>,
}
impl mmsdm_core::GetTable for SettlementsInterventionrecovery5 {
    type PrimaryKey = SettlementsInterventionrecovery5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("INTERVENTIONRECOVERY".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> SettlementsInterventionrecovery5PrimaryKey {
        SettlementsInterventionrecovery5PrimaryKey {
            contractid: self.contractid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "settlements_interventionrecovery_v5_{}_{}", self.partition_suffix().year,
            self.partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsInterventionrecovery5PrimaryKey {
    pub contractid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsInterventionrecovery5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsInterventionrecovery5 {
    type Row = SettlementsInterventionrecovery5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.participantid == row.participantid
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsInterventionrecovery5 {
    type PrimaryKey = SettlementsInterventionrecovery5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsInterventionrecovery5PrimaryKey {
    type Row = SettlementsInterventionrecovery5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.participantid == row.participantid
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsInterventionrecovery5PrimaryKey {
    type PrimaryKey = SettlementsInterventionrecovery5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsInterventionrecovery5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("rcf",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantdemand",
                arrow2::datatypes::DataType::Decimal(12, 5), true),
                arrow2::datatypes::Field::new("totaldemand",
                arrow2::datatypes::DataType::Decimal(12, 5), true),
                arrow2::datatypes::Field::new("interventionpayment",
                arrow2::datatypes::DataType::Decimal(12, 5), true),
                arrow2::datatypes::Field::new("interventionamount",
                arrow2::datatypes::DataType::Decimal(12, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut rcf_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut participantdemand_array = Vec::new();
        let mut totaldemand_array = Vec::new();
        let mut interventionpayment_array = Vec::new();
        let mut interventionamount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut regionid_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            contractid_array.push(row.contractid);
            rcf_array.push(row.rcf.map(|val| val.to_string()));
            participantid_array.push(row.participantid);
            participantdemand_array
                .push({
                    row.participantdemand
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totaldemand_array
                .push({
                    row.totaldemand
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            interventionpayment_array
                .push({
                    row.interventionpayment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            interventionamount_array
                .push({
                    row.interventionamount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            regionid_array.push(row.regionid);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(rcf_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(participantdemand_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totaldemand_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interventionpayment_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interventionamount_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_MR_PAYMENT
///  _SET_MR_PAYMENT shows trading interval payments on a dispatchable unit basis for accepted MR capacity._
///
/// * Data Set Name: Settlements
/// * File Name: Mr Payment
/// * Data Version: 5
///
/// # Description
///  SET_MR_PAYMENT data is confidential to the relevant participant. Source SET_MR_PAYMENT updates are ad hoc, being for MR events only. Volume 24000 rows per year
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrPayment5 {
    /// Settlement Date (Calendar)
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    pub versionno: rust_decimal::Decimal,
    /// Unique Region Identifier
    pub regionid: String,
    /// Unique Participant identifier
    pub participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    pub duid: String,
    /// Calendar day Trading Interval number
    pub periodid: rust_decimal::Decimal,
    /// Accepted MR Capacity
    pub mr_capacity: Option<rust_decimal::Decimal>,
    /// Uncapped Trading Interval Payment
    pub uncapped_payment: Option<rust_decimal::Decimal>,
    /// Capped Trading Interval Payment
    pub capped_payment: Option<rust_decimal::Decimal>,
    /// Date/Time record inserted/modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsMrPayment5 {
    type PrimaryKey = SettlementsMrPayment5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("MR_PAYMENT".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> SettlementsMrPayment5PrimaryKey {
        SettlementsMrPayment5PrimaryKey {
            duid: self.duid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "settlements_mr_payment_v5_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsMrPayment5PrimaryKey {
    pub duid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsMrPayment5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsMrPayment5 {
    type Row = SettlementsMrPayment5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid
            && self.regionid == row.regionid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsMrPayment5 {
    type PrimaryKey = SettlementsMrPayment5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsMrPayment5PrimaryKey {
    type Row = SettlementsMrPayment5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid
            && self.regionid == row.regionid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsMrPayment5PrimaryKey {
    type PrimaryKey = SettlementsMrPayment5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsMrPayment5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("mr_capacity",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("uncapped_payment",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("capped_payment",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut mr_capacity_array = Vec::new();
        let mut uncapped_payment_array = Vec::new();
        let mut capped_payment_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            participantid_array.push(row.participantid);
            duid_array.push(row.duid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            mr_capacity_array
                .push({
                    row.mr_capacity
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            uncapped_payment_array
                .push({
                    row.uncapped_payment
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            capped_payment_array
                .push({
                    row.capped_payment
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mr_capacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(uncapped_payment_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capped_payment_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_MR_RECOVERY
///  _SET_MR_RECOVERY shows the trading interval recovery charges on a dispatchable unit basis for spot market income from dispatch of MR capacity._
///
/// * Data Set Name: Settlements
/// * File Name: Mr Recovery
/// * Data Version: 5
///
/// # Description
///  SET_MR_RECOVERY data is confidential to the relevant participant. Source SET_MR_RECOVERY updates are ad hoc, being for  MR events only. Volume 24000 rows per year
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrRecovery5 {
    /// Settlement Date (Calendar)
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    pub versionno: rust_decimal::Decimal,
    /// Unique Region Identifier
    pub regionid: String,
    /// Unique Participant identifier
    pub participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    pub duid: String,
    /// Calendar day Trading Interval number
    pub periodid: rust_decimal::Decimal,
    /// Accepted Restriction Offer Dispatched Energy Factor
    pub arodef: Option<rust_decimal::Decimal>,
    /// The amount payable to AEMO for that accepted restriction offer and trading interval
    pub nta: Option<rust_decimal::Decimal>,
    /// Date/Time record inserted/modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsMrRecovery5 {
    type PrimaryKey = SettlementsMrRecovery5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("MR_RECOVERY".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> SettlementsMrRecovery5PrimaryKey {
        SettlementsMrRecovery5PrimaryKey {
            duid: self.duid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "settlements_mr_recovery_v5_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsMrRecovery5PrimaryKey {
    pub duid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsMrRecovery5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsMrRecovery5 {
    type Row = SettlementsMrRecovery5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid
            && self.regionid == row.regionid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsMrRecovery5 {
    type PrimaryKey = SettlementsMrRecovery5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsMrRecovery5PrimaryKey {
    type Row = SettlementsMrRecovery5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid
            && self.regionid == row.regionid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsMrRecovery5PrimaryKey {
    type PrimaryKey = SettlementsMrRecovery5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsMrRecovery5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("arodef",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("nta",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut arodef_array = Vec::new();
        let mut nta_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            participantid_array.push(row.participantid);
            duid_array.push(row.duid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            arodef_array
                .push({
                    row.arodef
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            nta_array
                .push({
                    row.nta
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(arodef_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nta_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## TRADINGLOAD
///  _TRADINGLOAD shows half-hourly average dispatch levels, including fields to handle the Ancillary Services functionality._
///
/// * Data Set Name: Trading
/// * File Name: Unit Solution
/// * Data Version: 2
///
/// # Description
///  Source Own (confidential) TRADINGLOAD data updates half hourly, with public availability of all data on next day.
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
/// * TRADETYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingUnitSolution2 {
    /// Date that this data applies to
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: String,
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    pub availability: Option<rust_decimal::Decimal>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for TradingUnitSolution2 {
    type PrimaryKey = TradingUnitSolution2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("UNIT_SOLUTION".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> TradingUnitSolution2PrimaryKey {
        TradingUnitSolution2PrimaryKey {
            duid: self.duid.clone(),
            periodid: self.periodid,
            runno: self.runno,
            settlementdate: self.settlementdate,
            tradetype: self.tradetype,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "trading_unit_solution_v2_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct TradingUnitSolution2PrimaryKey {
    pub duid: String,
    pub periodid: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub tradetype: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for TradingUnitSolution2PrimaryKey {}
impl mmsdm_core::CompareWithRow for TradingUnitSolution2 {
    type Row = TradingUnitSolution2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.tradetype == row.tradetype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingUnitSolution2 {
    type PrimaryKey = TradingUnitSolution2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.tradetype == key.tradetype
    }
}
impl mmsdm_core::CompareWithRow for TradingUnitSolution2PrimaryKey {
    type Row = TradingUnitSolution2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.periodid == row.periodid && self.runno == row.runno
            && self.settlementdate == row.settlementdate
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("tradetype",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("initialmw",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalcleared",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rampdownrate",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rampuprate",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("lowerreg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raisereg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("availability",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("semidispatchcap",
                arrow2::datatypes::DataType::Decimal(3, 0), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut tradetype_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut initialmw_array = Vec::new();
        let mut totalcleared_array = Vec::new();
        let mut rampdownrate_array = Vec::new();
        let mut rampuprate_array = Vec::new();
        let mut lower5min_array = Vec::new();
        let mut lower60sec_array = Vec::new();
        let mut lower6sec_array = Vec::new();
        let mut raise5min_array = Vec::new();
        let mut raise60sec_array = Vec::new();
        let mut raise6sec_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut lowerreg_array = Vec::new();
        let mut raisereg_array = Vec::new();
        let mut availability_array = Vec::new();
        let mut semidispatchcap_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    let mut val = row.runno;
                    val.rescale(0);
                    val.mantissa()
                });
            duid_array.push(row.duid);
            tradetype_array
                .push({
                    let mut val = row.tradetype;
                    val.rescale(0);
                    val.mantissa()
                });
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            initialmw_array
                .push({
                    row.initialmw
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalcleared_array
                .push({
                    row.totalcleared
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rampdownrate_array
                .push({
                    row.rampdownrate
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rampuprate_array
                .push({
                    row.rampuprate
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5min_array
                .push({
                    row.lower5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60sec_array
                .push({
                    row.lower60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6sec_array
                .push({
                    row.lower6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5min_array
                .push({
                    row.raise5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60sec_array
                .push({
                    row.raise60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6sec_array
                .push({
                    row.raise6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            lowerreg_array
                .push({
                    row.lowerreg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raisereg_array
                .push({
                    row.raisereg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            availability_array
                .push({
                    row.availability
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            semidispatchcap_array
                .push({
                    row.semidispatchcap
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(tradetype_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(initialmw_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalcleared_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rampdownrate_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rampuprate_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(semidispatchcap_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## TRADINGREGIONSUM
///  _TRADINGREGIONSUM sets out the half-hourly average regional demand and frequency control services. TRADINGREGIONSUM includes fields for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations._
///
/// * Data Set Name: Trading
/// * File Name: Regionsum
/// * Data Version: 4
///
/// # Description
///  TRADINGREGIONSUM is public data, and is available to all participants. Source TRADINGREGIONSUM is updated every 30 minutes.
///
///
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingRegionsum4 {
    /// Date that this data applies to
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
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
}
impl mmsdm_core::GetTable for TradingRegionsum4 {
    type PrimaryKey = TradingRegionsum4PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("REGIONSUM".into()),
            version: 4,
        }
    }
    fn primary_key(&self) -> TradingRegionsum4PrimaryKey {
        TradingRegionsum4PrimaryKey {
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "trading_regionsum_v4_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct TradingRegionsum4PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for TradingRegionsum4PrimaryKey {}
impl mmsdm_core::CompareWithRow for TradingRegionsum4 {
    type Row = TradingRegionsum4;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingRegionsum4 {
    type PrimaryKey = TradingRegionsum4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for TradingRegionsum4PrimaryKey {
    type Row = TradingRegionsum4;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("totaldemand",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("availablegeneration",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("availableload",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("demandforecast",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("dispatchablegeneration",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("dispatchableload",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("netinterchange",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("excessgeneration",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5mindispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minlocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minlocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minlocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5mindispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minlocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minlocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minlocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("initialsupply",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("clearedsupply",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerregimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerreglocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerreglocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerregreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raiseregimport",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raisereglocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raisereglocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raiseregreq",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minlocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raisereglocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minlocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerreglocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raiseregviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerregviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("totalintermittentgeneration",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("demand_and_nonschedgen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("uigf",
                arrow2::datatypes::DataType::Decimal(15, 5), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut totaldemand_array = Vec::new();
        let mut availablegeneration_array = Vec::new();
        let mut availableload_array = Vec::new();
        let mut demandforecast_array = Vec::new();
        let mut dispatchablegeneration_array = Vec::new();
        let mut dispatchableload_array = Vec::new();
        let mut netinterchange_array = Vec::new();
        let mut excessgeneration_array = Vec::new();
        let mut lower5mindispatch_array = Vec::new();
        let mut lower5minimport_array = Vec::new();
        let mut lower5minlocaldispatch_array = Vec::new();
        let mut lower5minlocalprice_array = Vec::new();
        let mut lower5minlocalreq_array = Vec::new();
        let mut lower5minprice_array = Vec::new();
        let mut lower5minreq_array = Vec::new();
        let mut lower5minsupplyprice_array = Vec::new();
        let mut lower60secdispatch_array = Vec::new();
        let mut lower60secimport_array = Vec::new();
        let mut lower60seclocaldispatch_array = Vec::new();
        let mut lower60seclocalprice_array = Vec::new();
        let mut lower60seclocalreq_array = Vec::new();
        let mut lower60secprice_array = Vec::new();
        let mut lower60secreq_array = Vec::new();
        let mut lower60secsupplyprice_array = Vec::new();
        let mut lower6secdispatch_array = Vec::new();
        let mut lower6secimport_array = Vec::new();
        let mut lower6seclocaldispatch_array = Vec::new();
        let mut lower6seclocalprice_array = Vec::new();
        let mut lower6seclocalreq_array = Vec::new();
        let mut lower6secprice_array = Vec::new();
        let mut lower6secreq_array = Vec::new();
        let mut lower6secsupplyprice_array = Vec::new();
        let mut raise5mindispatch_array = Vec::new();
        let mut raise5minimport_array = Vec::new();
        let mut raise5minlocaldispatch_array = Vec::new();
        let mut raise5minlocalprice_array = Vec::new();
        let mut raise5minlocalreq_array = Vec::new();
        let mut raise5minprice_array = Vec::new();
        let mut raise5minreq_array = Vec::new();
        let mut raise5minsupplyprice_array = Vec::new();
        let mut raise60secdispatch_array = Vec::new();
        let mut raise60secimport_array = Vec::new();
        let mut raise60seclocaldispatch_array = Vec::new();
        let mut raise60seclocalprice_array = Vec::new();
        let mut raise60seclocalreq_array = Vec::new();
        let mut raise60secprice_array = Vec::new();
        let mut raise60secreq_array = Vec::new();
        let mut raise60secsupplyprice_array = Vec::new();
        let mut raise6secdispatch_array = Vec::new();
        let mut raise6secimport_array = Vec::new();
        let mut raise6seclocaldispatch_array = Vec::new();
        let mut raise6seclocalprice_array = Vec::new();
        let mut raise6seclocalreq_array = Vec::new();
        let mut raise6secprice_array = Vec::new();
        let mut raise6secreq_array = Vec::new();
        let mut raise6secsupplyprice_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut initialsupply_array = Vec::new();
        let mut clearedsupply_array = Vec::new();
        let mut lowerregimport_array = Vec::new();
        let mut lowerreglocaldispatch_array = Vec::new();
        let mut lowerreglocalreq_array = Vec::new();
        let mut lowerregreq_array = Vec::new();
        let mut raiseregimport_array = Vec::new();
        let mut raisereglocaldispatch_array = Vec::new();
        let mut raisereglocalreq_array = Vec::new();
        let mut raiseregreq_array = Vec::new();
        let mut raise5minlocalviolation_array = Vec::new();
        let mut raisereglocalviolation_array = Vec::new();
        let mut raise60seclocalviolation_array = Vec::new();
        let mut raise6seclocalviolation_array = Vec::new();
        let mut lower5minlocalviolation_array = Vec::new();
        let mut lowerreglocalviolation_array = Vec::new();
        let mut lower60seclocalviolation_array = Vec::new();
        let mut lower6seclocalviolation_array = Vec::new();
        let mut raise5minviolation_array = Vec::new();
        let mut raiseregviolation_array = Vec::new();
        let mut raise60secviolation_array = Vec::new();
        let mut raise6secviolation_array = Vec::new();
        let mut lower5minviolation_array = Vec::new();
        let mut lowerregviolation_array = Vec::new();
        let mut lower60secviolation_array = Vec::new();
        let mut lower6secviolation_array = Vec::new();
        let mut totalintermittentgeneration_array = Vec::new();
        let mut demand_and_nonschedgen_array = Vec::new();
        let mut uigf_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    let mut val = row.runno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            totaldemand_array
                .push({
                    row.totaldemand
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            availablegeneration_array
                .push({
                    row.availablegeneration
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            availableload_array
                .push({
                    row.availableload
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            demandforecast_array
                .push({
                    row.demandforecast
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            dispatchablegeneration_array
                .push({
                    row.dispatchablegeneration
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            dispatchableload_array
                .push({
                    row.dispatchableload
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            netinterchange_array
                .push({
                    row.netinterchange
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            excessgeneration_array
                .push({
                    row.excessgeneration
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5mindispatch_array
                .push({
                    row.lower5mindispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minimport_array
                .push({
                    row.lower5minimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minlocaldispatch_array
                .push({
                    row.lower5minlocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minlocalprice_array
                .push({
                    row.lower5minlocalprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minlocalreq_array
                .push({
                    row.lower5minlocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minprice_array
                .push({
                    row.lower5minprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minreq_array
                .push({
                    row.lower5minreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minsupplyprice_array
                .push({
                    row.lower5minsupplyprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secdispatch_array
                .push({
                    row.lower60secdispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secimport_array
                .push({
                    row.lower60secimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60seclocaldispatch_array
                .push({
                    row.lower60seclocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60seclocalprice_array
                .push({
                    row.lower60seclocalprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60seclocalreq_array
                .push({
                    row.lower60seclocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secprice_array
                .push({
                    row.lower60secprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secreq_array
                .push({
                    row.lower60secreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secsupplyprice_array
                .push({
                    row.lower60secsupplyprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secdispatch_array
                .push({
                    row.lower6secdispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secimport_array
                .push({
                    row.lower6secimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6seclocaldispatch_array
                .push({
                    row.lower6seclocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6seclocalprice_array
                .push({
                    row.lower6seclocalprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6seclocalreq_array
                .push({
                    row.lower6seclocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secprice_array
                .push({
                    row.lower6secprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secreq_array
                .push({
                    row.lower6secreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secsupplyprice_array
                .push({
                    row.lower6secsupplyprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5mindispatch_array
                .push({
                    row.raise5mindispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minimport_array
                .push({
                    row.raise5minimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minlocaldispatch_array
                .push({
                    row.raise5minlocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minlocalprice_array
                .push({
                    row.raise5minlocalprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minlocalreq_array
                .push({
                    row.raise5minlocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minprice_array
                .push({
                    row.raise5minprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minreq_array
                .push({
                    row.raise5minreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minsupplyprice_array
                .push({
                    row.raise5minsupplyprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secdispatch_array
                .push({
                    row.raise60secdispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secimport_array
                .push({
                    row.raise60secimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60seclocaldispatch_array
                .push({
                    row.raise60seclocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60seclocalprice_array
                .push({
                    row.raise60seclocalprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60seclocalreq_array
                .push({
                    row.raise60seclocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secprice_array
                .push({
                    row.raise60secprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secreq_array
                .push({
                    row.raise60secreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secsupplyprice_array
                .push({
                    row.raise60secsupplyprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secdispatch_array
                .push({
                    row.raise6secdispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secimport_array
                .push({
                    row.raise6secimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6seclocaldispatch_array
                .push({
                    row.raise6seclocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6seclocalprice_array
                .push({
                    row.raise6seclocalprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6seclocalreq_array
                .push({
                    row.raise6seclocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secprice_array
                .push({
                    row.raise6secprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secreq_array
                .push({
                    row.raise6secreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secsupplyprice_array
                .push({
                    row.raise6secsupplyprice
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            initialsupply_array
                .push({
                    row.initialsupply
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            clearedsupply_array
                .push({
                    row.clearedsupply
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerregimport_array
                .push({
                    row.lowerregimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerreglocaldispatch_array
                .push({
                    row.lowerreglocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerreglocalreq_array
                .push({
                    row.lowerreglocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerregreq_array
                .push({
                    row.lowerregreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raiseregimport_array
                .push({
                    row.raiseregimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raisereglocaldispatch_array
                .push({
                    row.raisereglocaldispatch
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raisereglocalreq_array
                .push({
                    row.raisereglocalreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raiseregreq_array
                .push({
                    row.raiseregreq
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minlocalviolation_array
                .push({
                    row.raise5minlocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raisereglocalviolation_array
                .push({
                    row.raisereglocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60seclocalviolation_array
                .push({
                    row.raise60seclocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6seclocalviolation_array
                .push({
                    row.raise6seclocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minlocalviolation_array
                .push({
                    row.lower5minlocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerreglocalviolation_array
                .push({
                    row.lowerreglocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60seclocalviolation_array
                .push({
                    row.lower60seclocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6seclocalviolation_array
                .push({
                    row.lower6seclocalviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minviolation_array
                .push({
                    row.raise5minviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raiseregviolation_array
                .push({
                    row.raiseregviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secviolation_array
                .push({
                    row.raise60secviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secviolation_array
                .push({
                    row.raise6secviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minviolation_array
                .push({
                    row.lower5minviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerregviolation_array
                .push({
                    row.lowerregviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secviolation_array
                .push({
                    row.lower60secviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secviolation_array
                .push({
                    row.lower6secviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            totalintermittentgeneration_array
                .push({
                    row.totalintermittentgeneration
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            demand_and_nonschedgen_array
                .push({
                    row.demand_and_nonschedgen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            uigf_array
                .push({
                    row.uigf
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totaldemand_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availablegeneration_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availableload_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demandforecast_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(dispatchablegeneration_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(dispatchableload_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(netinterchange_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(excessgeneration_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5mindispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minlocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minlocalprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minlocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minsupplyprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secdispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60seclocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60seclocalprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60seclocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secsupplyprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secdispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6seclocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6seclocalprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6seclocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secsupplyprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5mindispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minlocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minlocalprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minlocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minsupplyprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secdispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60seclocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60seclocalprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60seclocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secsupplyprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secdispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6seclocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6seclocalprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6seclocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secsupplyprice_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(initialsupply_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(clearedsupply_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerregimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreglocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreglocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerregreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raiseregimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereglocaldispatch_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereglocalreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raiseregreq_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minlocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereglocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60seclocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6seclocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minlocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreglocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60seclocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6seclocalviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raiseregviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerregviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalintermittentgeneration_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand_and_nonschedgen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(uigf_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "sql_server")]
pub async fn save<'a, S>(
    mms_file: &mut mmsdm_core::MmsFile<'a>,
    file_key: &mmsdm_core::FileKey,
    client: &mut tiberius::Client<S>,
    chunk_size: Option<usize>,
) -> mmsdm_core::Result<()>
where
    S: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin + Send,
{
    match (file_key.table_name.as_deref(), file_key.version) {
        (Some("BIDPEROFFER"), version) if version <= 1_i32 => {
            let d: Vec<OfferBidperoffer1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertOfferBidperoffer1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("CASESOLUTION"), version) if version <= 1_i32 => {
            let d: Vec<DispatchbncCasesolution1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertDispatchbncCasesolution1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("CASE"), version) if version <= 1_i32 => {
            let d: Vec<DispatchocdCase1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertDispatchocdCase1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("UNITSOLUTION"), version) if version <= 1_i32 => {
            let d: Vec<DispatchbncUnitsolution1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertDispatchbncUnitsolution1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("CUSTOMER"), version) if version <= 1_i32 => {
            let d: Vec<MeterDataCustomer1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterDataCustomer1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("CUSTOMER_TRK"), version) if version <= 1_i32 => {
            let d: Vec<MeterDataCustomerTrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterDataCustomerTrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("GEN_DUID"), version) if version <= 1_i32 => {
            let d: Vec<MeterDataGenDuid1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterDataGenDuid1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("TRK"), version) if version <= 1_i32 => {
            let d: Vec<MeterdataTrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterdataTrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("MNSP_FILETRK"), version) if version <= 1_i32 => {
            let d: Vec<BidMnspFiletrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBidMnspFiletrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("MNSP_OFFERTRK"), version) if version <= 1_i32 => {
            let d: Vec<BidMnspOffertrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBidMnspOffertrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("MNSP_PEROFFER"), version) if version <= 1_i32 => {
            let d: Vec<BidMnspPeroffer1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBidMnspPeroffer1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DAYOFFER_STACK"), version) if version <= 1_i32 => {
            let d: Vec<MrDayofferStack1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMrDayofferStack1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("EVENT"), version) if version <= 1_i32 => {
            let d: Vec<MrEvent1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMrEvent1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("EVENT_SCHEDULE"), version) if version <= 1_i32 => {
            let d: Vec<MrEventSchedule1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMrEventSchedule1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("PEROFFER_STACK"), version) if version <= 1_i32 => {
            let d: Vec<MrPerofferStack1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMrPerofferStack1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("FCASCOMP"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsFcascomp5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertSettlementsFcascomp5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERVENTION"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsIntervention5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertSettlementsIntervention5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERVENTIONRECOVERY"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsInterventionrecovery5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertSettlementsInterventionrecovery5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("MR_PAYMENT"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsMrPayment5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertSettlementsMrPayment5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("MR_RECOVERY"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsMrRecovery5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertSettlementsMrRecovery5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("UNIT_SOLUTION"), version) if version <= 2_i32 => {
            let d: Vec<TradingUnitSolution2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertTradingUnitSolution2 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONSUM"), version) if version <= 4_i32 => {
            let d: Vec<TradingRegionsum4> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertTradingRegionsum4 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        _ => {
            log::error!("Unexpected file key {:?}", file_key);
        }
    }
    Ok(())
}
