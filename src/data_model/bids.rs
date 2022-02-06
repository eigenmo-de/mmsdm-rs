/// # Summary
///
/// ## BIDDAYOFFER
///  _BIDDAYOFFER shows the Energy and Ancillary Service bid data for each Market Day. BIDDAYOFFER is the parent table to BIDOFFERPERIOD. BIDDAYOFFER is a child table to BIDOFFERFILETRK_
///
/// * Data Set Name: Bids
/// * File Name: Biddayoffer
/// * Data Version: 1
///
/// # Description
///  The ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDOFFERPERIOD) facilitate ancillary service bidding and include energy bidding.  BIDDAYOFFER data is confidential to the submitting participant until made public after 4am the next day. Source BIDDAYOFFER updates as ancillary service bids are processed. BIDDAYOFFER includes all accepted energy and ancillary service bids. Volume Approximately 1,500,000 records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * OFFERDATE
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidsBiddayoffer1 {
    /// Dispatchable unit identifier
    pub duid: String,
    /// Bid Type Identifier
    pub bidtype: String,
    /// Market date for applying the bid
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Version No. for given offer date
    pub versionno: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    pub participantid: String,
    /// Maximum energy available from Energy Constrained Plant. (Energy Bids Only)
    pub dailyenergyconstraint: Option<rust_decimal::Decimal>,
    /// Explanation for all rebids and inflexibilities
    pub rebidexplanation: Option<String>,
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
    pub normalstatus: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Offer Factor
    pub mr_factor: Option<rust_decimal::Decimal>,
    /// Daily if processed before BidCutOff of previous day, otherwise REBID
    pub entrytype: Option<String>,
    /// The time of the event(s) or other occurrence(s) cited/adduced as the reason for the rebid. Required for rebids, not required for fixed load or low ramp rates. Expected in the format: HH:MM:SS e.g. 20:11:00
    pub rebid_event_time: Option<String>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant became aware of the event(s) / occurrence(s) that prompted the rebid.Not validated by AEMO
    pub rebid_aware_time: Option<String>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant made the decision to rebid. Not validated by AEMO
    pub rebid_decision_time: Option<String>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. A provided rebid category. Not validated by AEMO
    pub rebid_category: Option<String>,
    /// A participants unique Reference Id
    pub reference_id: Option<String>,
}
impl crate::GetTable for BidsBiddayoffer1 {
    type PrimaryKey = BidsBiddayoffer1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("BIDDAYOFFER".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BidsBiddayoffer1PrimaryKey {
        BidsBiddayoffer1PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            offerdate: self.offerdate,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "bids_biddayoffer_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for BidsBiddayoffer1 {
    type Row = BidsBiddayoffer1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.offerdate == row.offerdate
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for BidsBiddayoffer1 {
    type PrimaryKey = BidsBiddayoffer1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.offerdate == key.offerdate
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidsBiddayoffer1PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub offerdate: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for BidsBiddayoffer1PrimaryKey {
    type Row = BidsBiddayoffer1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.offerdate == row.offerdate
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for BidsBiddayoffer1PrimaryKey {
    type PrimaryKey = BidsBiddayoffer1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.offerdate == key.offerdate
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for BidsBiddayoffer1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BidsBiddayoffer1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "dailyenergyconstraint",
                arrow2::datatypes::DataType::Decimal(12, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebidexplanation",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband1",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband2",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband3",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband4",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband5",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband6",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband7",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband8",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband9",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband10",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "minimumload",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new("t1", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new("t2", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new("t3", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new("t4", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new(
                "normalstatus",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "mr_factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "entrytype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_event_time",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_aware_time",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_decision_time",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_category",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "reference_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut duid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut dailyenergyconstraint_array = Vec::new();
        let mut rebidexplanation_array = Vec::new();
        let mut priceband1_array = Vec::new();
        let mut priceband2_array = Vec::new();
        let mut priceband3_array = Vec::new();
        let mut priceband4_array = Vec::new();
        let mut priceband5_array = Vec::new();
        let mut priceband6_array = Vec::new();
        let mut priceband7_array = Vec::new();
        let mut priceband8_array = Vec::new();
        let mut priceband9_array = Vec::new();
        let mut priceband10_array = Vec::new();
        let mut minimumload_array = Vec::new();
        let mut t1_array = Vec::new();
        let mut t2_array = Vec::new();
        let mut t3_array = Vec::new();
        let mut t4_array = Vec::new();
        let mut normalstatus_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut mr_factor_array = Vec::new();
        let mut entrytype_array = Vec::new();
        let mut rebid_event_time_array = Vec::new();
        let mut rebid_aware_time_array = Vec::new();
        let mut rebid_decision_time_array = Vec::new();
        let mut rebid_category_array = Vec::new();
        let mut reference_id_array = Vec::new();
        for (_, row) in partition {
            duid_array.push(row.duid);
            bidtype_array.push(row.bidtype);
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            offerdate_array.push(row.offerdate.timestamp_millis());
            versionno_array.push({
                row.versionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            participantid_array.push(row.participantid);
            dailyenergyconstraint_array.push({
                row.dailyenergyconstraint.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            rebidexplanation_array.push(row.rebidexplanation);
            priceband1_array.push({
                row.priceband1.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband2_array.push({
                row.priceband2.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband3_array.push({
                row.priceband3.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband4_array.push({
                row.priceband4.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband5_array.push({
                row.priceband5.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband6_array.push({
                row.priceband6.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband7_array.push({
                row.priceband7.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband8_array.push({
                row.priceband8.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband9_array.push({
                row.priceband9.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband10_array.push({
                row.priceband10.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            minimumload_array.push({
                row.minimumload.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t1_array.push({
                row.t1.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t2_array.push({
                row.t2.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t3_array.push({
                row.t3.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t4_array.push({
                row.t4.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            normalstatus_array.push(row.normalstatus);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            mr_factor_array.push({
                row.mr_factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            entrytype_array.push(row.entrytype);
            rebid_event_time_array.push(row.rebid_event_time);
            rebid_aware_time_array.push(row.rebid_aware_time);
            rebid_decision_time_array.push(row.rebid_decision_time);
            rebid_category_array.push(row.rebid_category);
            reference_id_array.push(row.reference_id);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dailyenergyconstraint_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebidexplanation_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband1_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband2_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband3_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband4_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband5_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband6_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband7_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband8_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband9_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband10_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(minimumload_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t1_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t2_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t3_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t4_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(normalstatus_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mr_factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(entrytype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebid_event_time_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebid_aware_time_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebid_decision_time_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(rebid_category_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reference_id_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BIDDAYOFFER_D
///  _BIDDAYOFFER_D shows the public summary of the energy and FCAS offers applicable in the Dispatch for the<br>intervals identified. BIDDAYOFFER_D is the parent table to BIDPEROFFER_D._
///
/// * Data Set Name: Bid
/// * File Name: Biddayoffer D
/// * Data Version: 2
///
/// # Description
///  BIDDAYOFFER_D data is made public after 4am the next day. Source BIDDAYOFFER_D updates as ancillary service bids are processed. BIDDAYOFFER_D shows latest accepted energy and ancillary service bids. Volume Summary - approximately 1,000 rows per day
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidBiddayofferD2 {
    /// Market date for which the bid applied
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Bid Type Identifier
    pub bidtype: String,
    /// Market date for which the bid was submitted.
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Offer date and time
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Version No. for given offer date
    pub versionno: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Maximum energy available from Energy Constrained Plant. (Energy Bids Only)
    pub dailyenergyconstraint: Option<rust_decimal::Decimal>,
    /// Explanation for all rebids and inflexibilities
    pub rebidexplanation: Option<String>,
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
    pub normalstatus: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Scaling Factor
    pub mr_factor: Option<rust_decimal::Decimal>,
    /// Daily if processed before BidCutOff of previous day, otherwise REBID
    pub entrytype: Option<String>,
}
impl crate::GetTable for BidBiddayofferD2 {
    type PrimaryKey = BidBiddayofferD2PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("BIDDAYOFFER_D".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> BidBiddayofferD2PrimaryKey {
        BidBiddayofferD2PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "bid_biddayoffer_d_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for BidBiddayofferD2 {
    type Row = BidBiddayofferD2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for BidBiddayofferD2 {
    type PrimaryKey = BidBiddayofferD2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidBiddayofferD2PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for BidBiddayofferD2PrimaryKey {
    type Row = BidBiddayofferD2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for BidBiddayofferD2PrimaryKey {
    type PrimaryKey = BidBiddayofferD2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for BidBiddayofferD2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BidBiddayofferD2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "bidsettlementdate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "dailyenergyconstraint",
                arrow2::datatypes::DataType::Decimal(12, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebidexplanation",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband1",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband2",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband3",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband4",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband5",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband6",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband7",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband8",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband9",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband10",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "minimumload",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new("t1", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new("t2", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new("t3", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new("t4", arrow2::datatypes::DataType::Decimal(22, 0), true),
            arrow2::datatypes::Field::new(
                "normalstatus",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "mr_factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "entrytype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut bidsettlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut dailyenergyconstraint_array = Vec::new();
        let mut rebidexplanation_array = Vec::new();
        let mut priceband1_array = Vec::new();
        let mut priceband2_array = Vec::new();
        let mut priceband3_array = Vec::new();
        let mut priceband4_array = Vec::new();
        let mut priceband5_array = Vec::new();
        let mut priceband6_array = Vec::new();
        let mut priceband7_array = Vec::new();
        let mut priceband8_array = Vec::new();
        let mut priceband9_array = Vec::new();
        let mut priceband10_array = Vec::new();
        let mut minimumload_array = Vec::new();
        let mut t1_array = Vec::new();
        let mut t2_array = Vec::new();
        let mut t3_array = Vec::new();
        let mut t4_array = Vec::new();
        let mut normalstatus_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut mr_factor_array = Vec::new();
        let mut entrytype_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            duid_array.push(row.duid);
            bidtype_array.push(row.bidtype);
            bidsettlementdate_array.push(row.bidsettlementdate.map(|val| val.timestamp_millis()));
            offerdate_array.push(row.offerdate.map(|val| val.timestamp_millis()));
            versionno_array.push({
                row.versionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            participantid_array.push(row.participantid);
            dailyenergyconstraint_array.push({
                row.dailyenergyconstraint.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            rebidexplanation_array.push(row.rebidexplanation);
            priceband1_array.push({
                row.priceband1.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband2_array.push({
                row.priceband2.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband3_array.push({
                row.priceband3.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband4_array.push({
                row.priceband4.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband5_array.push({
                row.priceband5.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband6_array.push({
                row.priceband6.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband7_array.push({
                row.priceband7.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband8_array.push({
                row.priceband8.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband9_array.push({
                row.priceband9.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband10_array.push({
                row.priceband10.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            minimumload_array.push({
                row.minimumload.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t1_array.push({
                row.t1.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t2_array.push({
                row.t2.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t3_array.push({
                row.t3.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            t4_array.push({
                row.t4.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            normalstatus_array.push(row.normalstatus);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            mr_factor_array.push({
                row.mr_factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            entrytype_array.push(row.entrytype);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidsettlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dailyenergyconstraint_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebidexplanation_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband1_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband2_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband3_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband4_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband5_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband6_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband7_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband8_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband9_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband10_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(minimumload_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t1_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t2_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t3_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(t4_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(normalstatus_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mr_factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(entrytype_array)),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * OFFERDATE
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidsBidofferfiletrk1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Submitted file name
    pub filename: String,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant agent who created the Offer
    pub authorisedby: Option<String>,
    /// When the Offer was processed - synonymous with LastChanged
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// A GUID used to identify the submission transaction in AEMOs systems
    pub transaction_id: Option<String>,
    /// A participant provided reference, which is required to be unique per submission (for a PARTICIPANTID)
    pub reference_id: Option<String>,
    /// The participant provided date/time for the submission
    #[serde(with = "crate::mms_datetime_opt")]
    pub submission_timestamp: Option<chrono::NaiveDateTime>,
    /// A participant provided comment
    pub comments: Option<String>,
    /// Method by which this submission was made typically FTP, API, WEB
    pub submission_method: Option<String>,
}
impl crate::GetTable for BidsBidofferfiletrk1 {
    type PrimaryKey = BidsBidofferfiletrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("BIDOFFERFILETRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BidsBidofferfiletrk1PrimaryKey {
        BidsBidofferfiletrk1PrimaryKey {
            offerdate: self.offerdate,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "bids_bidofferfiletrk_v1".to_string()
    }
}
impl crate::CompareWithRow for BidsBidofferfiletrk1 {
    type Row = BidsBidofferfiletrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.offerdate == row.offerdate && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for BidsBidofferfiletrk1 {
    type PrimaryKey = BidsBidofferfiletrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidsBidofferfiletrk1PrimaryKey {
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: String,
}
impl crate::CompareWithRow for BidsBidofferfiletrk1PrimaryKey {
    type Row = BidsBidofferfiletrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.offerdate == row.offerdate && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for BidsBidofferfiletrk1PrimaryKey {
    type PrimaryKey = BidsBidofferfiletrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for BidsBidofferfiletrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BidsBidofferfiletrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "filename",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "transaction_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "reference_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "submission_timestamp",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("comments", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "submission_method",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantid_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut status_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut transaction_id_array = Vec::new();
        let mut reference_id_array = Vec::new();
        let mut submission_timestamp_array = Vec::new();
        let mut comments_array = Vec::new();
        let mut submission_method_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            offerdate_array.push(row.offerdate.timestamp_millis());
            filename_array.push(row.filename);
            status_array.push(row.status);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            transaction_id_array.push(row.transaction_id);
            reference_id_array.push(row.reference_id);
            submission_timestamp_array
                .push(row.submission_timestamp.map(|val| val.timestamp_millis()));
            comments_array.push(row.comments);
            submission_method_array.push(row.submission_method);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(filename_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(transaction_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reference_id_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(submission_timestamp_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(comments_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    submission_method_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BIDOFFERPERIOD
///  _BIDOFFERPERIOD shows 5-minute period-based Energy and Ancillary Service bid data.BIDOFFERPERIOD is a child table of BIDDAYOFFER_
///
/// * Data Set Name: Bids
/// * File Name: Bidofferperiod
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidsBidofferperiod1 {
    /// Dispatchable Unit ID
    pub duid: String,
    /// The type of bid, one-of ENERGY, RAISE6SEC, RAISE60SEC, RAISE5MIN, RAISEREG, LOWER6SEC, LOWER60SEC, LOWER5MIN, LOWERREG
    pub bidtype: String,
    /// The trading date this bid is for
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
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
}
impl crate::GetTable for BidsBidofferperiod1 {
    type PrimaryKey = BidsBidofferperiod1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("BIDOFFERPERIOD".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BidsBidofferperiod1PrimaryKey {
        BidsBidofferperiod1PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            offerdatetime: self.offerdatetime,
            periodid: self.periodid,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "bids_bidofferperiod_v1".to_string()
    }
}
impl crate::CompareWithRow for BidsBidofferperiod1 {
    type Row = BidsBidofferperiod1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for BidsBidofferperiod1 {
    type PrimaryKey = BidsBidofferperiod1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidsBidofferperiod1PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for BidsBidofferperiod1PrimaryKey {
    type Row = BidsBidofferperiod1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for BidsBidofferperiod1PrimaryKey {
    type PrimaryKey = BidsBidofferperiod1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for BidsBidofferperiod1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BidsBidofferperiod1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "maxavail",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "fixedload",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new("rampuprate", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("rampdownrate", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new(
                "enablementmin",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablementmax",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowbreakpoint",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "highbreakpoint",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail1",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail2",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail3",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail4",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail5",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail6",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail7",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail8",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail9",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail10",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pasaavailability",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut duid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut tradingdate_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut maxavail_array = Vec::new();
        let mut fixedload_array = Vec::new();
        let mut rampuprate_array = Vec::new();
        let mut rampdownrate_array = Vec::new();
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
        let mut pasaavailability_array = Vec::new();
        for (_, row) in partition {
            duid_array.push(row.duid);
            bidtype_array.push(row.bidtype);
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            maxavail_array.push({
                row.maxavail.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            fixedload_array.push({
                row.fixedload.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            rampuprate_array.push(row.rampuprate);
            rampdownrate_array.push(row.rampdownrate);
            enablementmin_array.push({
                row.enablementmin.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            enablementmax_array.push({
                row.enablementmax.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            lowbreakpoint_array.push({
                row.lowbreakpoint.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            highbreakpoint_array.push({
                row.highbreakpoint.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail1_array.push({
                row.bandavail1.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail2_array.push({
                row.bandavail2.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail3_array.push({
                row.bandavail3.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail4_array.push({
                row.bandavail4.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail5_array.push({
                row.bandavail5.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail6_array.push({
                row.bandavail6.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail7_array.push({
                row.bandavail7.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail8_array.push({
                row.bandavail8.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail9_array.push({
                row.bandavail9.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail10_array.push({
                row.bandavail10.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            pasaavailability_array.push({
                row.pasaavailability.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxavail_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fixedload_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rampuprate_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rampdownrate_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablementmin_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablementmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(highbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail1_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail2_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail3_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail4_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail5_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail6_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail7_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail8_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail9_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail10_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pasaavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BIDPEROFFER_D
///  _BIDPEROFFER_D shows the public summary of the energy and FCAS offers applicable in the Dispatch for the<br>intervals identified. BIDPEROFFER_D is the child to BIDDAYOFFER_D._
///
/// * Data Set Name: Bid
/// * File Name: Bidperoffer D
/// * Data Version: 2
///
/// # Description
///  BIDPEROFFER_D is public data, so is available to all participants. Source BIDPEROFFER_D updates daily shortly after 4am.  See also BIDPEROFFER.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * INTERVAL_DATETIME
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidBidperofferD2 {
    /// Market date for which the bid applied
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Bid Type Identifier
    pub bidtype: String,
    /// Market date for which the bid was submitted
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Offer date and time
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// The trading interval period identifier (1-48)
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Date and Time of the dispatch interval to which the offer applied
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Mandatory Restriction Offer amount
    pub mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for BidBidperofferD2 {
    type PrimaryKey = BidBidperofferD2PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("BIDPEROFFER_D".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> BidBidperofferD2PrimaryKey {
        BidBidperofferD2PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            interval_datetime: self.interval_datetime,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "bid_bidperoffer_d_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for BidBidperofferD2 {
    type Row = BidBidperofferD2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for BidBidperofferD2 {
    type PrimaryKey = BidBidperofferD2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidBidperofferD2PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for BidBidperofferD2PrimaryKey {
    type Row = BidBidperofferD2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for BidBidperofferD2PrimaryKey {
    type PrimaryKey = BidBidperofferD2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for BidBidperofferD2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BidBidperofferD2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "bidsettlementdate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxavail",
                arrow2::datatypes::DataType::Decimal(12, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "fixedload",
                arrow2::datatypes::DataType::Decimal(12, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rocup",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rocdown",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablementmin",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablementmax",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowbreakpoint",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "highbreakpoint",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail1",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail2",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail3",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail4",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail5",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail6",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail7",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail8",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail9",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail10",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "pasaavailability",
                arrow2::datatypes::DataType::Decimal(12, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "mr_capacity",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut bidsettlementdate_array = Vec::new();
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
        let mut interval_datetime_array = Vec::new();
        let mut mr_capacity_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            duid_array.push(row.duid);
            bidtype_array.push(row.bidtype);
            bidsettlementdate_array.push(row.bidsettlementdate.map(|val| val.timestamp_millis()));
            offerdate_array.push(row.offerdate.map(|val| val.timestamp_millis()));
            periodid_array.push({
                row.periodid.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            versionno_array.push({
                row.versionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            maxavail_array.push({
                row.maxavail.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            fixedload_array.push({
                row.fixedload.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            rocup_array.push({
                row.rocup.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rocdown_array.push({
                row.rocdown.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            enablementmin_array.push({
                row.enablementmin.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            enablementmax_array.push({
                row.enablementmax.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lowbreakpoint_array.push({
                row.lowbreakpoint.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            highbreakpoint_array.push({
                row.highbreakpoint.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail1_array.push({
                row.bandavail1.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail2_array.push({
                row.bandavail2.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail3_array.push({
                row.bandavail3.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail4_array.push({
                row.bandavail4.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail5_array.push({
                row.bandavail5.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail6_array.push({
                row.bandavail6.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail7_array.push({
                row.bandavail7.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail8_array.push({
                row.bandavail8.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail9_array.push({
                row.bandavail9.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bandavail10_array.push({
                row.bandavail10.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            pasaavailability_array.push({
                row.pasaavailability.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            mr_capacity_array.push({
                row.mr_capacity.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidsettlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxavail_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fixedload_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rocup_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rocdown_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablementmin_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablementmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(highbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail1_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail2_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail3_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail4_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail5_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail6_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail7_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail8_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail9_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail10_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pasaavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mr_capacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * LINKID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidsMnspBidofferperiod1 {
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    /// The trading date this bid is for
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime")]
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
}
impl crate::GetTable for BidsMnspBidofferperiod1 {
    type PrimaryKey = BidsMnspBidofferperiod1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("MNSP_BIDOFFERPERIOD".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BidsMnspBidofferperiod1PrimaryKey {
        BidsMnspBidofferperiod1PrimaryKey {
            linkid: self.linkid.clone(),
            offerdatetime: self.offerdatetime,
            periodid: self.periodid,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "bids_mnsp_bidofferperiod_v1".to_string()
    }
}
impl crate::CompareWithRow for BidsMnspBidofferperiod1 {
    type Row = BidsMnspBidofferperiod1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for BidsMnspBidofferperiod1 {
    type PrimaryKey = BidsMnspBidofferperiod1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidsMnspBidofferperiod1PrimaryKey {
    pub linkid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for BidsMnspBidofferperiod1PrimaryKey {
    type Row = BidsMnspBidofferperiod1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for BidsMnspBidofferperiod1PrimaryKey {
    type PrimaryKey = BidsMnspBidofferperiod1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for BidsMnspBidofferperiod1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BidsMnspBidofferperiod1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("linkid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "maxavail",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "fixedload",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new("rampuprate", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new(
                "bandavail1",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail2",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail3",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail4",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail5",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail6",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail7",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail8",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail9",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bandavail10",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pasaavailability",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut linkid_array = Vec::new();
        let mut tradingdate_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut maxavail_array = Vec::new();
        let mut fixedload_array = Vec::new();
        let mut rampuprate_array = Vec::new();
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
        let mut pasaavailability_array = Vec::new();
        for (_, row) in partition {
            linkid_array.push(row.linkid);
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            maxavail_array.push({
                row.maxavail.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            fixedload_array.push({
                row.fixedload.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            rampuprate_array.push(row.rampuprate);
            bandavail1_array.push({
                row.bandavail1.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail2_array.push({
                row.bandavail2.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail3_array.push({
                row.bandavail3.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail4_array.push({
                row.bandavail4.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail5_array.push({
                row.bandavail5.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail6_array.push({
                row.bandavail6.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail7_array.push({
                row.bandavail7.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail8_array.push({
                row.bandavail8.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail9_array.push({
                row.bandavail9.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            bandavail10_array.push({
                row.bandavail10.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            pasaavailability_array.push({
                row.pasaavailability.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(linkid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxavail_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fixedload_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rampuprate_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail1_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail2_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail3_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail4_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail5_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail6_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail7_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail8_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail9_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bandavail10_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pasaavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MNSP_DAYOFFER
///  _MNSP_DAYOFFER updates as bids are processed. All bids are available as part of next day market data. MNSP_DAYOFFER is the parent table to MNSP_BIDOFFERPERIOD, and joins to BIDOFFERFILETRK for 5MS Bids._
///
/// * Data Set Name: Bids
/// * File Name: Mnsp Dayoffer
/// * Data Version: 1
///
/// # Description
///  MNSP_DAYOFFER shows own (confidential) data updates as bids are processed. All bids are available as part of next day market data. Volume 4, 000 per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * LINKID
/// * OFFERDATE
/// * PARTICIPANTID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidsMnspDayoffer1 {
    /// Market Date from which bid is active
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    /// Bid type. Either Rebid or Daily
    pub entrytype: Option<String>,
    /// Explanation for all rebids and inflexibilities
    pub rebidexplanation: Option<String>,
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Offer Factor
    pub mr_factor: Option<rust_decimal::Decimal>,
    /// The time of the event(s) or other occurrence(s) cited/adduced as the reason for the rebid. Required for rebids, not required for fixed load or low ramp rates. Expected in the format: HH:MM:SS e.g. 20:11:00
    pub rebid_event_time: Option<String>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant became aware of the event(s) / occurrence(s) that prompted the rebid. Not validated by AEMO
    pub rebid_aware_time: Option<String>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. The time at which the participant made the decision to rebid. Not validated by AEMO
    pub rebid_decision_time: Option<String>,
    /// Intended to support the Rebidding and Technical Parameters Guideline. A provided rebid category. Not validated by AEMO
    pub rebid_category: Option<String>,
    /// A participants unique Reference Id
    pub reference_id: Option<String>,
}
impl crate::GetTable for BidsMnspDayoffer1 {
    type PrimaryKey = BidsMnspDayoffer1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("MNSP_DAYOFFER".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> BidsMnspDayoffer1PrimaryKey {
        BidsMnspDayoffer1PrimaryKey {
            linkid: self.linkid.clone(),
            offerdate: self.offerdate,
            participantid: self.participantid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "bids_mnsp_dayoffer_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for BidsMnspDayoffer1 {
    type Row = BidsMnspDayoffer1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.offerdate == row.offerdate
            && self.participantid == row.participantid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for BidsMnspDayoffer1 {
    type PrimaryKey = BidsMnspDayoffer1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.offerdate == key.offerdate
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BidsMnspDayoffer1PrimaryKey {
    pub linkid: String,
    pub offerdate: chrono::NaiveDateTime,
    pub participantid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for BidsMnspDayoffer1PrimaryKey {
    type Row = BidsMnspDayoffer1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.offerdate == row.offerdate
            && self.participantid == row.participantid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for BidsMnspDayoffer1PrimaryKey {
    type PrimaryKey = BidsMnspDayoffer1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.offerdate == key.offerdate
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for BidsMnspDayoffer1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for BidsMnspDayoffer1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("linkid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "entrytype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebidexplanation",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband1",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband2",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband3",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband4",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband5",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband6",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband7",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband8",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband9",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "priceband10",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "mr_factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_event_time",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_aware_time",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_decision_time",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebid_category",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "reference_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut linkid_array = Vec::new();
        let mut entrytype_array = Vec::new();
        let mut rebidexplanation_array = Vec::new();
        let mut priceband1_array = Vec::new();
        let mut priceband2_array = Vec::new();
        let mut priceband3_array = Vec::new();
        let mut priceband4_array = Vec::new();
        let mut priceband5_array = Vec::new();
        let mut priceband6_array = Vec::new();
        let mut priceband7_array = Vec::new();
        let mut priceband8_array = Vec::new();
        let mut priceband9_array = Vec::new();
        let mut priceband10_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut mr_factor_array = Vec::new();
        let mut rebid_event_time_array = Vec::new();
        let mut rebid_aware_time_array = Vec::new();
        let mut rebid_decision_time_array = Vec::new();
        let mut rebid_category_array = Vec::new();
        let mut reference_id_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            offerdate_array.push(row.offerdate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            linkid_array.push(row.linkid);
            entrytype_array.push(row.entrytype);
            rebidexplanation_array.push(row.rebidexplanation);
            priceband1_array.push({
                row.priceband1.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband2_array.push({
                row.priceband2.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband3_array.push({
                row.priceband3.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband4_array.push({
                row.priceband4.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband5_array.push({
                row.priceband5.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband6_array.push({
                row.priceband6.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband7_array.push({
                row.priceband7.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband8_array.push({
                row.priceband8.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband9_array.push({
                row.priceband9.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            priceband10_array.push({
                row.priceband10.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            mr_factor_array.push({
                row.mr_factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            rebid_event_time_array.push(row.rebid_event_time);
            rebid_aware_time_array.push(row.rebid_aware_time);
            rebid_decision_time_array.push(row.rebid_decision_time);
            rebid_category_array.push(row.rebid_category);
            reference_id_array.push(row.reference_id);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(linkid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(entrytype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebidexplanation_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband1_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband2_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband3_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband4_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband5_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband6_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband7_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband8_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband9_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(priceband10_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mr_factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebid_event_time_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebid_aware_time_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    rebid_decision_time_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(rebid_category_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reference_id_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MTPASA_OFFERDATA
///  _Participant submitted Offers for MTPASA process_
///
/// * Data Set Name: Offer
/// * File Name: Mtpasa Offerdata
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private;
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * OFFERDATETIME
/// * PARTICIPANTID
/// * UNITID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMtpasaOfferdata1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Date time file processed
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// either duid or mnsp linkid
    pub unitid: String,
    /// trade date when the offer becomes effective
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for OfferMtpasaOfferdata1 {
    type PrimaryKey = OfferMtpasaOfferdata1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: Some("MTPASA_OFFERDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> OfferMtpasaOfferdata1PrimaryKey {
        OfferMtpasaOfferdata1PrimaryKey {
            effectivedate: self.effectivedate,
            offerdatetime: self.offerdatetime,
            participantid: self.participantid.clone(),
            unitid: self.unitid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "offer_mtpasa_offerdata_v1".to_string()
    }
}
impl crate::CompareWithRow for OfferMtpasaOfferdata1 {
    type Row = OfferMtpasaOfferdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.offerdatetime == row.offerdatetime
            && self.participantid == row.participantid
            && self.unitid == row.unitid
    }
}
impl crate::CompareWithPrimaryKey for OfferMtpasaOfferdata1 {
    type PrimaryKey = OfferMtpasaOfferdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.offerdatetime == key.offerdatetime
            && self.participantid == key.participantid
            && self.unitid == key.unitid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct OfferMtpasaOfferdata1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub offerdatetime: chrono::NaiveDateTime,
    pub participantid: String,
    pub unitid: String,
}
impl crate::CompareWithRow for OfferMtpasaOfferdata1PrimaryKey {
    type Row = OfferMtpasaOfferdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.offerdatetime == row.offerdatetime
            && self.participantid == row.participantid
            && self.unitid == row.unitid
    }
}
impl crate::CompareWithPrimaryKey for OfferMtpasaOfferdata1PrimaryKey {
    type PrimaryKey = OfferMtpasaOfferdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.offerdatetime == key.offerdatetime
            && self.participantid == key.participantid
            && self.unitid == key.unitid
    }
}
impl crate::PrimaryKey for OfferMtpasaOfferdata1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for OfferMtpasaOfferdata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("unitid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("energy", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("capacity1", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("capacity2", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("capacity3", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("capacity4", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("capacity5", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("capacity6", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("capacity7", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut unitid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut energy_array = Vec::new();
        let mut capacity1_array = Vec::new();
        let mut capacity2_array = Vec::new();
        let mut capacity3_array = Vec::new();
        let mut capacity4_array = Vec::new();
        let mut capacity5_array = Vec::new();
        let mut capacity6_array = Vec::new();
        let mut capacity7_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            unitid_array.push(row.unitid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            energy_array.push(row.energy);
            capacity1_array.push(row.capacity1);
            capacity2_array.push(row.capacity2);
            capacity3_array.push(row.capacity3);
            capacity4_array.push(row.capacity4);
            capacity5_array.push(row.capacity5);
            capacity6_array.push(row.capacity6);
            capacity7_array.push(row.capacity7);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(unitid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energy_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacity1_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacity2_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacity3_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacity4_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacity5_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacity6_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacity7_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * OFFERDATETIME
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMtpasaOfferfiletrk1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Date time file processed
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Submitted file name
    pub filename: Option<String>,
}
impl crate::GetTable for OfferMtpasaOfferfiletrk1 {
    type PrimaryKey = OfferMtpasaOfferfiletrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: Some("MTPASA_OFFERFILETRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> OfferMtpasaOfferfiletrk1PrimaryKey {
        OfferMtpasaOfferfiletrk1PrimaryKey {
            offerdatetime: self.offerdatetime,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "offer_mtpasa_offerfiletrk_v1".to_string()
    }
}
impl crate::CompareWithRow for OfferMtpasaOfferfiletrk1 {
    type Row = OfferMtpasaOfferfiletrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.offerdatetime == row.offerdatetime && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for OfferMtpasaOfferfiletrk1 {
    type PrimaryKey = OfferMtpasaOfferfiletrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdatetime == key.offerdatetime && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct OfferMtpasaOfferfiletrk1PrimaryKey {
    pub offerdatetime: chrono::NaiveDateTime,
    pub participantid: String,
}
impl crate::CompareWithRow for OfferMtpasaOfferfiletrk1PrimaryKey {
    type Row = OfferMtpasaOfferfiletrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.offerdatetime == row.offerdatetime && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for OfferMtpasaOfferfiletrk1PrimaryKey {
    type PrimaryKey = OfferMtpasaOfferfiletrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdatetime == key.offerdatetime && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for OfferMtpasaOfferfiletrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for OfferMtpasaOfferfiletrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut filename_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            filename_array.push(row.filename);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)),
            ],
        )
        .map_err(Into::into)
    }
}
