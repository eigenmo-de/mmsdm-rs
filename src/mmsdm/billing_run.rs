/// Data Set Name: Billing
/// File Name: Aspayments
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAspayments6 {
    /// Region Identifier
    regionid: Option<String>,
    /// Contract Year
    contractyear: rust_decimal::Decimal,
    /// Week No
    weekno: rust_decimal::Decimal,
    /// Billing Run No.
    billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Connection point identifier
    connectionpointid: String,
    /// Raise 6 Sec Payments
    raise6sec: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Payments
    lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Payments
    raise60sec: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Payments
    lower60sec: Option<rust_decimal::Decimal>,
    /// AGC Payments
    agc: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Payments
    fcascomp: Option<rust_decimal::Decimal>,
    /// Load Shed Payments
    loadshed: Option<rust_decimal::Decimal>,
    /// Rapid Generator unit Loading Payments
    rgul: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Payments
    rguu: Option<rust_decimal::Decimal>,
    /// Reactive Power Payments
    reactivepower: Option<rust_decimal::Decimal>,
    /// System Restart Payments
    systemrestart: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower 5 Minute Payment
    lower5min: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Payment
    raise5min: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Regulation Payment
    lowerreg: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Regulation Payment
    raisereg: Option<rust_decimal::Decimal>,
    /// The total availability payment
    availability_reactive: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate
    availability_reactive_rbt: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingAspayments6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "ASPAYMENTS".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Res Trader Recovery
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingResTraderRecovery1 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week number
    weekno: i64,
    /// Billing run number
    billrunno: i64,
    /// Region id for the aggregated recovery amount
    regionid: String,
    /// Participant identifier
    participantid: String,
    /// Payment amount to be recovered from the participant
    recovery_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingResTraderRecovery1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "RES_TRADER_RECOVERY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Apc Recovery
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcRecovery2 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week number
    weekno: i64,
    /// Billing run number
    billrunno: i64,
    /// AP Event Id
    apeventid: i64,
    /// AP Event Claim Id
    claimid: i64,
    /// Participant identifier
    participantid: String,
    /// Region Identifier
    regionid: String,
    /// Recovery amount attributable to the participant in that region
    recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    eligibility_start_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    eligibility_end_interval: Option<chrono::NaiveDateTime>,
    /// The participant demand in the cost recovery region
    participant_demand: Option<rust_decimal::Decimal>,
    /// The sum of demand of all participants in the cost recovery region (Region Sum)
    region_demand: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingApcRecovery2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "APC_RECOVERY".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Eftshortfall Amount
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallAmount1 {
    /// The shortfall affected billing contract year
    contractyear: rust_decimal::Decimal,
    /// The shortfall affected billing week no
    weekno: rust_decimal::Decimal,
    /// The shortfall affected billing week run no
    billrunno: rust_decimal::Decimal,
    /// The participant affected by the shortfall calculation
    participantid: String,
    /// The Participant shortfall amount
    shortfall_amount: Option<rust_decimal::Decimal>,
    /// The market shortfall amount
    shortfall: Option<rust_decimal::Decimal>,
    /// The Company ID associated with the Participant ID used in the shortfall calculation
    shortfall_company_id: Option<String>,
    /// The shortfall amount for the Company ID associated with the Participant ID used in the shortfall calculation
    company_shortfall_amount: Option<rust_decimal::Decimal>,
    /// The participant NET energy used in shortfall calculation 
    participant_net_energy: Option<rust_decimal::Decimal>,
    /// The NET energy for the Company ID associated with the Participant ID used in the shortfall calculation
    company_net_energy: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingEftshortfallAmount1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "EFTSHORTFALL_AMOUNT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Irnspsurplus
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplus5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    contractid: String,
    /// Unique participant identifier
    participantid: String,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Total residues allocated to participant
    totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    adjustment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingIrnspsurplus5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "IRNSPSURPLUS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Nmas Tst Recvry Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryTrk1 {
    /// AEMO Contract Year number starting in week containing 1 January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    billrunno: rust_decimal::Decimal,
    /// AEMO Contract Year for energy data used in recovery calculation
    recovery_contractyear: rust_decimal::Decimal,
    /// Week no for energy data used in recovery calculation
    recovery_weekno: rust_decimal::Decimal,
    /// Billing RunNo for energy data used in recovery calculation
    recovery_billrunno: rust_decimal::Decimal,
}
impl crate::GetTable<BillingNmasTstRecvryTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "NMAS_TST_RECVRY_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Gst Summary
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstSummary5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// The BAS classification
    bas_class: String,
    /// The GST exclusive amount paid by/to the participant to/by AEMO for this BAS classification.
    gst_exclusive_amount: Option<rust_decimal::Decimal>,
    /// The GST amount for this BAS classification.
    gst_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingGstSummary5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "GST_SUMMARY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Nmas Tst Recovery
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecovery1 {
    /// AEMO Contract Year number starting in week containing 1 January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    billrunno: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>- RESTART<br>
    service: String,
    /// The NMAS Contract Id
    contractid: String,
    /// The region from where the amount is recovered
    regionid: String,
    /// The Benefitting Factor for the RegionId
    rbf: Option<rust_decimal::Decimal>,
    /// The total Testing Payment Amount to recover from all benefitting regions
    test_payment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    recovery_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    recovery_end_date: Option<chrono::NaiveDateTime>,
    /// The Participant energy in MWh for the recovery period
    participant_energy: Option<rust_decimal::Decimal>,
    /// The RegionId energy in MWh for the recovery period
    region_energy: Option<rust_decimal::Decimal>,
    /// The NEM energy in MWh for the recovery period
    nem_energy: Option<rust_decimal::Decimal>,
    /// The Customer Proportion for recovery amount in Percent
    customer_proportion: Option<rust_decimal::Decimal>,
    /// The Generator Proportion for recovery amount in Percent (100-Customer Portion)
    generator_proportion: Option<rust_decimal::Decimal>,
    /// The Participant Generation for the recovery period
    participant_generation: Option<rust_decimal::Decimal>,
    /// The NEM Generation for the recovery period
    nem_generation: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the billing week, being the sum of the customer and generator proportions for the PARTICIPANTID in REGIONID
    recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingNmasTstRecovery1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "NMAS_TST_RECOVERY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Daily Energy Summary
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDailyEnergySummary1 {
    /// Billing Contract Year
    contractyear: rust_decimal::Decimal,
    /// Billing Week number
    weekno: rust_decimal::Decimal,
    /// Billing Run number
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// participant identifier
    participantid: String,
    /// Unique Region Identifier
    regionid: String,
    /// customer energy amount purchased on this settlement day by the participant in the region
    customer_energy_purchased: Option<rust_decimal::Decimal>,
    /// generator energy amount sold on this settlement day by the participant in the region
    generator_energy_sold: Option<rust_decimal::Decimal>,
    /// generator energy amount purchased on this settlement day by the participant in the region
    generator_energy_purchased: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingDailyEnergySummary1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "DAILY_ENERGY_SUMMARY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Iraucsurplus
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplus5 {
    /// SRA Contracted Year (calendar year)
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    contractid: String,
    /// Unique participant identifier
    participantid: String,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Total residues allocated to participant
    totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    adjustment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingIraucsurplus5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "IRAUCSURPLUS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Cpdata
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingCpdata5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Unique connection point identifier
    connectionpointid: String,
    /// Aggregate energy purchased/sold by customer, in MWh
    aggregateenergy: Option<rust_decimal::Decimal>,
    /// Value of energy purchased/sold by customer, in $
    purchases: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// relevant MDA for this connection point.
    mda: String,
}
impl crate::GetTable<BillingCpdata5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "CPDATA".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Regionfigures
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionfigures5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    regionid: String,
    /// MWh Energy output in the region during the billing period
    energyout: Option<rust_decimal::Decimal>,
    /// $ Value of energy output in region during billing period
    valueout: Option<rust_decimal::Decimal>,
    /// MWh Amount of energy purchased in region during billing period
    energypurchased: Option<rust_decimal::Decimal>,
    /// $ Value of energy purchased during billing period
    valuepurchased: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    excessgen: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    reservetrading: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    intcompo: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    adminpricecompo: Option<rust_decimal::Decimal>,
    /// Intraregional residues in $
    settsurplus: Option<rust_decimal::Decimal>,
    /// Ancillary service payments in $
    aspayment: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    poolfees: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingRegionfigures5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "REGIONFIGURES".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Fees
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFees5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Market fee identifier
    marketfeeid: String,
    /// Market fee rate
    rate: Option<rust_decimal::Decimal>,
    /// Energy, in MWh
    energy: Option<rust_decimal::Decimal>,
    /// Fee in $
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The participant category pertaining to the market fee recovery. Corresponds to the PARTICIPANTCATEGORYID column of the SETMARKETFEES table.
    participantcategoryid: String,
}
impl crate::GetTable<BillingFees5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "FEES".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Daytrk
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDaytrk5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Settlement run number used for each settlement date in that billing run.
    runno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingDaytrk5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "DAYTRK".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Nmas Tst Payments
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstPayments1 {
    /// AEMO Contract Year number starting in week containing 1 January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    billrunno: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    service: String,
    /// The NMAS Contract Id
    contractid: String,
    /// The Testing Payment Amount to recover
    payment_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingNmasTstPayments1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "NMAS_TST_PAYMENTS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Interresidues
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingInterresidues5 {
    /// May not be necessary
    allocation: Option<rust_decimal::Decimal>,
    /// May not be necessary
    totalsurplus: Option<rust_decimal::Decimal>,
    /// Unique identifier for an interconnector which joins two regions.
    interconnectorid: String,
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Amount NSP is paid for Inter-Regional Residues
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Region ID
    regionid: String,
}
impl crate::GetTable<BillingInterresidues5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "INTERRESIDUES".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Billing Direction Recon Other
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingDirectionReconOther1 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week no
    weekno: i64,
    /// Billing run no
    billrunno: i64,
    /// Direction identifier
    direction_id: String,
    /// Region Identifier
    regionid: String,
    /// Direction description
    direction_desc: Option<String>,
    /// The service for which the direction occurred (ENERGY, ANCILLARY, NON_ENERGY_NON_AS, etc)
    direction_type_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_end_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_start_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_end_interval: Option<chrono::NaiveDateTime>,
    /// The final compensation amount for the direction.  The same value for all regions
    compensation_amount: Option<rust_decimal::Decimal>,
    /// The interest amount calculated on the final compensation amount for the direction.  The same value for all regions
    interest_amount: Option<rust_decimal::Decimal>,
    /// The independent expert fee amount for the direction.  The same value for all regions
    independent_expert_fee: Option<rust_decimal::Decimal>,
    /// The total recovery amount for the direction.  The same value for all regions
    cra: Option<rust_decimal::Decimal>,
    /// The total customer energy for this region, over the duration of the direction
    regional_customer_energy: Option<rust_decimal::Decimal>,
    /// The total generator energy for this region, over the duration of the direction
    regional_generator_energy: Option<rust_decimal::Decimal>,
    /// The regional benefit factor allocated to this region for the direction
    regional_benefit_factor: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingBillingDirectionReconOther1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "BILLING_DIRECTION_RECON_OTHER".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Regionimports
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionimports5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    regionid: String,
    /// Region energy imported from
    importfrom: String,
    /// Amount of energy imported
    energy: Option<rust_decimal::Decimal>,
    /// Value of energy imported
    value: Option<rust_decimal::Decimal>,
    /// Populated with 0
    surplusenergy: Option<rust_decimal::Decimal>,
    /// Interregional residue
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingRegionimports5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "REGIONIMPORTS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Apc Compensation
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcCompensation2 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week number
    weekno: i64,
    /// Billing run number
    billrunno: i64,
    /// AP Event Id
    apeventid: i64,
    /// AP Event Claim Id
    claimid: i64,
    /// Participant identifier
    participantid: Option<String>,
    /// Payment amount to the participant
    compensation_amount: Option<rust_decimal::Decimal>,
    /// The Administered Price Event Type. Valid values: ENERGY, FCAS, BOTH
    event_type: Option<String>,
    /// The Type of Administered Price Compensation Claim. Valid values: DIRECT_COST, OTHER_COST
    compensation_type: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingApcCompensation2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "APC_COMPENSATION".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Billing Co2e Publication
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublication1 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week no
    weekno: i64,
    /// Billing run no
    billrunno: i64,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Region identifier
    regionid: String,
    /// Total sent out energy for region (MWh)
    sentoutenergy: Option<rust_decimal::Decimal>,
    /// Total generator emissions for region (Co2-e)
    generatoremissions: Option<rust_decimal::Decimal>,
    /// Carbon Dioxide Intensity index for region (CO2-e/MWh)
    intensityindex: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingBillingCo2ePublication1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "BILLING_CO2E_PUBLICATION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Irpartsurplus
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplus5 {
    /// SRA Contracted Year (calendar year)
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    contractid: String,
    /// Unique participant identifier
    participantid: String,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Total residues allocated to participant
    totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    adjustment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Net actual payment to participant, including auction fees
    actualpayment: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingIrpartsurplus5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "IRPARTSURPLUS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Regionexports
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionexports5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    regionid: String,
    /// Region exported to
    exportto: String,
    /// MWh Energy value exported
    energy: Option<rust_decimal::Decimal>,
    /// $ Value of energy exported
    value: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    surplusenergy: Option<rust_decimal::Decimal>,
    /// $ Interregional residue
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingRegionexports5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "REGIONEXPORTS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Secdep Interest Pay
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestPay1 {
    /// The billing contract year the SDA application is processed and interest calculated
    contractyear: rust_decimal::Decimal,
    /// The billing week no. the SDA application is processed and interest calculated
    weekno: rust_decimal::Decimal,
    /// The billing run no. the SDA application is processed and interest calculated
    billrunno: rust_decimal::Decimal,
    /// The security deposit ID for which billing has calculated the Interest amount
    security_deposit_id: String,
    /// The participant ID of the security deposit for whom the interest is paid
    participantid: String,
    /// The security deposit interest amount calculated by billing
    interest_amount: Option<rust_decimal::Decimal>,
    /// FIXED or DAILY
    interest_calc_type: Option<String>,
    /// The interest account ID used by billing for calculating the interest. <br>NULL if INTEREST_CALC_TYPE = FIXED<br>
    interest_acct_id: Option<String>,
    /// The STATIC Interest Rate used by Billing for calculating the interest. This is NULL if INTEREST_CALC_TYPE &lt;&gt; FIXED
    interest_rate: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingSecdepInterestPay1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "SECDEP_INTEREST_PAY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Gendata
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGendata5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Connection point identifier
    connectionpointid: String,
    /// not populated
    stationid: Option<String>,
    /// not populated
    duid: Option<String>,
    /// Aggregate energy sold, in MWh
    aggregateenergy: Option<rust_decimal::Decimal>,
    /// $ income
    sales: Option<rust_decimal::Decimal>,
    /// $ outgoing
    purchases: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Amount of energy purchased in MWh
    purchasedenergy: Option<rust_decimal::Decimal>,
    /// Metering Data Agent supplying data
    mda: Option<String>,
}
impl crate::GetTable<BillingGendata5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "GENDATA".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Smelterreduction
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSmelterreduction5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Rate in $/MWh
    rate1: Option<rust_decimal::Decimal>,
    /// Payment
    ra1: Option<rust_decimal::Decimal>,
    /// Rate in $/MWh
    rate2: Option<rust_decimal::Decimal>,
    /// Payment
    ra2: Option<rust_decimal::Decimal>,
    /// Tabulated Energy
    te: Option<rust_decimal::Decimal>,
    /// Victorian Demand as defined by Code Chapter 9 definitions
    pcsd: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingSmelterreduction5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "SMELTERREDUCTION".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Irfm
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrfm5 {
    /// Settlement Year
    contractyear: rust_decimal::Decimal,
    /// Week number starting 1 Jan each year.
    weekno: rust_decimal::Decimal,
    /// Unique bill run
    billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Industrial Relations Forced Majeure payment for the billing period.
    irfmpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingIrfm5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "IRFM".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Mr Shortfall
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrShortfall5 {
    /// Billing Contract Year
    contractyear: rust_decimal::Decimal,
    /// Billing Week number
    weekno: rust_decimal::Decimal,
    /// Billing Run number
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    regionid: String,
    /// Unique Participant Identifier
    participantid: String,
    /// The adjusted gross energy for the market customer in the restricted region for the duration of the mandatory restriction event (MWh)
    age: Option<rust_decimal::Decimal>,
    /// Restriction Shortfall amount payable to AEMO for a mandatory restriction period
    rsa: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingMrShortfall5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "MR_SHORTFALL".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Eftshortfall Detail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallDetail1 {
    /// The shortfall affected billing contract year
    contractyear: rust_decimal::Decimal,
    /// The shortfall affected billing week no
    weekno: rust_decimal::Decimal,
    /// The shortfall affected billing week run no
    billrunno: rust_decimal::Decimal,
    /// The participant affected by the shortfall calculation
    participantid: String,
    /// The transaction type details associated with the shortfall calculation
    transaction_type: String,
    /// The amount for each transaction type
    amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingEftshortfallDetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "EFTSHORTFALL_DETAIL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Whitehole
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingWhitehole5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Sum of billing week (RRP * interconnector flow) 
    nl: Option<rust_decimal::Decimal>,
    /// The sum of all customer purchases in MWh
    participantdemand: Option<rust_decimal::Decimal>,
    /// Sum of all region purchases in MWh
    regiondemand: Option<rust_decimal::Decimal>,
    /// Payment in $
    whiteholepayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Interconnector ID
    interconnectorid: String,
}
impl crate::GetTable<BillingWhitehole5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "WHITEHOLE".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Irnspsurplussum
/// Data Version: 6
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplussum6 {
    /// SRA Contracted Year (calendar year)
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    residueyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Unique participant identifier
    participantid: String,
    /// Total residue amount allocated to participant
    totalsurplus: Option<rust_decimal::Decimal>,
    /// This field is 0.
    auctionfees: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingIrnspsurplussum6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "IRNSPSURPLUSSUM".into(),
                        version: 6,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Runtrk
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRuntrk5 {
    /// Year of the run
    contractyear: rust_decimal::Decimal,
    /// Week number of the run
    weekno: rust_decimal::Decimal,
    /// Sequential run number
    billrunno: rust_decimal::Decimal,
    /// The billing run type, PRELIM, FINAL, REVISE or INTERIM
    status: Option<String>,
    /// Flag
    adj_cleared: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// null, since not used
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    postdate: Option<chrono::NaiveDateTime>,
    /// Who posted the results
    postby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    receiptpostdate: Option<chrono::NaiveDateTime>,
    /// null, since not used
    receiptpostby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    paymentpostdate: Option<chrono::NaiveDateTime>,
    /// Who posted the payment
    paymentpostby: Option<String>,
    /// Payment shortfall amount
    shortfall: Option<rust_decimal::Decimal>,
    /// Not Used
    makeup: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingRuntrk5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "RUNTRK".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Secdeposit Application
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepositApplication1 {
    /// The billing contract year where (security deposit application) SDA is applied
    contractyear: rust_decimal::Decimal,
    /// The billing week no. where the SDA is applied
    weekno: rust_decimal::Decimal,
    /// The billing run no. where the SDA is applied
    billrunno: rust_decimal::Decimal,
    /// The Participant ID lodging the SDA
    participantid: String,
    /// The SDA application amount
    application_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingSecdepositApplication1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "SECDEPOSIT_APPLICATION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Prioradjustments
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingPrioradjustments5 {
    /// Settlement year.
    contractyear: rust_decimal::Decimal,
    /// Settlement week number.
    weekno: rust_decimal::Decimal,
    /// Billing run number.
    billrunno: rust_decimal::Decimal,
    /// ContractYear of the posted revision statement inserted to the Final Statement
    adjcontractyear: rust_decimal::Decimal,
    /// WeekNo of the posted revision statement inserted to the Final Statement
    adjweekno: rust_decimal::Decimal,
    /// Bill run number of the posted revision statement inserted to the Final Statement
    adjbillrunno: rust_decimal::Decimal,
    /// Participant ID
    participantid: String,
    /// Statement total of the previous posted revision statement inserted to the Final Statement.
    prevamount: Option<rust_decimal::Decimal>,
    /// Adjusted amount. 
    adjamount: Option<rust_decimal::Decimal>,
    /// Interest rate applied to the revision adjustment
    irn: Option<rust_decimal::Decimal>,
    /// unused; always null
    irp: Option<rust_decimal::Decimal>,
    /// Interest amount.
    interestamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// unused; always null
    irsr_prevamount: Option<rust_decimal::Decimal>,
    /// unused; always null
    irsr_adjamount: Option<rust_decimal::Decimal>,
    /// unused; always null
    irsr_interestamount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingPrioradjustments5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "PRIORADJUSTMENTS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Mr Payment
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrPayment5 {
    /// Billing Contract Year
    contractyear: rust_decimal::Decimal,
    /// Billing Week number
    weekno: rust_decimal::Decimal,
    /// Billing Run number
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    regionid: String,
    /// Unique Participant identifier
    participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    duid: String,
    /// Payment amount by AEMO
    mr_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingMrPayment5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "MR_PAYMENT".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Nmas Tst Recvry Rbf
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryRbf1 {
    /// AEMO Contract Year number starting in week containing 1 January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    billrunno: rust_decimal::Decimal,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    service: String,
    /// The NMAS Contract Id
    contractid: String,
    /// The region from where the amount is recovered
    regionid: String,
    /// The Benefitting Factor for the RegionId
    rbf: Option<rust_decimal::Decimal>,
    /// The total Testing Payment Amount to recover from all benefitting regions
    payment_amount: Option<rust_decimal::Decimal>,
    /// The Testing Payment amount to recover from RegionId
    recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingNmasTstRecvryRbf1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "NMAS_TST_RECVRY_RBF".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Irpartsurplussum
/// Data Version: 7
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplussum7 {
    /// SRA Contracted Year (calendar year)
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    residueyear: rust_decimal::Decimal,
    /// Residue Contract Quarter
    quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Unique participant identifier
    participantid: String,
    /// Total residue amount allocated to participant
    totalsurplus: Option<rust_decimal::Decimal>,
    /// Total auction fees payable in this week (negative amount). If AUCTIONFEES + AUCTIONFEES_GST &gt;= TOTALSURPLUS then ACTUALPAYMENT is zero.
    auctionfees: Option<rust_decimal::Decimal>,
    /// Net payment to participant, including auction fees
    actualpayment: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    unadjusted_irsr: Option<rust_decimal::Decimal>,
    /// The adjusted total Auction fees for the Directional Interconnector. Calculated as the amount of the total fees due from the SRA Auction Participant, pro-rated based on the total surplus for each Directional Interconnector the SRA Auction Participant contracted.
    auctionfees_totalgross_adj: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingIrpartsurplussum7> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "IRPARTSURPLUSSUM".into(),
                        version: 7,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Billing Co2e Publication Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublicationTrk1 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week no
    weekno: i64,
    /// Billing run no
    billrunno: Option<i64>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingBillingCo2ePublicationTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "BILLING_CO2E_PUBLICATION_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Direction Reconciliatn
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirectionReconciliatn1 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week no
    weekno: i64,
    /// Billing run no
    billrunno: i64,
    /// Direction identifier
    direction_id: String,
    /// Direction description
    direction_desc: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_end_date: Option<chrono::NaiveDateTime>,
    /// Direction compensation amount
    compensation_amount: Option<rust_decimal::Decimal>,
    /// Independent expert fee charged on calculating direction compensation amount
    independent_expert_fee: Option<rust_decimal::Decimal>,
    /// Interest occurred on direction compensation amount
    interest_amount: Option<rust_decimal::Decimal>,
    /// Direction compensation recovery amount
    cra: Option<rust_decimal::Decimal>,
    /// Fixed settlement fee identifier for direction purpose
    nem_fee_id: Option<String>,
    /// Fixed settlement fee for participants between direction start and end date
    nem_fixed_fee_amount: Option<rust_decimal::Decimal>,
    /// Direction compensation recovery amount percentage breakdown among customers
    mkt_customer_perc: Option<rust_decimal::Decimal>,
    /// Direction compensation recovery amount percentage breakdown among generators
    generator_perc: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingDirectionReconciliatn1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "DIRECTION_RECONCILIATN".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Iraucsurplussum
/// Data Version: 7
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplussum7 {
    /// Contracted Year (calendar year) 
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    residueyear: rust_decimal::Decimal,
    /// Residue Contract Quarter
    quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Unique participant identifier
    participantid: String,
    /// Total residue amount allocated to participant
    totalsurplus: Option<rust_decimal::Decimal>,
    /// Total auction fees payable in this week (negative amount). If AUCTIONFEES + AUCTIONFEES_GST &gt;= TOTALSURPLUS then ACTUALPAYMENT is zero
    auctionfees: Option<rust_decimal::Decimal>,
    /// Net payment to participant, including auction fees
    actualpayment: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    unadjusted_irsr: Option<rust_decimal::Decimal>,
    /// Negative residues in the billing week for this participant in the SRA Year/Quarter
    negative_residues: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingIraucsurplussum7> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "IRAUCSURPLUSSUM".into(),
                        version: 7,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Mr Recovery
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrRecovery5 {
    /// Billing Contract Year
    contractyear: rust_decimal::Decimal,
    /// Billing Week number
    weekno: rust_decimal::Decimal,
    /// Billing Run number
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    regionid: String,
    /// Unique Participant identifier
    participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    duid: String,
    /// Payment amount to AEMO
    mr_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingMrRecovery5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "MR_RECOVERY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Gst Detail
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstDetail5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// The BAS classification that the transaction type belongs to.
    bas_class: String,
    /// The transaction type (e.g. CUSTOMER_ENERGY_PURCHASES)
    transaction_type: String,
    /// The GST exclusive amount paid by/to the participant to/by AEMO for this transaction type.
    gst_exclusive_amount: Option<rust_decimal::Decimal>,
    /// The GST amount for this transaction type. 
    gst_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingGstDetail5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "GST_DETAIL".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Asrecovery
/// Data Version: 7
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAsrecovery7 {
    /// Region Identifier
    regionid: String,
    /// Contract Year
    contractyear: rust_decimal::Decimal,
    /// Week No
    weekno: rust_decimal::Decimal,
    /// Billing Run No.
    billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    participantid: String,
    /// Raise 6 Sec Recovery
    raise6sec: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Recovery
    lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Recovery
    raise60sec: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Recovery
    lower60sec: Option<rust_decimal::Decimal>,
    /// AGC Recovery - Not used since circa 2000
    agc: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Recovery - Not used since circa 2000
    fcascomp: Option<rust_decimal::Decimal>,
    /// Load Shed Recovery
    loadshed: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Loading Recovery - Not used since December 2001
    rgul: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Recovery - Not used since December 2001
    rguu: Option<rust_decimal::Decimal>,
    /// Reactive Power Recovery
    reactivepower: Option<rust_decimal::Decimal>,
    /// System Restart Recovery
    systemrestart: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Raise 6 Sec Recovery for Generator
    raise6sec_gen: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Recovery for Generator
    lower6sec_gen: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Recovery for Generator
    raise60sec_gen: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Recovery for Generator
    lower60sec_gen: Option<rust_decimal::Decimal>,
    /// AGC Recovery for Generator
    agc_gen: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Recovery for Generator
    fcascomp_gen: Option<rust_decimal::Decimal>,
    /// Load Shed Recovery for Generator
    loadshed_gen: Option<rust_decimal::Decimal>,
    /// Rapid Generator unit Loading Recovery for. Generator - Not used since December 2001
    rgul_gen: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Recovery for Generator - Not used since December 2001
    rguu_gen: Option<rust_decimal::Decimal>,
    /// Reactive Power Recovery for Generator
    reactivepower_gen: Option<rust_decimal::Decimal>,
    /// System Restart Recovery for Generator
    systemrestart_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to customer connection points
    lower5min: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to customer connection points
    raise5min: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to customer connection points
    lowerreg: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to customer connection points
    raisereg: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to generator connection points
    lower5min_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to generator connection points
    raise5min_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to generator connection points
    lowerreg_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to generator connection points
    raisereg_gen: Option<rust_decimal::Decimal>,
    /// The total availability payment recovery amount (customer).
    availability_reactive: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate recovery amount (customer).
    availability_reactive_rbt: Option<rust_decimal::Decimal>,
    /// The total availability payment recovery amount (Generator).
    availability_reactive_gen: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate recovery amount (Generator).
    availability_reactive_rbt_gen: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingAsrecovery7> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "ASRECOVERY".into(),
                        version: 7,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Realloc
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRealloc5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Participant who is the counter party to this contract
    counterparty: String,
    /// Value billed on this contract
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingRealloc5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "REALLOC".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Res Trader Payment
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingResTraderPayment1 {
    /// Billing contract year
    contractyear: i64,
    /// Billing week number
    weekno: i64,
    /// Billing run number
    billrunno: i64,
    /// Reserve trader contract identifier
    contractid: String,
    /// Payment type for the reserve trader contract payment amount
    payment_type: String,
    /// Participant identifier associated with the contract
    participantid: String,
    /// Payment amount to the participant
    payment_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingResTraderPayment1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "RES_TRADER_PAYMENT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Financialadjustments
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFinancialadjustments5 {
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Not Used
    participanttype: Option<String>,
    /// Description of the adjustment being made
    adjustmentitem: String,
    /// The amount of the manual adjustment line item
    amount: Option<rust_decimal::Decimal>,
    /// Not Used
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The GL financial code of the manual adjustment line item. Used internally by AEMO systems.
    financialcode: Option<rust_decimal::Decimal>,
    /// The BAS classification of the manual adjustment line item.
    bas_class: Option<String>,
}
impl crate::GetTable<BillingFinancialadjustments5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "FINANCIALADJUSTMENTS".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Mr Summary
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrSummary5 {
    /// Billing Contract Year
    contractyear: rust_decimal::Decimal,
    /// Billing Week number
    weekno: rust_decimal::Decimal,
    /// Billing Run number
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    regionid: String,
    /// Total payments by AEMO
    total_payments: Option<rust_decimal::Decimal>,
    /// Total payments to AEMO
    total_recovery: Option<rust_decimal::Decimal>,
    /// Total Restriction Shortfall Amount
    total_rsa: Option<rust_decimal::Decimal>,
    /// The aggregate of then adjusted gross energy of all the market customer in the restricted region for the duration of the mandatory restriction period (MWh)
    aage: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingMrSummary5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "MR_SUMMARY".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Intraresidues
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIntraresidues5 {
    /// TNSP allocation
    allocation: Option<rust_decimal::Decimal>,
    /// Total $ residue amount for the region
    totalsurplus: Option<rust_decimal::Decimal>,
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    participantid: String,
    /// Amount TNSP is paid for Intra-Regional Residues
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Region ID
    regionid: String,
}
impl crate::GetTable<BillingIntraresidues5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "INTRARESIDUES".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Realloc Detail
/// Data Version: 5
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingReallocDetail5 {
    /// BILLING CONTRACTYEAR
    contractyear: rust_decimal::Decimal,
    /// BILLING WEEKNO
    weekno: rust_decimal::Decimal,
    /// BILLING RUN NO
    billrunno: rust_decimal::Decimal,
    /// REALLOCATION PARTICIPANTID
    participantid: String,
    /// REALLOCATION COUNTERPARTY PARTICIPANTID
    counterparty: String,
    /// REALLOCATIONID
    reallocationid: String,
    /// REALLOCATION VALUE
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingReallocDetail5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "REALLOC_DETAIL".into(),
                        version: 5,
                    }
                    
    }
}
/// Data Set Name: Billing
/// File Name: Secdep Interest Rate
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestRate1 {
    /// The billing contract year the SDA application is processed and interest calculated
    contractyear: rust_decimal::Decimal,
    /// The billing week no. the SDA application is processed and interest calculated
    weekno: rust_decimal::Decimal,
    /// The billing run no. the SDA application is processed and interest calculated
    billrunno: rust_decimal::Decimal,
    /// The interest account ID used by security deposit interest calculation 
    interest_acct_id: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// The interest rate to apply from the effective date 
    interest_rate: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingSecdepInterestRate1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "BILLING".into(),
                        table_name: "SECDEP_INTEREST_RATE".into(),
                        version: 1,
                    }
                    
    }
}
