/// # Summary
///
/// ## CONTRACTAGC
///  _CONTRACTAGC shows Automatic Generation Control (AGC) contract details for each dispatchable unit. There is a separate contract for each unit._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractagc
/// * Data Version: 1
///
/// # Description
///  CONTRACTAGC data is confidential to the relevant participant. Source CONTRACTAGC updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractagc1 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: Option<String>,
    /// Control Range Raise 5 Min MW
    pub crr: Option<rust_decimal::Decimal>,
    /// Control Range Lower 5 Min MW
    pub crl: Option<rust_decimal::Decimal>,
    /// Enabling Price in $
    pub rlprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap in $
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Block Size
    pub bs: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractagc1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTAGC".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## CONTRACTLOADSHED
///  _CONTRACTLOADSHED shows Governor contract details used in the settlement and dispatch of this service. Note: services are dispatched as 6 and 60 raise Frequency Control Ancillary Services (FCAS). Mandatory requirements and breakpoint details are not used for load shed._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractloadshed
/// * Data Version: 2
///
/// # Description
///  CONTRACTLOADSHED data is confidential to the relevant participant. Source CONTRACTLOADSHED updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractloadshed2 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No.
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: Option<String>,
    /// The load shed enabling price for this contract
    pub lseprice: Option<rust_decimal::Decimal>,
    /// Minimum Compensation price
    pub mcpprice: Option<rust_decimal::Decimal>,
    /// Price Tendered for Compensation per Trading interval - Not used since 13/12/1998
    pub tenderedprice: Option<rust_decimal::Decimal>,
    /// Load Shed Control Range
    pub lscr: Option<rust_decimal::Decimal>,
    /// SPD scaling factor for load shed vs dispatched, (1 = dispatched)
    pub ilscalingfactor: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower60secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower60secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower6secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower6secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise60secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise60seccapacity: Option<rust_decimal::Decimal>,
    /// Maximum 60 second raise
    pub raise60secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise6secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise6seccapacity: Option<rust_decimal::Decimal>,
    /// Limit Equation Raise 6 Second Maximum MW
    pub raise6secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub price6secraisemandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant6secraisemandatory: Option<rust_decimal::Decimal>,
    /// Contract Price for 6 Second Raise
    pub price6secraisecontract: Option<rust_decimal::Decimal>,
    /// Contract Quantity for 6 Second Raise
    pub quant6secraisecontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60secraisemandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60secraisemandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60secraisecontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60secraisecontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub price6seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant6seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub price6seclowercontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant6seclowercontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60seclowercontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60seclowercontract: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The NMAS default payment amount
    pub default_testingpayment_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub service_start_date: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractloadshed2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTLOADSHED".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## CONTRACTREACTIVEPOWER
///  _CONTRACTREACTIVEPOWER shows Reactive Power contract details used in the settlement and dispatch of this service._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractreactivepower
/// * Data Version: 4
///
/// # Description
///  CONTRACTREACTIVEPOWER data is confidential to the relevant participant. Source CONTRACTREACTIVEPOWER updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractreactivepower4 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No.
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: Option<String>,
    /// Sync Compensation Flag - Y for SYNCCOMP
    pub synccompensation: Option<String>,
    /// Availability price per MVAr of RP absorption capability
    pub mvaraprice: Option<rust_decimal::Decimal>,
    /// Enabling price
    pub mvareprice: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP generation capability
    pub mvargprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVAr)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVAr)
    pub mtg: Option<rust_decimal::Decimal>,
    /// Minimum Capability for MVAr Absorption required by Code
    pub mmca: Option<rust_decimal::Decimal>,
    /// Minimum Capability for MVAr Generation required by Code
    pub mmcg: Option<rust_decimal::Decimal>,
    /// Estimated Power consumption of unit when operating on SYNCCOMP
    pub eu: Option<rust_decimal::Decimal>,
    /// Estimated Price for supply
    pub pp: Option<rust_decimal::Decimal>,
    /// Block Size of Unit
    pub bs: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The NMAS default payment amount
    pub default_testingpayment_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub service_start_date: Option<chrono::NaiveDateTime>,
    /// The MWh the unit must produce in a trading interval to be eligible for an excess-to-gen availability payment
    pub availability_mwh_threshold: Option<rust_decimal::Decimal>,
    /// The threshold value for MegaVar (MVAr) to check whether the service is fully available.
    pub mvar_threshold: Option<rust_decimal::Decimal>,
    /// The maximum capped amount for the rebate payment.
    pub rebate_cap: Option<rust_decimal::Decimal>,
    /// The per MVAR rebate amount used to calculate the rebate payment.
    pub rebate_amount_per_mvar: Option<rust_decimal::Decimal>,
    /// Used to check whether the contract is eligible for rebate. For new NSCAS contracts to apply new payment methodology this flag is 1.
    pub isrebateapplicable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for AncilliaryServicesContractreactivepower4 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTREACTIVEPOWER".into()),
            version: 4,
        }
    }
}
/// # Summary
///
/// ## CONTRACTRESTARTSERVICES
///  _CONTRACTRESTARTSERVICES shows Restart Services contract details used in the settlement and dispatch of this service._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractrestartservices
/// * Data Version: 2
///
/// # Description
///  CONTRACTRESTARTSERVICES data is confidential to the participant holding the contract. Source CONTRACTRESTARTSERVICES updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractrestartservices2 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No.
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Restart Type - 0 = BlackStart, 1 = Combination, 2 = Trip To House
    pub restarttype: Option<rust_decimal::Decimal>,
    /// Availability Price
    pub rcprice: Option<rust_decimal::Decimal>,
    /// Trip To House Level
    pub triptohouselevel: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The NMAS default payment amount
    pub default_testingpayment_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub service_start_date: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractrestartservices2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTRESTARTSERVICES".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## CONTRACTRESTARTUNITS
///  _CONTRACTRESTARTUNITS shows Restart units provided under a system restart contract. A service can have multiple units._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractrestartunits
/// * Data Version: 1
///
/// # Description
///  CONTRACTRESTARTUNITS data is confidential to each participant with a restart contract. Source CONTRACTRESTARTUNITS updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * DUID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractrestartunits1 {
    /// Contract Identifier
    pub contractid: String,
    /// Version No of contract
    pub versionno: rust_decimal::Decimal,
    /// Dispatchable Unit identifier
    pub duid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractrestartunits1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTRESTARTUNITS".into()),
            version: 1,
        }
    }
}
