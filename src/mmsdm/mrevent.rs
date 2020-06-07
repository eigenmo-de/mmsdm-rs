/// Data Set Name: Mr
/// File Name: Peroffer Stack
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrPerofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// LAOF Stack order
    stack_position: rust_decimal::Decimal,
    /// Trade Period for the MR Offer
    periodid: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID. Only required here for CSV reports
    duid: Option<String>,
    /// MR Capacity to be Dispatched
    accepted_capacity: Option<rust_decimal::Decimal>,
    /// Requested capacity reduction amount
    deducted_capacity: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrPerofferStack1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: "PEROFFER_STACK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Mr
/// File Name: Event Schedule
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEventSchedule1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    demand_effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    demand_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to ResDemandTrk.VersionNo
    demand_versionno: Option<rust_decimal::Decimal>,
    /// Authorised person confirming Offer Stack (AKA Acceptance)
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrEventSchedule1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: "EVENT_SCHEDULE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Mr
/// File Name: Dayoffer Stack
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrDayofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    /// Loss Adjusted Offer Factor Stack order starting at 1
    stack_position: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID
    duid: Option<String>,
    /// Confirms the unit is allowed to Contribute MR Capacity
    authorised: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_settlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to XXXX_DayOffer.VersionNo
    offer_versionno: Option<rust_decimal::Decimal>,
    /// Source tables - ENERGY or MNSP
    offer_type: Option<String>,
    /// Loss Adjusted Offer Factor = TLF times MR_Factor
    laof: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrDayofferStack1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: "DAYOFFER_STACK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Mr
/// File Name: Event
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEvent1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    regionid: String,
    /// Description of MR
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Ignored - Tracking purpose only
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_cut_off_time: Option<chrono::NaiveDateTime>,
    /// Flag:1 = MR settlement figures locked. Do not recalculate, ·&nbsp;&nbsp;&nbsp;0 = MR settlements to be recalculated
    settlement_complete: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrEvent1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: "EVENT".into(),
                        version: 1,
                    }
                    
    }
}
