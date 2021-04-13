/// # Summary
/// 
/// ## GENMETER
///  _GENMETER shows details of generator meter sets._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Genmeter
/// * Data Version: 1
/// 
/// # Description
///  GENMETER data is confidential to the relevant participant. Source GENMETER updates only when meter details change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * APPLYDATE
/// * METERID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenmeter1 {
    /// Meter Id
    pub meterid: String,
    /// Generator Set ID
    pub gensetid: Option<String>,
    /// Not used
    pub connectionpointid: Option<String>,
    /// Station Identifier
    pub stationid: Option<String>,
    /// LOAD
    pub metertype: Option<String>,
    /// WATT or AUXILARY
    pub meterclass: Option<String>,
    /// Voltage
    pub voltagelevel: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    pub applydate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// AEMO user authorising
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub comdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub decomdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationGenmeter1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("GENMETER".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENUNITS
///  _GENUNITS shows Genset details for each physical unit with the relevant station.<br>_
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Genunits
/// * Data Version: 2
/// 
/// # Description
///  GENUNITS data is confidential to the relevant participant. Source GENUNITS updates whenever plant details change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * GENSETID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunits2 {
    /// Physical Unit identifier
    pub gensetid: String,
    /// Station Identifier
    pub stationid: Option<String>,
    /// Not used
    pub setlossfactor: Option<rust_decimal::Decimal>,
    /// Centrally dispatched Indicator
    pub cdindicator: Option<String>,
    /// AGC Available flag
    pub agcflag: Option<String>,
    /// Not used
    pub spinningflag: Option<String>,
    /// Voltage level
    pub voltlevel: Option<rust_decimal::Decimal>,
    /// Registered capacity
    pub registeredcapacity: Option<rust_decimal::Decimal>,
    /// Scheduled indicator
    pub dispatchtype: Option<String>,
    /// Fast / Slow / Not Dispatched
    pub starttype: Option<String>,
    /// Market Generator Indicator Flag
    pub mktgeneratorind: Option<String>,
    /// On / Off for load
    pub normalstatus: Option<String>,
    /// Maximum capacity
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Genset type
    pub gensettype: Option<String>,
    /// Genset name
    pub gensetname: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The emissions factor for the generating unit, as calculated by Settlements staff members
    pub co2e_emissions_factor: Option<rust_decimal::Decimal>,
    /// The energy source for the generating unit, as used in the calculation of the CO2-e emissions factor.  Distinct from the Energy Source for a generating unit published as part of the Registration Master List
    pub co2e_energy_source: Option<String>,
    /// An indicator as to the source of the emission factor used in the calculation of the index. The applicable values for this field would be NTNDP which indicates the emission factor is quoted from the National Transmission Network Development Plan or Estimated to indicate the emission factor has been calculated using an internal AEMO procedure based upon the Department of Climate Change and Energy Efficiency NGA factors
    pub co2e_data_source: Option<String>,
}
impl crate::GetTable for ParticipantRegistrationGenunits2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("GENUNITS".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCATEGORY
///  _PARTICIPANTCATEGORY sets out valid participant categories._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantcategory
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCATEGORY is public data, so is available to all participants. Source PARTICIPANTCATEGORY updates as categories change. PARTICIPANTCATEGORY changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTCATEGORYID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategory1 {
    /// Participant category identifier
    pub participantcategoryid: String,
    /// Category description
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantcategory1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("PARTICIPANTCATEGORY".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DUALLOC
///  _DUALLOC cross references dispatch unit identifier to genset ID for each participant._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dualloc
/// * Data Version: 1
/// 
/// # Description
///  Source DUALLOC updates where changed.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * GENSETID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDualloc1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    pub versionno: rust_decimal::Decimal,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Physical unit identifier
    pub gensetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationDualloc1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("DUALLOC".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DUDETAIL
///  _DUDETAIL sets out a records specific details for each unit including start type and whether normally on or off load. Much of this data is information only and is not used in dispatch or settlements._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dudetail
/// * Data Version: 3
/// 
/// # Description
///  DUDETAIL is public data, and is available to all participants. Source DUDETAIL updates only when registration details change. Note To find the current set of details for selected dispatchable units, query the participant's local database as follows. Select du.* from dudetail du where (du.EFFECTIVEDATE, du.VERSIONNO) = ( select effectivedate, max(versionno) from dudetail where EFFECTIVEDATE = (select max(effectivedate) from  dudetail where EFFECTIVEDATE &lt;= sysdate and duid = du.duid and authoriseddate is not null) and duid = du.duid and authoriseddate is not null group by effectivedate ) and du.duid in ('UNIT1', 'UNIT2') ; The following notes apply to this SQL code: ·	 This table is specific to dispatch units only. ·	 If you wish to query details for a different date, substitute a date expression for "sysdate" in the "where EFFECTIVEDATE &lt;= sysdate" clause. ·	 If you wish to list all the units, remove the line "and du.duid in ('UNIT1', 'UNIT2')" ·	 The DUDETAIL table does not indicate if a unit is active;  this is done through ownership (STADUALLOC) by an active station owned by an active participant (STATIONOWNER ) ·	 If you wish to query Station details refer to STATION, STATIONOWNER and STADUALLOC. ·	 If you wish to look at connection point loss factors, refer to TRANSMISSIONLOSSFACTOR.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetail3 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// version of Dispatchable Unit details for this effective date
    pub versionno: rust_decimal::Decimal,
    /// Country wide - Unique id of a connection point
    pub connectionpointid: Option<String>,
    /// Voltage Level
    pub voltlevel: Option<String>,
    /// Registered capacity for normal operations
    pub registeredcapacity: Option<rust_decimal::Decimal>,
    /// AGC Capability flag
    pub agccapability: Option<String>,
    /// Identifies LOAD or GENERATOR
    pub dispatchtype: Option<String>,
    /// Maximum Capacity as used for bid validation
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Identify unit as Fast or Slow
    pub starttype: Option<String>,
    /// For a dispatchable load indicates that the load is normally on or off.
    pub normallyonflag: Option<String>,
    /// Indicates that the physical details for this unit are to be recorded
    pub physicaldetailsflag: Option<String>,
    /// Indicates spinning reserve capability
    pub spinningreserveflag: Option<String>,
    /// User authorising record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Indicate whether a unit is intermittent (e.g. a wind farm)
    pub intermittentflag: Option<String>,
    /// Indicates if the DUID is a Semi-Scheduled Unit
    pub semi_schedule_flag: Option<String>,
    /// Maximum ramp up rate for Unit (Mw/min)
    pub maxrateofchangeup: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min)
    pub maxrateofchangedown: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ParticipantRegistrationDudetail3 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("DUDETAIL".into()),
                        version: 3,
                    }
                    
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCREDITDETAIL
///  _&nbsp; _
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantcreditdetail
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCREDITDETAIL data is confidential to each participant. Source PARTICIPANTCREDITDETAIL updates infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcreditdetail1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// &nbsp; 
    pub participantid: String,
    /// &nbsp; 
    pub creditlimit: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantcreditdetail1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("PARTICIPANTCREDITDETAIL".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DUDETAILSUMMARY
///  _DUDETAILSUMMARY sets out a single summary unit table so reducing the need for participants to use the various dispatchable unit detail and owner tables to establish generating unit specific details._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dudetailsummary
/// * Data Version: 4
/// 
/// # Description
///  DUDETAILSUMMARY is a public table, and is available to all participants. Source DUDETAILSUMMARY updates only when registration details change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * START_DATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetailsummary4 {
    /// Dispatchable Unit Identifier
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub start_date: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub end_date: chrono::NaiveDateTime,
    /// Either Generator or Load
    pub dispatchtype: Option<String>,
    /// Country wide - Unique id of a connection point
    pub connectionpointid: Option<String>,
    /// Region identifier that unit is in
    pub regionid: Option<String>,
    /// Station that unit is in
    pub stationid: Option<String>,
    /// Participant that owns unit during effective record period
    pub participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The transmission level loss factor for currently assigned connection point
    pub transmissionlossfactor: Option<rust_decimal::Decimal>,
    /// Unit start type. At this time restricted to Fast, Slow or Non Dispatched 
    pub starttype: Option<String>,
    /// The distribution loss factor to the currently assigned connection point
    pub distributionlossfactor: Option<rust_decimal::Decimal>,
    /// Floored Offer/Bid Energy Price adjusted for TLF, DLF and MPF
    pub minimum_energy_price: Option<rust_decimal::Decimal>,
    /// Capped Offer/Bid Energy Price adjusted for TLF, DLF and VoLL
    pub maximum_energy_price: Option<rust_decimal::Decimal>,
    /// Scheduled status of the unit:<br>    'SCHEDULED'<br>    'NON-SCHEDULED'<br>    'SEMI-SCHEDULED'
    pub schedule_type: Option<String>,
    /// MW/Min. Calculated Minimum Ramp Rate Up value accepted for Energy Offers or Bids with explanation
    pub min_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// MW/Min. Calculated Minimum Ramp Rate Down value accepted for Energy Offers or Bids with explanation
    pub min_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Maximum ramp up rate for Unit (Mw/min) - from DUDetail table
    pub max_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min) - from DUDetail table
    pub max_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Whether the DUID is classified as an "Aggregated Unit" under the rules. This impacts the Minimum Ramp Rate calculation
    pub is_aggregated: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ParticipantRegistrationDudetailsummary4 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("DUDETAILSUMMARY".into()),
                        version: 4,
                    }
                    
    }
}
/// # Summary
/// 
/// ## STATIONOPERATINGSTATUS
///  _STATIONOPERATINGSTATUS sets out the operating status of each station._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stationoperatingstatus
/// * Data Version: 1
/// 
/// # Description
///  STATIONOWNER is public data, and is available to all participants. Source STATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationoperatingstatus1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique station identifier
    pub stationid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// The operating status of this station, valid values are COMMISSIONED and DECOMMISSIONED
    pub status: Option<String>,
    /// User authorising record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStationoperatingstatus1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("STATIONOPERATINGSTATUS".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## BIDDUIDDETAILSTRK
///  _BIDDUIDDETAILSTRK shows the tracking for the associated object BIDDUIDDETAILS. Together, BIDDUIDDETAILSTRK and BIDDUIDDETAILS define the registration data for each ancillary service a dispatchable unit is registered to provide. The registration data is required to validate a dispatchable unit bid submitted for that ancillary service._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Bidduiddetailstrk
/// * Data Version: 1
/// 
/// # Description
///  BIDDUIDDETAILSTRK data is public to participants. Source BIDDUIDDETAILSTRK updates as dispatchable unit registration details are modified. Volume Approximately 200 records per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetailstrk1 {
    /// Dispatchable unit identifier
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User that authorised record. A NULL value indicates the record is not authorised.
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationBidduiddetailstrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("BIDDUIDDETAILSTRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## BIDDUIDDETAILS
///  _BIDDUIDDETAILS and the associated tracking object BIDDUIDDETAILSTRK define the registration data for each ancillary service a dispatchable unit is registered to provide. The registration data is required to validate a dispatchable unit bid submitted for that ancillary service._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Bidduiddetails
/// * Data Version: 1
/// 
/// # Description
///  BIDDUIDDETAILS data is public to participants. Source BIDDUIDDETAILS updates as dispatchable unit registration details are modified. Volume Approximately 1000 records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetails1 {
    /// Dispatchable unit identifier
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Bid Type Identifier
    pub bidtype: String,
    /// Maximum Capacity of this DUID for this BIDTYPE
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which this ancillary service becomes available (AS Only)
    pub minenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which this ancillary service can be supplied (AS Only)
    pub maxenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the lower end of the ancillary service profile (Degrees)
    pub maxlowerangle: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the upper end of the ancillary service profile (Degrees)
    pub maxupperangle: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationBidduiddetails1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("BIDDUIDDETAILS".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## STATION
///  _STATION sets out valid station identifiers._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Station
/// * Data Version: 1
/// 
/// # Description
///  STATION is public data, and is available to all participants. Source STATION updates whenever there is a station configuration change or new unit registration.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * STATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStation1 {
    /// Station Identifier
    pub stationid: String,
    /// Full name of station
    pub stationname: Option<String>,
    /// Station Address
    pub address1: Option<String>,
    /// Station Address
    pub address2: Option<String>,
    /// Station Address
    pub address3: Option<String>,
    /// Station Address
    pub address4: Option<String>,
    /// City
    pub city: Option<String>,
    /// State of Australia
    pub state: Option<String>,
    /// Post Code
    pub postcode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used. Do not use as the Connection Point Identifier for station load
    pub connectionpointid: Option<String>,
}
impl crate::GetTable for ParticipantRegistrationStation1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("STATION".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## GENUNITS_UNIT
///  _Physical units within a Gen Unit Set_
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Genunits Unit
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
/// * GENSETID
/// * UNIT_GROUPING_LABEL
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunitsUnit1 {
    /// System wide unique Generating Set ID
    pub gensetid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    pub versionno: rust_decimal::Decimal,
    /// Label of Physical Units within the station
    pub unit_grouping_label: String,
    /// Number of units in this Gen Unit grouping
    pub unit_count: Option<rust_decimal::Decimal>,
    /// Nameplate Capacity for each unit in this grouping
    pub unit_size: Option<rust_decimal::Decimal>,
    /// Maximum Capacity for each unit in this grouping
    pub unit_max_size: Option<rust_decimal::Decimal>,
    /// Indicator that Unit is part of an Aggregated Unit (at the DUID level)
    pub aggregation_flag: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationGenunitsUnit1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("GENUNITS_UNIT".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## PARTICIPANTACCOUNT
///  _PARTICIPANTACCOUNT shows financial details on participants._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantaccount
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTACCOUNT data is confidential to the relevant participant. Source PARTICIPANTACCOUNT updates as new participants register or existing participants change details.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantaccount1 {
    /// Name of the account
    pub accountname: Option<String>,
    /// Unique participant identifier
    pub participantid: String,
    /// Account number
    pub accountnumber: Option<String>,
    /// Bank name
    pub bankname: Option<String>,
    /// Bank number
    pub banknumber: Option<rust_decimal::Decimal>,
    /// Branch name
    pub branchname: Option<String>,
    /// Branch number
    pub branchnumber: Option<rust_decimal::Decimal>,
    /// BSB number
    pub bsbnumber: Option<String>,
    /// AEMO credit account number
    pub nemmcocreditaccountnumber: Option<rust_decimal::Decimal>,
    /// AEMO debit account number
    pub nemmcodebitaccountnumber: Option<rust_decimal::Decimal>,
    /// User authorising record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Australian Business Number
    pub abn: Option<String>,
}
impl crate::GetTable for ParticipantRegistrationParticipantaccount1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("PARTICIPANTACCOUNT".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## MNSP_INTERCONNECTOR
///  _MNSP_INTERCONNECTOR sets out attributes of each interconnector._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Mnsp Interconnector
/// * Data Version: 2
/// 
/// # Description
///  MNSP_INTERCONNECTOR data is public, so is available to all participants. Source MNSP_INTERCONNECTOR changes infrequently, typically annually. Volume Twice the number of MNSPs.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * LINKID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspInterconnector2 {
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Interconnector Identifier
    pub interconnectorid: Option<String>,
    /// Nominated source region for Interconnector
    pub fromregion: Option<String>,
    /// Nominated destination region for Interconnector
    pub toregion: Option<String>,
    /// Maximum capacity
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor (redundant from May 2012)
    pub tlf: Option<rust_decimal::Decimal>,
    /// Factor applied to the LHS of constraint equations; set by AEMO
    pub lhsfactor: Option<rust_decimal::Decimal>,
    /// Obsolete; no longer applied. Ignore.
    pub meterflowconstant: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Transmission Loss Factor for Link "From Region" end
    pub from_region_tlf: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor for Link at "To Region" end
    pub to_region_tlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ParticipantRegistrationMnspInterconnector2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("MNSP_INTERCONNECTOR".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## MNSP_PARTICIPANT
///  _MNSP_PARTICIPANT registers MNSP ownership._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Mnsp Participant
/// * Data Version: 1
/// 
/// # Description
///  MNSP_PARTICIPANT data is public, so is available to all participants. Source MNSP_PARTICIPANT updates infrequently, typically annually. Volume Number of MNSPs.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspParticipant1 {
    /// Interconnector Identifier
    pub interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationMnspParticipant1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("MNSP_PARTICIPANT".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCATEGORYALLOC
///  _PARTICIPANTCATEGORYALLOC sets out the assignment of participants to particular categories._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantcategoryalloc
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCATEGORYALLOC data is public, so is available to all participants. Source PARTICIPANTCATEGORYALLOC updates for new participants or when categories change. PARTICIPANTCATEGORYALLOC changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategoryalloc1 {
    /// Category unique identifier
    pub participantcategoryid: String,
    /// Unique participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantcategoryalloc1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("PARTICIPANTCATEGORYALLOC".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## PARTICIPANT
///  _PARTICIPANT sets out Participant ID, name and class for all participants._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participant
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANT is public data, so is available to all participants. Source PARTICIPANT updates as new participants register or existing participants change details.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipant1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Class of participant
    pub participantclassid: Option<String>,
    /// Full name of participant
    pub name: Option<String>,
    /// Not used
    pub description: Option<String>,
    /// Australian Company Number; Nine Numbers XXX-XXX-XXX
    pub acn: Option<String>,
    /// Identifies primary business activity of participant
    pub primarybusiness: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipant1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("PARTICIPANT".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCLASS
///  _PARTICIPANTCLASS sets out valid participant classifications._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantclass
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCLASS data is public, so is available to all participants. Source PARTICIPANTCLASS updates only if classifications change. This table changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTCLASSID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantclass1 {
    /// Class of participant
    pub participantclassid: String,
    /// Description of participant class
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantclass1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("PARTICIPANTCLASS".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DISPATCHABLEUNIT
///  _DISPATCHABLEUNIT sets out the unit name and type of each dispatchable unit in the market._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dispatchableunit
/// * Data Version: 1
/// 
/// # Description
///  DISPATCHABLEUNIT data is public data, and is available to all participants. Source DISPATCHABLEUNIT pdates as new units added or names changed.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDispatchableunit1 {
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Dispatchable Unit full description
    pub duname: Option<String>,
    /// Generation or Load
    pub unittype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationDispatchableunit1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("DISPATCHABLEUNIT".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## STATIONOWNER
///  _STATIONOWNER sets out the owner details of each station._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stationowner
/// * Data Version: 1
/// 
/// # Description
///  STATIONOWNER is public data, and is available to all participants. Source STATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationowner1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: String,
    /// Station Identifier
    pub stationid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStationowner1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("STATIONOWNER".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## STATIONOWNERTRK
///  _STATIONOWNERTRK shows the tracking for the associated object STATIONOWNER. Together, STATIONOWNERTRK and STATIONOWNER sets out the owner details of each station._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stationownertrk
/// * Data Version: 1
/// 
/// # Description
///  STATIONOWNER is public data, and is available to all participants. Source STATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
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
pub struct ParticipantRegistrationStationownertrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// User authorising record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStationownertrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("STATIONOWNERTRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## STADUALLOC
///  _STADUALLOC sets out details on the allocation of dispatchable units to particular sites or stations._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stadualloc
/// * Data Version: 1
/// 
/// # Description
///  STADUALLOC is public data, and is available to all participants. Source STADUALLOC is updated whenever there is a station configuration change or new unit registration.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStadualloc1 {
    /// Dispatchable Unit Identifier
    pub duid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Station Identifier
    pub stationid: String,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStadualloc1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "PARTICIPANT_REGISTRATION".into(),
                        table_name: Some("STADUALLOC".into()),
                        version: 1,
                    }
                    
    }
}
