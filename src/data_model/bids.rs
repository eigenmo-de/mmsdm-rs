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
///  The ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDPEROFFER) facilitate ancillary service bidding and include energy bidding.  BIDDAYOFFER data is confidential to the submitting participant until made public after 4am the next day. Source BIDDAYOFFER updates as ancillary service bids are processed. BIDDAYOFFER includes all accepted energy and ancillary service bids. Volume Approximately 1,500,000 records per year
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
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
    pub offerdate: chrono::NaiveDateTime,
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
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("BIDDAYOFFER".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## BIDOFFERFILETRK
///  _BIDOFFERFILETRK shows an audit trail of all files submitted containing an FCAS bid, including corrupt bids and rebids._
///
/// * Data Set Name: Bids
/// * File Name: Bidofferfiletrk
/// * Data Version: 1
///
/// # Description
///  BIDOFFERFILETRK data is confidential to the submitting participant. The new ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three new tables facilitate ancillary service bidding. The new tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDPEROFFER) are similar in structure to energy bidding tables (OFFERFILETRK, DAYOFFER and PEROFFER). The significant differences with the new tables are. ·	 The OFFERDATE field reflects the time the bid was loaded and this field alone provides the key for versioning of bids. The VERSIONNO field is retained for participant use as information only. ·	 The new tables support bids for multiple services. The BIDTYPE field defines the service to which the bid applies. ·	 There are no default bids. In the absence of a bid for a specific settlement date, the latest bid submitted for a previous settlement date applies. Source This data is updated as bids are processed. It includes all bids submitted including corrupt bids. Volume Approximately 100,000 records per year Note Confirmation is via CSV bid acknowledgement file
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
    pub offerdate: chrono::NaiveDateTime,
    /// Submitted file name
    pub filename: String,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant agent who created the Offer
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// A GUID used to identify the submission transaction in AEMOs systems
    pub transaction_id: Option<String>,
    /// A participant provided reference, which is required to be unique per submission (for a PARTICIPANTID)
    pub reference_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub submission_timestamp: Option<chrono::NaiveDateTime>,
    /// A participant provided comment
    pub comments: Option<String>,
}
impl crate::GetTable for BidsBidofferfiletrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("BIDOFFERFILETRK".into()),
            version: 1,
        }
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
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
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
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("BIDOFFERPERIOD".into()),
            version: 1,
        }
    }
}
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
///  The new ancillary service arrangements require availability and prices for each Frequency Control Ancillary Service to be bid on a similar basis to energy. Three new tables facilitate ancillary service bidding. The new tables (BIDOFFERFILETRK, BIDDAYOFFER and BIDPEROFFER) are similar in structure to energy bidding tables (OFFERFILETRK, DAYOFFER and PEROFFER). The significant differences with the new tables are: ·	 The OFFERDATE field reflects the time the bid was loaded and this field alone provides the key for versioning of bids. The VERSIONNO field is retained for participant use as information only. ·	 The new tables support bids for multiple services. The BIDTYPE field defines the service to which the bid applies. ·	 There are no default bids. In the absence of a bid for a specific settlement date, the latest bid submitted for a previous settlement date applies. BIDPEROFFER data is confidential to the submitting participant until made public after 4am the next day. Source BIDPEROFFER updates as energy and ancillary service bids are processed. BIDPEROFFER includes all accepted energy and ancillary service bids. Volume Approximately 72,000,000  records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
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
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Mandatory Restriction Offer amount
    pub mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for OfferBidperoffer1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: Some("BIDPEROFFER".into()),
            version: 1,
        }
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
    #[serde(with = "crate::mms_datetime")]
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
}
impl crate::GetTable for BidsMnspBidofferperiod1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("MNSP_BIDOFFERPERIOD".into()),
            version: 1,
        }
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
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Time this bid was processed and loaded
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
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BIDS".into(),
            table_name: Some("MNSP_DAYOFFER".into()),
            version: 1,
        }
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
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * FILENAME
/// * OFFERDATE
/// * PARTICIPANTID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidMnspFiletrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Participant Identifier
    pub participantid: String,
    /// File name for default bids, bids, rebids, re-offers or meter files, as appropriate to table
    pub filename: String,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    /// Acknowledge file name for bids, rebids
    pub ackfilename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for BidMnspFiletrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("MNSP_FILETRK".into()),
            version: 1,
        }
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
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
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
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// &nbsp;
    pub versionno: rust_decimal::Decimal,
    /// &nbsp;
    pub participantid: String,
    /// &nbsp;
    pub filename: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for BidMnspOffertrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("MNSP_OFFERTRK".into()),
            version: 1,
        }
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
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
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
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
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
impl crate::GetTable for BidMnspPeroffer1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: Some("MNSP_PEROFFER".into()),
            version: 1,
        }
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
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// either duid or mnsp linkid
    pub unitid: String,
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for OfferMtpasaOfferdata1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: Some("MTPASA_OFFERDATA".into()),
            version: 1,
        }
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
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Submitted file name
    pub filename: Option<String>,
}
impl crate::GetTable for OfferMtpasaOfferfiletrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: Some("MTPASA_OFFERFILETRK".into()),
            version: 1,
        }
    }
}
