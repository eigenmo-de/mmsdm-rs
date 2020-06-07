/// Data Set Name: Network
/// File Name: Outagestatuscode
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagestatuscode1 {
    /// A code representing the status of an outage
    outagestatuscode: String,
    /// A description of the status code
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkOutagestatuscode1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "OUTAGESTATUSCODE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Network
/// File Name: Equipmentdetail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkEquipmentdetail1 {
    /// ID uniquely identifying the substation this equipment is located at
    substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    equipmenttype: String,
    /// A unique identifier for this type of equipment at this substation
    equipmentid: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    /// The voltage in KV for this equipment.<br>Transformers may have multiple voltages defined.<br>E.g. 132_110_33<br>
    voltage: Option<String>,
    /// A short description for this equipment.
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkEquipmentdetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "EQUIPMENTDETAIL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Network
/// File Name: Outagedetail
/// Data Version: 3
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagedetail3 {
    /// ID uniquely identifying the outage
    outageid: rust_decimal::Decimal,
    /// The substation this equipment is located at
    substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    equipmenttype: String,
    /// A unique identifier for this equipment at this substation, and based on its type
    equipmentid: String,
    #[serde(with = "crate::mms_datetime")]
    starttime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    endtime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    submitteddate: Option<chrono::NaiveDateTime>,
    /// A code representing the status of the outage.<br>The OUTAGESTATUSCODE table will store a detailed description of each code.<br>
    outagestatuscode: Option<String>,
    /// Changes to an outage key details may require the outage to be resubmitted.<br>A new outage id will then be allocated and the outage will be reassessed.<br>This field will detail the reason for the change.<br>
    resubmitreason: Option<String>,
    /// The new outage id created from a resubmit.
    resubmitoutageid: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the day
    recalltimeday: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the night
    recalltimenight: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The reason provided by the asset owner for this outage
    reason: Option<String>,
    /// 1 = The outage is for a secondary piece of equipment that has an associated constraint set. The transmission equipment is still in service. 0 = The outage is for the transmission equipment
    issecondary: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    actual_starttime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    actual_endtime: Option<chrono::NaiveDateTime>,
    /// The asset owners reference code for this outage
    companyrefcode: Option<String>,
}
impl crate::GetTable<NetworkOutagedetail3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "OUTAGEDETAIL".into(),
                        version: 3,
                    }
                    
    }
}
/// Data Set Name: Network
/// File Name: Rating
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRating1 {
    /// ID defining this data source for use in constraints 
    spd_id: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    /// The region that this rating is for
    regionid: Option<String>,
    /// The substation the equipment is located at
    substationid: Option<String>,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    equipmenttype: Option<String>,
    /// A unique identifier for this equipment at this substation, and based on its type
    equipmentid: Option<String>,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding<br>
    ratinglevel: Option<String>,
    /// One of:<br>1 = Normally uses dynamic ratings<br>0 = No dynamic ratings, static ratings are used<br>
    isdynamic: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkRating1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "RATING".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Network
/// File Name: Realtimerating
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRealtimerating1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// ID defining this data source for use in constraints
    spd_id: String,
    /// The defined equipment rating value in MVA
    ratingvalue: rust_decimal::Decimal,
}
impl crate::GetTable<NetworkRealtimerating1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "REALTIMERATING".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Network
/// File Name: Outageconstraintset
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutageconstraintset1 {
    /// ID uniquely identifying the outage
    outageid: rust_decimal::Decimal,
    /// ID for the constraint set
    genconsetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    startinterval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    endinterval: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkOutageconstraintset1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "OUTAGECONSTRAINTSET".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Network
/// File Name: Substationdetail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkSubstationdetail1 {
    /// ID uniquely identifying this substation
    substationid: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    /// Description of the substation
    description: Option<String>,
    /// The NEM region the substation is in
    regionid: Option<String>,
    /// The TNSP who is responsible for this substation
    ownerid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkSubstationdetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "SUBSTATIONDETAIL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Network
/// File Name: Staticrating
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkStaticrating1 {
    /// The substation the equipment is located at
    substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    equipmenttype: String,
    /// A unique identifier for this type of equipment at this substation
    equipmentid: String,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding
    ratinglevel: String,
    /// The applicationid which defines the application timeframes of this rating.
    applicationid: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    /// The rating value in MVA that applies. This may be positive or negative depending on which side of the nominal MW flow direction the rating value applies.<br>Flow into a transmission device is positive, flow out of the device is negative.<br>
    ratingvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkStaticrating1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "NETWORK".into(),
                        table_name: "STATICRATING".into(),
                        version: 1,
                    }
                    
    }
}
