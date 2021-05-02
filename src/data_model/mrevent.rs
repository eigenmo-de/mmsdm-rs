/// # Summary
/// 
/// ## MR_PEROFFER_STACK
///  _MR_PEROFFER_STACK defines the accepted capacity on a period basis for the Acceptance Schedule, is a child table to MR_DayOffer_Stack and only includes records or units with accepted_capacity &gt; 0 for the specific period.<br>_
/// 
/// * Data Set Name: Mr
/// * File Name: Peroffer Stack
/// * Data Version: 1
/// 
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack. MR_PEROFFER_STACK reports the accepted MR capacity (Accepted_Capacity) required from each unit for each trading interval. MR_PEROFFER_STACK is sparse so lists only units with accepted capacity &gt; 0 for that trading interval.  The Deducted_Capacity field allows the tracking and implementation of participant requested reductions to accepted MR capacity to be tracked and applied. MR_PEROFFER_STACK is reported confidentially to each participant to notify acceptance of an MR offer. Source MR_PEROFFER_STACK updates are ad hoc. Volume 4800 rows per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * MR_DATE
/// * PERIODID
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrPerofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// LAOF Stack order
    pub stack_position: rust_decimal::Decimal,
    /// Trade Period for the MR Offer
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID. Only required here for CSV reports
    pub duid: Option<String>,
    /// MR Capacity to be Dispatched
    pub accepted_capacity: Option<rust_decimal::Decimal>,
    /// Requested capacity reduction amount
    pub deducted_capacity: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrPerofferStack1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: Some("PEROFFER_STACK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## MR_DAYOFFER_STACK
///  _MR_DAYOFFER_STACK defines the Stack order for each version of the Acceptance Schedule, including all units submitting MR offers for that event. MR_DAYOFFER_STACK is the child to MR_EVENT_SCHEDULE, and parent to MR_PEROFFER_STACK._
/// 
/// * Data Set Name: Mr
/// * File Name: Dayoffer Stack
/// * Data Version: 1
/// 
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_EVENT_SCHEDULE, MR_DAYOFFER_STACK and MR_PEROFFER_STACK. MR_DAYOFFER_STACK sets includes all generators/MNSPs in the region that submitted an MR offer and a primary key reference to the Offer tables to identify the specific offer used for that unit. MR_DAYOFFER_STACK also includes a Stack Order, irrespective of whether the unit is required to meet the Schedule. MR_DAYOFFER_STACK updates are confidential on day of submission, with public exposure the next day. Source MR_DAYOFFER_STACK updates are ad hoc. Volume 100 rows per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * MR_DATE
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrDayofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Loss Adjusted Offer Factor Stack order starting at 1
    pub stack_position: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID
    pub duid: Option<String>,
    /// Confirms the unit is allowed to Contribute MR Capacity
    pub authorised: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_settlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to XXXX_DayOffer.VersionNo
    pub offer_versionno: Option<rust_decimal::Decimal>,
    /// Source tables - ENERGY or MNSP
    pub offer_type: Option<String>,
    /// Loss Adjusted Offer Factor = TLF times MR_Factor
    pub laof: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrDayofferStack1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: Some("DAYOFFER_STACK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## MR_EVENT
///  _MR_EVENT defines an MR Event for a given region on a specific trading date.<br>_
/// 
/// * Data Set Name: Mr
/// * File Name: Event
/// * Data Version: 1
/// 
/// # Description
///  MR_EVENT defines a mandatory restriction event for a given region and trading date (04:30 to 04:00). Data within MR_EVENT includes the cut-off time for submission of MR offers for this event and a notification that the settlements figures are locked due to results from an independent expert being engaged to allocate settlement of a significant shortfall. If mandatory restrictions are defined in two regions on the same trading day, two MR events are defined. MR_EVENT data is public, so is available to all participants. Source MR_EVENT updates are ad hoc. Volume 1 Row per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * MR_DATE
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEvent1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    /// Description of MR
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Ignored - Tracking purpose only
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_cut_off_time: Option<chrono::NaiveDateTime>,
    /// Flag:1 = MR settlement figures locked. Do not recalculate, ·&nbsp;&nbsp;&nbsp;0 = MR settlements to be recalculated
    pub settlement_complete: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrEvent1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: Some("EVENT".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## MR_EVENT_SCHEDULE
///  _MR_EVENT_SCHEDULE defines the Stack version of the Acceptance Schedule and is the parent table to MR_DayOffer_Stack and MR_PerOffer_Stack.<br>_
/// 
/// * Data Set Name: Mr
/// * File Name: Event Schedule
/// * Data Version: 1
/// 
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack table. The MR_EVENT_SCHEDULE table determines the existence of an MR offer acceptance stack for a specific MR schedule of an MR event. The MR_EVENT_SCHEDULE table also tracks the time each stack is exercised. MR_EVENT_SCHEDULE is public and notifies the market that a new offer stack has been created. Source MR_EVENT_SCHEDULE updates are ad hoc. Volume 2 Rows per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * MR_DATE
/// * REGIONID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEventSchedule1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub demand_effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub demand_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to ResDemandTrk.VersionNo
    pub demand_versionno: Option<rust_decimal::Decimal>,
    /// Authorised person confirming Offer Stack (AKA Acceptance)
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrEventSchedule1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MR".into(),
                        table_name: Some("EVENT_SCHEDULE".into()),
                        version: 1,
                    }
                    
    }
}
