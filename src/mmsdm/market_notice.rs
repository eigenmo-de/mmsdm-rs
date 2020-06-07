/// Data Set Name: Market Notice
/// File Name: Participantnoticetrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeParticipantnoticetrk1 {
    /// Unique participant identifier
    participantid: String,
    /// Market notice identifier
    noticeid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Notice
/// File Name: Marketnoticetype
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticetype1 {
    /// Identifier for market notice type
    typeid: String,
    /// Type description
    description: Option<String>,
    /// Not used
    raisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Notice
/// File Name: Marketnoticedata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticedata1 {
    /// Notice Identifier
    noticeid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    /// Market Notice Type Identifier (Market - all participants. Participant - selected participants) 
    typeid: Option<String>,
    /// Market Notice Type
    noticetype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Detail of market notices.
    reason: Option<String>,
    /// External Reference for extra data pertaining to market notice
    externalreference: Option<String>,
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
