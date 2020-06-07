/// Data Set Name: Irauction
/// File Name: Residue Price Funds Bid
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePriceFundsBid1 {
    /// Unique id for each contract specified by AEMO
    contractid: String,
    /// Unique interconnector identifier
    interconnectorid: String,
    /// Unique region identifier
    fromregionid: String,
    /// Quantity of units bid
    units: Option<rust_decimal::Decimal>,
    /// Price bid for each unit
    bidprice: Option<rust_decimal::Decimal>,
    /// A unique option id, with respect to the auction, created to show which bid elements are linked.
    linkedbidflag: rust_decimal::Decimal,
    /// Unique id for each auction date
    auctionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionResiduePriceFundsBid1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_PRICE_FUNDS_BID".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Bids
/// File Name: Funds Bid
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFundsBid1 {
    /// SRA Contract identifier
    contractid: String,
    /// Participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    /// Unique option identifier (1..20)
    optionid: rust_decimal::Decimal,
    /// Interconnector Identifier
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Quantity of units bid for
    units: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionBidsFundsBid1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_BIDS".into(),
                        table_name: "FUNDS_BID".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Cash Security
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraCashSecurity1 {
    /// Unique identifier for the cash security.
    cash_security_id: String,
    /// Unique identifier for the auction participant lodging the cash security.
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    provision_date: Option<chrono::NaiveDateTime>,
    /// Dollar amount of the cash security.
    cash_amount: Option<rust_decimal::Decimal>,
    /// The interest account ID for calculating the interest payment
    interest_acct_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    finalreturndate: Option<chrono::NaiveDateTime>,
    /// Returned Dollar amount of the Cash Security.
    cash_security_returned: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    deletiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionSraCashSecurity1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_CASH_SECURITY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Residue Public Data
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePublicData1 {
    /// Unique id for each contract to be specified by AEMO
    contractid: String,
    /// Version Number
    versionno: rust_decimal::Decimal,
    /// Unique interconnector identifier
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Total units offered for auction
    unitsoffered: Option<rust_decimal::Decimal>,
    /// Units Sold (modified format and usage in March 2003 to support SRA Inter-Temporal Linking)
    unitssold: Option<rust_decimal::Decimal>,
    /// Clearing price
    clearingprice: Option<rust_decimal::Decimal>,
    /// Reserve price
    reserveprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionResiduePublicData1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_PUBLIC_DATA".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Financial Aucpay Sum
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpaySum1 {
    /// Year of the Tranche
    sra_year: i64,
    /// Relevant Quarter of the Tranche
    sra_quarter: i64,
    /// SRA Run Number
    sra_runno: i64,
    /// Unique participant identifier
    participantid: String,
    /// The total auction proceeds allocated to the TNSP
    gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The total auction proceeds allocated to all TNSPs in the SRA quarter
    total_gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount for in the SRA Quarter for the TNSP
    shortfall_amount: Option<rust_decimal::Decimal>,
    /// The total shortfall amount for in the SRA Quarter for all TNSPs
    total_shortfall_amount: Option<rust_decimal::Decimal>,
    /// The net payment amount owed by AEMO to the TNSP
    net_payment_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionSraFinancialAucpaySum1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_FINANCIAL_AUCPAY_SUM".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Offer Product
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProduct1 {
    /// Unique ID for each Auction date
    auctionid: String,
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    /// Unique Product identifier (1 - 2000)
    optionid: i64,
    /// Unique Directional Interconnector identifier
    interconnectorid: Option<String>,
    /// The source Region identifier for the Directional Interconnector
    fromregionid: Option<String>,
    /// The Offer quantity for this Product
    offer_quantity: Option<i64>,
    /// The Offer price for this Product
    offer_price: Option<rust_decimal::Decimal>,
    /// Tranche identifier
    trancheid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionSraOfferProduct1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_OFFER_PRODUCT".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Residue Con Funds
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConFunds1 {
    /// SRA Contract unique identifier as specified by AEMO
    contractid: String,
    /// Identifier for the Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Actual number of units allocated based on the auction default percentage for the tranche and the total number of units to be auctioned for this quarter
    defaultunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter
    rolloverunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter because they were not taken up by the participant
    reallocatedunits: Option<rust_decimal::Decimal>,
    /// Total units offered for Contract
    unitsoffered: Option<rust_decimal::Decimal>,
    /// Average reserve price calculated from the selected estimates
    meanreserveprice: Option<rust_decimal::Decimal>,
    /// Scaling factor for regional Frequency control Ancillary Service requirement
    scalefactor: Option<rust_decimal::Decimal>,
    /// Actual reserve price
    actualreserveprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionResidueConFunds1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_CON_FUNDS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Financial Auc Receipts
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucReceipts1 {
    /// Year of the Tranche
    sra_year: i64,
    /// Relevant Quarter of the Tranche
    sra_quarter: i64,
    /// SRA Run Number
    sra_runno: i64,
    /// Unique participant identifier
    participantid: String,
    /// The identifier for the Directional Interconnector
    interconnectorid: String,
    /// The source region identifier for the Directional Interconnector
    fromregionid: String,
    /// The SRA contract identifier
    contractid: String,
    /// The number of units purchased
    units_purchased: Option<rust_decimal::Decimal>,
    /// The clearing price of the auction
    clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed to AEMO
    receipt_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Dollar value of Cancelled Units in the Auction for the Auction Participant
    proceeds_amount: Option<rust_decimal::Decimal>,
    /// Units cancelled in the auction by the Auction  participant.
    units_sold: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionSraFinancialAucReceipts1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_FINANCIAL_AUC_RECEIPTS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Residue Con Data
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConData2 {
    /// SRA Contract unique identifier
    contractid: String,
    /// Contract run version
    versionno: rust_decimal::Decimal,
    /// Identifier of Contracted Participant
    participantid: String,
    /// Identifier of Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Units purchased on the directional interconnector (i.e. Contracted quantity)
    unitspurchased: Option<rust_decimal::Decimal>,
    /// Payment due (i.e. total purchase price)
    linkpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// The number of cancelled Units for all Auction Participants.
    secondary_units_sold: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionResidueConData2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_CON_DATA".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Valuationid
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionValuationid1 {
    /// Identifier of the estimator
    valuationid: String,
    /// Full name of estimator
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionValuationid1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "VALUATIONID".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Config
/// File Name: Auction Tranche
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionTranche1 {
    /// SRA Contracted Year
    contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data will take precedence
    versionno: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    tranche: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    auctiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifydate: Option<chrono::NaiveDateTime>,
    /// Percentage of units allocated to the tranche
    unitallocation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    changedate: Option<chrono::NaiveDateTime>,
    /// Name of person who changed this record
    changedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionConfigAuctionTranche1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_CONFIG".into(),
                        table_name: "AUCTION_TRANCHE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Financial Runtrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialRuntrk1 {
    /// Year of the Tranche
    sra_year: i64,
    /// Relevant Quarter of the Tranche
    sra_quarter: i64,
    /// SRA Run Number
    sra_runno: i64,
    /// The type of SRA run
    runtype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    rundate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    posteddate: Option<chrono::NaiveDateTime>,
    /// Version number of the interest component used in the payments run
    interest_versionno: Option<i64>,
    /// Version number of the makeup component used in the makeup run
    makeup_versionno: Option<i64>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionSraFinancialRuntrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_FINANCIAL_RUNTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Config
/// File Name: Auction Rp Estimate
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRpEstimate1 {
    /// SRA Contracted Year
    contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    valuationid: String,
    /// Version of data for other key data - a higher version for same key data takes precedence
    versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Estimate of reserve price
    rpestimate: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionConfigAuctionRpEstimate1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_CONFIG".into(),
                        table_name: "AUCTION_RP_ESTIMATE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Settlement Config
/// File Name: Residuecontractpayments
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigResiduecontractpayments1 {
    /// SRA Contract ID
    contractid: String,
    /// Participant Identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigResiduecontractpayments1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "SETTLEMENT_CONFIG".into(),
                        table_name: "RESIDUECONTRACTPAYMENTS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Financial Auc Mardetail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMardetail1 {
    /// Year of the Tranche
    sra_year: i64,
    /// Relevant Quarter of the Tranche
    sra_quarter: i64,
    /// SRA Run Number
    sra_runno: i64,
    /// The participant identifier.
    participantid: String,
    /// Unique identifier for the cash security.
    cash_security_id: String,
    /// The amount returned to the Auction participant from this cash security.
    returned_amount: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to the returned amount.
    returned_interest: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionSraFinancialAucMardetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_FINANCIAL_AUC_MARDETAIL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Bids
/// File Name: Price Bid
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsPriceBid1 {
    /// Not to be used. Unique id for each SRA contract (specified by AEMO)
    contractid: Option<String>,
    /// Participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    /// Unique option (bid) identifier (1..800)
    optionid: rust_decimal::Decimal,
    /// Price offered for each unit
    bidprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking)
    auctionid: String,
}
impl crate::GetTable<IrauctionBidsPriceBid1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_BIDS".into(),
                        table_name: "PRICE_BID".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Financial Auc Margin
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMargin1 {
    /// Year of the Tranche
    sra_year: i64,
    /// Relevant Quarter of the Tranche
    sra_quarter: i64,
    /// SRA Run Number
    sra_runno: i64,
    /// Unique  participant identifier.
    participantid: String,
    /// Total cash security held by the participant.
    total_cash_security: Option<rust_decimal::Decimal>,
    /// The amount of trading  cash security required to be held by the participant after settlement.
    required_margin: Option<rust_decimal::Decimal>,
    /// The amount of cash security returned to the participant.
    returned_margin: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to returned cash security amounts.
    returned_margin_interest: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionSraFinancialAucMargin1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_FINANCIAL_AUC_MARGIN".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Prudential Comp Position
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCompPosition1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    prudential_runno: i64,
    /// Unique participant identifier
    participantid: String,
    /// The Trading Limit of the company at the time of the prudential run 
    trading_limit: Option<rust_decimal::Decimal>,
    /// Current Prudential Exposure of the Auction Participant including Offers
    prudential_exposure_amount: Option<rust_decimal::Decimal>,
    /// The amount of Trading Margin available to the Auction Participant to trade (including Offered Units and Cancelled Units). Equal to TRADING_LIMIT â€“ PRUDENTIAL_EXPOSURE_AMOUNT.
    trading_margin: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionSraPrudentialCompPosition1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_PRUDENTIAL_COMP_POSITION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Config
/// File Name: Auction Calendar
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionCalendar2 {
    /// SRA Contracted Year
    contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    paymentdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    reconciliationdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    prelimpurchasestmtdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    prelimproceedsstmtdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    finalpurchasestmtdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    finalproceedsstmtdate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionConfigAuctionCalendar2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_CONFIG".into(),
                        table_name: "AUCTION_CALENDAR".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Residue Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueTrk1 {
    /// SRA Contract identifier
    contractid: Option<String>,
    /// Contract run version
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    rundate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer or process
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    postdate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    postedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Load status [SUCCESSFUL/CORRUPT]
    status: Option<String>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    auctionid: String,
}
impl crate::GetTable<IrauctionResidueTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Residue Bid Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueBidTrk1 {
    /// SRA Contract unique identifier
    contractid: Option<String>,
    /// Version of Bid used
    versionno: rust_decimal::Decimal,
    /// Identifier of participant
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidloaddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking) 
    auctionid: String,
}
impl crate::GetTable<IrauctionResidueBidTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_BID_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Prudential Exposure
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialExposure1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date.
    prudential_runno: i64,
    /// Unique participant identifier
    participantid: String,
    /// AEMO Contract Year number starting the week beginning 1 January
    sra_year: i64,
    /// Contract Relevant Quarter 
    sra_quarter: i64,
    /// The identifier for the Directional Interconnector
    interconnectorid: String,
    /// The source Region identifier for the Directional Interconnector
    fromregionid: String,
    /// The largest Tranche where the Unit was either sold or offered. Used in the calculation of the Average Purchase Price for the Trading Position of the Product. The most recent Tranche where Units were cancelled or offered (if the Offer is below the Average Purchase Price)
    max_tranche: Option<i64>,
    /// Unique identifier for the Auction having the Offer. Has a null value when no Offer is made for the Relevant Quarter
    auctionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_submissiontime: Option<chrono::NaiveDateTime>,
    /// Calculated Average Purchase Price for the Product 
    average_purchase_price: Option<rust_decimal::Decimal>,
    /// Calculated average cancellation price for product.
    average_cancellation_price: Option<rust_decimal::Decimal>,
    /// Calculated cancellation volume for product.
    cancellation_volume: Option<rust_decimal::Decimal>,
    /// Calculated trading position for product.
    trading_position: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionSraPrudentialExposure1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_PRUDENTIAL_EXPOSURE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Config
/// File Name: Auction
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuction1 {
    /// Unique id for each auction date
    auctionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    auctiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    /// Description of an auction
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// &nbsp; 
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionConfigAuction1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_CONFIG".into(),
                        table_name: "AUCTION".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Residue Contracts
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueContracts1 {
    /// SRA Contracted Year
    contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    tranche: rust_decimal::Decimal,
    /// Unique identifier for each SRA Contract as specified by AEMO
    contractid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    auctiondate: Option<chrono::NaiveDateTime>,
    /// Identifies methodology used
    calcmethod: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifypostdate: Option<chrono::NaiveDateTime>,
    /// Name of notifying person
    notifyby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    postdate: Option<chrono::NaiveDateTime>,
    /// Name of publishing officer or process
    postedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Description of Contract
    description: Option<String>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking) 
    auctionid: Option<String>,
}
impl crate::GetTable<IrauctionResidueContracts1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_CONTRACTS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Config
/// File Name: Auction Ic Allocations
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionIcAllocations2 {
    /// SRA Contracted Year
    contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data takes precedence
    versionno: rust_decimal::Decimal,
    /// Contracted Interconnector Identifier
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Number of units on the interconnector
    maximumunits: Option<rust_decimal::Decimal>,
    /// Percentage of the total residue for each Unit
    proportion: Option<rust_decimal::Decimal>,
    /// Daily auction fee
    auctionfee: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    changedate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this data set
    changedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Fees for Cancelled Units.
    auctionfee_sales: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionConfigAuctionIcAllocations2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_CONFIG".into(),
                        table_name: "AUCTION_IC_ALLOCATIONS".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Irauction Config
/// File Name: Auction Revenue Estimate
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueEstimate1 {
    /// SRA Contracted Year
    contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    valuationid: String,
    /// Version of data for other key data - a higher version for same key data will take precedence
    versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    interconnectorid: String,
    /// Nominated source region for Interconnector
    fromregionid: String,
    /// Month number within quarter (1..3) 
    monthno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    /// Estimated Revenue
    revenue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionConfigAuctionRevenueEstimate1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_CONFIG".into(),
                        table_name: "AUCTION_REVENUE_ESTIMATE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Bids
/// File Name: File Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFileTrk1 {
    /// SRA ContractID
    contractid: Option<String>,
    /// Participant Identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    /// SRA offer file name
    filename: Option<String>,
    /// SRA acknowledgment file name
    ackfilename: Option<String>,
    /// Load status [SUCCESSFUL/CORRUPT]
    status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    auctionid: String,
}
impl crate::GetTable<IrauctionBidsFileTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_BIDS".into(),
                        table_name: "FILE_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Residue Con Estimates Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConEstimatesTrk1 {
    /// SRA Contract unique identifier
    contractid: String,
    /// AEMO Contract Year number starting in week containing 1st January
    contractyear: rust_decimal::Decimal,
    /// Contract Quarter
    quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    valuationid: String,
    /// Version of a record, as nominated by the participant
    versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionResidueConEstimatesTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "RESIDUE_CON_ESTIMATES_TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Financial Aucpay Detail
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpayDetail1 {
    /// Year of the Tranche
    sra_year: i64,
    /// Relevant Quarter of the Tranche
    sra_quarter: i64,
    /// SRA Run Number
    sra_runno: i64,
    /// Unique  participant identifier
    participantid: String,
    /// The identifier for the Directional Interconnector
    interconnectorid: String,
    /// The source Region identifier for the Directional Interconnector
    fromregionid: String,
    /// The SRA contract identifier
    contractid: String,
    /// The Maximum Units Available for purchase in the Auction
    maximum_units: Option<rust_decimal::Decimal>,
    /// The total number of Allocated Units in the Auction, including Cancelled Units by an Auction Participant
    units_sold: Option<rust_decimal::Decimal>,
    /// The total number of units unpaid for in the auction
    shortfall_units: Option<rust_decimal::Decimal>,
    /// The reserve price of the auction
    reserve_price: Option<rust_decimal::Decimal>,
    /// The Market Clearing Price of the Auction
    clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO before shortfall
    payment_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount
    shortfall_amount: Option<rust_decimal::Decimal>,
    /// The percentage of the auction proceeds allocated to the TNSP on the directional link
    allocation: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO, including shortfall
    net_payment_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionSraFinancialAucpayDetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_FINANCIAL_AUCPAY_DETAIL".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Prudential Cash Security
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCashSecurity1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    prudential_runno: i64,
    /// Unique participant identifier for the Auction Participant lodging the Cash Security
    participantid: String,
    /// Unique identifier for the cash security.
    cash_security_id: String,
    /// Remaining Cash Security deposit available
    cash_security_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<IrauctionSraPrudentialCashSecurity1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_PRUDENTIAL_CASH_SECURITY".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Prudential Run
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialRun1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    /// The prudential run number for the run
    prudential_runno: i64,
}
impl crate::GetTable<IrauctionSraPrudentialRun1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_PRUDENTIAL_RUN".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction
/// File Name: Sra Offer Profile
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProfile1 {
    /// Unique ID for each Auction date
    auctionid: String,
    /// Unique participant identifier
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    /// SRA Offer File name
    filename: Option<String>,
    /// SRA acknowledgment file name
    ackfilename: Option<String>,
    /// Transaction ID used for tracking
    transactionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionSraOfferProfile1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION".into(),
                        table_name: "SRA_OFFER_PROFILE".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Irauction Config
/// File Name: Auction Revenue Track
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueTrack1 {
    /// SRA Contracted Year
    contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    valuationid: String,
    /// Version of data for other key data - a higher version for same key data takes precedence
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    /// Internal use
    status: Option<String>,
    /// Reference to methodology document
    documentref: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<IrauctionConfigAuctionRevenueTrack1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "IRAUCTION_CONFIG".into(),
                        table_name: "AUCTION_REVENUE_TRACK".into(),
                        version: 1,
                    }
                    
    }
}
