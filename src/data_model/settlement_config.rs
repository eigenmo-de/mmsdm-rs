/// # Summary
///
/// ## ANCILLARY_RECOVERY_SPLIT
///  _ANCILLARY_RECOVERY_SPLIT holds the actual customer portion for each service and payment type. A single EFFECTIVEDATE/VERSIONNO combination applies to all services (i.e. the latest EFFECTIVEDATE/VERSIONNO is not retrieved for a single service, but applies to a data set)._
///
/// * Data Set Name: Settlement Config
/// * File Name: Ancillary Recovery Split
/// * Data Version: 1
///
/// # Description
///  ANCILLARY_RECOVERY_SPLIT is public data, and is available to all participants. Source This table is updated infrequently.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PAYMENTTYPE
/// * SERVICE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigAncillaryRecoverySplit1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the record for the given date.
    pub versionno: rust_decimal::Decimal,
    /// Ancillary service name (e.g. AGC, FCASCOMP)
    pub service: String,
    /// A payment type associated with the service (can be ENABLING, AVAILABILITY, USAGE, or COMPENSATION).
    pub paymenttype: String,
    /// The percentage value of the recovery funded by market customers.
    pub customer_portion: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigAncillaryRecoverySplit1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("ANCILLARY_RECOVERY_SPLIT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKETFEE
///  _MARKETFEE sets out fee type and period for each market fee._
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfee
/// * Data Version: 1
///
/// # Description
///  MARKETFEE data is public, so is available to all participants. Source MARKETFEE updates when fees change.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * MARKETFEEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfee1 {
    /// Identifier for Market Fee
    pub marketfeeid: String,
    /// Period type - PERIOD, DAILY, WEEKLY
    pub marketfeeperiod: Option<String>,
    /// Type - MW or $
    pub marketfeetype: Option<String>,
    /// Description of market fee
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub gl_tcode: Option<String>,
    /// &nbsp;
    pub gl_financialcode: Option<String>,
    /// &nbsp;
    pub fee_class: Option<String>,
}
impl crate::GetTable for SettlementConfigMarketfee1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKETFEE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKETFEEDATA
///  _MARKETFEEDATA sets out actual fee rates, as adjusted from time to time._
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfeedata
/// * Data Version: 1
///
/// # Description
///  MARKETFEEDATA is public data, and is available to all participants. Source MARKETFEEDATA updates whenever fee rates change.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * MARKETFEEVERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeedata1 {
    /// Identifier for Market Fee
    pub marketfeeid: String,
    /// Version of fees for this id
    pub marketfeeversionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Market fee rate/MWh, a dollar amount
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketfeedata1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKETFEEDATA".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKETFEETRK
///  _MARKETFEETRK sets out versions of each market fee used and its effective date._
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfeetrk
/// * Data Version: 1
///
/// # Description
///  MARKETFEETRK data is public, so is available to all participants. Source MARKETFEETRK updated infrequently, when new annual rates must be inserted. Volume One record inserted per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEVERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeetrk1 {
    /// Version of fees for this ID
    pub marketfeeversionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// User authorising record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketfeetrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKETFEETRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKET_FEE_CAT_EXCL
///  _Market fee exclusions for participant categories. _
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Cat Excl
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANT_CATEGORYID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExcl1 {
    /// The excluded market fee
    pub marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Participant category to be excluded from this market fee
    pub participant_categoryid: String,
}
impl crate::GetTable for SettlementConfigMarketFeeCatExcl1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_CAT_EXCL".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKET_FEE_CAT_EXCL_TRK
///  _Tracking table for market fee exclusions for participant categories._
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Cat Excl Trk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExclTrk1 {
    /// The excluded market fee
    pub marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketFeeCatExclTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_CAT_EXCL_TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKET_FEE_EXCLUSION
///  _MARKET_FEE_EXCLUSION shows the list of market fees from which a participant is excluded from funding after a particular settlement date._
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Exclusion
/// * Data Version: 1
///
/// # Description
///  MARKET_FEE_EXCLUSION data is confidential to the relevant participant. Source MARKET_FEE_EXCLUSION updates only on change of participant configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusion1 {
    /// Unique participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    pub versionno: rust_decimal::Decimal,
    /// Identifier for Market Fee
    pub marketfeeid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketFeeExclusion1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_EXCLUSION".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## MARKET_FEE_EXCLUSIONTRK
///  _MARKET_FEE_EXCLUSIONTRK shows authorisation details of participant market fee exclusion data sets._
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Exclusion Trk
/// * Data Version: 1
///
/// # Description
///  MARKET_FEE_EXCLUSIONTRK is confidential to the participant. Source MARKET_FEE_EXCLUSIONTRK updates only on change of participant configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusionTrk1 {
    /// Unique participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    pub versionno: rust_decimal::Decimal,
    /// User authorising record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketFeeExclusionTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_EXCLUSION_TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## PARTICIPANT_BANDFEE_ALLOC
///  _PARTICIPANT_BANDFEE_ALLOC shows the market fee for each Participant/Participant Category over time._
///
/// * Data Set Name: Settlement Config
/// * File Name: Participant Bandfee Alloc
/// * Data Version: 1
///
/// # Description
///  Source This view updates only on change of participant configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigParticipantBandfeeAlloc1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Identifier for Market Fee
    pub marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Period identifier
    pub versionno: rust_decimal::Decimal,
    /// The participant category that the market fee recovery amount pertains to.
    pub participantcategoryid: String,
    /// The value of this market fee
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigParticipantBandfeeAlloc1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("PARTICIPANT_BANDFEE_ALLOC".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## REALLOCATION
///  _The REALLOCATION table shows the financial transactions agreed between two participants that are settled through the AEMO pool settlements process._
///
/// * Data Set Name: Setcfg
/// * File Name: Reallocation
/// * Data Version: 2
///
/// # Description
///  Note The column REALLOCATION_TYPE can be used in conjunction with CREDITPARTICIPANT or DEBITPARTICIPANT to determine who submitted a reallocation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * REALLOCATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocation2 {
    /// Reallocation identifier
    pub reallocationid: String,
    /// The participant to be credited for the reallocation
    pub creditparticipantid: Option<String>,
    /// The participant to be debited for the reallocation
    pub debitparticipantid: Option<String>,
    /// Region identifier, being the spot price reference node for this reallocation
    pub regionid: Option<String>,
    /// $, (Quantity) Mwh, SWAP, CAP or FLOOR
    pub agreementtype: Option<String>,
    /// Optional reference detail for credit participant
    pub creditreference: Option<String>,
    /// Optional reference detail for debit participant
    pub debitreference: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Reallocation state. One of SUBMITTED, AUTHORISED, CANCELLED.
    pub current_stepid: Option<String>,
    /// The day type profile for which the reallocation applies over the start and end date range. Valid entries are BUSINESS, NON_BUSINESS or FLAT.
    pub daytype: Option<String>,
    /// Denotes a Credit or Debit reallocation with a value of "C" or "D" respectively
    pub reallocation_type: Option<String>,
    /// Unique ID of the calendar for which data is requested
    pub calendarid: Option<String>,
    /// The length of settlement intervals (in minutes) in the reallocation profile
    pub intervallength: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SetcfgReallocation2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETCFG".into(),
            table_name: Some("REALLOCATION".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## REALLOCATIONINTERVAL
///  _30-minute or (5-minute for 5MS) data comprising a single reallocation transaction._
///
/// * Data Set Name: Setcfg
/// * File Name: Reallocationinterval
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REALLOCATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocationinterval1 {
    /// Reallocation identifier
    pub reallocationid: String,
    /// Trading Interval
    pub periodid: i64,
    /// Reallocation value in the units of the agreement type
    pub value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Nominated Reallocation Price, only used in agreement types of SWAP, CAP and FLOOR, being the contract strike price in $/MWh
    pub nrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SetcfgReallocationinterval1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETCFG".into(),
            table_name: Some("REALLOCATIONINTERVAL".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SETCFG_PARTICIPANT_MPF
///  _SETCFG_PARTICIPANT_MPF shows the Market Participation Factors (MPF) for each participant for each connection point. The MPF values are used to determine recovery amounts for regulation FCAS._
///
/// * Data Set Name: Settlement Config
/// * File Name: Setcfg Participant Mpf
/// * Data Version: 1
///
/// # Description
///  SETCFG_PARTICIPANT_MPF data is available to all participants. Volume Approximately 20,000 records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpf1 {
    /// Participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    pub versionno: rust_decimal::Decimal,
    /// Participant Category
    pub participantcategoryid: String,
    /// Connection point identifier
    pub connectionpointid: String,
    /// Market Participation Factor
    pub mpf: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigSetcfgParticipantMpf1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("SETCFG_PARTICIPANT_MPF".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SETCFG_PARTICIPANT_MPFTRK
///  _SETCFG_PARTICIPANT_MPFTRK is the tracking table for Market Participation Factors (MPF) data stored in the SETCFG_PARTICIPANT_MPF table for each participant._
///
/// * Data Set Name: Settlement Config
/// * File Name: Setcfg Participant Mpftrk
/// * Data Version: 1
///
/// # Description
///  SETCFG_PARTICIPANT_MPFTRK data is public, so is available to all participants. Volume Approximately 2,000 records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpftrk1 {
    /// Participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    pub versionno: rust_decimal::Decimal,
    /// Authorising user
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigSetcfgParticipantMpftrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("SETCFG_PARTICIPANT_MPFTRK".into()),
            version: 1,
        }
    }
}
