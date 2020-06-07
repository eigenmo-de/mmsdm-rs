/// Data Set Name: Offer
/// File Name: Bidperoffer
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferBidperoffer1 {
    /// Dispatchable Unit identifier
    duid: String,
    /// Bid Type Identifier
    bidtype: String,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Period ID
    periodid: rust_decimal::Decimal,
    /// Version number of offer
    versionno: Option<rust_decimal::Decimal>,
    /// Maximum availability for this BidType in this period
    maxavail: Option<rust_decimal::Decimal>,
    /// Fixed unit output MW (Energy Bids Only)  A value of zero means no fixed load so the unit is dispatched according to bid and market (rather than zero fixed load)
    fixedload: Option<rust_decimal::Decimal>,
    /// MW/min for raise (Energy Bids Only)
    rocup: Option<rust_decimal::Decimal>,
    /// MW/Min for lower (Energy Bids Only)
    rocdown: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which this ancillary service becomes available (AS Only)
    enablementmin: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which this ancillary service can be supplied (AS Only)
    enablementmax: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which the unit can provide the full availability (MAXAVAIL) for this ancillary service (AS Only)
    lowbreakpoint: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which the unit can provide the full availability (MAXAVAIL) for this ancillary service (AS Only)
    highbreakpoint: Option<rust_decimal::Decimal>,
    /// Availability at price band 1
    bandavail1: Option<rust_decimal::Decimal>,
    /// Availability at price band 2
    bandavail2: Option<rust_decimal::Decimal>,
    /// Availability at price band 3
    bandavail3: Option<rust_decimal::Decimal>,
    /// Availability at price band 4
    bandavail4: Option<rust_decimal::Decimal>,
    /// Availability at price band 5
    bandavail5: Option<rust_decimal::Decimal>,
    /// Availability at price band 6
    bandavail6: Option<rust_decimal::Decimal>,
    /// Availability at price band 7
    bandavail7: Option<rust_decimal::Decimal>,
    /// Availability at price band 8
    bandavail8: Option<rust_decimal::Decimal>,
    /// Availability at price band 9
    bandavail9: Option<rust_decimal::Decimal>,
    /// Availability at price band 10
    bandavail10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pasaavailability: Option<rust_decimal::Decimal>,
    /// Mandatory Restriction Offer amount
    mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<OfferBidperoffer1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "BIDPEROFFER".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Bid
/// File Name: Biddayoffer D
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidBiddayofferD2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    duid: String,
    /// Bid Type Identifier
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// Version No. for given offer date
    versionno: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    participantid: Option<String>,
    /// Maximum energy available from Energy Constrained Plant. (Energy Bids Only)
    dailyenergyconstraint: Option<rust_decimal::Decimal>,
    /// Explanation for all rebids and inflexibilities
    rebidexplanation: Option<String>,
    /// Price for Availability Band 1
    priceband1: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 2
    priceband2: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 3
    priceband3: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 4
    priceband4: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 5
    priceband5: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    priceband6: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 7
    priceband7: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 8
    priceband8: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 9
    priceband9: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 10
    priceband10: Option<rust_decimal::Decimal>,
    /// Minimum MW load fast start plant
    minimumload: Option<rust_decimal::Decimal>,
    /// Time to synchronise in minutes (Energy Bids Only)
    t1: Option<rust_decimal::Decimal>,
    /// Time to minimum load in minutes (Energy Bids Only)
    t2: Option<rust_decimal::Decimal>,
    /// Time at minimum load in minutes (Energy Bids Only)
    t3: Option<rust_decimal::Decimal>,
    /// Time to shutdown in minutes (Energy Bids Only)
    t4: Option<rust_decimal::Decimal>,
    /// ON/OFF for loads (Energy Bids Only)
    normalstatus: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Scaling Factor
    mr_factor: Option<rust_decimal::Decimal>,
    /// Daily if processed before BidCutOff of previous day, otherwise REBID
    entrytype: Option<String>,
}
impl crate::GetTable<BidBiddayofferD2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BID".into(),
                        table_name: "BIDDAYOFFER_D".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Offer
/// File Name: Bidofferfiletrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferBidofferfiletrk1 {
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Submitted file name
    filename: String,
    /// Load status [SUCCESSFUL/CORRUPT]
    status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant agent who created the Offer
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OfferBidofferfiletrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "BIDOFFERFILETRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Offer
/// File Name: Mtpasa Offerfiletrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMtpasaOfferfiletrk1 {
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// Submitted file name
    filename: Option<String>,
}
impl crate::GetTable<OfferMtpasaOfferfiletrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "MTPASA_OFFERFILETRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Offer
/// File Name: Mnsp Dayoffer
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMnspDayoffer2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    linkid: String,
    /// Bid type. Either Rebid or Daily
    entrytype: Option<String>,
    /// Explanation for all rebids and inflexibilities
    rebidexplanation: Option<String>,
    /// Price for Availability Band 1
    priceband1: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 2
    priceband2: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 3
    priceband3: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 4
    priceband4: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 5
    priceband5: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    priceband6: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 7
    priceband7: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 8
    priceband8: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 9
    priceband9: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 10
    priceband10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Offer Factor
    mr_factor: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<OfferMnspDayoffer2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "MNSP_DAYOFFER".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Bid
/// File Name: Mnsp Filetrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidMnspFiletrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Participant Identifier
    participantid: String,
    /// File name for default bids, bids, rebids, re-offers or meter files, as appropriate to table
    filename: String,
    /// Load status [SUCCESSFUL/CORRUPT]
    status: Option<String>,
    /// Acknowledge file name for bids, rebids
    ackfilename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BidMnspFiletrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BID".into(),
                        table_name: "MNSP_FILETRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Offer
/// File Name: Mtpasa Offerdata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMtpasaOfferdata1 {
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    /// either duid or mnsp linkid
    unitid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// weekly energy constraint value
    energy: Option<i64>,
    /// capacity value day 1 (sunday)
    capacity1: Option<i64>,
    /// capacity value day 2 (monday)
    capacity2: Option<i64>,
    /// capacity value day 3 (tuesday)
    capacity3: Option<i64>,
    /// capacity value day 4 (wednesday)
    capacity4: Option<i64>,
    /// capacity value day 5 (thursday)
    capacity5: Option<i64>,
    /// capacity value day 6 (friday)
    capacity6: Option<i64>,
    /// capacity value day 7 (saturday)
    capacity7: Option<i64>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OfferMtpasaOfferdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "MTPASA_OFFERDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Offer
/// File Name: Mnsp Offertrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMnspOffertrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// &nbsp; 
    versionno: rust_decimal::Decimal,
    /// &nbsp; 
    participantid: String,
    /// &nbsp; 
    filename: String,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// &nbsp; 
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OfferMnspOffertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "MNSP_OFFERTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Bid
/// File Name: Bidperoffer D
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidBidperofferD2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatchable Unit identifier
    duid: String,
    /// Bid Type Identifier
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// The trading interval period identifier (1-48)
    periodid: Option<rust_decimal::Decimal>,
    /// Version number of offer
    versionno: Option<rust_decimal::Decimal>,
    /// Maximum availability for this BidType in this period
    maxavail: Option<rust_decimal::Decimal>,
    /// Fixed unit output MW (Energy Bids Only).  A value of zero means no fixed load so the unit is dispatched according to bid and market (rather than zero fixed load)
    fixedload: Option<rust_decimal::Decimal>,
    /// MW/min for raise (Energy Bids Only)
    rocup: Option<rust_decimal::Decimal>,
    /// MW/Min for lower (Energy Bids Only)
    rocdown: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which this ancillary service becomes available (AS Only)
    enablementmin: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which this ancillary service can be supplied (AS Only)
    enablementmax: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which the unit can provide the full availability (MAXAVAIL) for this ancillary service (AS Only)
    lowbreakpoint: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which the unit can provide the full availability (MAXAVAIL) for this ancillary service (AS Only)
    highbreakpoint: Option<rust_decimal::Decimal>,
    /// Availability at price band 1
    bandavail1: Option<rust_decimal::Decimal>,
    /// Availability at price band 2
    bandavail2: Option<rust_decimal::Decimal>,
    /// Availability at price band 3
    bandavail3: Option<rust_decimal::Decimal>,
    /// Availability at price band 4
    bandavail4: Option<rust_decimal::Decimal>,
    /// Availability at price band 5
    bandavail5: Option<rust_decimal::Decimal>,
    /// Availability at price band 6
    bandavail6: Option<rust_decimal::Decimal>,
    /// Availability at price band 7
    bandavail7: Option<rust_decimal::Decimal>,
    /// Availability at price band 8
    bandavail8: Option<rust_decimal::Decimal>,
    /// Availability at price band 9
    bandavail9: Option<rust_decimal::Decimal>,
    /// Availability at price band 10
    bandavail10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pasaavailability: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Mandatory Restriction Offer amount
    mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BidBidperofferD2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BID".into(),
                        table_name: "BIDPEROFFER_D".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Offer
/// File Name: Mnsp Peroffer
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMnspPeroffer1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    linkid: String,
    /// Trading Interval number
    periodid: rust_decimal::Decimal,
    /// Maximum planned availability MW
    maxavail: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail1: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail2: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail3: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail4: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail5: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail6: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail7: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail8: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail9: Option<rust_decimal::Decimal>,
    /// Band Availability for current Period
    bandavail10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Inflexibility flag and availability. Fixed unit output MW. A value of zero means no fixed load so the unit is dispatched according to bid and market (rather than zero fixed load) 
    fixedload: Option<rust_decimal::Decimal>,
    /// Ramp rate (MW / min) in the positive direction of flow for this MNSP link for this half-hour period
    rampuprate: Option<rust_decimal::Decimal>,
    /// Allows for future use for energy bids, being the physical plant capability including any capability potentially available within 24 hours
    pasaavailability: Option<rust_decimal::Decimal>,
    /// Mandatory Restriction Offer amount
    mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<OfferMnspPeroffer1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "MNSP_PEROFFER".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Offer
/// File Name: Biddayoffer
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferBiddayoffer2 {
    /// Dispatchable unit identifier
    duid: String,
    /// Bid Type Identifier
    bidtype: String,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Version No. for given offer date
    versionno: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    participantid: Option<String>,
    /// Maximum energy available from Energy Constrained Plant. (Energy Bids Only)
    dailyenergyconstraint: Option<rust_decimal::Decimal>,
    /// Explanation for all rebids and inflexibilities
    rebidexplanation: Option<String>,
    /// Price for Availability Band 1
    priceband1: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 2
    priceband2: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 3
    priceband3: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 4
    priceband4: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 5
    priceband5: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    priceband6: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 6
    priceband7: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 8
    priceband8: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 9
    priceband9: Option<rust_decimal::Decimal>,
    /// Price for Availability Band 10
    priceband10: Option<rust_decimal::Decimal>,
    /// Minimum MW load fast start plant
    minimumload: Option<rust_decimal::Decimal>,
    /// Time to synchronise in minutes (Energy Bids Only)
    t1: Option<rust_decimal::Decimal>,
    /// Time to minimum load in minutes (Energy Bids Only)
    t2: Option<rust_decimal::Decimal>,
    /// Time at minimum load in minutes (Energy Bids Only)
    t3: Option<rust_decimal::Decimal>,
    /// Time to shutdown in minutes (Energy Bids Only)
    t4: Option<rust_decimal::Decimal>,
    /// not used; was ON/OFF for loads (Energy Bids Only)
    normalstatus: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Mandatory Restriction Offer Factor
    mr_factor: Option<rust_decimal::Decimal>,
    /// Daily if processed before BidCutOff of previous day, otherwise REBID
    entrytype: Option<String>,
}
impl crate::GetTable<OfferBiddayoffer2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "OFFER".into(),
                        table_name: "BIDDAYOFFER".into(),
                        version: 2,
                    }
                    
    }
}
