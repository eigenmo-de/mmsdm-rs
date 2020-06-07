/// Data Set Name: Settlements
/// File Name: Iraucsurplus
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIraucsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    contractid: String,
    /// Settlement Period in day (1..48)
    periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Contracted Interconnector identifier
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Total value of surplus before allocation
    totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage allocated to participant
    contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsIraucsurplus6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "IRAUCSURPLUS".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Apc Recovery
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcRecovery1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    versionno: i64,
    /// AP Event Id
    apeventid: i64,
    /// AP Event Claim Id
    claimid: i64,
    /// Participant identifier
    participantid: String,
    /// Trading interval identifier
    periodid: i64,
    /// Region id for the recovery amount
    regionid: String,
    /// Recovery amount in the region attributable to the participant for the event claim in this interval
    recovery_amount: Option<rust_decimal::Decimal>,
    /// Total Recovery amount in the region for the event claim in this interval
    region_recovery_br_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsApcRecovery1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "APC_RECOVERY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Mr Recovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrRecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    versionno: rust_decimal::Decimal,
    /// Unique Region Identifier
    regionid: String,
    /// Unique Participant identifier
    participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    duid: String,
    /// Calendar day Trading Interval number
    periodid: rust_decimal::Decimal,
    /// Accepted Restriction Offer Dispatched Energy Factor
    arodef: Option<rust_decimal::Decimal>,
    /// The amount payable to AEMO for that accepted restriction offer and trading interval
    nta: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsMrRecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "MR_RECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Gendataregion
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendataregion5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Generated energy - Not used in MMS Data Model
    genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy - Not used in MMS Data Model
    aenergy: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    gpower: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    apower: Option<rust_decimal::Decimal>,
    /// Net energy MW/hours
    netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    excessenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy (Generator Purchases)
    expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost
    expenergycost: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsGendataregion5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "GENDATAREGION".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Mr Payment
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrPayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    versionno: rust_decimal::Decimal,
    /// Unique Region Identifier
    regionid: String,
    /// Unique Participant identifier
    participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    duid: String,
    /// Calendar day Trading Interval number
    periodid: rust_decimal::Decimal,
    /// Accepted MR Capacity
    mr_capacity: Option<rust_decimal::Decimal>,
    /// Uncapped Trading Interval Payment
    uncapped_payment: Option<rust_decimal::Decimal>,
    /// Capped Trading Interval Payment
    capped_payment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsMrPayment5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "MR_PAYMENT".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Fcascomp
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcascomp5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Dispatchable Unit ID
    duid: String,
    /// Region Identifier
    regionid: Option<String>,
    /// Period Identifier
    periodid: rust_decimal::Decimal,
    /// Compensation Cap
    ccprice: Option<rust_decimal::Decimal>,
    /// Cleared MW of Unit in First Dispatch period in Trading Interval
    clearedmw: Option<rust_decimal::Decimal>,
    /// Initial MW of Unit in First Dispatch period in Trading Interval
    unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    ebp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor of Unit
    tlf: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Excess Generation Payment in trading interval
    excessgen: Option<rust_decimal::Decimal>,
    /// Frequency Control AS Compensation payment to Generator
    fcascomp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsFcascomp5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "FCASCOMP".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Apc Compensation
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcCompensation1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    versionno: i64,
    /// AP Event Id
    apeventid: i64,
    /// AP Event Claim Id
    claimid: i64,
    /// Participant identifier
    participantid: String,
    /// Trading interval identifier
    periodid: i64,
    /// Compensation amount for the event claim in this interval
    compensation_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsApcCompensation1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "APC_COMPENSATION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Nmas Recovery
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecovery2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    versionno: rust_decimal::Decimal,
    /// Half Hourly Interval
    periodid: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>- RESTART
    service: String,
    /// The NMAS Contract Id
    contractid: String,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    paymenttype: String,
    /// The region from where the amount is recovered
    regionid: String,
    /// The Benefitting Factor for the RegionId
    rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all  benefitting regions 
    payment_amount: Option<rust_decimal::Decimal>,
    /// The Participant energy in MWh for the period
    participant_energy: Option<rust_decimal::Decimal>,
    /// The RegionId energy in MWh for the period
    region_energy: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the PARTICIPANTID and REGIONID
    recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant Generator Energy in the benefitting region
    participant_generation: Option<rust_decimal::Decimal>,
    /// The generator energy in the benefitting region
    region_generation: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to customers
    recovery_amount_customer: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to generators
    recovery_amount_generator: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsNmasRecovery2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "NMAS_RECOVERY".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Lshedpayment
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLshedpayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// AS Contract Identifier
    contractid: String,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    duid: Option<String>,
    /// Region Identifier
    regionid: Option<String>,
    /// Transmission Loss Factor
    tlf: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Price
    lseprice: Option<rust_decimal::Decimal>,
    /// Minimum Compensation Price
    mcpprice: Option<rust_decimal::Decimal>,
    /// Load Shed Control Range
    lscr: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Payment
    lsepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    ccpayment: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit at time of load shed usage
    constrainedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit at time of load shed usage
    unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Amount of load shed
    als: Option<rust_decimal::Decimal>,
    /// Initial demand of unit at time of load shed usage
    initialdemand: Option<rust_decimal::Decimal>,
    /// Final demand of unit at time of load shed usage
    finaldemand: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Payment amount for the Load Shed Availability service
    availabilitypayment: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsLshedpayment5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "LSHEDPAYMENT".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Smallgendata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSmallgendata1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Version number of the record for the settlement date
    versionno: rust_decimal::Decimal,
    /// Transmission Node Identifier (TNI)
    connectionpointid: String,
    /// Settlement interval identifier (half hour period)
    periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Region Identifier
    regionid: Option<String>,
    /// The import direction value for the meter read (MWh)
    importenergy: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    exportenergy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    tlf: Option<rust_decimal::Decimal>,
    /// Import Energy Cost ($)
    impenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    expenergycost: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsSmallgendata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "SMALLGENDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Daytrack
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsDaytrack5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Not Used
    regionid: Option<String>,
    /// Not Used
    exanterunstatus: Option<String>,
    /// Not Used
    exanterunno: Option<rust_decimal::Decimal>,
    /// Not Used
    expostrunstatus: Option<String>,
    /// Settlement Run No
    expostrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsDaytrack5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "DAYTRACK".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Vicboundaryenergy
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicboundaryenergy5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Version number
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Period Identifier
    periodid: rust_decimal::Decimal,
    /// Interval energy purchases in Victoria when host distributor = Pool (in MWh)
    boundaryenergy: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsVicboundaryenergy5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "VICBOUNDARYENERGY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Set Fcas Regulation Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSetFcasRegulationTrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    /// Generic Constraint ID
    constraintid: String,
    /// Constraint Market Participant Factor
    cmpf: Option<rust_decimal::Decimal>,
    /// Constraint Residual Market Participant Factor
    crmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CMPF based recovery
    recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CRMPF based recovery
    recovery_factor_crmpf: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsSetFcasRegulationTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "SET_FCAS_REGULATION_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Lunloadpayment
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLunloadpayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    versionno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Ancillary Services contract identifier
    contractid: String,
    /// Region Identifier
    periodid: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    duid: Option<String>,
    /// Region identifier
    regionid: Option<String>,
    /// Transmission Loss Factor
    tlf: Option<rust_decimal::Decimal>,
    /// Eligible bid price
    ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Enabling price
    enablingprice: Option<rust_decimal::Decimal>,
    /// Usage Price
    usageprice: Option<rust_decimal::Decimal>,
    /// Compensation cap
    ccprice: Option<rust_decimal::Decimal>,
    /// Cleared MW of Unit in Dispatch, Predispatch or Trading period.
    clearedmw: Option<rust_decimal::Decimal>,
    /// MW output the generator would have been running at had it not been constrained up to provide unit unloading
    unconstrainedmw: Option<rust_decimal::Decimal>,
    /// The MW output achieved in 5 minutes from startup and is what payment is based on.
    controlrange: Option<rust_decimal::Decimal>,
    /// Enabling payment
    enablingpayment: Option<rust_decimal::Decimal>,
    /// Usage Payment
    usagepayment: Option<rust_decimal::Decimal>,
    /// Compensation payment
    compensationpayment: Option<rust_decimal::Decimal>,
    /// Contract version number
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsLunloadpayment5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "LUNLOADPAYMENT".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Restartpayment
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartpayment6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Contract Identifier
    contractid: String,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: Option<String>,
    /// System Restart Type (0 = FRC, 1 = GRC, 2 = TTH)
    restarttype: Option<rust_decimal::Decimal>,
    /// Availability Flag
    avaflag: Option<rust_decimal::Decimal>,
    /// Availability Price
    availabilityprice: Option<rust_decimal::Decimal>,
    /// Service Test Flag
    tcf: Option<rust_decimal::Decimal>,
    /// Availability Payment
    availabilitypayment: Option<rust_decimal::Decimal>,
    /// Contract Version No.
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The enabling payment made for system restart in this half-hour interval
    enablingpayment: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsRestartpayment6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "RESTARTPAYMENT".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Irsurplus
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    settlementrunno: rust_decimal::Decimal,
    /// Trading interval
    periodid: rust_decimal::Decimal,
    /// Interconnector
    interconnectorid: String,
    /// Side of interconnector
    regionid: String,
    /// Net flow at the regional node (MWh), including losses
    mwflow: Option<rust_decimal::Decimal>,
    /// MW losses along interconnector NOTE: This is not a loss factor, but a loss figure expressed in MWH
    lossfactor: Option<rust_decimal::Decimal>,
    /// Amount of surplus in $
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsIrsurplus6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "IRSURPLUS".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Irpartsurplus
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrpartsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    settlementrunno: rust_decimal::Decimal,
    /// Ancillary Service Contract
    contractid: String,
    /// Settlement period in the day (1..48)
    periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    participantid: String,
    /// Identifier of the Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Total value of surplus before allocation
    totalsurplus: Option<rust_decimal::Decimal>,
    /// Allocated percentage to participant
    contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsIrpartsurplus6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "IRPARTSURPLUS".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Rpowerrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRpowerrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    participantid: String,
    /// AS Contract Identifier
    contractid: Option<String>,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Availability Payment
    availabilitypayment: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    enablingpayment: Option<rust_decimal::Decimal>,
    /// Compensation payment
    ccpayment: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand
    participantdemand: Option<rust_decimal::Decimal>,
    /// Total NEM Demand
    regiondemand: Option<rust_decimal::Decimal>,
    /// Availability Recovery
    availabilityrecovery: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    enablingrecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    ccrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Availability Recovery for Generator
    availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    /// Enabling Recovery for Generator
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    ccrecovery_gen: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand for Generator
    participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total NEM Demand for Generator
    regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsRpowerrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "RPOWERRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Vicenergyflow
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicenergyflow5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Version number
    versionno: rust_decimal::Decimal,
    /// Settlement period
    periodid: rust_decimal::Decimal,
    /// Net metered energy flowing across the V-SN and V-SA interconnectors
    netflow: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsVicenergyflow5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "VICENERGYFLOW".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Rpowerpayment
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRpowerpayment6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// AS Contract Identifier
    contractid: String,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    duid: Option<String>,
    /// Region Identifier
    regionid: Option<String>,
    /// Transmission Loss Factor
    tlf: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP absorption capability
    mvaraprice: Option<rust_decimal::Decimal>,
    /// Enabling Price
    mvareprice: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP generation capability
    mvargprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap
    ccprice: Option<rust_decimal::Decimal>,
    /// Sync Compensation Flag
    synccompensation: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVAr)
    mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVAr)
    mtg: Option<rust_decimal::Decimal>,
    /// Block size of unit
    blocksize: Option<rust_decimal::Decimal>,
    /// Availability Flag
    avaflag: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit
    clearedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit
    unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Availability Payment
    availabilitypayment: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    enablingpayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    ccpayment: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The rebate amount if MegaVar (MVAr) is below the threshold.
    availabilitypayment_rebate: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsRpowerpayment6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "RPOWERPAYMENT".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Restartrecovery
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartrecovery6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    participantid: String,
    /// Contract Identifier
    contractid: Option<String>,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Availability Payment
    availabilitypayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    participantdemand: Option<rust_decimal::Decimal>,
    /// NEM Demand (NB sum of ALL Regions)
    regiondemand: Option<rust_decimal::Decimal>,
    /// Availability Recovery
    availabilityrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Availability Recovery for Generator
    availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Sum of all generation including SGA generation across all regions of the NEM and floored to zero
    regiondemand_gen: Option<rust_decimal::Decimal>,
    /// The enabling payment made for system restart in this half-hour interval
    enablingpayment: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to customer activity
    enablingrecovery: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to generator activity
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsRestartrecovery6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "RESTARTRECOVERY".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Gendata
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendata6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: Option<String>,
    /// Station Identifier
    stationid: String,
    /// Dispatchable Unit identifier
    duid: String,
    /// Physical unit identifier
    gensetid: String,
    /// Region Identifier
    regionid: String,
    /// Generated energy
    genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy
    aenergy: Option<rust_decimal::Decimal>,
    /// Not used
    gpower: Option<rust_decimal::Decimal>,
    /// Not used
    apower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    cpeep: Option<rust_decimal::Decimal>,
    /// Net energy (MWh)
    netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    excessenergycost: Option<rust_decimal::Decimal>,
    /// Administered Price Compensation
    apc: Option<rust_decimal::Decimal>,
    /// Not used
    resc: Option<rust_decimal::Decimal>,
    /// Not used
    resp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Export Energy (Generator Purchases) (MWh)
    expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    expenergycost: Option<rust_decimal::Decimal>,
    /// Identifier of the meter run used in this settlement calculation
    meterrunno: Option<rust_decimal::Decimal>,
    /// Metering Data Agent
    mda: Option<String>,
    /// Secondary Transmission Loss Factor
    secondary_tlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsGendata6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "GENDATA".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Reallocations
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsReallocations5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    runno: rust_decimal::Decimal,
    /// Settlement Period identifier
    periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Reallocation contract identifier
    reallocationid: String,
    /// Reallocation value in $
    reallocationvalue: Option<rust_decimal::Decimal>,
    /// Energy in MWh if reallocation agreement type is MWh
    energy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsReallocations5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "REALLOCATIONS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Agcpayment
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAgcpayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Contract Identifier
    contractid: String,
    /// Settlement Period Identifier
    periodid: rust_decimal::Decimal,
    /// Dispatchable Unit ID
    duid: Option<String>,
    /// Region Identifier
    regionid: Option<String>,
    /// Transmission Loss Factor of Unit
    tlf: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Cleared MW of Unit in Enabled Dispatch period
    clearedmw: Option<rust_decimal::Decimal>,
    /// Initial MW of Unit in Enabled Dispatch period
    initialmw: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    enablingpayment: Option<rust_decimal::Decimal>,
    /// AS contract version no
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsAgcpayment5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "AGCPAYMENT".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Fcas Payment
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasPayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    versionno: rust_decimal::Decimal,
    /// Participant identifier
    participantid: Option<String>,
    /// Dispatchable unit identifier
    duid: String,
    /// Region Identifier
    regionid: Option<String>,
    /// Trading interval
    periodid: rust_decimal::Decimal,
    /// Lower 6 Second Payment
    lower6sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 6 Second Payment
    raise6sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 60 Second Payment
    lower60sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 60 Second Payment
    raise60sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Payment
    lower5min_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Payment
    raise5min_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Regulation Payment
    lowerreg_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Regulation Payment
    raisereg_payment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsFcasPayment5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "FCAS_PAYMENT".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Intervention
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIntervention5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    periodid: rust_decimal::Decimal,
    /// Intervention Contract Identifier
    contractid: Option<String>,
    /// Intervention Contract Version
    contractversion: Option<rust_decimal::Decimal>,
    /// Unique participant identifier
    participantid: Option<String>,
    /// Region Identifier
    regionid: Option<String>,
    /// Dispatchable Unit ID
    duid: String,
    /// Regional Recovery Flag
    rcf: Option<char>,
    /// Payment to Generator for Intervention
    interventionpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsIntervention5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "INTERVENTION".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Run Parameter
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRunParameter5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    versionno: i64,
    /// Parameter Identifier
    parameterid: String,
    /// Settlement Run Amount for the Constant Identifier
    numvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsRunParameter5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "RUN_PARAMETER".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Interventionrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsInterventionrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    periodid: rust_decimal::Decimal,
    /// Intervention Contract Identifier
    contractid: String,
    /// Regional Recovery Flag
    rcf: Option<char>,
    /// Unique participant identifier
    participantid: String,
    /// Demand of Participant in Region/Market
    participantdemand: Option<rust_decimal::Decimal>,
    /// Total Demand of Region/Market
    totaldemand: Option<rust_decimal::Decimal>,
    /// Payment to Generator for Intervention
    interventionpayment: Option<rust_decimal::Decimal>,
    /// Retailer Payment to Pool for Intervention
    interventionamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Region Identifier
    regionid: Option<String>,
}
impl crate::GetTable<SettlementsInterventionrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "INTERVENTIONRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Vicenergyfigures
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicenergyfigures5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Version number
    versionno: rust_decimal::Decimal,
    /// Settlement period
    periodid: rust_decimal::Decimal,
    /// Total generator output
    totalgenoutput: Option<rust_decimal::Decimal>,
    /// Total participant demand
    totalpcsd: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Transmission loss factor
    tlr: Option<rust_decimal::Decimal>,
    /// Marginal loss factor
    mlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsVicenergyfigures5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "VICENERGYFIGURES".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Nmas Recovery Rbf
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecoveryRbf1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    versionno: rust_decimal::Decimal,
    /// Half Hourly Interval
    periodid: rust_decimal::Decimal,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    service: String,
    /// The NMAS Contract Id
    contractid: String,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    paymenttype: String,
    /// The region from where the amount is recovered
    regionid: String,
    /// The Benefitting Factor for the RegionId
    rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all benefitting regions 
    payment_amount: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the REGIONID
    recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsNmasRecoveryRbf1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "NMAS_RECOVERY_RBF".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Lunloadrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLunloadrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    participantid: String,
    /// AS Contract
    contractid: Option<String>,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Enabling Payment
    enablingpayment: Option<rust_decimal::Decimal>,
    /// Usage Payment
    usagepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    compensationpayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    participantdemand: Option<rust_decimal::Decimal>,
    /// Total Regional Demand
    regiondemand: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    enablingrecovery: Option<rust_decimal::Decimal>,
    /// Usage Recovery
    usagerecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    compensationrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Enabling Recovery for Generator
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Usage Recovery for Generator
    usagerecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    compensationrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total Regional Demand for Generator
    regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsLunloadrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "LUNLOADRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Cpdata
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdata5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Connection point identifier
    tcpid: String,
    /// Region Identifier
    regionid: Option<String>,
    /// Import Gross energy into the pool - MWh
    igenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    xgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh
    inenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh
    xnenergy: Option<rust_decimal::Decimal>,
    /// Import reactive power
    ipower: Option<rust_decimal::Decimal>,
    /// Export reactive power
    xpower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    cpeep: Option<rust_decimal::Decimal>,
    /// Export - Import of Net energy (MWh)
    ta: Option<rust_decimal::Decimal>,
    /// settlement amount in $ for trading period
    ep: Option<rust_decimal::Decimal>,
    /// Not used
    apc: Option<rust_decimal::Decimal>,
    /// Not used
    resc: Option<rust_decimal::Decimal>,
    /// Not used
    resp: Option<rust_decimal::Decimal>,
    /// Meter Run Number = version number of the meter file
    meterrunno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used
    hostdistributor: Option<String>,
    /// Metering Data Agent
    mda: String,
}
impl crate::GetTable<SettlementsCpdata5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "CPDATA".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Irnspsurplus
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrnspsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    contractid: String,
    /// Settlement period in day (1..48)
    periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    participantid: String,
    /// Identifier of Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Total value of surplus
    totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage of total surplus allocated to participant
    contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced by the participant
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsIrnspsurplus6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "IRNSPSURPLUS".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Fcas Recovery
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasRecovery6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    versionno: String,
    /// Participant identifier
    participantid: String,
    /// Region Identifier
    regionid: String,
    /// Trading interval
    periodid: rust_decimal::Decimal,
    /// Recovery amount for the Lower 6 Second service attributable to customer connection points
    lower6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to customer connection points
    raise6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to customer connection points
    lower60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to customer connection points
    raise60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to customer connection points
    lower5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to customer connection points
    raise5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to customer connection points
    lowerreg_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to customer connection points
    raisereg_recovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Recovery amount for the Lower 6 Second service attributable to generator connection points
    lower6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to generator connection points
    raise6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to generator connection points
    lower60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to generator connection points
    raise60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to generator connection points
    lower5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to generator connection points
    raise5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to generator connection points
    lowerreg_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to generator connection points
    raisereg_recovery_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsFcasRecovery6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "FCAS_RECOVERY".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Luloadrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLuloadrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    participantid: String,
    /// AS Contract ID
    contractid: Option<String>,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Enabling Payment
    enablingpayment: Option<rust_decimal::Decimal>,
    /// Usage Payment
    usagepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    compensationpayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    participantdemand: Option<rust_decimal::Decimal>,
    /// Total Regional Demand
    regiondemand: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    enablingrecovery: Option<rust_decimal::Decimal>,
    /// Usage Recovery
    usagerecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    compensationrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Enabling Recovery for Generator
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Usage Recovery for Generator
    usagerecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    compensationrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total Regional Demand for Generator
    regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsLuloadrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "LULOADRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Intraregionresidues
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIntraregionresidues5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    runno: i64,
    /// Settlement period in the day (1..48)
    periodid: i64,
    /// Region Identifier
    regionid: String,
    /// Energy payments to generators
    ep: Option<rust_decimal::Decimal>,
    /// Energy purchased by customers
    ec: Option<rust_decimal::Decimal>,
    /// Regional price
    rrp: Option<rust_decimal::Decimal>,
    /// Net import in MWh into the region calculated at the regional reference node (export is negative)
    exp: Option<rust_decimal::Decimal>,
    /// Intra-regional surplus (a negative sign indicates surplus, and a positive sign indicates a deficiency)
    irss: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsIntraregionresidues5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "INTRAREGIONRESIDUES".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Lshedrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLshedrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    participantid: String,
    /// Contract Identifier for reserve, intervention, settlement and ancillary service contracts. Contracts are coded by type and unit.
    contractid: Option<String>,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Load Shed Enabling Payment
    lsepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    ccpayment: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand
    participantdemand: Option<rust_decimal::Decimal>,
    /// Total NEM Demand
    regiondemand: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Recovery
    lserecovery: Option<rust_decimal::Decimal>,
    /// Compensation Recovery
    ccrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Load Shed Enabling Recovery for Generator
    lserecovery_gen: Option<rust_decimal::Decimal>,
    /// Compensation Recovery for Generator
    ccrecovery_gen: Option<rust_decimal::Decimal>,
    /// Total Participant NEM Demand for Generator
    participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total NEM Demand for Generator
    regiondemand_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Load Shed Availability service attributable to customer connection points
    availabilityrecovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Load Shed Availability service attributable to generator connection points
    availabilityrecovery_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsLshedrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "LSHEDRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Marketfees
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMarketfees5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    runno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Period identifier
    periodid: rust_decimal::Decimal,
    /// Market fee identifier (e.g. V_EST)
    marketfeeid: String,
    /// Fee charge
    marketfeevalue: Option<rust_decimal::Decimal>,
    /// Energy amount for variable fees
    energy: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The participant category that the market fee recovery pertains to. Corresponds to the PARTICIPANTCATEGORYID column of the PARTICIPANT_BANDFEE_CATEGORYALLOC_C_V view for BAND$ type fees, or to the MARKETFEETYPE column of the MARKETFEE_P_V view for all other fee types.
    participantcategoryid: String,
}
impl crate::GetTable<SettlementsMarketfees5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "MARKETFEES".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Fcasregionrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasregionrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    versionno: rust_decimal::Decimal,
    /// FCAS Service Type
    bidtype: String,
    /// RegionID
    regionid: String,
    /// Trading interval periodid (01 to 48)
    periodid: rust_decimal::Decimal,
    /// Generator Regional Energy Amount
    generatorregionenergy: Option<rust_decimal::Decimal>,
    /// Customer Region Energy Amount
    customerregionenergy: Option<rust_decimal::Decimal>,
    /// The NEM Regional Recovery Amount for FCAS
    regionrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsFcasregionrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "FCASREGIONRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Cpdataregion
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdataregion5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    versionno: rust_decimal::Decimal,
    /// Settlement Period identifier
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Import Gross energy into the pool - MWh
    sumigenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    sumxgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh
    suminenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh
    sumxnenergy: Option<rust_decimal::Decimal>,
    /// Not used
    sumipower: Option<rust_decimal::Decimal>,
    /// Not used
    sumxpower: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of energy price across the region
    sumep: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsCpdataregion5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "CPDATAREGION".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Ancillary Summary
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAncillarySummary5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    versionno: rust_decimal::Decimal,
    /// Ancillary service identifier (e.g. REACTIVE_POWER)
    service: String,
    /// Payment type identifier (e.g. COMPENSATION)
    paymenttype: String,
    /// Region Identifier
    regionid: String,
    /// Trading interval
    periodid: rust_decimal::Decimal,
    /// The NEM ancillary summary regional payment amount ($)
    paymentamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsAncillarySummary5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "ANCILLARY_SUMMARY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Agcrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAgcrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    participantid: String,
    /// &nbsp; 
    contractid: Option<String>,
    /// Trading Interval
    periodid: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Enabling Payment
    enablingpayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    participantdemand: Option<rust_decimal::Decimal>,
    /// Total Regional Demand
    regiondemand: Option<rust_decimal::Decimal>,
    /// Enabling Recovery
    enablingrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Enabling Recovery for Generator
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Total Regional Demand for Generator
    regiondemand_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SettlementsAgcrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "AGCRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Settlements
/// File Name: Irfmrecovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrfmrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Version number
    versionno: rust_decimal::Decimal,
    /// Settlement period ID
    periodid: rust_decimal::Decimal,
    /// Industrial Relations Forced Majeure event number
    irfmid: String,
    /// Industrial Relations Forced Majeure event number
    irmfversion: Option<rust_decimal::Decimal>,
    /// Participant unique identifier
    participantid: String,
    /// Participant demand
    participantdemand: Option<rust_decimal::Decimal>,
    /// Total non franchised load in Victoria.
    totaltcd: Option<rust_decimal::Decimal>,
    /// Total franchised load in Victoria.
    totaltfd: Option<rust_decimal::Decimal>,
    /// Industrial Relations Forced Majeure event amount in $.
    irfmamount: Option<rust_decimal::Decimal>,
    /// Industrial Relations Forced Majeure payment amount in $.
    irfmpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementsIrfmrecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENTS".into(),
                        table_name: "IRFMRECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
