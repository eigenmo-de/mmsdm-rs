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
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * NOTICEID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeParticipantnoticetrk1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Market notice identifier
    pub noticeid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketNoticeParticipantnoticetrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_NOTICE".into(),
                        table_name: "PARTICIPANTNOTICETRK".into(),
                        version: 1,
                    }
                    
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * TYPEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticetype1 {
    /// Identifier for market notice type
    pub typeid: String,
    /// Type description
    pub description: Option<String>,
    /// Not used
    pub raisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketNoticeMarketnoticetype1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_NOTICE".into(),
                        table_name: "MARKETNOTICETYPE".into(),
                        version: 1,
                    }
                    
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Private &amp; Public
/// 
/// # Primary Key Columns
/// 
/// * NOTICEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticedata1 {
    /// Notice Identifier
    pub noticeid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Market Notice Type Identifier (Market - all participants. Participant - selected participants) 
    pub typeid: Option<String>,
    /// Market Notice Type
    pub noticetype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Detail of market notices.
    pub reason: Option<String>,
    /// External Reference for extra data pertaining to market notice
    pub externalreference: Option<String>,
}
impl crate::GetTable<MarketNoticeMarketnoticedata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_NOTICE".into(),
                        table_name: "MARKETNOTICEDATA".into(),
                        version: 1,
                    }
                    
    }
}
