/// # Summary
/// 
/// ## BILLINGIRNSPSURPLUSSUM
///  _BILLINGIRNSPSURPLUSSUM contains derogated payments made to TNSPs arising from the Settlements Residue Auction process._
/// 
/// * Data Set Name: Billing
/// * File Name: Irnspsurplussum
/// * Data Version: 6
/// 
/// # Description
///  BILLINGIRNSPSURPLUSSUM data is confidential to the relevant participant. Source BILLINGIRNSPSURPLUSSUM is populated by the posting of a billing run where derogated payments apply. Volume An indicative maximum is two records inserted per billing run, or 22 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * QUARTER
/// * RESIDUEYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplussum6 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Total residue amount allocated to participant
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// This field is 0.
    pub auctionfees: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    pub auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGINTERRESIDUES
///  _BILLINGINTERRESIDUES shows interregion residues payable to NSP._
/// 
/// * Data Set Name: Billing
/// * File Name: Interresidues
/// * Data Version: 5
/// 
/// # Description
///  Source Obsolete, was weekly with billing run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingInterresidues5 {
    /// May not be necessary
    pub allocation: Option<rust_decimal::Decimal>,
    /// May not be necessary
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Unique identifier for an interconnector which joins two regions.
    pub interconnectorid: String,
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Amount NSP is paid for Inter-Regional Residues
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Region ID
    pub regionid: String,
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
/// # Summary
/// 
/// ## BILLING_NMAS_TST_RECVRY_RBF
///  _BILLING_NMAS_TEST_RECVRY_RBF sets out the NSCAS/SRAS Testing Payment recovery data for the posted billing week._
/// 
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Recvry Rbf
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * REGIONID
/// * SERVICE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryRbf1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Testing Payment Amount to recover from all benefitting regions
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Testing Payment amount to recover from RegionId
    pub recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_DAILY_ENERGY_SUMMARY
///  _Billing result table containing daily summary data for customer and generator energy amounts_
/// 
/// * Data Set Name: Billing
/// * File Name: Daily Energy Summary
/// * Data Version: 1
/// 
/// # Description
///  BILLING_DAILY_ENERGY_SUMMARY data is confidential  to the relevant participant. Source Populated by the posting of a billing run. Volume Approximately 20 records per billrunno.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * SETTLEMENTDATE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDailyEnergySummary1 {
    /// Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Billing Week number
    pub weekno: rust_decimal::Decimal,
    /// Billing Run number
    pub billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// participant identifier
    pub participantid: String,
    /// Unique Region Identifier
    pub regionid: String,
    /// customer energy amount purchased on this settlement day by the participant in the region
    pub customer_energy_purchased: Option<rust_decimal::Decimal>,
    /// generator energy amount sold on this settlement day by the participant in the region
    pub generator_energy_sold: Option<rust_decimal::Decimal>,
    /// generator energy amount purchased on this settlement day by the participant in the region
    pub generator_energy_purchased: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGREGIONFIGURES
///  _BILLINGREGIONFIGURES sets out additional summary region details including ancillary service amounts for each billing run._
/// 
/// * Data Set Name: Billing
/// * File Name: Regionfigures
/// * Data Version: 5
/// 
/// # Description
///  BILLINGREGIONFIGURES is public data, and is available to all participants. Source BILLINGREGIONFIGURES is populated by the posting of a billing run. Volume Five records inserted per billing run, or 55 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionfigures5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    pub regionid: String,
    /// MWh Energy output in the region during the billing period
    pub energyout: Option<rust_decimal::Decimal>,
    /// $ Value of energy output in region during billing period
    pub valueout: Option<rust_decimal::Decimal>,
    /// MWh Amount of energy purchased in region during billing period
    pub energypurchased: Option<rust_decimal::Decimal>,
    /// $ Value of energy purchased during billing period
    pub valuepurchased: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub excessgen: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub reservetrading: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub intcompo: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub adminpricecompo: Option<rust_decimal::Decimal>,
    /// Intraregional residues in $
    pub settsurplus: Option<rust_decimal::Decimal>,
    /// Ancillary service payments in $
    pub aspayment: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub poolfees: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_CO2E_PUBLICATION_TRK
///  _Carbon Dioxide Intensity Index publication tracking table_
/// 
/// * Data Set Name: Billing
/// * File Name: Billing Co2e Publication Trk
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublicationTrk1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week no
    pub weekno: i64,
    /// Billing run no
    pub billrunno: Option<i64>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLINGIRAUCSURPLUS
///  _BILLINGIRAUCSURPLUS supports the Settlements Residue Auction, by showing the weekly billing Interconnector Residue (IR) payments as calculated for each bill run for Network Service Providers (NSPs) from the amount not auctioned._
/// 
/// * Data Set Name: Billing
/// * File Name: Iraucsurplus
/// * Data Version: 5
/// 
/// # Description
///  Source Obsolete Volume This view contains a maximum of 30, 000 records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplus5 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    pub quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total residues allocated to participant
    pub totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    pub adjustment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_APC_COMPENSATION
///  _Billing result table for APC compensation payments._
/// 
/// * Data Set Name: Billing
/// * File Name: Apc Compensation
/// * Data Version: 2
/// 
/// # Description
///  Updated with each billing run
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * APEVENTID
/// * BILLRUNNO
/// * CLAIMID
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcCompensation2 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week number
    pub weekno: i64,
    /// Billing run number
    pub billrunno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: Option<String>,
    /// Payment amount to the participant
    pub compensation_amount: Option<rust_decimal::Decimal>,
    /// The Administered Price Event Type. Valid values: ENERGY, FCAS, BOTH
    pub event_type: Option<String>,
    /// The Type of Administered Price Compensation Claim. Valid values: DIRECT_COST, OTHER_COST
    pub compensation_type: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLINGGENDATA
///  _BILLINGGENDATA shows the total energy sold and purchased per participant transmission connection point for a billing period._
/// 
/// * Data Set Name: Billing
/// * File Name: Gendata
/// * Data Version: 5
/// 
/// # Description
///  BILLINGGENDATA data is confidential to the the relevant participant. Source BILLINGGENDATA is populated by the posting of a billing run, being several times each week. Volume The number of records depends on the number of transmission ConnectionPointIDs a Participant may have sold energy from per billrunno.  An indicative maximum is approximately 15 records inserted per billrunno, or about 165 records inserted per week. BILLINGGENDATA is confidential to the the relevant participant.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGendata5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Connection point identifier
    pub connectionpointid: String,
    /// not populated
    pub stationid: Option<String>,
    /// not populated
    pub duid: Option<String>,
    /// Aggregate energy sold, in MWh
    pub aggregateenergy: Option<rust_decimal::Decimal>,
    /// $ income
    pub sales: Option<rust_decimal::Decimal>,
    /// $ outgoing
    pub purchases: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Amount of energy purchased in MWh
    pub purchasedenergy: Option<rust_decimal::Decimal>,
    /// Metering Data Agent supplying data
    pub mda: Option<String>,
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
/// # Summary
/// 
/// ## BILLINGREGIONEXPORTS
///  _BILLINGREGIONEXPORTS sets out the region summary table of overall energy exported to and from each region for each billing run._
/// 
/// * Data Set Name: Billing
/// * File Name: Regionexports
/// * Data Version: 5
/// 
/// # Description
///  BILLINGREGIONEXPORTS  data is public, and is available to all participants. Source BILLINGREGIONEXPORTS is populated by the posting of a billing run. Volume Eight records inserted per billing run, or 88 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * EXPORTTO
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionexports5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    pub regionid: String,
    /// Region exported to
    pub exportto: String,
    /// MWh Energy value exported
    pub energy: Option<rust_decimal::Decimal>,
    /// $ Value of energy exported
    pub value: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub surplusenergy: Option<rust_decimal::Decimal>,
    /// $ Interregional residue
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_GST_SUMMARY
///  _BILLING_GST_SUMMARY shows the GST_Exclusive and GST amount (if any)  attributable to a participant for each BAS class._
/// 
/// * Data Set Name: Billing
/// * File Name: Gst Summary
/// * Data Version: 5
/// 
/// # Description
///  BILLING_GST_SUMMARY data is confidential to NSP participants. Source Populated by the posting of a billing run. Volume Approximately 5 records are inserted per billrunno, or about 55 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BAS_CLASS
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstSummary5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// The BAS classification
    pub bas_class: String,
    /// The GST exclusive amount paid by/to the participant to/by AEMO for this BAS classification.
    pub gst_exclusive_amount: Option<rust_decimal::Decimal>,
    /// The GST amount for this BAS classification.
    pub gst_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLINGFINANCIALADJUSTMENTS
///  _BILLINGFINANCIALADJUSTMENTS contains any manual adjustments included in the billing run._
/// 
/// * Data Set Name: Billing
/// * File Name: Financialadjustments
/// * Data Version: 5
/// 
/// # Description
///  BILLINGFINANCIALADJUSTMENTS data is confidential to the relevant participant. Source BILLINGFINANCIALADJUSTMENTS is populated by the posting of a billing run, being several times each week. The insertion of a manual adjustment in a billing run is infrequent. Volume Infrequent and, if included in a billing run, low volume. An indicative maximum is 15 records inserted.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * ADJUSTMENTITEM
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFinancialadjustments5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Not Used
    pub participanttype: Option<String>,
    /// Description of the adjustment being made
    pub adjustmentitem: String,
    /// The amount of the manual adjustment line item
    pub amount: Option<rust_decimal::Decimal>,
    /// Not Used
    pub value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The GL financial code of the manual adjustment line item. Used internally by AEMO systems.
    pub financialcode: Option<rust_decimal::Decimal>,
    /// The BAS classification of the manual adjustment line item.
    pub bas_class: Option<String>,
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
/// # Summary
/// 
/// ## BILLINGIRPARTSURPLUSSUM
///  _BILLINGIRPARTSURPLUSSUM supports the Settlements Residue Auction, by showing the weekly billing SRA distribution and associated fees to Auction participants._
/// 
/// * Data Set Name: Billing
/// * File Name: Irpartsurplussum
/// * Data Version: 7
/// 
/// # Description
///  BILLINGIRPARTSURPLUSSUM data is confidential to the relevant participant. Source BILLINGIRPARTSURPLUSSUM is populated by the posting of a billing run where the participant has purchased auction units relating to that billing run. Volume An indicative maximum is 16 records inserted per billing run, or 166 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * QUARTER
/// * RESIDUEYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplussum7 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: rust_decimal::Decimal,
    /// Residue Contract Quarter
    pub quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Total residue amount allocated to participant
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Total auction fees payable in this week (negative amount). If AUCTIONFEES + AUCTIONFEES_GST &gt;= TOTALSURPLUS then ACTUALPAYMENT is zero.
    pub auctionfees: Option<rust_decimal::Decimal>,
    /// Net payment to participant, including auction fees
    pub actualpayment: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    pub auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    /// The adjusted total Auction fees for the Directional Interconnector. Calculated as the amount of the total fees due from the SRA Auction Participant, pro-rated based on the total surplus for each Directional Interconnector the SRA Auction Participant contracted.
    pub auctionfees_totalgross_adj: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGFEES
///  _BILLINGFEES presents pool fees applied to the statement, per billing run._
/// 
/// * Data Set Name: Billing
/// * File Name: Fees
/// * Data Version: 5
/// 
/// # Description
///  BILLINGFEES data is confidential to the relevant participant. Source BILLINGFEES is populated by the posting of a billing run, being several times each week. Volume The number of records varies according to the number of pool fee types the participant may be subject to. An indicative maximum is about 13 records inserted per billrunno or 143 records inserted per week. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFees5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Market fee identifier
    pub marketfeeid: String,
    /// Market fee rate
    pub rate: Option<rust_decimal::Decimal>,
    /// Energy, in MWh
    pub energy: Option<rust_decimal::Decimal>,
    /// Fee in $
    pub value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The participant category pertaining to the market fee recovery. Corresponds to the PARTICIPANTCATEGORYID column of the SETMARKETFEES table.
    pub participantcategoryid: String,
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
/// # Summary
/// 
/// ## BILLINGREGIONIMPORTS
///  _BILLINGREGIONIMPORTS sets out the region summary table of overall energy imported to and from each region for each billing run._
/// 
/// * Data Set Name: Billing
/// * File Name: Regionimports
/// * Data Version: 5
/// 
/// # Description
///  BILLINGREGIONIMPORTS is public data, and is available to all participants. Source BILLINGREGIONIMPORTS is populated by the posting of a billing run. Volume Eight records inserted per billing run, or 88 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * IMPORTFROM
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionimports5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    pub regionid: String,
    /// Region energy imported from
    pub importfrom: String,
    /// Amount of energy imported
    pub energy: Option<rust_decimal::Decimal>,
    /// Value of energy imported
    pub value: Option<rust_decimal::Decimal>,
    /// Populated with 0
    pub surplusenergy: Option<rust_decimal::Decimal>,
    /// Interregional residue
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_SECDEPOSIT_APPLICATION
///  _The security deposit application details_
/// 
/// * Data Set Name: Billing
/// * File Name: Secdeposit Application
/// * Data Version: 1
/// 
/// # Description
///  BILLING_SECDEPOSIT_APPLICATION data is confidential, and is available only to the relevant participant.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepositApplication1 {
    /// The billing contract year where (security deposit application) SDA is applied
    pub contractyear: rust_decimal::Decimal,
    /// The billing week no. where the SDA is applied
    pub weekno: rust_decimal::Decimal,
    /// The billing run no. where the SDA is applied
    pub billrunno: rust_decimal::Decimal,
    /// The Participant ID lodging the SDA
    pub participantid: String,
    /// The SDA application amount
    pub application_amount: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLING_MR_PAYMENT
///  _BILLING_MR_PAYMENT shows aggregate payments on a dispatchable unit/MR Event basis for accepted MR capacity_
/// 
/// * Data Set Name: Billing
/// * File Name: Mr Payment
/// * Data Version: 5
/// 
/// # Description
///  BILLING_MR_PAYMENT data is confidential, and is available only to the relevant participant. Source Ad hoc - MR events only. Volume 3500 rows per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DUID
/// * MR_DATE
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrPayment5 {
    /// Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Billing Week number
    pub weekno: rust_decimal::Decimal,
    /// Billing Run number
    pub billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    pub regionid: String,
    /// Unique Participant identifier
    pub participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    pub duid: String,
    /// Payment amount by AEMO
    pub mr_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLINGINTRARESIDUES
///  _BILLINGINTRARESIDUES shows intra-region settlement residue details for each Transmission Network Service Provider participant by region._
/// 
/// * Data Set Name: Billing
/// * File Name: Intraresidues
/// * Data Version: 5
/// 
/// # Description
///  BILLINGINTRARESIDUES is confidential to the relevant participant. Source BILLINGINTRARESIDUES is populated by the posting of a billing run, being several times each week. Volume An indicative maximum is two records inserted per billing run, or 22 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIntraresidues5 {
    /// TNSP allocation
    pub allocation: Option<rust_decimal::Decimal>,
    /// Total $ residue amount for the region
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Amount TNSP is paid for Intra-Regional Residues
    pub surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Region ID
    pub regionid: String,
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
/// # Summary
/// 
/// ## BILLINGREALLOC_DETAIL
///  _Billing Reallocation Data aggregated by REALLOCATIONID for each billing run over the billing week._
/// 
/// * Data Set Name: Billing
/// * File Name: Realloc Detail
/// * Data Version: 5
/// 
/// # Description
///  The BILLINGREALLOC_DETAIL table that will give a breakdown of the reallocations that form part of that billing run. This assists participants in their settlement reconciliation process. &nbsp; Private data Volume max 100 rows per day
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * COUNTERPARTY
/// * PARTICIPANTID
/// * REALLOCATIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingReallocDetail5 {
    /// BILLING CONTRACTYEAR
    pub contractyear: rust_decimal::Decimal,
    /// BILLING WEEKNO
    pub weekno: rust_decimal::Decimal,
    /// BILLING RUN NO
    pub billrunno: rust_decimal::Decimal,
    /// REALLOCATION PARTICIPANTID
    pub participantid: String,
    /// REALLOCATION COUNTERPARTY PARTICIPANTID
    pub counterparty: String,
    /// REALLOCATIONID
    pub reallocationid: String,
    /// REALLOCATION VALUE
    pub value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLINGREALLOC
///  _BILLINGREALLOC shows reallocation contract values in each billing run, where participants have used reallocations._
/// 
/// * Data Set Name: Billing
/// * File Name: Realloc
/// * Data Version: 5
/// 
/// # Description
///  BILLINGREALLOC data is confidential to the relevant participant. Source BILLINGREALLOC is populated by the posting of a billing run. Volume An indicative maximum is two records inserted per billing run, or 22 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * COUNTERPARTY
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRealloc5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Participant who is the counter party to this contract
    pub counterparty: String,
    /// Value billed on this contract
    pub value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_DIRECTION_RECON_OTHER
///  _Billing reconciliation result table for both provisional and final directions_
/// 
/// * Data Set Name: Billing
/// * File Name: Billing Direction Recon Other
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingDirectionReconOther1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week no
    pub weekno: i64,
    /// Billing run no
    pub billrunno: i64,
    /// Direction identifier
    pub direction_id: String,
    /// Region Identifier
    pub regionid: String,
    /// Direction description
    pub direction_desc: Option<String>,
    /// The service for which the direction occurred (ENERGY, ANCILLARY, NON_ENERGY_NON_AS, etc)
    pub direction_type_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub direction_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub direction_end_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub direction_start_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub direction_end_interval: Option<chrono::NaiveDateTime>,
    /// The final compensation amount for the direction.  The same value for all regions
    pub compensation_amount: Option<rust_decimal::Decimal>,
    /// The interest amount calculated on the final compensation amount for the direction.  The same value for all regions
    pub interest_amount: Option<rust_decimal::Decimal>,
    /// The independent expert fee amount for the direction.  The same value for all regions
    pub independent_expert_fee: Option<rust_decimal::Decimal>,
    /// The total recovery amount for the direction.  The same value for all regions
    pub cra: Option<rust_decimal::Decimal>,
    /// The total customer energy for this region, over the duration of the direction
    pub regional_customer_energy: Option<rust_decimal::Decimal>,
    /// The total generator energy for this region, over the duration of the direction
    pub regional_generator_energy: Option<rust_decimal::Decimal>,
    /// The regional benefit factor allocated to this region for the direction
    pub regional_benefit_factor: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGSMELTERREDUCTION
///  _BILLINGSMELTERREDUCTION shows the smelter reduction payment (only applies to  participants with Victorian customer connection points)._
/// 
/// * Data Set Name: Billing
/// * File Name: Smelterreduction
/// * Data Version: 5
/// 
/// # Description
///  BILLINGSMELTERREDUCTION data is confidential to the relevant participant. Source BILLINGSMELTERREDUCTION is populated by the posting of a billing run where the participant has Victorian customer connectionpoints. Volume One record inserted per billing run, or 11 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSmelterreduction5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Rate in $/MWh
    pub rate1: Option<rust_decimal::Decimal>,
    /// Payment
    pub ra1: Option<rust_decimal::Decimal>,
    /// Rate in $/MWh
    pub rate2: Option<rust_decimal::Decimal>,
    /// Payment
    pub ra2: Option<rust_decimal::Decimal>,
    /// Tabulated Energy
    pub te: Option<rust_decimal::Decimal>,
    /// Victorian Demand as defined by Code Chapter 9 definitions
    pub pcsd: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLINGPRIORADJUSTMENTS
///  _BILLINGPRIORADJUSTMENTS sets out prior period adjustments and associated interest inserted in subsequent Final Statements arising from Revision Statement postings._
/// 
/// * Data Set Name: Billing
/// * File Name: Prioradjustments
/// * Data Version: 5
/// 
/// # Description
///  BILLINGPRIORADJUSTMENTS data is confidential to the relevant participant. Source BILLINGPRIORADJUSTMENTS is populated on the posting of a Final billing run only. Volume Approximately two records inserted per week. Note Actual adjustment payable is ADJAMOUNT - PERAMOUNT + INTEREST AMOUNT.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * ADJBILLRUNNO
/// * ADJCONTRACTYEAR
/// * ADJWEEKNO
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingPrioradjustments5 {
    /// Settlement year.
    pub contractyear: rust_decimal::Decimal,
    /// Settlement week number.
    pub weekno: rust_decimal::Decimal,
    /// Billing run number.
    pub billrunno: rust_decimal::Decimal,
    /// ContractYear of the posted revision statement inserted to the Final Statement
    pub adjcontractyear: rust_decimal::Decimal,
    /// WeekNo of the posted revision statement inserted to the Final Statement
    pub adjweekno: rust_decimal::Decimal,
    /// Bill run number of the posted revision statement inserted to the Final Statement
    pub adjbillrunno: rust_decimal::Decimal,
    /// Participant ID
    pub participantid: String,
    /// Statement total of the previous posted revision statement inserted to the Final Statement.
    pub prevamount: Option<rust_decimal::Decimal>,
    /// Adjusted amount. 
    pub adjamount: Option<rust_decimal::Decimal>,
    /// Interest rate applied to the revision adjustment
    pub irn: Option<rust_decimal::Decimal>,
    /// unused; always null
    pub irp: Option<rust_decimal::Decimal>,
    /// Interest amount.
    pub interestamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// unused; always null
    pub irsr_prevamount: Option<rust_decimal::Decimal>,
    /// unused; always null
    pub irsr_adjamount: Option<rust_decimal::Decimal>,
    /// unused; always null
    pub irsr_interestamount: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLING_NMAS_TST_PAYMENTS
///  _BILLING_NMAS_TEST_PAYMENTS publish the NSCAS/SRAS Testing Payments data for a posted billing week._
/// 
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Payments
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * SERVICE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstPayments1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    pub participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The Testing Payment Amount to recover
    pub payment_amount: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGIRFM
///  _BILLINGIRFM shows billing amounts associated with Industrial Relations Forced Majeure events for each participant._
/// 
/// * Data Set Name: Billing
/// * File Name: Irfm
/// * Data Version: 5
/// 
/// # Description
///  BILLINGIRFM is confidential to the relevant participant. Source BILLINGIRFM is updated with each billing run as required.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrfm5 {
    /// Settlement Year
    pub contractyear: rust_decimal::Decimal,
    /// Week number starting 1 Jan each year.
    pub weekno: rust_decimal::Decimal,
    /// Unique bill run
    pub billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Industrial Relations Forced Majeure payment for the billing period.
    pub irfmpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_SECDEP_INTEREST_RATE
///  _The DAILY interest rates used by billing when calculating the interest amount_
/// 
/// * Data Set Name: Billing
/// * File Name: Secdep Interest Rate
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * EFFECTIVEDATE
/// * INTEREST_ACCT_ID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestRate1 {
    /// The billing contract year the SDA application is processed and interest calculated
    pub contractyear: rust_decimal::Decimal,
    /// The billing week no. the SDA application is processed and interest calculated
    pub weekno: rust_decimal::Decimal,
    /// The billing run no. the SDA application is processed and interest calculated
    pub billrunno: rust_decimal::Decimal,
    /// The interest account ID used by security deposit interest calculation 
    pub interest_acct_id: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The interest rate to apply from the effective date 
    pub interest_rate: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLWHITEHOLE
///  _BILLWHITEHOLE shows white hole payments based on participant vs region demand._
/// 
/// * Data Set Name: Billing
/// * File Name: Whitehole
/// * Data Version: 5
/// 
/// # Description
///  Confidential Source Obsolete; was updated weekly with each billing run.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingWhitehole5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Sum of billing week (RRP * interconnector flow) 
    pub nl: Option<rust_decimal::Decimal>,
    /// The sum of all customer purchases in MWh
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// Sum of all region purchases in MWh
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Payment in $
    pub whiteholepayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Interconnector ID
    pub interconnectorid: String,
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
/// # Summary
/// 
/// ## BILLINGRUNTRK
///  _BILLINGRUNTRK identifies the Statement type (i.e. Status of PRELIM, FINAL, REVISE) and date of the BillRunNo posted, per WeekNo. This provides a further extension of tracking data from the BILLINGDAYTRK table._
/// 
/// * Data Set Name: Billing
/// * File Name: Runtrk
/// * Data Version: 5
/// 
/// # Description
///  BILLINGRUNTRK is public data, and is available to all participants. Source BILLINGRUNTRK is populated by the posting of a billing run. Volume An indicative maximum is one record inserted per billing run, or 11 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRuntrk5 {
    /// Year of the run
    pub contractyear: rust_decimal::Decimal,
    /// Week number of the run
    pub weekno: rust_decimal::Decimal,
    /// Sequential run number
    pub billrunno: rust_decimal::Decimal,
    /// The billing run type, PRELIM, FINAL, REVISE or INTERIM
    pub status: Option<String>,
    /// Flag
    pub adj_cleared: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// null, since not used
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Who posted the results
    pub postby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub receiptpostdate: Option<chrono::NaiveDateTime>,
    /// null, since not used
    pub receiptpostby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub paymentpostdate: Option<chrono::NaiveDateTime>,
    /// Who posted the payment
    pub paymentpostby: Option<String>,
    /// Payment shortfall amount
    pub shortfall: Option<rust_decimal::Decimal>,
    /// Not Used
    pub makeup: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGASRECOVERY
///  _BILLINGASRECOVERY shows participant charges for Ancillary Services for the billing period. This view shows the billing amounts for Ancillary Service Recovery._
/// 
/// * Data Set Name: Billing
/// * File Name: Asrecovery
/// * Data Version: 7
/// 
/// # Description
///  BILLINGASRECOVERY data is confidential to relevant participant. Source Updated  with each billing run. Volume Approximately 5 records are inserted per billrunno, or about 55 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAsrecovery7 {
    /// Region Identifier
    pub regionid: String,
    /// Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Week No
    pub weekno: rust_decimal::Decimal,
    /// Billing Run No.
    pub billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Raise 6 Sec Recovery
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Recovery
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Recovery
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Recovery
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// AGC Recovery - Not used since circa 2000
    pub agc: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Recovery - Not used since circa 2000
    pub fcascomp: Option<rust_decimal::Decimal>,
    /// Load Shed Recovery
    pub loadshed: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Loading Recovery - Not used since December 2001
    pub rgul: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Recovery - Not used since December 2001
    pub rguu: Option<rust_decimal::Decimal>,
    /// Reactive Power Recovery
    pub reactivepower: Option<rust_decimal::Decimal>,
    /// System Restart Recovery
    pub systemrestart: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Raise 6 Sec Recovery for Generator
    pub raise6sec_gen: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Recovery for Generator
    pub lower6sec_gen: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Recovery for Generator
    pub raise60sec_gen: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Recovery for Generator
    pub lower60sec_gen: Option<rust_decimal::Decimal>,
    /// AGC Recovery for Generator
    pub agc_gen: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Recovery for Generator
    pub fcascomp_gen: Option<rust_decimal::Decimal>,
    /// Load Shed Recovery for Generator
    pub loadshed_gen: Option<rust_decimal::Decimal>,
    /// Rapid Generator unit Loading Recovery for. Generator - Not used since December 2001
    pub rgul_gen: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Recovery for Generator - Not used since December 2001
    pub rguu_gen: Option<rust_decimal::Decimal>,
    /// Reactive Power Recovery for Generator
    pub reactivepower_gen: Option<rust_decimal::Decimal>,
    /// System Restart Recovery for Generator
    pub systemrestart_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to customer connection points
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to customer connection points
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to customer connection points
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to customer connection points
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to generator connection points
    pub lower5min_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to generator connection points
    pub raise5min_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to generator connection points
    pub lowerreg_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to generator connection points
    pub raisereg_gen: Option<rust_decimal::Decimal>,
    /// The total availability payment recovery amount (customer).
    pub availability_reactive: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate recovery amount (customer).
    pub availability_reactive_rbt: Option<rust_decimal::Decimal>,
    /// The total availability payment recovery amount (Generator).
    pub availability_reactive_gen: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate recovery amount (Generator).
    pub availability_reactive_rbt_gen: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGIRAUCSURPLUSSUM
///  _BILLINGIRAUCSURPLUSSUM contains Auction fees and Settlements Residue Auction distribution that may arise from unpurchased auction units that accrue to Transmission Network Service Providers._
/// 
/// * Data Set Name: Billing
/// * File Name: Iraucsurplussum
/// * Data Version: 7
/// 
/// # Description
///  BILLINGIRAUCSURPLUSSUM is confidential to the relevant participant. Source BILLINGIRAUCSURPLUSSUM is populated by the posting of a billing run where there are unpurchased auction units. Volume An indicative maximum is eight records inserted per billing run, or 88 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * QUARTER
/// * RESIDUEYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplussum7 {
    /// Contracted Year (calendar year) 
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: rust_decimal::Decimal,
    /// Residue Contract Quarter
    pub quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Total residue amount allocated to participant
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Total auction fees payable in this week (negative amount). If AUCTIONFEES + AUCTIONFEES_GST &gt;= TOTALSURPLUS then ACTUALPAYMENT is zero
    pub auctionfees: Option<rust_decimal::Decimal>,
    /// Net payment to participant, including auction fees
    pub actualpayment: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    pub auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    /// Negative residues in the billing week for this participant in the SRA Year/Quarter
    pub negative_residues: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLING_DIRECTION_RECONCILIATN
///  _Billing reconciliation result table for both provisional and final directions using the FPP methodology (prior to 1st July 2011)_
/// 
/// * Data Set Name: Billing
/// * File Name: Direction Reconciliatn
/// * Data Version: 1
/// 
/// # Description
///  Source BILLING_DIRECTION_RECONCILIATN is populated by the posting of a billing run. Volume One record inserted per direction per billing run, or 11 records inserted per week. Presently 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirectionReconciliatn1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week no
    pub weekno: i64,
    /// Billing run no
    pub billrunno: i64,
    /// Direction identifier
    pub direction_id: String,
    /// Direction description
    pub direction_desc: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub direction_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub direction_end_date: Option<chrono::NaiveDateTime>,
    /// Direction compensation amount
    pub compensation_amount: Option<rust_decimal::Decimal>,
    /// Independent expert fee charged on calculating direction compensation amount
    pub independent_expert_fee: Option<rust_decimal::Decimal>,
    /// Interest occurred on direction compensation amount
    pub interest_amount: Option<rust_decimal::Decimal>,
    /// Direction compensation recovery amount
    pub cra: Option<rust_decimal::Decimal>,
    /// Fixed settlement fee identifier for direction purpose
    pub nem_fee_id: Option<String>,
    /// Fixed settlement fee for participants between direction start and end date
    pub nem_fixed_fee_amount: Option<rust_decimal::Decimal>,
    /// Direction compensation recovery amount percentage breakdown among customers
    pub mkt_customer_perc: Option<rust_decimal::Decimal>,
    /// Direction compensation recovery amount percentage breakdown among generators
    pub generator_perc: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLINGIRNSPSURPLUS
///  _BILLINGIRNSPSURPLUS supports the Settlements Residue Auction (SRA), by showing the weekly billing Interconnector Residue (IR) payments as calculated for each bill run for Transmission Network Service Providers (TNSP) from the amount paid by participants (i.e. derogated amounts)._
/// 
/// * Data Set Name: Billing
/// * File Name: Irnspsurplus
/// * Data Version: 5
/// 
/// # Description
///  BILLINGIRNSPSURPLUS data is confidential to the relevant participant. Source BILLINGIRNSPSURPLUS updates in a billing run where any derogated Settlement Residue Auction purchase flows to a TNSP. Volume BILLINGIRNSPSURPLUS contains a maximum of 30, 000 records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplus5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    pub quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total residues allocated to participant
    pub totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    pub adjustment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_RES_TRADER_RECOVERY
///  _Billing result table for reserve trader contract recovery_
/// 
/// * Data Set Name: Billing
/// * File Name: Res Trader Recovery
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingResTraderRecovery1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week number
    pub weekno: i64,
    /// Billing run number
    pub billrunno: i64,
    /// Region id for the aggregated recovery amount
    pub regionid: String,
    /// Participant identifier
    pub participantid: String,
    /// Payment amount to be recovered from the participant
    pub recovery_amount: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGASPAYMENTS
///  _BILLINGASPAYMENTS shows Ancillary Service payments for each billing period by each of the Ancillary Service types for each participant’s connection points._
/// 
/// * Data Set Name: Billing
/// * File Name: Aspayments
/// * Data Version: 6
/// 
/// # Description
///  BILLINGASPAYMENTS data is confidential to relevant participant. Source Updated  with each billing run. Volume The volume is according to the number of Transmission ConnectionPointIDs a Participant may have subject to ancillary payment per billrunno. An indicative maximum is approximately 20 records are inserted per billrunno, or about 220 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAspayments6 {
    /// Region Identifier
    pub regionid: Option<String>,
    /// Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Week No
    pub weekno: rust_decimal::Decimal,
    /// Billing Run No.
    pub billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Connection point identifier
    pub connectionpointid: String,
    /// Raise 6 Sec Payments
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Payments
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Payments
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Payments
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// AGC Payments
    pub agc: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Payments
    pub fcascomp: Option<rust_decimal::Decimal>,
    /// Load Shed Payments
    pub loadshed: Option<rust_decimal::Decimal>,
    /// Rapid Generator unit Loading Payments
    pub rgul: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Payments
    pub rguu: Option<rust_decimal::Decimal>,
    /// Reactive Power Payments
    pub reactivepower: Option<rust_decimal::Decimal>,
    /// System Restart Payments
    pub systemrestart: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower 5 Minute Payment
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Payment
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Regulation Payment
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Regulation Payment
    pub raisereg: Option<rust_decimal::Decimal>,
    /// The total availability payment
    pub availability_reactive: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate
    pub availability_reactive_rbt: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLING_MR_SUMMARY
///  _BILLING_MR_SUMMARY shows aggregate payment/recovery and shortfall figures for an MR Event._
/// 
/// * Data Set Name: Billing
/// * File Name: Mr Summary
/// * Data Version: 5
/// 
/// # Description
///  BILLING_MR_SUMMARY data is public to all participants. Source Ad hoc - MR events only. Volume 200 rows per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * MR_DATE
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrSummary5 {
    /// Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Billing Week number
    pub weekno: rust_decimal::Decimal,
    /// Billing Run number
    pub billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    pub regionid: String,
    /// Total payments by AEMO
    pub total_payments: Option<rust_decimal::Decimal>,
    /// Total payments to AEMO
    pub total_recovery: Option<rust_decimal::Decimal>,
    /// Total Restriction Shortfall Amount
    pub total_rsa: Option<rust_decimal::Decimal>,
    /// The aggregate of then adjusted gross energy of all the market customer in the restricted region for the duration of the mandatory restriction period (MWh)
    pub aage: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_APC_RECOVERY
///  _Billing result table for recovery of APC compensation payments_
/// 
/// * Data Set Name: Billing
/// * File Name: Apc Recovery
/// * Data Version: 2
/// 
/// # Description
///  Updated with each billing run
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * APEVENTID
/// * BILLRUNNO
/// * CLAIMID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcRecovery2 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week number
    pub weekno: i64,
    /// Billing run number
    pub billrunno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: String,
    /// Region Identifier
    pub regionid: String,
    /// Recovery amount attributable to the participant in that region
    pub recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub eligibility_start_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub eligibility_end_interval: Option<chrono::NaiveDateTime>,
    /// The participant demand in the cost recovery region
    pub participant_demand: Option<rust_decimal::Decimal>,
    /// The sum of demand of all participants in the cost recovery region (Region Sum)
    pub region_demand: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_RES_TRADER_PAYMENT
///  _Billing result table for reserve trader contract payments_
/// 
/// * Data Set Name: Billing
/// * File Name: Res Trader Payment
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * PAYMENT_TYPE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingResTraderPayment1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week number
    pub weekno: i64,
    /// Billing run number
    pub billrunno: i64,
    /// Reserve trader contract identifier
    pub contractid: String,
    /// Payment type for the reserve trader contract payment amount
    pub payment_type: String,
    /// Participant identifier associated with the contract
    pub participantid: String,
    /// Payment amount to the participant
    pub payment_amount: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGCPDATA
///  _BILLINGCPDATA shows energy quantity and $ value purchased per participant connection point._
/// 
/// * Data Set Name: Billing
/// * File Name: Cpdata
/// * Data Version: 5
/// 
/// # Description
///  BILLINGCPDATA data is confidential to relevant participant. Source Populated by the posting of a billing run, being several times each week. Volume The number of records depends on  the number of Transmission ConnectionPointIDs a participant may use to purchase energy. An indicative maximum is approximately 150 records per billrunno, or about 1,500 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * MDA
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingCpdata5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Unique connection point identifier
    pub connectionpointid: String,
    /// Aggregate energy purchased/sold by customer, in MWh
    pub aggregateenergy: Option<rust_decimal::Decimal>,
    /// Value of energy purchased/sold by customer, in $
    pub purchases: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// relevant MDA for this connection point.
    pub mda: String,
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
/// # Summary
/// 
/// ## BILLINGDAYTRK
///  _BILLINGDAYTRK is key for matching settlement versions with billing runs. BILLINGDAYTRK displays the billrunnos per billing week, and the settlement version numbers per settlement day comprising the billrunno._
/// 
/// * Data Set Name: Billing
/// * File Name: Daytrk
/// * Data Version: 5
/// 
/// # Description
///  BILLINGDAYTRK  is public data, and is available to all participants. Source BILLINGDAYTRK is populated by the posting of a billing run, being several times each week. Volume Each billing run inserts approximately 7 records, being about 77 records per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * SETTLEMENTDATE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDaytrk5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number used for each settlement date in that billing run.
    pub runno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_GST_DETAIL
///  _BILLING_GST_DETAIL shows the BAS class, GST_Exclusive and GST amount (if any) attributable to a participant for each transaction type._
/// 
/// * Data Set Name: Billing
/// * File Name: Gst Detail
/// * Data Version: 5
/// 
/// # Description
///  BILLING_GST_DETAIL data is confidential to NSP participants. Source Populated by the posting of a billing run. Volume Approximately 20 records are inserted per billrunno, or about 220 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BAS_CLASS
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * TRANSACTION_TYPE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstDetail5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// The BAS classification that the transaction type belongs to.
    pub bas_class: String,
    /// The transaction type (e.g. CUSTOMER_ENERGY_PURCHASES)
    pub transaction_type: String,
    /// The GST exclusive amount paid by/to the participant to/by AEMO for this transaction type.
    pub gst_exclusive_amount: Option<rust_decimal::Decimal>,
    /// The GST amount for this transaction type. 
    pub gst_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_SECDEP_INTEREST_PAY
///  _The interest amount for security deposit calculated by billing, based on whether it is a fixed/floating rate_
/// 
/// * Data Set Name: Billing
/// * File Name: Secdep Interest Pay
/// * Data Version: 1
/// 
/// # Description
///  BILLING_SECDEP_INTEREST_PAY data is confidential, and is available only to the relevant participant.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * SECURITY_DEPOSIT_ID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestPay1 {
    /// The billing contract year the SDA application is processed and interest calculated
    pub contractyear: rust_decimal::Decimal,
    /// The billing week no. the SDA application is processed and interest calculated
    pub weekno: rust_decimal::Decimal,
    /// The billing run no. the SDA application is processed and interest calculated
    pub billrunno: rust_decimal::Decimal,
    /// The security deposit ID for which billing has calculated the Interest amount
    pub security_deposit_id: String,
    /// The participant ID of the security deposit for whom the interest is paid
    pub participantid: String,
    /// The security deposit interest amount calculated by billing
    pub interest_amount: Option<rust_decimal::Decimal>,
    /// FIXED or DAILY
    pub interest_calc_type: Option<String>,
    /// The interest account ID used by billing for calculating the interest. <br>NULL if INTEREST_CALC_TYPE = FIXED<br>
    pub interest_acct_id: Option<String>,
    /// The STATIC Interest Rate used by Billing for calculating the interest. This is NULL if INTEREST_CALC_TYPE &lt;&gt; FIXED
    pub interest_rate: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLING_EFTSHORTFALL_AMOUNT
///  _The billing shortfall run amounts_
/// 
/// * Data Set Name: Billing
/// * File Name: Eftshortfall Amount
/// * Data Version: 1
/// 
/// # Description
///  BILLING_EFTSHORTFALL_AMOUNT data is confidential, and is available only to the relevant participant.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallAmount1 {
    /// The shortfall affected billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// The shortfall affected billing week no
    pub weekno: rust_decimal::Decimal,
    /// The shortfall affected billing week run no
    pub billrunno: rust_decimal::Decimal,
    /// The participant affected by the shortfall calculation
    pub participantid: String,
    /// The Participant shortfall amount
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The market shortfall amount
    pub shortfall: Option<rust_decimal::Decimal>,
    /// The Company ID associated with the Participant ID used in the shortfall calculation
    pub shortfall_company_id: Option<String>,
    /// The shortfall amount for the Company ID associated with the Participant ID used in the shortfall calculation
    pub company_shortfall_amount: Option<rust_decimal::Decimal>,
    /// The participant NET energy used in shortfall calculation 
    pub participant_net_energy: Option<rust_decimal::Decimal>,
    /// The NET energy for the Company ID associated with the Participant ID used in the shortfall calculation
    pub company_net_energy: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLING_MR_SHORTFALL
///  _BILLING_MR_SHORTFALL shows aggregate MR shortfall payments (or recovery charges) to each participant in the region for the MR event._
/// 
/// * Data Set Name: Billing
/// * File Name: Mr Shortfall
/// * Data Version: 5
/// 
/// # Description
///  BILLING_MR_SHORTFALL data is confidential, and is available only to the relevant participant. Source Ad hoc - MR events only. Volume 400 rows per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * MR_DATE
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrShortfall5 {
    /// Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Billing Week number
    pub weekno: rust_decimal::Decimal,
    /// Billing Run number
    pub billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    pub regionid: String,
    /// Unique Participant Identifier
    pub participantid: String,
    /// The adjusted gross energy for the market customer in the restricted region for the duration of the mandatory restriction event (MWh)
    pub age: Option<rust_decimal::Decimal>,
    /// Restriction Shortfall amount payable to AEMO for a mandatory restriction period
    pub rsa: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_NMAS_TST_RECOVERY
///  _BILLING_NMAS_TEST_RECOVERY sets out the recovery of NMAS testing payments_
/// 
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Recovery
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * SERVICE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecovery1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    pub participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>- RESTART<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Testing Payment Amount to recover from all benefitting regions
    pub test_payment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub recovery_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub recovery_end_date: Option<chrono::NaiveDateTime>,
    /// The Participant energy in MWh for the recovery period
    pub participant_energy: Option<rust_decimal::Decimal>,
    /// The RegionId energy in MWh for the recovery period
    pub region_energy: Option<rust_decimal::Decimal>,
    /// The NEM energy in MWh for the recovery period
    pub nem_energy: Option<rust_decimal::Decimal>,
    /// The Customer Proportion for recovery amount in Percent
    pub customer_proportion: Option<rust_decimal::Decimal>,
    /// The Generator Proportion for recovery amount in Percent (100-Customer Portion)
    pub generator_proportion: Option<rust_decimal::Decimal>,
    /// The Participant Generation for the recovery period
    pub participant_generation: Option<rust_decimal::Decimal>,
    /// The NEM Generation for the recovery period
    pub nem_generation: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the billing week, being the sum of the customer and generator proportions for the PARTICIPANTID in REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_EFTSHORTFALL_DETAIL
///  _The Billing Shortfall Run Amount details_
/// 
/// * Data Set Name: Billing
/// * File Name: Eftshortfall Detail
/// * Data Version: 1
/// 
/// # Description
///  BILLING_EFTSHORTFALL_DETAIL data is confidential, and is available only to the relevant participant.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private &amp; Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * TRANSACTION_TYPE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallDetail1 {
    /// The shortfall affected billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// The shortfall affected billing week no
    pub weekno: rust_decimal::Decimal,
    /// The shortfall affected billing week run no
    pub billrunno: rust_decimal::Decimal,
    /// The participant affected by the shortfall calculation
    pub participantid: String,
    /// The transaction type details associated with the shortfall calculation
    pub transaction_type: String,
    /// The amount for each transaction type
    pub amount: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLINGIRPARTSURPLUS
///  _BILLINGIRPARTSURPLUS supports the Settlements Residue Auction, by showing the weekly billing SRA distribution to Auction participants by Contract Identifier._
/// 
/// * Data Set Name: Billing
/// * File Name: Irpartsurplus
/// * Data Version: 5
/// 
/// # Description
///  BILLINGIRPARTSURPLUS data is confidential to the relevant participant. Source BILLINGIRPARTSURPLUS is populated by the posting of a billing run where the participant has purchased auction units relating to that billing run. Volume An indicative maximum is 64 records inserted per billing run, or 700 records inserted per week.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplus5 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    pub quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total residues allocated to participant
    pub totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    pub adjustment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Net actual payment to participant, including auction fees
    pub actualpayment: Option<rust_decimal::Decimal>,
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
/// # Summary
/// 
/// ## BILLING_MR_RECOVERY
///  _BILLING_MR_RECOVERY shows aggregate recovery charges on a dispatchable unit / MR Event basis for spot market income from dispatch of MR capacity._
/// 
/// * Data Set Name: Billing
/// * File Name: Mr Recovery
/// * Data Version: 5
/// 
/// # Description
///  BILLING_MR_RECOVERY data is confidential, and is available only to the relevant participant. Source Ad hoc - MR events only. Volume 3500 rows per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DUID
/// * MR_DATE
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrRecovery5 {
    /// Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Billing Week number
    pub weekno: rust_decimal::Decimal,
    /// Billing Run number
    pub billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique Region Identifier
    pub regionid: String,
    /// Unique Participant identifier
    pub participantid: Option<String>,
    /// Unique identifier for DUID / MNSP LinkID
    pub duid: String,
    /// Payment amount to AEMO
    pub mr_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
/// # Summary
/// 
/// ## BILLING_NMAS_TST_RECVRY_TRK
///  _BILLING_NMAS_TEST_RECVRY_TRK tracks the energy data used to allocate the test payment recovery over the recovery period._
/// 
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Recvry Trk
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * RECOVERY_BILLRUNNO
/// * RECOVERY_CONTRACTYEAR
/// * RECOVERY_WEEKNO
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryTrk1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// AEMO Contract Year for energy data used in recovery calculation
    pub recovery_contractyear: rust_decimal::Decimal,
    /// Week no for energy data used in recovery calculation
    pub recovery_weekno: rust_decimal::Decimal,
    /// Billing RunNo for energy data used in recovery calculation
    pub recovery_billrunno: rust_decimal::Decimal,
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
/// # Summary
/// 
/// ## BILLING_CO2E_PUBLICATION
///  _Carbon Dioxide Intensity Index publication table_
/// 
/// * Data Set Name: Billing
/// * File Name: Billing Co2e Publication
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTYEAR
/// * REGIONID
/// * SETTLEMENTDATE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublication1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week no
    pub weekno: i64,
    /// Billing run no
    pub billrunno: i64,
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    /// Total sent out energy for region (MWh)
    pub sentoutenergy: Option<rust_decimal::Decimal>,
    /// Total generator emissions for region (Co2-e)
    pub generatoremissions: Option<rust_decimal::Decimal>,
    /// Carbon Dioxide Intensity index for region (CO2-e/MWh)
    pub intensityindex: Option<rust_decimal::Decimal>,
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
