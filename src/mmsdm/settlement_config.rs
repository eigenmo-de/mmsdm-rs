/// Data Set Name: Settlement Config
/// File Name: Marketfeedata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeedata1 {
    /// Identifier for Market Fee
    marketfeeid: String,
    /// Version of fees for this id
    marketfeeversionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Market fee rate/MWh, a dollar amount
    marketfeevalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketfeedata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "MARKETFEEDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Marketfee
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfee1 {
    /// Identifier for Market Fee
    marketfeeid: String,
    /// Period type - PERIOD, DAILY, WEEKLY
    marketfeeperiod: Option<String>,
    /// Type - MW or $
    marketfeetype: Option<String>,
    /// Description of market fee
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp; 
    gl_tcode: Option<String>,
    /// &nbsp; 
    gl_financialcode: Option<String>,
    /// &nbsp; 
    fee_class: Option<String>,
}
impl crate::GetTable<SettlementConfigMarketfee1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "MARKETFEE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Market Fee Cat Excl
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExcl1 {
    /// The excluded market fee
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// Participant category to be excluded from this market fee
    participant_categoryid: String,
}
impl crate::GetTable<SettlementConfigMarketFeeCatExcl1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "MARKET_FEE_CAT_EXCL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Setcfg Participant Mpftrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpftrk1 {
    /// Participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    versionno: rust_decimal::Decimal,
    /// Authorising user
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigSetcfgParticipantMpftrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "SETCFG_PARTICIPANT_MPFTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Setcfg
/// File Name: Reallocation
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocation2 {
    /// Reallocation identifier
    reallocationid: String,
    /// The participant to be credited for the reallocation
    creditparticipantid: Option<String>,
    /// The participant to be debited for the reallocation
    debitparticipantid: Option<String>,
    /// Region identifier, being the spot price reference node for this reallocation
    regionid: Option<String>,
    /// $, (Quantity) Mwh, SWAP, CAP or FLOOR
    agreementtype: Option<String>,
    /// Optional reference detail for credit participant
    creditreference: Option<String>,
    /// Optional reference detail for debit participant
    debitreference: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    /// Reallocation state. One of SUBMITTED, AUTHORISED, CANCELLED.
    current_stepid: Option<String>,
    /// The day type profile for which the reallocation applies over the start and end date range. Valid entries are BUSINESS, NON_BUSINESS or FLAT.
    daytype: Option<String>,
    /// Denotes a Credit or Debit reallocation with a value of "C" or "D" respectively
    reallocation_type: Option<String>,
    /// Unique ID of the calendar for which data is requested
    calendarid: Option<String>,
    /// The length of settlement intervals (in minutes) in the reallocation profile
    intervallength: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SetcfgReallocation2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETCFG".into(),
                        table_name: "REALLOCATION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Market Fee Exclusion Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusionTrk1 {
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    versionno: rust_decimal::Decimal,
    /// User authorising record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketFeeExclusionTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "MARKET_FEE_EXCLUSION_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Market Fee Exclusion
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusion1 {
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    versionno: rust_decimal::Decimal,
    /// Identifier for Market Fee
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketFeeExclusion1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "MARKET_FEE_EXCLUSION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Marketfeetrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeetrk1 {
    /// Version of fees for this ID
    marketfeeversionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// User authorising record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketfeetrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "MARKETFEETRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Ancillary Recovery Split
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigAncillaryRecoverySplit1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version number of the record for the given date. 
    versionno: rust_decimal::Decimal,
    /// Ancillary service name (e.g. AGC, FCASCOMP)
    service: String,
    /// A payment type associated with the service (can be ENABLING, AVAILABILITY, USAGE, or COMPENSATION). 
    paymenttype: String,
    /// The percentage value of the recovery funded by market customers. 
    customer_portion: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigAncillaryRecoverySplit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "ANCILLARY_RECOVERY_SPLIT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Market Fee Cat Excl Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExclTrk1 {
    /// The excluded market fee
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketFeeCatExclTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "MARKET_FEE_CAT_EXCL_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Participant Bandfee Alloc
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigParticipantBandfeeAlloc1 {
    /// Unique participant identifier
    participantid: String,
    /// Identifier for Market Fee
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Period identifier
    versionno: rust_decimal::Decimal,
    /// The participant category that the market fee recovery amount pertains to.
    participantcategoryid: String,
    /// The value of this market fee
    marketfeevalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigParticipantBandfeeAlloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "PARTICIPANT_BANDFEE_ALLOC".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Setcfg
/// File Name: Reallocationinterval
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocationinterval1 {
    /// Reallocation identifier
    reallocationid: String,
    /// Period identifier (1..48)
    periodid: i64,
    /// Reallocation value in the units of the agreement type
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Nominated Reallocation Price, only used in agreement types of SWAP, CAP and FLOOR, being the contract strike price in $/MWh
    nrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SetcfgReallocationinterval1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETCFG".into(),
                        table_name: "REALLOCATIONINTERVAL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Setcfg Participant Mpf
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpf1 {
    /// Participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    versionno: rust_decimal::Decimal,
    /// Participant Category
    participantcategoryid: String,
    /// Connection point identifier
    connectionpointid: String,
    /// Market Participation Factor
    mpf: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigSetcfgParticipantMpf1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "SETCFG_PARTICIPANT_MPF".into(),
                        version: 1,
                    }
                    
    }
}
