/// # Summary
/// 
/// ## SETINTERVENTIONRECOVERY
///  _SETINTERVENTIONRECOVERY shows intervention recovery details by participant._
/// 
/// * Data Set Name: Settlements
/// * File Name: Interventionrecovery
/// * Data Version: 5
/// 
/// # Description
///  Status SETINTERVENTIONRECOVERY became unused when Ancillary Services Review was implemented. For more details, see Change Notice 126. Confidential to participant Source Unused; was updating when intervention occurred in a billing run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsInterventionrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    pub periodid: rust_decimal::Decimal,
    /// Intervention Contract Identifier
    pub contractid: String,
    /// Regional Recovery Flag
    pub rcf: Option<char>,
    /// Unique participant identifier
    pub participantid: String,
    /// Demand of Participant in Region/Market
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total Demand of Region/Market
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Payment to Generator for Intervention
    pub interventionpayment: Option<rust_decimal::Decimal>,
    /// Retailer Payment to Pool for Intervention
    pub interventionamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Region Identifier
    pub regionid: Option<String>,
}
impl crate::GetTable for SettlementsInterventionrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("INTERVENTIONRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETCPDATA
///  _SETCPDATA shows meter settlement data for each connection point. This is the key view for retailers to verify energy charges. A regional summary view is also provided. As the view has values for each connection point by period, for each meter data file, it is a very large view._
/// 
/// * Data Set Name: Settlements
/// * File Name: Cpdata
/// * Data Version: 6
/// 
/// # Description
///  The Connection point details (in SETCPDATA) are confidential to the participant and host retailer that the connection points relate to. By comparison, the regional data (SETCPDATAREGION) is publically available. Source SETCPDATA updates with each Settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * MDA
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * TCPID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdata6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Connection point identifier
    pub tcpid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Import Gross energy into the pool - MWh
    pub igenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub xgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh, plus UFEA if the UFEA amount is positive. When GS commences, this includes the UFEA amount in the settlement runs.
    pub inenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh, plus (UFEA * -1) if the UFEA amount is negative. When GS commences, this includes the UFEA amount in the settlement runs.
    pub xnenergy: Option<rust_decimal::Decimal>,
    /// Import reactive power
    pub ipower: Option<rust_decimal::Decimal>,
    /// Export reactive power
    pub xpower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Export - Import of Net energy (MWh)
    pub ta: Option<rust_decimal::Decimal>,
    /// settlement amount in $ for trading period
    pub ep: Option<rust_decimal::Decimal>,
    /// Not used
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    /// Meter Run Number = version number of the meter file
    pub meterrunno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used
    pub hostdistributor: Option<String>,
    /// Metering Data Agent
    pub mda: String,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval, excluding any UFEA component.
    pub afe: Option<rust_decimal::Decimal>,
    /// Sum of ME- for all NMIs at this Market Customer FRMP and TNI in the Settlements Trading Interval.
    pub dme: Option<rust_decimal::Decimal>,
    /// Share of UFE allocated to this FRMP and TNI in the Settlements Trading Interval.
    pub ufea: Option<rust_decimal::Decimal>,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval. When GS commences, this includes the UFEA amount in the settlement runs.
    pub age: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsCpdata6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("CPDATA".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_MR_RECOVERY
///  _SET_MR_RECOVERY shows the trading interval recovery charges on a dispatchable unit basis for spot market income from dispatch of MR capacity._
/// 
/// * Data Set Name: Settlements
/// * File Name: Mr Recovery
/// * Data Version: 5
/// 
/// # Description
///  SET_MR_RECOVERY data is confidential to the relevant participant. Source SET_MR_RECOVERY updates are ad hoc, being for  MR events only. Volume 24000 rows per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrRecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    pub versionno: rust_decimal::Decimal,
    /// Unique Region Identifier
    pub regionid: String,
    /// Unique Participant identifier
    pub participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    pub duid: String,
    /// Calendar day Trading Interval number
    pub periodid: rust_decimal::Decimal,
    /// Accepted Restriction Offer Dispatched Energy Factor
    pub arodef: Option<rust_decimal::Decimal>,
    /// The amount payable to AEMO for that accepted restriction offer and trading interval
    pub nta: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsMrRecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("MR_RECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_NMAS_RECOVERY
///  _SET_NMAS_RECOVERY sets out the NSCAS recovery data for payments other than testing._
/// 
/// * Data Set Name: Settlements
/// * File Name: Nmas Recovery
/// * Data Version: 2
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PARTICIPANTID
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecovery2 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    pub participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>- RESTART
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    pub paymenttype: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all  benefitting regions 
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Participant energy in MWh for the period
    pub participant_energy: Option<rust_decimal::Decimal>,
    /// The RegionId energy in MWh for the period
    pub region_energy: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the PARTICIPANTID and REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant Generator Energy in the benefitting region
    pub participant_generation: Option<rust_decimal::Decimal>,
    /// The generator energy in the benefitting region
    pub region_generation: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to customers
    pub recovery_amount_customer: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to generators
    pub recovery_amount_generator: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsNmasRecovery2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("NMAS_RECOVERY".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETMARKETFEES
///  _SETMARKETFEES shows payments for market fees for each settlement date._
/// 
/// * Data Set Name: Settlements
/// * File Name: Marketfees
/// * Data Version: 6
/// 
/// # Description
///  SETMARKETFEES is confidential data. Source SETMARKETFEES updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMarketfees6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub runno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Market fee identifier (e.g. V_EST)
    pub marketfeeid: String,
    /// Fee charge
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    /// Energy amount for variable fees
    pub energy: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The participant category that the market fee recovery pertains to. Corresponds to the PARTICIPANTCATEGORYID column of the PARTICIPANT_BANDFEE_CATEGORYALLOC_C_V view for BAND$ type fees, or to the MARKETFEETYPE column of the MARKETFEE_P_V view for all other fee types.
    pub participantcategoryid: String,
    /// The rate applied to this fee for the participant at the settlement date
    pub feerate: Option<rust_decimal::Decimal>,
    /// The number of units applicable to this fee for the participant, in the trading interval.
    pub feeunits: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsMarketfees6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("MARKETFEES".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETLOCALAREATNI
///  _SETLOCALAREATNI shows the list of TNIs constituent to a local area in a settlement run. _
/// 
/// * Data Set Name: Settlements
/// * File Name: Localareatni
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * LOCALAREAID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
/// * TNI
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLocalareatni1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number of the settlement run
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the local area
    pub localareaid: String,
    /// Unique identifier for a TNI constituent to the local area as at the settlement run
    pub tni: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsLocalareatni1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("LOCALAREATNI".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_NMAS_RECOVERY_RBF
///  _SET_NMAS_RECOVERY_RBF publishes the RBF for NSCAS non testing payments on a half hourly basis._
/// 
/// * Data Set Name: Settlements
/// * File Name: Nmas Recovery Rbf
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecoveryRbf1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    pub paymenttype: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all benefitting regions 
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsNmasRecoveryRbf1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("NMAS_RECOVERY_RBF".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## DAYTRACK
///  _DAYTRACK identifies the actual settlement run processed for each settlement day. Settlement run is in the column EXPOSTRUNNO. Generally the number of the settlement run used in the latest statement is the maximum number._
/// 
/// * Data Set Name: Settlements
/// * File Name: Daytrack
/// * Data Version: 6
/// 
/// # Description
///  DAYTRACK is a public data, and is available to all participants. Source DAYTRACK is populated by the posting of a billing run. Volume Daily billing runs insert one row per day. A non-interim statement has seven records inserted per week. An indicative maximum is 35 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EXPOSTRUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsDaytrack6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Not Used
    pub regionid: Option<String>,
    /// Not Used
    pub exanterunstatus: Option<String>,
    /// Not Used
    pub exanterunno: Option<rust_decimal::Decimal>,
    /// Not Used
    pub expostrunstatus: Option<String>,
    /// Settlement Run No
    pub expostrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Length of a settlement interval, in minutes (was 30 minutes, will be 5 minutes starting the commencement of 5MS rule change date).
    pub settlementintervallength: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsDaytrack6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("DAYTRACK".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETIRPARTSURPLUS
///  _This view supports the Settlements Residue Auction, holding the participant allocations of IRSurplus._
/// 
/// * Data Set Name: Settlements
/// * File Name: Irpartsurplus
/// * Data Version: 6
/// 
/// # Description
///  SETIRPARTSURPLUS data is confidential to each participant. Source SETIRPARTSURPLUS updates with each settlement run. Volume SETIRPARTSURPLUS contains a maximum of 20 million records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrpartsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Ancillary Service Contract
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    pub participantid: String,
    /// Identifier of the Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total value of surplus before allocation
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Allocated percentage to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsIrpartsurplus6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("IRPARTSURPLUS".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETLSHEDPAYMENT
///  _SETLSHEDPAYMENT shows specific payment details for load shed services by period._
/// 
/// * Data Set Name: Settlements
/// * File Name: Lshedpayment
/// * Data Version: 5
/// 
/// # Description
///  SETLSHEDPAYMENT data is confidential to the relevant participant. Source SETLSHEDPAYMENT updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLshedpayment5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// AS Contract Identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Price
    pub lseprice: Option<rust_decimal::Decimal>,
    /// Minimum Compensation Price
    pub mcpprice: Option<rust_decimal::Decimal>,
    /// Load Shed Control Range
    pub lscr: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Payment
    pub lsepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit at time of load shed usage
    pub constrainedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit at time of load shed usage
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Amount of load shed
    pub als: Option<rust_decimal::Decimal>,
    /// Initial demand of unit at time of load shed usage
    pub initialdemand: Option<rust_decimal::Decimal>,
    /// Final demand of unit at time of load shed usage
    pub finaldemand: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Payment amount for the Load Shed Availability service
    pub availabilitypayment: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsLshedpayment5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("LSHEDPAYMENT".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_APC_COMPENSATION
///  _APC Compensation payment amounts in the Settlements timeframe_
/// 
/// * Data Set Name: Settlements
/// * File Name: Apc Compensation
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * APEVENTID
/// * CLAIMID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcCompensation1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: String,
    /// Trading interval identifier
    pub periodid: i64,
    /// Compensation amount for the event claim in this interval
    pub compensation_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsApcCompensation1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("APC_COMPENSATION".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETLULOADRECOVERY
///  _SETLULOADRECOVERY shows reimbursements for rapid-unit-load Ancillary Services to be recovered from participants._
/// 
/// * Data Set Name: Settlements
/// * File Name: Luloadrecovery
/// * Data Version: 5
/// 
/// # Description
///  SETLULOADRECOVERY became unused when Ancillary Services Review was implemented. For more details, see Change Notice 126. SETLULOADRECOVERY data is confidential to each participant. Source SETLULOADRECOVERY is unused; was updated with each settlement run. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLuloadrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: String,
    /// AS Contract ID
    pub contractid: Option<String>,
    /// Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Usage Payment
    pub usagepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub compensationpayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total Regional Demand
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    pub enablingrecovery: Option<rust_decimal::Decimal>,
    /// Usage Recovery
    pub usagerecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    pub compensationrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Enabling Recovery for Generator
    pub enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Usage Recovery for Generator
    pub usagerecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    pub compensationrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total Regional Demand for Generator
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsLuloadrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("LULOADRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETRESTARTRECOVERY
///  _SETRESTARTRECOVERY shows reimbursements for system restart Ancillary Services to be recovered from participants. (Data no longer created for Settlement Days from 01/07/2012)_
/// 
/// * Data Set Name: Settlements
/// * File Name: Restartrecovery
/// * Data Version: 6
/// 
/// # Description
///  SETRESTARTRECOVERY data is confidential to the relevant participant. Source SETRESTARTRECOVERY updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartrecovery6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: String,
    /// Contract Identifier
    pub contractid: Option<String>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// NEM Demand (NB sum of ALL Regions)
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Availability Recovery
    pub availabilityrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Availability Recovery for Generator
    pub availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Sum of all generation including SGA generation across all regions of the NEM and floored to zero
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
    /// The enabling payment made for system restart in this half-hour interval
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to customer activity
    pub enablingrecovery: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to generator activity
    pub enablingrecovery_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsRestartrecovery6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("RESTARTRECOVERY".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETVICENERGYFLOW
///  _SETVICENERGYFLOW is used in settlement of Victorian Vesting contracts._
/// 
/// * Data Set Name: Settlements
/// * File Name: Vicenergyflow
/// * Data Version: 5
/// 
/// # Description
///  SETVICENERGYFLOW data is public, so is available to all participants. Source SETVICENERGYFLOW updates daily, with settlements
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicenergyflow5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number
    pub versionno: rust_decimal::Decimal,
    /// Settlement period
    pub periodid: rust_decimal::Decimal,
    /// Net metered energy flowing across the V-SN and V-SA interconnectors
    pub netflow: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsVicenergyflow5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("VICENERGYFLOW".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETLOCALAREAENERGY
///  _SETLOCALAREAENERGY shows the UFE, AGE and associated values for each local area and trading interval in a settlement run._
/// 
/// * Data Set Name: Settlements
/// * File Name: Localareaenergy
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * LOCALAREAID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLocalareaenergy1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number of the settlement run
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the local area
    pub localareaid: String,
    /// Settlement Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Total unaccounted-for energy for the local area in this trading interval, in MWh
    pub ufe: Option<rust_decimal::Decimal>,
    /// DDME component of UFE for the local area in this trading interval, in MWh.
    pub ddme: Option<rust_decimal::Decimal>,
    /// TME component of UFE for the local area in this trading interval, in MWh.
    pub tme: Option<rust_decimal::Decimal>,
    /// ADME component of UFE for the local area in this trading interval, in MWh.
    pub adme: Option<rust_decimal::Decimal>,
    /// The sum of all DME amounts for each Market Customer FRMP and TNI in the local area, in this trading interval.
    pub admela: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsLocalareaenergy1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("LOCALAREAENERGY".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETIRAUCSURPLUS
///  _This view supports the Settlements Residue Auction, by holding the NSP participant allocations of IRSurplus arising as a result of the unsold units for a quarter._
/// 
/// * Data Set Name: Settlements
/// * File Name: Iraucsurplus
/// * Data Version: 6
/// 
/// # Description
///  SETIRAUCSURPLUS data is confidential to the relevant participant. Source SETIRAUCSURPLUS updates with each settlement run. Volume SETIRAUCSURPLUS contains a maximum of 10 million records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIraucsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total value of surplus before allocation
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage allocated to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsIraucsurplus6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("IRAUCSURPLUS".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETGENDATA
///  _SETGENDATA shows meter settlement data for each generation meter point. A regional summary is also provided._
/// 
/// * Data Set Name: Settlements
/// * File Name: Gendata
/// * Data Version: 6
/// 
/// # Description
///  SETGENDATA shows generator meter details, and SETGENDATA data is confidential to the participant. By comparison, the regional summary (SETGENDATAREGION) is public data. Source SETGENDATA updates with each Settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * GENSETID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendata6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Station Identifier
    pub stationid: String,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Physical unit identifier
    pub gensetid: String,
    /// Region Identifier
    pub regionid: String,
    /// Generated energy
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used
    pub apower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Net energy (MWh)
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Administered Price Compensation
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Export Energy (Generator Purchases) (MWh)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// Identifier of the meter run used in this settlement calculation
    pub meterrunno: Option<rust_decimal::Decimal>,
    /// Metering Data Agent
    pub mda: Option<String>,
    /// Secondary Transmission Loss Factor
    pub secondary_tlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsGendata6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("GENDATA".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_APC_RECOVERY
///  _APC Compensation recovery amounts in the Settlements timeframe_
/// 
/// * Data Set Name: Settlements
/// * File Name: Apc Recovery
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * APEVENTID
/// * CLAIMID
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcRecovery1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: String,
    /// Settlements Trading Interval.
    pub periodid: i64,
    /// Region id for the recovery amount
    pub regionid: String,
    /// Recovery amount in the region attributable to the participant for the event claim in this interval
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// Total Recovery amount in the region for the event claim in this interval
    pub region_recovery_br_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsApcRecovery1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("APC_RECOVERY".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETFCASCOMP
///  _SETFCASCOMP shows the compensation details for Frequency Controlled Ancillary Services (FCAS). These compensation values are calculated by a separate “what if” run of the LP Solver and entered as an unconstrained MW value into settlements._
/// 
/// * Data Set Name: Settlements
/// * File Name: Fcascomp
/// * Data Version: 5
/// 
/// # Description
///  SETFCASCOMP data is confidential to the relevant participant Source SETFCASCOMP updates with each Settlement run, if required.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcascomp5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Dispatchable Unit ID
    pub duid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Period Identifier
    pub periodid: rust_decimal::Decimal,
    /// Compensation Cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Cleared MW of Unit in First Dispatch period in Trading Interval
    pub clearedmw: Option<rust_decimal::Decimal>,
    /// Initial MW of Unit in First Dispatch period in Trading Interval
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    pub ebp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor of Unit
    pub tlf: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Generation Payment in trading interval
    pub excessgen: Option<rust_decimal::Decimal>,
    /// Frequency Control AS Compensation payment to Generator
    pub fcascomp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsFcascomp5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("FCASCOMP".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_FCAS_RECOVERY
///  _SET_FCAS_RECOVERY shows reimbursements for the Frequency Control Ancillary Services (FCAS) to be recovered from participants. Beware of potential confusion with the table SETFCASRECOVERY, which reports reimbursements for Frequency Control Ancillary Services Compensation (now unused)._
/// 
/// * Data Set Name: Settlements
/// * File Name: Fcas Recovery
/// * Data Version: 6
/// 
/// # Description
///  SET_FCAS_RECOVERY data is confidential to the relevant participant. Volume Approximately 1, 500, 000 per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasRecovery6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: String,
    /// Participant identifier
    pub participantid: String,
    /// Region Identifier
    pub regionid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Recovery amount for the Lower 6 Second service attributable to customer connection points
    pub lower6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to customer connection points
    pub raise6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to customer connection points
    pub lower60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to customer connection points
    pub raise60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to customer connection points
    pub lower5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to customer connection points
    pub raise5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to customer connection points
    pub lowerreg_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to customer connection points
    pub raisereg_recovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Recovery amount for the Lower 6 Second service attributable to generator connection points
    pub lower6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to generator connection points
    pub raise6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to generator connection points
    pub lower60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to generator connection points
    pub raise60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to generator connection points
    pub lower5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to generator connection points
    pub raise5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to generator connection points
    pub lowerreg_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to generator connection points
    pub raisereg_recovery_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsFcasRecovery6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("FCAS_RECOVERY".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_FCAS_PAYMENT
///  _SET_FCAS_PAYMENT sets out the enabling payment details for frequency controlled Ancillary Services._
/// 
/// * Data Set Name: Settlements
/// * File Name: Fcas Payment
/// * Data Version: 5
/// 
/// # Description
///  SET_FCAS_PAYMENT data is confidential to the relevant participant. Volume Approximately 150,000 per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasPayment5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Participant identifier
    pub participantid: Option<String>,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Lower 6 Second Payment
    pub lower6sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 6 Second Payment
    pub raise6sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 60 Second Payment
    pub lower60sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 60 Second Payment
    pub raise60sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Payment
    pub lower5min_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Payment
    pub raise5min_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Regulation Payment
    pub lowerreg_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Regulation Payment
    pub raisereg_payment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsFcasPayment5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("FCAS_PAYMENT".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETLSHEDRECOVERY
///  _SETLSHEDRECOVERY shows reimbursements for Load shed Ancillary Services to be recovered from participants. (Data no longer created for Settlement Days from 01/07/2012)_
/// 
/// * Data Set Name: Settlements
/// * File Name: Lshedrecovery
/// * Data Version: 5
/// 
/// # Description
///  SETLSHEDRECOVERY data is confidential to the relevant participant. Source SETLSHEDRECOVERY updates with each settlement run. Note Only the payment fields (LSEPAYMENT and CCPAYMENT) are on a regional basis. All other demand and recovery fields are on NEM basis rather than a regional basis.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLshedrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: String,
    /// Contract Identifier for reserve, intervention, settlement and ancillary service contracts. Contracts are coded by type and unit.
    pub contractid: Option<String>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Load Shed Enabling Payment
    pub lsepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total NEM Demand
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Recovery
    pub lserecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    pub ccrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Load Shed Enabling Recovery for Generator
    pub lserecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    pub ccrecovery_gen: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total NEM Demand for Generator
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Load Shed Availability service attributable to customer connection points
    pub availabilityrecovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Load Shed Availability service attributable to generator connection points
    pub availabilityrecovery_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsLshedrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("LSHEDRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETCPDATAREGION
///  _SETCPDATAREGION sets out summary meter settlement data for each region._
/// 
/// * Data Set Name: Settlements
/// * File Name: Cpdataregion
/// * Data Version: 5
/// 
/// # Description
///  SETCPDATAREGION data is public, so is available to all participants. Source SETCPDATAREGION is a summary based on grouping on SETCPDATA and is updated with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdataregion5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Import Gross energy into the pool - MWh
    pub sumigenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub sumxgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh
    pub suminenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh
    pub sumxnenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumipower: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumxpower: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of energy price across the region
    pub sumep: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsCpdataregion5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("CPDATAREGION".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETLUNLOADRECOVERY
///  _SETLUNLOADRECOVERY shows reimbursements for rapid unit unloading Ancillary Services to be recovered from participants._
/// 
/// * Data Set Name: Settlements
/// * File Name: Lunloadrecovery
/// * Data Version: 5
/// 
/// # Description
///  SETLUNLOADRECOVERY data is confidential to the relevant participant. Source SETLUNLOADRECOVERY updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLunloadrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: String,
    /// AS Contract
    pub contractid: Option<String>,
    /// Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Usage Payment
    pub usagepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub compensationpayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total Regional Demand
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    pub enablingrecovery: Option<rust_decimal::Decimal>,
    /// Usage Recovery
    pub usagerecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    pub compensationrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Enabling Recovery for Generator
    pub enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Usage Recovery for Generator
    pub usagerecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    pub compensationrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total Regional Demand for Generator
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsLunloadrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("LUNLOADRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETIRSURPLUS
///  _SETIRSURPLUS records the interregional residue calculation for each interconnector and each side of the interconnector._
/// 
/// * Data Set Name: Settlements
/// * File Name: Irsurplus
/// * Data Version: 6
/// 
/// # Description
///  SETIRSURPLUS data is public, so is available to all participants. Source SETIRSURPLUS updates once a day at 8am. Note MWFLOW and LOSSFACTOR are now both calculated as MWh (energy) values for the half hour, and not MW (average demand) values. By way of clarification, the MWFLOW value is derived from half-hour revenue class metering, adjusted by a fixed fraction of the LOSSFACTOR value. The LOSSFACTOR value is taken to be exactly half of the MWLOSSES value in the TRADINGINTERCONNECT table. The METEREDMWFLOW field in the TRADINGINTERCONNECT table contains averaged SCADA metering demand values available in “real time”, whereas the MWFLOW field in the SETIRSURPLUS table contains settlement energy metering values available only after a settlement run is posted.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERCONNECTORID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Interconnector
    pub interconnectorid: String,
    /// Side of interconnector
    pub regionid: String,
    /// Net flow at the regional node (MWh), including losses
    pub mwflow: Option<rust_decimal::Decimal>,
    /// MW losses along interconnector NOTE: This is not a loss factor, but a loss figure expressed in MWH
    pub lossfactor: Option<rust_decimal::Decimal>,
    /// Amount of surplus in $
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsIrsurplus6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("IRSURPLUS".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETGENDATAREGION
///  _SETGENDATAREGION sets out summary settlement data for generation within the specified region._
/// 
/// * Data Set Name: Settlements
/// * File Name: Gendataregion
/// * Data Version: 5
/// 
/// # Description
///  SETGENDATAREGION shows the regional summary. SETGENDATAREGION is public data. Source SETGENDATAREGION updates with each Settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendataregion5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Generated energy - Not used in MMS Data Model
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy - Not used in MMS Data Model
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub apower: Option<rust_decimal::Decimal>,
    /// Net energy MW/hours
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy (Generator Purchases)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost
    pub expenergycost: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsGendataregion5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("GENDATAREGION".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETREALLOCATIONS
///  _SETREALLOCATIONS shows the trading interval value of reallocations processed, for those participants whose reallocation submissions have been accepted by AEMO._
/// 
/// * Data Set Name: Settlements
/// * File Name: Reallocations
/// * Data Version: 5
/// 
/// # Description
///  SETREALLOCATIONS data is confidential to participants party to the reallocation. Source SETREALLOCATIONS updates by the posting of a billing run. Volume Generally, there are approximately 550 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REALLOCATIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsReallocations5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub runno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Reallocation contract identifier
    pub reallocationid: String,
    /// Reallocation value in $
    pub reallocationvalue: Option<rust_decimal::Decimal>,
    /// Energy in MWh if reallocation agreement type is MWh
    pub energy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsReallocations5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("REALLOCATIONS".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETAGCRECOVERY
///  _SETAGCRECOVERY shows reimbursements for Automatic Generation Control (AGC) Ancillary Services to be recovered from participants._
/// 
/// * Data Set Name: Settlements
/// * File Name: Agcrecovery
/// * Data Version: 5
/// 
/// # Description
///  SETAGCRECOVERY data is confidential to the relevant participant Source SETAGCRECOVERY updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAgcrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: String,
    /// &nbsp; 
    pub contractid: Option<String>,
    /// Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total Regional Demand
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    pub enablingrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Enabling Recovery for Generator
    pub enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total Regional Demand for Generator
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsAgcrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("AGCRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETRPOWERRECOVERY
///  _SETRPOWERRECOVERY shows reimbursements for Reactive Power Ancillary Services to be recovered from participants. (Data no longer created for Settlement Days from 01/07/2012)_
/// 
/// * Data Set Name: Settlements
/// * File Name: Rpowerrecovery
/// * Data Version: 5
/// 
/// # Description
///  SETRPOWERRECOVERY data is confidential to the relevant participant. Source SETRPOWERRECOVERY updates with each settlement run. Note Only the payment fields (AVAILABILITYPAYMENT, ENABLINGPAYMENT and CCPAYMENT) are on a regional basis. All other demand and recovery fields are on NEM basis rather than a regional basis.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRpowerrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: String,
    /// AS Contract Identifier
    pub contractid: Option<String>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Compensation payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total NEM Demand
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Availability Recovery
    pub availabilityrecovery: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    pub enablingrecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    pub ccrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Availability Recovery for Generator
    pub availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    /// Enabling Recovery for Generator
    pub enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    pub ccrecovery_gen: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total NEM Demand for Generator
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsRpowerrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("RPOWERRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETVICBOUNDARYENERGY
///  _SETVICBOUNDARYENERGY is as requested by Participants for the settlement of Victorian Vesting contracts._
/// 
/// * Data Set Name: Settlements
/// * File Name: Vicboundaryenergy
/// * Data Version: 5
/// 
/// # Description
///  SETVICBOUNDARYENERGY data is confidential to the relevant participants. Source SETVICBOUNDARYENERGY is populated by the posting of a billing run. Volume Generally there are approximately 550 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicboundaryenergy5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Period Identifier
    pub periodid: rust_decimal::Decimal,
    /// Interval energy purchases in Victoria when host distributor = Pool (in MWh)
    pub boundaryenergy: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsVicboundaryenergy5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("VICBOUNDARYENERGY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_MR_PAYMENT
///  _SET_MR_PAYMENT shows trading interval payments on a dispatchable unit basis for accepted MR capacity._
/// 
/// * Data Set Name: Settlements
/// * File Name: Mr Payment
/// * Data Version: 5
/// 
/// # Description
///  SET_MR_PAYMENT data is confidential to the relevant participant. Source SET_MR_PAYMENT updates are ad hoc, being for MR events only. Volume 24000 rows per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrPayment5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    pub versionno: rust_decimal::Decimal,
    /// Unique Region Identifier
    pub regionid: String,
    /// Unique Participant identifier
    pub participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    pub duid: String,
    /// Calendar day Trading Interval number
    pub periodid: rust_decimal::Decimal,
    /// Accepted MR Capacity
    pub mr_capacity: Option<rust_decimal::Decimal>,
    /// Uncapped Trading Interval Payment
    pub uncapped_payment: Option<rust_decimal::Decimal>,
    /// Capped Trading Interval Payment
    pub capped_payment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsMrPayment5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("MR_PAYMENT".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETLUNLOADPAYMENT
///  _SETLUNLOADPAYMENT shows specific payment details for rapid unit unload service._
/// 
/// * Data Set Name: Settlements
/// * File Name: Lunloadpayment
/// * Data Version: 5
/// 
/// # Description
///  SETLUNLOADPAYMENT data is confidential to the relevant participant. Source SETLUNLOADPAYMENT updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLunloadpayment5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Ancillary Services contract identifier
    pub contractid: String,
    /// Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    pub duid: Option<String>,
    /// Region identifier
    pub regionid: Option<String>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Eligible bid price
    pub ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Enabling price
    pub enablingprice: Option<rust_decimal::Decimal>,
    /// Usage Price
    pub usageprice: Option<rust_decimal::Decimal>,
    /// Compensation cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Cleared MW of Unit in Dispatch, Predispatch or Trading period.
    pub clearedmw: Option<rust_decimal::Decimal>,
    /// MW output the generator would have been running at had it not been constrained up to provide unit unloading
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// The MW output achieved in 5 minutes from startup and is what payment is based on.
    pub controlrange: Option<rust_decimal::Decimal>,
    /// Enabling payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Usage Payment
    pub usagepayment: Option<rust_decimal::Decimal>,
    /// Compensation payment
    pub compensationpayment: Option<rust_decimal::Decimal>,
    /// Contract version number
    pub contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsLunloadpayment5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("LUNLOADPAYMENT".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETRPOWERPAYMENT
///  _SETRPOWERPAYMENT shows specific payment details for Reactive power services by period._
/// 
/// * Data Set Name: Settlements
/// * File Name: Rpowerpayment
/// * Data Version: 6
/// 
/// # Description
///  SETRPOWERPAYMENT data is confidential to the relevant participant. Source SETRPOWERPAYMENT updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRpowerpayment6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// AS Contract Identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    pub ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP absorption capability
    pub mvaraprice: Option<rust_decimal::Decimal>,
    /// Enabling Price
    pub mvareprice: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP generation capability
    pub mvargprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Sync Compensation Flag
    pub synccompensation: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVAr)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVAr)
    pub mtg: Option<rust_decimal::Decimal>,
    /// Block size of unit
    pub blocksize: Option<rust_decimal::Decimal>,
    /// Availability Flag
    pub avaflag: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit
    pub clearedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The rebate amount if MegaVar (MVAr) is below the threshold.
    pub availabilitypayment_rebate: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsRpowerpayment6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("RPOWERPAYMENT".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETAGCPAYMENT
///  _SETAGCPAYMENT sets out specific payment details for Automatic Generation Control (AGC) services by period._
/// 
/// * Data Set Name: Settlements
/// * File Name: Agcpayment
/// * Data Version: 5
/// 
/// # Description
///  SETAGCPAYMENT data is confidential to the relevant participant Source SETAGCPAYMENT updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAgcpayment5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Contract Identifier
    pub contractid: String,
    /// Settlement Period Identifier
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit ID
    pub duid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Transmission Loss Factor of Unit
    pub tlf: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    pub ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Cleared MW of Unit in Enabled Dispatch period
    pub clearedmw: Option<rust_decimal::Decimal>,
    /// Initial MW of Unit in Enabled Dispatch period
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// AS contract version no
    pub contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsAgcpayment5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("AGCPAYMENT".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETIRNSPSURPLUS
///  _This view supports the Settlements Residue Auction, by showing the TNSP participant allocations of  Interconnector Residue (IR) Surplus (i.e. derogated amounts) arising as a result of the sold units for a quarter._
/// 
/// * Data Set Name: Settlements
/// * File Name: Irnspsurplus
/// * Data Version: 6
/// 
/// # Description
///  SETIRNSPSURPLUS data is confidential to the relevant participant. Source SETIRNSPSURPLUS updates with each settlement run. Volume SETIRNSPSURPLUS contains a maximum of 10 million records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrnspsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    pub participantid: String,
    /// Identifier of Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total value of surplus
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage of total surplus allocated to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced by the participant
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsIrnspsurplus6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("IRNSPSURPLUS".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETRESTARTPAYMENT
///  _SETRESTARTPAYMENT shows specific payment details for System Restart services by period._
/// 
/// * Data Set Name: Settlements
/// * File Name: Restartpayment
/// * Data Version: 6
/// 
/// # Description
///  SETRESTARTPAYMENT data is confidential to the relevant participant. Source SETRESTARTPAYMENT updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartpayment6 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Contract Identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: Option<String>,
    /// System Restart Type (0 = FRC, 1 = GRC, 2 = TTH)
    pub restarttype: Option<rust_decimal::Decimal>,
    /// Availability Flag
    pub avaflag: Option<rust_decimal::Decimal>,
    /// Availability Price
    pub availabilityprice: Option<rust_decimal::Decimal>,
    /// Service Test Flag
    pub tcf: Option<rust_decimal::Decimal>,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The enabling payment made for system restart in this half-hour interval
    pub enablingpayment: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsRestartpayment6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("RESTARTPAYMENT".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_FCAS_REGULATION_TRK
///  _SET_FCAS_REGULATION_TRK shows FCAS Regulation Service Constraint tracking for Regional FCAS Regulation recovery_
/// 
/// * Data Set Name: Settlements
/// * File Name: Set Fcas Regulation Trk
/// * Data Version: 1
/// 
/// # Description
///  SET_FCAS_REGULATION_TRK contains public data and is available to all participants. Volume Approximately 350,000 per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSetFcasRegulationTrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Generic Constraint ID
    pub constraintid: String,
    /// Constraint Market Participant Factor
    pub cmpf: Option<rust_decimal::Decimal>,
    /// Constraint Residual Market Participant Factor
    pub crmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CMPF based recovery
    pub recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CRMPF based recovery
    pub recovery_factor_crmpf: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsSetFcasRegulationTrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("SET_FCAS_REGULATION_TRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETINTERVENTION
///  _SETINTERVENTION shows intervention settlement payment details by unit._
/// 
/// * Data Set Name: Settlements
/// * File Name: Intervention
/// * Data Version: 5
/// 
/// # Description
///  SETINTERVENTION became unused when Ancillary Services Review was implemented. For more details, see Change Notice 126. SETINTERVENTION data is confidential to each participant. Source SETINTERVENTION is unused; was updating when intervention occurred in a billing run. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIntervention5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    pub periodid: rust_decimal::Decimal,
    /// Intervention Contract Identifier
    pub contractid: Option<String>,
    /// Intervention Contract Version
    pub contractversion: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: String,
    /// Regional Recovery Flag
    pub rcf: Option<char>,
    /// Payment to Generator for Intervention
    pub interventionpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsIntervention5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("INTERVENTION".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_RUN_PARAMETER
///  _SET_RUN_PARAMETER shows the input parameters and value associated with each settlement run (e.g. Residual System Load Causer Pays Factor)._
/// 
/// * Data Set Name: Settlements
/// * File Name: Run Parameter
/// * Data Version: 5
/// 
/// # Description
///  Change History 19 August 2005 for 4.5.0: Changed index name again to have suffix of _LCX Note: primary key shows PK_ as prefix in Oracle SQL script, even though name of key has _PK as suffix - but cannot change since would not improve participant systems . &nbsp; 17 August 2005 for v4.5.0 Added tablespace (02) for recently added index, and gave index a better name
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARAMETERID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRunParameter5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    pub versionno: i64,
    /// Parameter Identifier
    pub parameterid: String,
    /// Settlement Run Amount for the Constant Identifier
    pub numvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsRunParameter5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("RUN_PARAMETER".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETFCASREGIONRECOVERY
///  _SETFCASREGIONRECOVERY shows FCAS Regional Recovery Data against each Trading Interval._
/// 
/// * Data Set Name: Settlements
/// * File Name: Fcasregionrecovery
/// * Data Version: 5
/// 
/// # Description
///  SETFCASREGIONRECOVERY contains public data and is available to all participants. Source SETFCASREGIONRECOVERY updates with each settlements run. Volume Approximately 10,000 rows per day
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasregionrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// FCAS Service Type
    pub bidtype: String,
    /// RegionID
    pub regionid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Generator Regional Energy Amount
    pub generatorregionenergy: Option<rust_decimal::Decimal>,
    /// Customer Region Energy Amount
    pub customerregionenergy: Option<rust_decimal::Decimal>,
    /// The NEM Regional Recovery Amount for FCAS
    pub regionrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsFcasregionrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("FCASREGIONRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETVICENERGYFIGURES
///  _SETVICENERGYFIGURES is used in settlement of Victorian Vesting contracts._
/// 
/// * Data Set Name: Settlements
/// * File Name: Vicenergyfigures
/// * Data Version: 5
/// 
/// # Description
///  SETVICENERGYFIGURES data is public, so is available to all participants. Source SETVICENERGYFIGURES updates daily, with settlements.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicenergyfigures5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number
    pub versionno: rust_decimal::Decimal,
    /// Settlement period
    pub periodid: rust_decimal::Decimal,
    /// Total generator output
    pub totalgenoutput: Option<rust_decimal::Decimal>,
    /// Total participant demand
    pub totalpcsd: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Transmission loss factor
    pub tlr: Option<rust_decimal::Decimal>,
    /// Marginal loss factor
    pub mlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SettlementsVicenergyfigures5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("VICENERGYFIGURES".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETIRFMRECOVERY
///  _SETIRFMRECOVERY sets out reimbursements for Industrial Relations Force Majeure to be recovered from participants._
/// 
/// * Data Set Name: Settlements
/// * File Name: Irfmrecovery
/// * Data Version: 5
/// 
/// # Description
///  SETIRFMRECOVERY data is confidential to the relevant participant. Source SETIRFMRECOVERY updates with each settlement run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * IRFMID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrfmrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number
    pub versionno: rust_decimal::Decimal,
    /// Settlement period ID
    pub periodid: rust_decimal::Decimal,
    /// Industrial Relations Forced Majeure event number
    pub irfmid: String,
    /// Industrial Relations Forced Majeure event number
    pub irmfversion: Option<rust_decimal::Decimal>,
    /// Participant unique identifier
    pub participantid: String,
    /// Participant demand
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Total non franchised load in Victoria.
    pub totaltcd: Option<rust_decimal::Decimal>,
    /// Total franchised load in Victoria.
    pub totaltfd: Option<rust_decimal::Decimal>,
    /// Industrial Relations Forced Majeure event amount in $.
    pub irfmamount: Option<rust_decimal::Decimal>,
    /// Industrial Relations Forced Majeure payment amount in $.
    pub irfmpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsIrfmrecovery5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("IRFMRECOVERY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETINTRAREGIONRESIDUES
///  _&nbsp; _
/// 
/// * Data Set Name: Settlements
/// * File Name: Intraregionresidues
/// * Data Version: 5
/// 
/// # Description
///  SETINTRAREGIONRESIDUES data is public to all participants. Source SETINTRAREGIONRESIDUES updates with each settlement run. Note The relationship between the data columns for each key is expressed in the following formula: EP + EC + (EXP * RRP) = IRSS
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PERIODID
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIntraregionresidues5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub runno: i64,
    /// Settlements Trading Interval.
    pub periodid: i64,
    /// Region Identifier
    pub regionid: String,
    /// Energy payments to generators
    pub ep: Option<rust_decimal::Decimal>,
    /// Energy purchased by customers
    pub ec: Option<rust_decimal::Decimal>,
    /// Regional price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Net import in MWh into the region calculated at the regional reference node (export is negative)
    pub exp: Option<rust_decimal::Decimal>,
    /// Intra-regional surplus (a negative sign indicates surplus, and a positive sign indicates a deficiency)
    pub irss: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsIntraregionresidues5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("INTRAREGIONRESIDUES".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SET_ANCILLARY_SUMMARY
///  _SET_ANCILLARY_SUMMARY summarises payments for all Ancillary Services to participants on the basis of regions and trading intervals._
/// 
/// * Data Set Name: Settlements
/// * File Name: Ancillary Summary
/// * Data Version: 5
/// 
/// # Description
///  SET_ANCILLARY_SUMMARY data is available to all participants. Volume Approximately 30, 000 per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAncillarySummary5 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Ancillary service identifier (e.g. REACTIVE_POWER)
    pub service: String,
    /// Payment type identifier (e.g. COMPENSATION)
    pub paymenttype: String,
    /// Region Identifier
    pub regionid: String,
    /// Trading interval
    pub periodid: rust_decimal::Decimal,
    /// The NEM ancillary summary regional payment amount ($)
    pub paymentamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsAncillarySummary5 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("ANCILLARY_SUMMARY".into()),
                        version: 5,
                    }
                    
    }
}
/// # Summary
/// 
/// ## SETSMALLGENDATA
///  _Publishes metering data and associated settlement values for with a registered Small Generator Aggregator participants connection points._
/// 
/// * Data Set Name: Settlements
/// * File Name: Smallgendata
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONNECTIONPOINTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSmallgendata1 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number of the record for the settlement date
    pub versionno: rust_decimal::Decimal,
    /// Transmission Node Identifier (TNI)
    pub connectionpointid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// The import direction value for the meter read (MWh)
    pub importenergy: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportenergy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Import Energy Cost ($)
    pub impenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsSmallgendata1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: Some("SMALLGENDATA".into()),
                        version: 1,
                    }
                    
    }
}
