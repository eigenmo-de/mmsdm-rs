/// # Summary
/// 
/// ## NETWORK_SUBSTATIONDETAIL
///  _NETWORK_SUBSTATIONDETAIL sets out the attributes of sub-stations across time_
/// 
/// * Data Set Name: Network
/// * File Name: Substationdetail
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkSubstationdetail1 {
    /// ID uniquely identifying this substation
    pub substationid: String,
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// Description of the substation
    pub description: Option<String>,
    /// The NEM region the substation is in
    pub regionid: Option<String>,
    /// The TNSP who is responsible for this substation
    pub ownerid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkSubstationdetail1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("SUBSTATIONDETAIL".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NETWORK_RATING
///  _NETWORK_RATING defines a list of the equipment ratings that may be used as inputs to market constraints.<br>If the rating is flagged as dynamic then in real-time the rating will be dynamically determined and the static value will be used as a fallback value should the dynamic value fail.<br>Note:<br>In some rare cases equipment has ratings provided from more than one TNSP. This is identified by a different SPD Id. The value used in the NEM is normally the more restrictive of the two values.<br>_
/// 
/// * Data Set Name: Network
/// * File Name: Rating
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * SPD_ID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRating1 {
    /// ID defining this data source for use in constraints 
    pub spd_id: String,
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// The region that this rating is for
    pub regionid: Option<String>,
    /// The substation the equipment is located at
    pub substationid: Option<String>,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: Option<String>,
    /// A unique identifier for this equipment at this substation, and based on its type
    pub equipmentid: Option<String>,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding<br>
    pub ratinglevel: Option<String>,
    /// One of:<br>1 = Normally uses dynamic ratings<br>0 = No dynamic ratings, static ratings are used<br>
    pub isdynamic: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkRating1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("RATING".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NETWORK_OUTAGEDETAIL
///  _Lists asset owners planned outages for transmission equipment. This also includes details for transmission equipment that will not have an outage, but associated secondary equipment has an outage and a related constraint set may be invoked. This scenario is indicated by the ISSECONDARY field in the table_
/// 
/// * Data Set Name: Network
/// * File Name: Outagedetail
/// * Data Version: 3
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * OUTAGEID
/// * STARTTIME
/// * SUBSTATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagedetail3 {
    /// ID uniquely identifying the outage
    pub outageid: rust_decimal::Decimal,
    /// The substation this equipment is located at
    pub substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: String,
    /// A unique identifier for this equipment at this substation, and based on its type
    pub equipmentid: String,
    #[serde(with = "crate::mms_datetime")]
    pub starttime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub endtime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub submitteddate: Option<chrono::NaiveDateTime>,
    /// A code representing the status of the outage.<br>The OUTAGESTATUSCODE table will store a detailed description of each code.<br>
    pub outagestatuscode: Option<String>,
    /// Changes to an outage key details may require the outage to be resubmitted.<br>A new outage id will then be allocated and the outage will be reassessed.<br>This field will detail the reason for the change.<br>
    pub resubmitreason: Option<String>,
    /// The new outage id created from a resubmit.
    pub resubmitoutageid: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the day
    pub recalltimeday: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the night
    pub recalltimenight: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The reason provided by the asset owner for this outage
    pub reason: Option<String>,
    /// 1 = The outage is for a secondary piece of equipment that has an associated constraint set. The transmission equipment is still in service. 0 = The outage is for the transmission equipment
    pub issecondary: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub actual_starttime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub actual_endtime: Option<chrono::NaiveDateTime>,
    /// The asset owners reference code for this outage
    pub companyrefcode: Option<String>,
}
impl crate::GetTable for NetworkOutagedetail3 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("OUTAGEDETAIL".into()),
                        version: 3,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NETWORK_REALTIMERATING
///  _The NETWORK_REALTIMERATING table shows the equipment rating values in MVA used as inputs to constraints in the dispatch solution. This includes values for both static and dynamic ratings. The NETWORK_RATING table can be used to determine the physical equipment the rating is for based on the SPD_ID value._
/// 
/// * Data Set Name: Network
/// * File Name: Realtimerating
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * SETTLEMENTDATE
/// * SPD_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRealtimerating1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// ID defining this data source for use in constraints
    pub spd_id: String,
    /// The defined equipment rating value in MVA
    pub ratingvalue: rust_decimal::Decimal,
}
impl crate::GetTable for NetworkRealtimerating1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("REALTIMERATING".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NETWORK_EQUIPMENTDETAIL
///  _NETWORK_EQUIPMENTDETAIL Provides details on equipment that may have outages or ratings. A single piece of equipment may have multiple records if its details change.<br>A line will typically have at least two valid records at a time, once for each end of the line.<br>_
/// 
/// * Data Set Name: Network
/// * File Name: Equipmentdetail
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkEquipmentdetail1 {
    /// ID uniquely identifying the substation this equipment is located at
    pub substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: String,
    /// A unique identifier for this type of equipment at this substation
    pub equipmentid: String,
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// The voltage in KV for this equipment.<br>Transformers may have multiple voltages defined.<br>E.g. 132_110_33<br>
    pub voltage: Option<String>,
    /// A short description for this equipment.
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkEquipmentdetail1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("EQUIPMENTDETAIL".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NETWORK_OUTAGESTATUSCODE
///  _NETWORK_OUTAGESTATUSCODE describes the different outage status codes_
/// 
/// * Data Set Name: Network
/// * File Name: Outagestatuscode
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * OUTAGESTATUSCODE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagestatuscode1 {
    /// A code representing the status of an outage
    pub outagestatuscode: String,
    /// A description of the status code
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkOutagestatuscode1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("OUTAGESTATUSCODE".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NETWORK_OUTAGECONSTRAINTSET
///  _NETWORK_OUTAGECONSTRAINTSET lists the Constraint Set or Sets that are expected to be invoked for the outage once it is confirmed to proceed._
/// 
/// * Data Set Name: Network
/// * File Name: Outageconstraintset
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * GENCONSETID
/// * OUTAGEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutageconstraintset1 {
    /// ID uniquely identifying the outage
    pub outageid: rust_decimal::Decimal,
    /// ID for the constraint set
    pub genconsetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startinterval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub endinterval: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkOutageconstraintset1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("OUTAGECONSTRAINTSET".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## NETWORK_STATICRATING
///  _NETWORK_STATICRATING lists the static rating values that will apply for a Rating Application ID.<br>This data does not provide information for when the rating actually applies in the NEM. This is dependent on the Rating Application definition.<br>For information on the Rating Applications please refer to the information published on the AEMO website under the topic "Transmission Equipment Ratings". The Rating Applications are referred to as Alternate Value Application Ratings.<br>Ratings that normally use dynamic values will also have static rating values defined. These are used as a fallback if the dynamic rating fails.<br>_
/// 
/// * Data Set Name: Network
/// * File Name: Staticrating
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * APPLICATIONID
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * RATINGLEVEL
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkStaticrating1 {
    /// The substation the equipment is located at
    pub substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: String,
    /// A unique identifier for this type of equipment at this substation
    pub equipmentid: String,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding
    pub ratinglevel: String,
    /// The applicationid which defines the application timeframes of this rating.
    pub applicationid: String,
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// The rating value in MVA that applies. This may be positive or negative depending on which side of the nominal MW flow direction the rating value applies.<br>Flow into a transmission device is positive, flow out of the device is negative.<br>
    pub ratingvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkStaticrating1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: Some("STATICRATING".into()),
                        version: 1,
                    }
                    
    }
}
