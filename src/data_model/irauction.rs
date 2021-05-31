/// # Summary
///
/// ## AUCTION
///  _AUCTION holds auction details. AUCTION is new in March 2003 to support SRA Inter-Temporal Linking._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction
/// * Data Version: 1
///
/// # Description
///  AUCTION is public data, and is available to all participants. Source Static. Volume 4 records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AUCTIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuction1 {
    /// Unique id for each auction date
    pub auctionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub auctiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Description of an auction
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuction1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## AUCTION_CALENDAR
///  _AUCTION_CALENDAR holds the definitions of each auction quarter in a contract year. AUCTION_CALENDAR supports the Settlement Residue Auction._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Calendar
/// * Data Version: 2
///
/// # Description
///  AUCTION_CALENDAR is public data, and is available to all participants. Source Updates are usually quarterly by the SRA team. Volume AUCTION_CALENDAR shows a maximum of 16 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionCalendar2 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub paymentdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub reconciliationdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub prelimpurchasestmtdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub prelimproceedsstmtdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub finalpurchasestmtdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub finalproceedsstmtdate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionCalendar2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_CALENDAR".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## AUCTION_IC_ALLOCATIONS
///  _AUCTION_IC_ALLOCATIONS supports the Settlement Residue Auction by providing the basis for setting up contracts for individual tranches. AUCTION_IC_ALLOCATIONS shows the default definitions for the total number of units and proportion applicable to each directional interconnector for a specified auction quarter._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Ic Allocations
/// * Data Version: 2
///
/// # Description
///  AUCTION_IC_ALLOCATIONS is public data, and is available to all participants. Source Updates are usually quarterly as auctions are held from Settlement Residue Auction team's SRIS interface. Volume AUCTION_IC_ALLOCATIONS contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * QUARTER
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionIcAllocations2 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector Identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Number of units on the interconnector
    pub maximumunits: Option<rust_decimal::Decimal>,
    /// Percentage of the total residue for each Unit
    pub proportion: Option<rust_decimal::Decimal>,
    /// Daily auction fee
    pub auctionfee: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub changedate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this data set
    pub changedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Fees for Cancelled Units.
    pub auctionfee_sales: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionConfigAuctionIcAllocations2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_IC_ALLOCATIONS".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## AUCTION_REVENUE_ESTIMATE
///  _AUCTION_REVENUE_ESTIMATE supports the Settlement Residue Auction, by holding the evaluator’s estimates of revenue for each month of a given quarter.<br>Since reserve prices are no longer applicable from the end of 2001, zero is used as a default to avoid rewriting the system._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Revenue Estimate
/// * Data Version: 1
///
/// # Description
///  AUCTION_REVENUE_ESTIMATE is public data, and is available to all participants. Source Updates are quarterly from SRA team via SRIS interface Volume AUCTION_REVENUE_ESTIMATE contains a maximum of 300 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * MONTHNO
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueEstimate1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Month number within quarter (1..3)
    pub monthno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Estimated Revenue
    pub revenue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionRevenueEstimate1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_REVENUE_ESTIMATE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## AUCTION_REVENUE_TRACK
///  _AUCTION_REVENUE_TRACK supports the Settlement Residue Auction, by holding the tracking information for each evaluator’s estimates for a given quarter. The status field is dynamic and is used for selection of estimates to be published._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Revenue Track
/// * Data Version: 1
///
/// # Description
///  AUCTION_REVENUE_TRACK is public data, and is available to all participants. Source Updates are quarterly after SRA team updates SRIS interface. Volume AUCTION_REVENUE_TRACK contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueTrack1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Internal use
    pub status: Option<String>,
    /// Reference to methodology document
    pub documentref: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionRevenueTrack1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_REVENUE_TRACK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## AUCTION_RP_ESTIMATE
///  _AUCTION_RP_ESTIMATE supports the Settlement Residue Auction, by holding the evaluator’s estimates of revenue prices for a given quarter.<br>Since reserve prices are no longer applicable from the end of 2001, zero is used as a default to avoid rewriting the system._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Rp Estimate
/// * Data Version: 1
///
/// # Description
///  AUCTION_RP_ESTIMATE is public data, and is available to all participants. Source Updates are quarterly by SRA team via SRIS interface. Volume This view contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRpEstimate1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Estimate of reserve price
    pub rpestimate: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionRpEstimate1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_RP_ESTIMATE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## AUCTION_TRANCHE
///  _AUCTION_TRANCHE supports the Settlement Residue Auction, by holding the default definitions for the percentage number of units allocated and dates applicable to each tranche for a specified auction quarter. This information provides the basis for setting up contracts for individual tranches._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Tranche
/// * Data Version: 1
///
/// # Description
///  AUCTION_TRANCHE is public data, and is available to all participants. Source Updates are quarterly from SRA team via SRIS interface. Volume AUCTION_TRANCHE contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * TRANCHE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionTranche1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    pub tranche: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub auctiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Percentage of units allocated to the tranche
    pub unitallocation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub changedate: Option<chrono::NaiveDateTime>,
    /// Name of person who changed this record
    pub changedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionTranche1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_TRANCHE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUECONTRACTPAYMENTS
///  _RESIDUECONTRACTPAYMENTS shows Settlement Residue Auction payment Participant notifications._
///
/// * Data Set Name: Settlement Config
/// * File Name: Residuecontractpayments
/// * Data Version: 1
///
/// # Description
///  RESIDUECONTRACTPAYMENTS data is confidential to the relevant participant.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigResiduecontractpayments1 {
    /// SRA Contract ID
    pub contractid: String,
    /// Participant Identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigResiduecontractpayments1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("RESIDUECONTRACTPAYMENTS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUEFILETRK
///  _RESIDUEFILETRK records all Settlement Residue Auction offers submitted by participants._
///
/// * Data Set Name: Irauction Bids
/// * File Name: File Trk
/// * Data Version: 1
///
/// # Description
///  RESIDUEFILETRK data is confidential to each participant Source RESIDUEFILETRK updates are ad hoc from participants Volume Assuming quarterly contracts RESIDUEFILETRK contains a maximum of 5,000 records per annum. Each bid file can contain many bids for each auction. Participants can input multiple bids (with the last acknowledged file being used in the auction).
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFileTrk1 {
    /// SRA ContractID
    pub contractid: Option<String>,
    /// Participant Identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// SRA offer file name
    pub filename: Option<String>,
    /// SRA acknowledgment file name
    pub ackfilename: Option<String>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionBidsFileTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_BIDS".into(),
            table_name: Some("FILE_TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_BID_TRK
///  _RESIDUE_BID_TRK supports the Settlement Residue Auction, by detailing which bid was used for which SRA Contract run._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Bid Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_BID_TRK updates are usually quarterly from participants before an Auction. RESIDUE_BID_TRK data is confidential to the relevant participant. RESIDUE_BID_TRK excludes contracts and versions without a valid publication date (i.e invalid bids are ignored). Volume Assuming monthly contracts, RESIDUE_BID_TRK shows a maximum of 500 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueBidTrk1 {
    /// SRA Contract unique identifier
    pub contractid: Option<String>,
    /// Version of Bid used
    pub versionno: rust_decimal::Decimal,
    /// Identifier of participant
    pub participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidloaddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionResidueBidTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_BID_TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_CONTRACTS
///  _RESIDUE_CONTRACTS supports the Settlement Residue Auction, by holding the contract details for each period for which a residue contract will be offered._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Contracts
/// * Data Version: 1
///
/// # Description
///  RESIDUE_CONTRACTS data is public, so is available to all participants. Source RESIDUE_CONTRACTS updates are quarterly by AEMO. Volume Assuming quarterly contracts, RESIDUE_CONTRACTS contains a maximum of 50 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * TRANCHE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueContracts1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    pub tranche: rust_decimal::Decimal,
    /// Unique identifier for each SRA Contract as specified by AEMO
    pub contractid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub auctiondate: Option<chrono::NaiveDateTime>,
    /// Identifies methodology used
    pub calcmethod: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifypostdate: Option<chrono::NaiveDateTime>,
    /// Name of notifying person
    pub notifyby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Name of publishing officer or process
    pub postedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Description of Contract
    pub description: Option<String>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: Option<String>,
}
impl crate::GetTable for IrauctionResidueContracts1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CONTRACTS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_CON_DATA
///  _RESIDUE_CON_DATA supports the Settlement Residue Auction, by holding for each participant the confidential data from the auction. RESIDUE_CON_DATA joins to RESIDUE_PUBLIC_DATA and RESIDUE_TRK._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Data
/// * Data Version: 2
///
/// # Description
///  Source RESIDUE_CON_DATA refreshes whenever a Settlement Residue Auction is run (i.e. quarterly). RESIDUE_CON_DATA data is confidential to the relevant participant. RESIDUE_CON_DATA excludes contracts and versions without a valid publication date (i.e invalid bids are ignored). Volume RESIDUE_CON_DATA shows a maximum of 6000 records per year.
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
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConData2 {
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Contract run version
    pub versionno: rust_decimal::Decimal,
    /// Identifier of Contracted Participant
    pub participantid: String,
    /// Identifier of Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Units purchased on the directional interconnector (i.e. Contracted quantity)
    pub unitspurchased: Option<rust_decimal::Decimal>,
    /// Payment due (i.e. total purchase price)
    pub linkpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The number of cancelled Units for all Auction Participants.
    pub secondary_units_sold: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionResidueConData2 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CON_DATA".into()),
            version: 2,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_CON_ESTIMATES_TRK
///  _RESIDUE_CON_ESTIMATES_TRK supports the Settlement Residue Auction, by holding the tracking details of the estimates used to generate the reserve price for each contract._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Estimates Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_CON_ESTIMATES_TRK updates are quarterly by SRA team. Volume Assuming monthly contracts, RESIDUE_CON_ESTIMATES_TRK shows a maximum of 50 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * CONTRACTYEAR
/// * QUARTER
/// * VALUATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConEstimatesTrk1 {
    /// SRA Contract unique identifier
    pub contractid: String,
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Contract Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of a record, as nominated by the participant
    pub versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResidueConEstimatesTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CON_ESTIMATES_TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_CON_FUNDS
///  _RESIDUE_CON_FUNDS supports the Settlement Residue Auction, by holding the fund details for each contract._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Funds
/// * Data Version: 1
///
/// # Description
///  RESIDUE_CON_FUNDS data is public, so is available to all participants. Source RESIDUE_CON_FUNDS updates are quarterly from SRA team via SRIS interface. Volume Assuming quarterly contracts, RESIDUE_CON_FUNDS contains a maximum of 600 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConFunds1 {
    /// SRA Contract unique identifier as specified by AEMO
    pub contractid: String,
    /// Identifier for the Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Actual number of units allocated based on the auction default percentage for the tranche and the total number of units to be auctioned for this quarter
    pub defaultunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter
    pub rolloverunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter because they were not taken up by the participant
    pub reallocatedunits: Option<rust_decimal::Decimal>,
    /// Total units offered for Contract
    pub unitsoffered: Option<rust_decimal::Decimal>,
    /// Average reserve price calculated from the selected estimates
    pub meanreserveprice: Option<rust_decimal::Decimal>,
    /// Scaling factor for regional Frequency control Ancillary Service requirement
    pub scalefactor: Option<rust_decimal::Decimal>,
    /// Actual reserve price
    pub actualreserveprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResidueConFunds1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CON_FUNDS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_FUNDS_BID
///  _RESIDUE_FUNDS_BID supports the Settlement Residue Auction, by showing the fund details for each SRA bid by each Participant._
///
/// * Data Set Name: Irauction Bids
/// * File Name: Funds Bid
/// * Data Version: 1
///
/// # Description
///  Source Participant's bid file. RESIDUE_FUNDS_BID data is confidential to the relevant participant. RESIDUE_FUNDS_BID shows a maximum of 30,000 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFundsBid1 {
    /// SRA Contract identifier
    pub contractid: String,
    /// Participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// Unique option identifier (1..20)
    pub optionid: rust_decimal::Decimal,
    /// Interconnector Identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Quantity of units bid for
    pub units: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionBidsFundsBid1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_BIDS".into(),
            table_name: Some("FUNDS_BID".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_PRICE_BID
///  _RESIDUE_PRICE_BID supports the Settlement Residue Auction, holding the unit and bid price details for each participant._
///
/// * Data Set Name: Irauction Bids
/// * File Name: Price Bid
/// * Data Version: 1
///
/// # Description
///  Source The participant's own bid file RESIDUE_PRICE_BID data is confidential to the relevant participant. The public version of the data is available to all auction participants post the associated auction date in RESIDUE_PRICE_FUNDS_BID. Volume RESIDUE_PRICE_BID shows a maximum of 10,000 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsPriceBid1 {
    /// Not to be used. Unique id for each SRA contract (specified by AEMO)
    pub contractid: Option<String>,
    /// Participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// Unique option (bid) identifier (1..800)
    pub optionid: rust_decimal::Decimal,
    /// Price offered for each unit
    pub bidprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionBidsPriceBid1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_BIDS".into(),
            table_name: Some("PRICE_BID".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_PRICE_FUNDS_BID
///  _RESIDUE_PRICE_FUNDS_BIDshows the bids producing the auction outcome, without exposing participant-specific details. RESIDUE_PRICE_FUNDS_BID is new in March 2003 to support SRA Inter-Temporal Linking._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Price Funds Bid
/// * Data Version: 1
///
/// # Description
///  RESIDUE_PRICE_FUNDS_BID data is public. The data is available to all auction participants post the associated auction date. Volume The volume is very dependent on the number of active bids. An indication is about 250,000 per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * LINKEDBIDFLAG
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePriceFundsBid1 {
    /// Unique id for each contract specified by AEMO
    pub contractid: String,
    /// Unique interconnector identifier
    pub interconnectorid: String,
    /// Unique region identifier
    pub fromregionid: String,
    /// Quantity of units bid
    pub units: Option<rust_decimal::Decimal>,
    /// Price bid for each unit
    pub bidprice: Option<rust_decimal::Decimal>,
    /// A unique option id, with respect to the auction, created to show which bid elements are linked.
    pub linkedbidflag: rust_decimal::Decimal,
    /// Unique id for each auction date
    pub auctionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResiduePriceFundsBid1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_PRICE_FUNDS_BID".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_PUBLIC_DATA
///  _RESIDUE_PUBLIC_DATA shows the public auction results.<br>RESIDUE_PUBLIC_DATA supports the Settlement Residue Auction, by holding the public details of the auction for a given contract. RESIDUE_PUBLIC_DATA joins to RESIDUE_CON_DATA and RESIDUE.<br>_
///
/// * Data Set Name: Irauction
/// * File Name: Residue Public Data
/// * Data Version: 1
///
/// # Description
///  RESIDUE_PUBLIC_DATA excludes contracts and versions without a valid publication date (i.e. invalid bids are ignored).  The data is available to all auction participants post the associated auction date. Source RESIDUE_PUBLIC_DATA updates are quarterly from NEMMCO. Volume RESIDUE_PUBLIC_DATA shows a maximum of 120 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePublicData1 {
    /// Unique id for each contract to be specified by AEMO
    pub contractid: String,
    /// Version Number
    pub versionno: rust_decimal::Decimal,
    /// Unique interconnector identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total units offered for auction
    pub unitsoffered: Option<rust_decimal::Decimal>,
    /// Units Sold (modified format and usage in March 2003 to support SRA Inter-Temporal Linking)
    pub unitssold: Option<rust_decimal::Decimal>,
    /// Clearing price
    pub clearingprice: Option<rust_decimal::Decimal>,
    /// Reserve price
    pub reserveprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResiduePublicData1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_PUBLIC_DATA".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## RESIDUE_TRK
///  _RESIDUE_TRK supports the Settlement Residue Auction, by showing the tracking records for different residue auction runs. RESIDUE_TRK joins to RESIDUE_PUBLIC_DATA and RESIDUE_CON_DATA._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_TRK updates whenever Settlement Residue Auctions are run and the results published (i.e. quarterly). The RESIDUE_TRK data is available to all participants post the associated auction date. Volume Assuming quarterly contracts, RESIDUE_TRK shows a maximum of 50 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueTrk1 {
    /// SRA Contract identifier
    pub contractid: Option<String>,
    /// Contract run version
    pub versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub rundate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer or process
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    pub postedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionResidueTrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_TRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_CASH_SECURITY
///  _Records the Cash Security details provided by an SRA Auction Participant as collateral to cover their Trading Position in the SRA market_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Cash Security
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraCashSecurity1 {
    /// Unique identifier for the cash security.
    pub cash_security_id: String,
    /// Unique identifier for the auction participant lodging the cash security.
    pub participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub provision_date: Option<chrono::NaiveDateTime>,
    /// Dollar amount of the cash security.
    pub cash_amount: Option<rust_decimal::Decimal>,
    /// The interest account ID for calculating the interest payment
    pub interest_acct_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub finalreturndate: Option<chrono::NaiveDateTime>,
    /// Returned Dollar amount of the Cash Security.
    pub cash_security_returned: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub deletiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraCashSecurity1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_CASH_SECURITY".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_AUCPAY_DETAIL
///  _Records details of the SRA financial auction payment_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Aucpay Detail
/// * Data Version: 1
///
///
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
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpayDetail1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique  participant identifier
    pub participantid: String,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: String,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: String,
    /// The SRA contract identifier
    pub contractid: String,
    /// The Maximum Units Available for purchase in the Auction
    pub maximum_units: Option<rust_decimal::Decimal>,
    /// The total number of Allocated Units in the Auction, including Cancelled Units by an Auction Participant
    pub units_sold: Option<rust_decimal::Decimal>,
    /// The total number of units unpaid for in the auction
    pub shortfall_units: Option<rust_decimal::Decimal>,
    /// The reserve price of the auction
    pub reserve_price: Option<rust_decimal::Decimal>,
    /// The Market Clearing Price of the Auction
    pub clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO before shortfall
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The percentage of the auction proceeds allocated to the TNSP on the directional link
    pub allocation: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO, including shortfall
    pub net_payment_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraFinancialAucpayDetail1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUCPAY_DETAIL".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_AUCPAY_SUM
///  _Records a summary of the Auction payment amount_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Aucpay Sum
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpaySum1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// The total auction proceeds allocated to the TNSP
    pub gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The total auction proceeds allocated to all TNSPs in the SRA quarter
    pub total_gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount for in the SRA Quarter for the TNSP
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The total shortfall amount for in the SRA Quarter for all TNSPs
    pub total_shortfall_amount: Option<rust_decimal::Decimal>,
    /// The net payment amount owed by AEMO to the TNSP
    pub net_payment_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraFinancialAucpaySum1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUCPAY_SUM".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_MARDETAIL
///  _This table stores details of the margins returned to the participants._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Mardetail
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMardetail1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// The participant identifier.
    pub participantid: String,
    /// Unique identifier for the cash security.
    pub cash_security_id: String,
    /// The amount returned to the Auction participant from this cash security.
    pub returned_amount: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to the returned amount.
    pub returned_interest: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraFinancialAucMardetail1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUC_MARDETAIL".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_MARGIN
///  _Records the amount of Cash Security required to be held by an Auction Participant after settlement_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Margin
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMargin1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique  participant identifier.
    pub participantid: String,
    /// Total cash security held by the participant.
    pub total_cash_security: Option<rust_decimal::Decimal>,
    /// The amount of trading  cash security required to be held by the participant after settlement.
    pub required_margin: Option<rust_decimal::Decimal>,
    /// The amount of cash security returned to the participant.
    pub returned_margin: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to returned cash security amounts.
    pub returned_margin_interest: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraFinancialAucMargin1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUC_MARGIN".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_RECEIPTS
///  _Records details of the Cancelled Units and their value for the Auction Participant_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Receipts
/// * Data Version: 1
///
///
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
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucReceipts1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: String,
    /// The source region identifier for the Directional Interconnector
    pub fromregionid: String,
    /// The SRA contract identifier
    pub contractid: String,
    /// The number of units purchased
    pub units_purchased: Option<rust_decimal::Decimal>,
    /// The clearing price of the auction
    pub clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed to AEMO
    pub receipt_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Dollar value of Cancelled Units in the Auction for the Auction Participant
    pub proceeds_amount: Option<rust_decimal::Decimal>,
    /// Units cancelled in the auction by the Auction  participant.
    pub units_sold: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraFinancialAucReceipts1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUC_RECEIPTS".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_RUNTRK
///  _Records details of the settlement process for the cancellation and purchase of SRA Auction Units_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Runtrk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialRuntrk1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// The type of SRA run
    pub runtype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub rundate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub posteddate: Option<chrono::NaiveDateTime>,
    /// Version number of the interest component used in the payments run
    pub interest_versionno: Option<i64>,
    /// Version number of the makeup component used in the makeup run
    pub makeup_versionno: Option<i64>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraFinancialRuntrk1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_RUNTRK".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_OFFER_PRODUCT
///  _Holds the Product details for each Offer File submitted by each SRA Auction Participant._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Offer Product
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProduct1 {
    /// Unique ID for each Auction date
    pub auctionid: String,
    /// Unique participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// Unique Product identifier (1 - 2000)
    pub optionid: i64,
    /// Unique Directional Interconnector identifier
    pub interconnectorid: Option<String>,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: Option<String>,
    /// The Offer quantity for this Product
    pub offer_quantity: Option<i64>,
    /// The Offer price for this Product
    pub offer_price: Option<rust_decimal::Decimal>,
    /// Tranche identifier
    pub trancheid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraOfferProduct1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_OFFER_PRODUCT".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_OFFER_PROFILE
///  _Holds the data of an SRA Auction Participant Offer Submission._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Offer Profile
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProfile1 {
    /// Unique ID for each Auction date
    pub auctionid: String,
    /// Unique participant identifier
    pub participantid: String,
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// SRA Offer File name
    pub filename: Option<String>,
    /// SRA acknowledgment file name
    pub ackfilename: Option<String>,
    /// Transaction ID used for tracking
    pub transactionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraOfferProfile1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_OFFER_PROFILE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_PRUDENTIAL_CASH_SECURITY
///  _Records the Cash Security details provided by an SRA Auction Participant as collateral to cover their Trading Position in the SRA market_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Cash Security
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCashSecurity1 {
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub prudential_runno: i64,
    /// Unique participant identifier for the Auction Participant lodging the Cash Security
    pub participantid: String,
    /// Unique identifier for the cash security.
    pub cash_security_id: String,
    /// Remaining Cash Security deposit available
    pub cash_security_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraPrudentialCashSecurity1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_CASH_SECURITY".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_PRUDENTIAL_COMP_POSITION
///  _The prudential position of each company at the date and time of a specific prudential run_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Comp Position
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCompPosition1 {
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub prudential_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// The Trading Limit of the company at the time of the prudential run
    pub trading_limit: Option<rust_decimal::Decimal>,
    /// Current Prudential Exposure of the Auction Participant including Offers
    pub prudential_exposure_amount: Option<rust_decimal::Decimal>,
    /// The amount of Trading Margin available to the Auction Participant to trade (including Offered Units and Cancelled Units). Equal to TRADING_LIMIT minus PRUDENTIAL_EXPOSURE_AMOUNT.
    pub trading_margin: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraPrudentialCompPosition1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_COMP_POSITION".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_PRUDENTIAL_EXPOSURE
///  _Records details of the Prudential Exposure of an SRA Auction Participant_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Exposure
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
/// * SRA_QUARTER
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialExposure1 {
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date.
    pub prudential_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// AEMO Contract Year number starting the week beginning 1 January
    pub sra_year: i64,
    /// Contract Relevant Quarter
    pub sra_quarter: i64,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: String,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: String,
    /// The largest Tranche where the Unit was either sold or offered. Used in the calculation of the Average Purchase Price for the Trading Position of the Product. The most recent Tranche where Units were cancelled or offered (if the Offer is below the Average Purchase Price)
    pub max_tranche: Option<i64>,
    /// Unique identifier for the Auction having the Offer. Has a null value when no Offer is made for the Relevant Quarter
    pub auctionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_submissiontime: Option<chrono::NaiveDateTime>,
    /// Calculated Average Purchase Price for the Product
    pub average_purchase_price: Option<rust_decimal::Decimal>,
    /// Calculated average cancellation price for product.
    pub average_cancellation_price: Option<rust_decimal::Decimal>,
    /// Calculated cancellation volume for product.
    pub cancellation_volume: Option<rust_decimal::Decimal>,
    /// Calculated trading position for product.
    pub trading_position: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraPrudentialExposure1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_EXPOSURE".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## SRA_PRUDENTIAL_RUN
///  _Records the prudential run details for each prudential date_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Run
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialRun1 {
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The prudential run number for the run
    pub prudential_runno: i64,
}
impl crate::GetTable for IrauctionSraPrudentialRun1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_RUN".into()),
            version: 1,
        }
    }
}
/// # Summary
///
/// ## VALUATIONID
///  _VALUATIONID shows the identifiers and descriptions of the valuers submitting estimates of upcoming settlement residues. VALUATIONID supports the Settlement Residue Auction._
///
/// * Data Set Name: Irauction
/// * File Name: Valuationid
/// * Data Version: 1
///
/// # Description
///  VALUATIONID is public data, and is available to all participants. Source VALUATIONID updates are quarterly from the Settlement Residues Information System [SRIS]. Volume VALUATIONID shows up to five (5) records. Updates are rare.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * VALUATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionValuationid1 {
    /// Identifier of the estimator
    pub valuationid: String,
    /// Full name of estimator
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionValuationid1 {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("VALUATIONID".into()),
            version: 1,
        }
    }
}
