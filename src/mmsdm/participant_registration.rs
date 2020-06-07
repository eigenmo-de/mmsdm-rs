/// Data Set Name: Participant Registration
/// File Name: Bidduiddetails
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetails1 {
    /// Dispatchable unit identifier
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Record version number
    versionno: rust_decimal::Decimal,
    /// Bid Type Identifier
    bidtype: String,
    /// Maximum Capacity of this DUID for this BIDTYPE
    maxcapacity: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which this ancillary service becomes available (AS Only)
    minenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which this ancillary service can be supplied (AS Only)
    maxenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the lower end of the ancillary service profile (Degrees)
    maxlowerangle: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the upper end of the ancillary service profile (Degrees)
    maxupperangle: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationBidduiddetails1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "BIDDUIDDETAILS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Dudetail
/// Data Version: 3
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetail3 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    duid: String,
    /// version of Dispatchable Unit details for this effective date
    versionno: rust_decimal::Decimal,
    /// Country wide - Unique id of a connection point
    connectionpointid: Option<String>,
    /// Voltage Level
    voltlevel: Option<String>,
    /// Registered capacity for normal operations
    registeredcapacity: Option<rust_decimal::Decimal>,
    /// AGC Capability flag
    agccapability: Option<String>,
    /// Identifies LOAD or GENERATOR
    dispatchtype: Option<String>,
    /// Maximum Capacity as used for bid validation
    maxcapacity: Option<rust_decimal::Decimal>,
    /// Identify unit as Fast or Slow
    starttype: Option<String>,
    /// For a dispatchable load indicates that the load is normally on or off.
    normallyonflag: Option<String>,
    /// Indicates that the physical details for this unit are to be recorded
    physicaldetailsflag: Option<String>,
    /// Indicates spinning reserve capability
    spinningreserveflag: Option<String>,
    /// User authorising record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Indicate whether a unit is intermittent (e.g. a wind farm)
    intermittentflag: Option<String>,
    /// Indicates if the DUID is a Semi-Scheduled Unit
    semi_schedule_flag: Option<String>,
    /// Maximum ramp up rate for Unit (Mw/min)
    maxrateofchangeup: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min)
    maxrateofchangedown: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ParticipantRegistrationDudetail3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "DUDETAIL".into(),
                        version: 3,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Stationowner
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationowner1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    participantid: String,
    /// Station Identifier
    stationid: String,
    /// Version no of record within the effective date
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStationowner1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "STATIONOWNER".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Mnsp Participant
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspParticipant1 {
    /// Interconnector Identifier
    interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data takes precedence
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationMnspParticipant1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "MNSP_PARTICIPANT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Participant
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipant1 {
    /// Unique participant identifier
    participantid: String,
    /// Class of participant
    participantclassid: Option<String>,
    /// Full name of participant
    name: Option<String>,
    /// Not used
    description: Option<String>,
    /// Australian Company Number; Nine Numbers XXX-XXX-XXX
    acn: Option<String>,
    /// Identifies primary business activity of participant
    primarybusiness: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipant1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "PARTICIPANT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Genunits
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunits2 {
    /// Physical Unit identifier
    gensetid: String,
    /// Station Identifier
    stationid: Option<String>,
    /// Not used
    setlossfactor: Option<rust_decimal::Decimal>,
    /// Centrally dispatched Indicator
    cdindicator: Option<String>,
    /// AGC Available flag
    agcflag: Option<String>,
    /// Not used
    spinningflag: Option<String>,
    /// Voltage level
    voltlevel: Option<rust_decimal::Decimal>,
    /// Registered capacity
    registeredcapacity: Option<rust_decimal::Decimal>,
    /// Scheduled indicator
    dispatchtype: Option<String>,
    /// Fast / Slow / Not Dispatched
    starttype: Option<String>,
    /// Market Generator Indicator Flag
    mktgeneratorind: Option<String>,
    /// On / Off for load
    normalstatus: Option<String>,
    /// Maximum capacity
    maxcapacity: Option<rust_decimal::Decimal>,
    /// Genset type
    gensettype: Option<String>,
    /// Genset name
    gensetname: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The emissions factor for the generating unit, as calculated by Settlements staff members
    co2e_emissions_factor: Option<rust_decimal::Decimal>,
    /// The energy source for the generating unit, as used in the calculation of the CO2-e emissions factor.  Distinct from the Energy Source for a generating unit published as part of the Registration Master List
    co2e_energy_source: Option<String>,
    /// An indicator as to the source of the emission factor used in the calculation of the index. The applicable values for this field would be NTNDP which indicates the emission factor is quoted from the National Transmission Network Development Plan or Estimated to indicate the emission factor has been calculated using an internal AEMO procedure based upon the Department of Climate Change and Energy Efficiency NGA factors
    co2e_data_source: Option<String>,
}
impl crate::GetTable<ParticipantRegistrationGenunits2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "GENUNITS".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Dispatchableunit
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDispatchableunit1 {
    /// Dispatchable Unit Identifier
    duid: String,
    /// Dispatchable Unit full description
    duname: Option<String>,
    /// Generation or Load
    unittype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationDispatchableunit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "DISPATCHABLEUNIT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Participantcategoryalloc
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategoryalloc1 {
    /// Category unique identifier
    participantcategoryid: String,
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantcategoryalloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "PARTICIPANTCATEGORYALLOC".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Genmeter
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenmeter1 {
    /// Meter Id
    meterid: String,
    /// Generator Set ID
    gensetid: Option<String>,
    /// Not used
    connectionpointid: Option<String>,
    /// Station Identifier
    stationid: Option<String>,
    /// LOAD
    metertype: Option<String>,
    /// WATT or AUXILARY
    meterclass: Option<String>,
    /// Voltage
    voltagelevel: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    applydate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    versionno: rust_decimal::Decimal,
    /// AEMO user authorising
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    comdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    decomdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationGenmeter1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "GENMETER".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Participantaccount
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantaccount1 {
    /// Name of the account
    accountname: Option<String>,
    /// Unique participant identifier
    participantid: String,
    /// Account number
    accountnumber: Option<String>,
    /// Bank name
    bankname: Option<String>,
    /// Bank number
    banknumber: Option<rust_decimal::Decimal>,
    /// Branch name
    branchname: Option<String>,
    /// Branch number
    branchnumber: Option<rust_decimal::Decimal>,
    /// BSB number
    bsbnumber: Option<String>,
    /// AEMO credit account number
    nemmcocreditaccountnumber: Option<rust_decimal::Decimal>,
    /// AEMO debit account number
    nemmcodebitaccountnumber: Option<rust_decimal::Decimal>,
    /// User authorising record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Australian Business Number
    abn: Option<String>,
}
impl crate::GetTable<ParticipantRegistrationParticipantaccount1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "PARTICIPANTACCOUNT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Stationownertrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationownertrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    participantid: String,
    /// Version no of record within the effective date
    versionno: rust_decimal::Decimal,
    /// User authorising record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStationownertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "STATIONOWNERTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Participantcategory
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategory1 {
    /// Participant category identifier
    participantcategoryid: String,
    /// Category description
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantcategory1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "PARTICIPANTCATEGORY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Station
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStation1 {
    /// Station Identifier
    stationid: String,
    /// Full name of station
    stationname: Option<String>,
    /// Station Address
    address1: Option<String>,
    /// Station Address
    address2: Option<String>,
    /// Station Address
    address3: Option<String>,
    /// Station Address
    address4: Option<String>,
    /// City
    city: Option<String>,
    /// State of Australia
    state: Option<String>,
    /// Post Code
    postcode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used. Do not use as the Connection Point Identifier for station load
    connectionpointid: Option<String>,
}
impl crate::GetTable<ParticipantRegistrationStation1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "STATION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Participantcreditdetail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcreditdetail1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// &nbsp; 
    participantid: String,
    /// &nbsp; 
    creditlimit: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantcreditdetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "PARTICIPANTCREDITDETAIL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Stadualloc
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStadualloc1 {
    /// Dispatchable Unit Identifier
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Station Identifier
    stationid: String,
    /// Version no of this record for the effective date
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStadualloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "STADUALLOC".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Bidduiddetailstrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetailstrk1 {
    /// Dispatchable unit identifier
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Record version number
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// User that authorised record. A NULL value indicates the record is not authorised.
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationBidduiddetailstrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "BIDDUIDDETAILSTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Participantclass
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantclass1 {
    /// Class of participant
    participantclassid: String,
    /// Description of participant class
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantclass1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "PARTICIPANTCLASS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Mnsp Interconnector
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspInterconnector2 {
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    linkid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    versionno: rust_decimal::Decimal,
    /// Interconnector Identifier
    interconnectorid: Option<String>,
    /// Nominated source region for Interconnector
    fromregion: Option<String>,
    /// Nominated destination region for Interconnector
    toregion: Option<String>,
    /// Maximum capacity
    maxcapacity: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor (redundant from May 2012)
    tlf: Option<rust_decimal::Decimal>,
    /// Factor applied to the LHS of constraint equations; set by AEMO
    lhsfactor: Option<rust_decimal::Decimal>,
    /// Obsolete; no longer applied. Ignore.
    meterflowconstant: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Transmission Loss Factor for Link "From Region" end
    from_region_tlf: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor for Link at "To Region" end
    to_region_tlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ParticipantRegistrationMnspInterconnector2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "MNSP_INTERCONNECTOR".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Genunits Unit
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunitsUnit1 {
    /// System wide unique Generating Set ID
    gensetid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    versionno: rust_decimal::Decimal,
    /// Label of Physical Units within the station
    unit_grouping_label: String,
    /// Number of units in this Gen Unit grouping
    unit_count: Option<rust_decimal::Decimal>,
    /// Nameplate Capacity for each unit in this grouping
    unit_size: Option<rust_decimal::Decimal>,
    /// Maximum Capacity for each unit in this grouping
    unit_max_size: Option<rust_decimal::Decimal>,
    /// Indicator that Unit is part of an Aggregated Unit (at the DUID level)
    aggregation_flag: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationGenunitsUnit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "GENUNITS_UNIT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Dudetailsummary
/// Data Version: 4
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetailsummary4 {
    /// Dispatchable Unit Identifier
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    start_date: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    end_date: chrono::NaiveDateTime,
    /// Either Generator or Load
    dispatchtype: Option<String>,
    /// Country wide - Unique id of a connection point
    connectionpointid: Option<String>,
    /// Region identifier that unit is in
    regionid: Option<String>,
    /// Station that unit is in
    stationid: Option<String>,
    /// Participant that owns unit during effective record period
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The transmission level loss factor for currently assigned connection point
    transmissionlossfactor: Option<rust_decimal::Decimal>,
    /// Unit start type. At this time restricted to Fast, Slow or Non Dispatched 
    starttype: Option<String>,
    /// The distribution loss factor to the currently assigned connection point
    distributionlossfactor: Option<rust_decimal::Decimal>,
    /// Floored Offer/Bid Energy Price adjusted for TLF, DLF and MPF
    minimum_energy_price: Option<rust_decimal::Decimal>,
    /// Capped Offer/Bid Energy Price adjusted for TLF, DLF and VoLL
    maximum_energy_price: Option<rust_decimal::Decimal>,
    /// Scheduled status of the unit:<br>    'SCHEDULED'<br>    'NON-SCHEDULED'<br>    'SEMI-SCHEDULED'
    schedule_type: Option<String>,
    /// MW/Min. Calculated Minimum Ramp Rate Up value accepted for Energy Offers or Bids with explanation
    min_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// MW/Min. Calculated Minimum Ramp Rate Down value accepted for Energy Offers or Bids with explanation
    min_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Maximum ramp up rate for Unit (Mw/min) - from DUDetail table
    max_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min) - from DUDetail table
    max_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Whether the DUID is classified as an "Aggregated Unit" under the rules. This impacts the Minimum Ramp Rate calculation
    is_aggregated: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ParticipantRegistrationDudetailsummary4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "DUDETAILSUMMARY".into(),
                        version: 4,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Stationoperatingstatus
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationoperatingstatus1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Unique station identifier
    stationid: String,
    /// Version no of record within the effective date
    versionno: rust_decimal::Decimal,
    /// The operating status of this station, valid values are COMMISSIONED and DECOMMISSIONED
    status: Option<String>,
    /// User authorising record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStationoperatingstatus1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "STATIONOPERATINGSTATUS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Participant Registration
/// File Name: Dualloc
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDualloc1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    versionno: rust_decimal::Decimal,
    /// Dispatchable Unit identifier
    duid: String,
    /// Physical unit identifier
    gensetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationDualloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: "DUALLOC".into(),
                        version: 1,
                    }
                    
    }
}
