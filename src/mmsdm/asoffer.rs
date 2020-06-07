/// Data Set Name: Asoffer
/// File Name: Offerrestartdata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrestartdata1 {
    /// Contract identifier
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    /// Version No of contract
    versionno: rust_decimal::Decimal,
    /// Available load
    availability: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    authorisedby: Option<String>,
    /// Name of reoffer file
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    periodid: rust_decimal::Decimal,
}
impl crate::GetTable<AsofferOfferrestartdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "ASOFFER".into(),
                        table_name: "OFFERRESTARTDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Asoffer
/// File Name: Offerlsheddata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferlsheddata1 {
    /// Contract identifier
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version No of reoffer
    versionno: rust_decimal::Decimal,
    /// Available load
    availableload: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    authorisedby: Option<String>,
    /// Name of reoffer file
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    periodid: rust_decimal::Decimal,
}
impl crate::GetTable<AsofferOfferlsheddata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "ASOFFER".into(),
                        table_name: "OFFERLSHEDDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Asoffer
/// File Name: Offeragcdata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferagcdata1 {
    /// Contract Identifier
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    versionno: rust_decimal::Decimal,
    /// Availability flag (0 or 1)
    availability: Option<rust_decimal::Decimal>,
    /// Upper control limit. This is used by SPD.
    upperlimit: Option<rust_decimal::Decimal>,
    /// Lower control limit MW. This is used by SPD.
    lowerlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    authorisedby: Option<String>,
    /// Name of reoffer file
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    periodid: rust_decimal::Decimal,
    /// AGC Ramp Rate Up. This is used by SPD.
    agcup: Option<rust_decimal::Decimal>,
    /// AGC Ramp Rate Down. This is used by SPD.
    agcdown: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<AsofferOfferagcdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "ASOFFER".into(),
                        table_name: "OFFERAGCDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Asoffer
/// File Name: Offerrpowerdata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrpowerdata1 {
    /// Contract Version No.
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version No. of Re-Offer
    versionno: rust_decimal::Decimal,
    /// Market trading interval
    periodid: rust_decimal::Decimal,
    /// Availability of service
    availability: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVar)
    mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVar)
    mtg: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// User Name
    authorisedby: Option<String>,
    /// File name of Re-Offer file
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<AsofferOfferrpowerdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "ASOFFER".into(),
                        table_name: "OFFERRPOWERDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Asoffer
/// File Name: Offerastrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferastrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version of the offer for that date
    versionno: rust_decimal::Decimal,
    /// Participant ID
    participantid: String,
    /// Submitted file name.
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<AsofferOfferastrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "ASOFFER".into(),
                        table_name: "OFFERASTRK".into(),
                        version: 1,
                    }
                    
    }
}
