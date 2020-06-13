/// # Summary
///
/// ## OFFERRESTARTDATA
///  _OFFERRESTARTDATA sets out reoffers of system restart availability._
///
/// * Data Set Name: Asoffer
/// * File Name: Offerrestartdata
/// * Data Version: 1
///
/// # Description
///  OFFERRESTARTDATA data is confidential to the relevant participant. Source OFFERRESTARTDATA updates as reoffers process.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * OFFERDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrestartdata1 {
    /// Contract identifier
    pub contractid: String,
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Version No of contract
    pub versionno: rust_decimal::Decimal,
    /// Available load
    pub availability: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    /// Name of reoffer file
    pub filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
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
/// # Summary
///
/// ## OFFERAGCDATA
///  _OFFERAGCDATA shows availability reoffers of Automatic Generation Control. _
///
/// * Data Set Name: Asoffer
/// * File Name: Offeragcdata
/// * Data Version: 1
///
/// # Description
///  OFFERAGCDATA data is confidential to the relevant participant. Source OFFERAGCDATA updates as reoffers submitted.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferagcdata1 {
    /// Contract Identifier
    pub contractid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    pub versionno: rust_decimal::Decimal,
    /// Availability flag (0 or 1)
    pub availability: Option<rust_decimal::Decimal>,
    /// Upper control limit. This is used by SPD.
    pub upperlimit: Option<rust_decimal::Decimal>,
    /// Lower control limit MW. This is used by SPD.
    pub lowerlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    /// Name of reoffer file
    pub filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
    /// AGC Ramp Rate Up. This is used by SPD.
    pub agcup: Option<rust_decimal::Decimal>,
    /// AGC Ramp Rate Down. This is used by SPD.
    pub agcdown: Option<rust_decimal::Decimal>,
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
/// # Summary
///
/// ## OFFERASTRK
///  _OFFERASTRK tracks successfully acknowledged ancillary service reoffers._
///
/// * Data Set Name: Asoffer
/// * File Name: Offerastrk
/// * Data Version: 1
///
/// # Description
///  OFFERASTRK data is confidential to the relevant participant. Source OFFERASTRK is updated as offers are successfully acknowledged.
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
pub struct AsofferOfferastrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of the offer for that date
    pub versionno: rust_decimal::Decimal,
    /// Participant ID
    pub participantid: String,
    /// Submitted file name.
    pub filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
///
/// ## OFFERRPOWERDATA
///  _OFFERRPOWERDATA shows reoffers of reactive power capability and settlement measurements._
///
/// * Data Set Name: Asoffer
/// * File Name: Offerrpowerdata
/// * Data Version: 1
///
/// # Description
///  OFFERRPOWERDATA data is confidential to the relevant participant. Source OFFERRPOWERDATA updates as reoffers process.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrpowerdata1 {
    /// Contract Version No.
    pub contractid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No. of Re-Offer
    pub versionno: rust_decimal::Decimal,
    /// Market trading interval
    pub periodid: rust_decimal::Decimal,
    /// Availability of service
    pub availability: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVar)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVar)
    pub mtg: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User Name
    pub authorisedby: Option<String>,
    /// File name of Re-Offer file
    pub filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
///
/// ## OFFERLSHEDDATA
///  _OFFERLSHEDDATA shows reoffers of load shed including available load shed quantity._
///
/// * Data Set Name: Asoffer
/// * File Name: Offerlsheddata
/// * Data Version: 1
///
/// # Description
///  OFFERLSHEDDATA data is confidential to the relevant participant. Source OFFERLSHEDDATA updates as reoffers process.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferlsheddata1 {
    /// Contract identifier
    pub contractid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No of reoffer
    pub versionno: rust_decimal::Decimal,
    /// Available load
    pub availableload: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    /// Name of reoffer file
    pub filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
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
