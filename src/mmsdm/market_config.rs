/// # Summary
/// 
/// ## BIDTYPES
///  _BIDTYPES, together with the associated tracking data in BIDTYPESTRK, define a set of ancillary services with bidding parameters from a given date.<br>BIDTYPES is static data describing each type of bid quantity, the number of applicable bands, how many days ahead a price lock down becomes effective and the validation rule that applies.<br>_
/// 
/// * Data Set Name: Market Config
/// * File Name: Bidtypes
/// * Data Version: 1
/// 
/// # Description
///  BIDTYPES  is public to participants Source BIDTYPES updates when the static data relating to an ancillary service type is modified. Volume Expect modifications to be rare. Allow for approximately 20 records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypes1 {
    /// Bid Type Identifier
    pub bidtype: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Description of this Bid Type
    pub description: Option<String>,
    /// Number of active bands (1 to 10)
    pub numberofbands: Option<rust_decimal::Decimal>,
    /// Number of days prior to the Market Day when prices are locked from 12:30pm
    pub numdaysaheadpricelocked: Option<rust_decimal::Decimal>,
    /// ENERGY or AS validation rules to apply.
    pub validationrule: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Alias for this BIDTYPE used in the SPD Solver
    pub spdalias: Option<String>,
}
impl crate::GetTable<MarketConfigBidtypes1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "BIDTYPES".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## REGIONSTANDINGDATA
///  _REGIONSTANDINGDATA sets out standing region data including the region reference node._
/// 
/// * Data Set Name: Market Config
/// * File Name: Regionstandingdata
/// * Data Version: 1
/// 
/// # Description
///  REGIONSTANDINGDATA data is public, so is available to all participants. Source REGIONSTANDINGDATA only changes when a change is made to a region. This table changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegionstandingdata1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No of the standing data that should be effective on this date
    pub versionno: rust_decimal::Decimal,
    /// Differentiates this region from all other regions
    pub regionid: String,
    /// the unique identifier of the participant with responsibility for the region.
    pub rsoid: Option<String>,
    /// unique id of a connection point, being the reference point for this region
    pub regionalreferencepointid: Option<String>,
    /// Period identifier of the peak trading period of this connection point
    pub peaktradingperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Scaling factor for regional FCAS requirement
    pub scalingfactor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigRegionstandingdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "REGIONSTANDINGDATA".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## MARKET_PRICE_THRESHOLDS
///  _MARKET_PRICE_THRESHOLDS sets out the market cap , floor and administered price thresholds applying to the electricity market_
/// 
/// * Data Set Name: Market Config
/// * File Name: Market Price Thresholds
/// * Data Version: 1
/// 
/// # Description
///  MARKET_PRICE_THRESHOLDS data is public, so is available to all participants. Source MARKET_PRICE_THRESHOLDS only changes when a change is made to a market price threshold. This table changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigMarketPriceThresholds1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// version no for the effective date
    pub versionno: rust_decimal::Decimal,
    /// value of lost load if total supply falls short of demand after load management then involuntary load
    pub voll: Option<rust_decimal::Decimal>,
    /// The floor price that the spot market price will not fall below.
    pub marketpricefloor: Option<rust_decimal::Decimal>,
    /// Threshold value beyond which Aggregate Prices per Region over 336 Trade Intervals (Energy), or 2016 Dispatch Intervals (FCAS), will result in an Administered Price declaration
    pub administered_price_threshold: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// user authorising
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigMarketPriceThresholds1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "MARKET_PRICE_THRESHOLDS".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## REGION
///  _REGION sets out valid region IDs._
/// 
/// * Data Set Name: Market Config
/// * File Name: Region
/// * Data Version: 1
/// 
/// # Description
///  REGION data is public, so is available to all participants. Source REGION updates if a change is ever made to a region. This table is static data and is likely to change very infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegion1 {
    /// Differentiates this region from all other regions
    pub regionid: String,
    /// Full description of region
    pub description: Option<String>,
    /// Status of the region e.g. working, inactive, archive.
    pub regionstatus: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigRegion1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "REGION".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## INTERCONNECTOR
///  _INTERCONNECTOR sets out valid identifiers for each interconnector._
/// 
/// * Data Set Name: Market Config
/// * File Name: Interconnector
/// * Data Version: 1
/// 
/// # Description
///  INTERCONNECTOR is public data, available to all participants. Source INTERCONNECTOR changes infrequently, usually annually. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnector1 {
    /// Unique Id of this interconnector
    pub interconnectorid: String,
    /// Starting region of the interconnect
    pub regionfrom: Option<String>,
    /// Not used
    pub rsoid: Option<String>,
    /// Ending region of the interconnect
    pub regionto: Option<String>,
    /// Description of interconnector
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigInterconnector1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "INTERCONNECTOR".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## INTERCONNECTORCONSTRAINT
///  _INTERCONNECTORCONSTRAINT sets out Interconnector limit data used as defaults in dispatch, predispatch and STPASA and used by SPD in calculating flows. INTERCONNECTORCONSTRAINT includes an additional field to restrict an interconnector from support transfer of FCAS._
/// 
/// * Data Set Name: Market Config
/// * File Name: Interconnectorconstraint
/// * Data Version: 1
/// 
/// # Description
///  INTERCONNECTORCONSTRAINT is public data, available to all participants. Source INTERCONNECTORCONSTRAINT changes infrequently, typically annually. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectorconstraint1 {
    /// SPD Factor
    pub reserveoverallloadfactor: Option<rust_decimal::Decimal>,
    /// Loss share attributable to from region
    pub fromregionlossshare: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version for this date
    pub versionno: rust_decimal::Decimal,
    /// Unique Id of this interconnector
    pub interconnectorid: String,
    /// Limit of energy flowing into the RegionFrom
    pub maxmwin: Option<rust_decimal::Decimal>,
    /// Limit of energy flowing out of the Region
    pub maxmwout: Option<rust_decimal::Decimal>,
    /// Constant Loss factor
    pub lossconstant: Option<rust_decimal::Decimal>,
    /// Linear coefficient of loss factor calculation
    pub lossflowcoefficient: Option<rust_decimal::Decimal>,
    /// Identifies the EMS entity that represents the interconnector flow
    pub emsmeasurand: Option<String>,
    /// User authorising record
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub dynamicrhs: Option<String>,
    /// Interconnector import limit
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Interconnector export limit
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// SPD Factor
    pub outagederationfactor: Option<rust_decimal::Decimal>,
    /// Factor for non-physical losses rerun
    pub nonphysicallossfactor: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 60 sec
    pub overloadfactor60sec: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 6 sec
    pub overloadfactor6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate that the interconnector cannot support FCAS Transfers
    pub fcassupportunavailable: Option<rust_decimal::Decimal>,
    /// Interconnector type - Currently either "REGULATED" or "MNSP"
    pub ictype: Option<String>,
}
impl crate::GetTable<MarketConfigInterconnectorconstraint1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "INTERCONNECTORCONSTRAINT".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## INTRAREGIONALLOC
///  _INTRAREGIONALLOC shows allocations of intra-regional residues to participants._
/// 
/// * Data Set Name: Market Config
/// * File Name: Intraregionalloc
/// * Data Version: 1
/// 
/// # Description
///  INTRAREGIONALLOC data is confidential to the relevant participant. Source The data in INTRAREGIONALLOC changes infrequently. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigIntraregionalloc1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Allocation Percent / 100
    pub allocation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigIntraregionalloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "INTRAREGIONALLOC".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## INTERCONNECTORALLOC
///  _INTERCONNECTORALLOC shows allocations of interconnector residues to Network Service Providers._
/// 
/// * Data Set Name: Market Config
/// * File Name: Interconnectoralloc
/// * Data Version: 1
/// 
/// # Description
///  INTERCONNECTORALLOC data is confidential to the relevant participant. Source INTERCONNECTORALLOC changes infrequently, typically annually. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectoralloc1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// Region Identifier
    pub regionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Allocation % / 100
    pub allocation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigInterconnectoralloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "INTERCONNECTORALLOC".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## BIDTYPESTRK
///  _BIDTYPESTRK, together with the associated data in BIDTYPES, define a set of ancillary services with bidding parameters from a given date._
/// 
/// * Data Set Name: Market Config
/// * File Name: Bidtypestrk
/// * Data Version: 1
/// 
/// # Description
///  BIDTYPESTRK is public to participants Source BIDTYPESTRK updates when the static data relating to an ancillary service type is modified. Volume Expect modifications to be rare. Allow for approximately 20 records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypestrk1 {
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
impl crate::GetTable<MarketConfigBidtypestrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "BIDTYPESTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## LOSSMODEL
///  _LOSSMODEL sets out segment breakpoints in loss model for each interconnector, used by LP Solver modelling of interconnector flows._
/// 
/// * Data Set Name: Market Config
/// * File Name: Lossmodel
/// * Data Version: 1
/// 
/// # Description
///  LOSSMODEL data is public, so is available to all participants. Source LOSSMODEL only changes annually, when there is a change in the interconnector. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * LOSSSEGMENT
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossmodel1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date
    pub versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// Not used
    pub periodid: Option<String>,
    /// Segment Identifier (1 to 80 at present)
    pub losssegment: rust_decimal::Decimal,
    /// MW Value for segment
    pub mwbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub lossfactor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigLossmodel1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "LOSSMODEL".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## LOSSFACTORMODEL
///  _LOSSFACTORMODEL sets out the demand coefficients for each interconnector, used by LP Solver modelling of interconnector flows._
/// 
/// * Data Set Name: Market Config
/// * File Name: Lossfactormodel
/// * Data Version: 1
/// 
/// # Description
///  LOSSFACTORMODEL is public data, so is available to all participants. Source LOSSFACTORMODEL only changes annually, when there is a change in the interconnector. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossfactormodel1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date of the status proposed
    pub versionno: rust_decimal::Decimal,
    /// The unique identifier for the interconnector.
    pub interconnectorid: String,
    /// The unique region identifier for a connection point of the interconnector
    pub regionid: String,
    /// The coefficient applied to the region demand in the calculation of the interconnector loss factor
    pub demandcoefficient: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketConfigLossfactormodel1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "LOSSFACTORMODEL".into(),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## TRANSMISSIONLOSSFACTOR
///  _TRANSMISSIONLOSSFACTOR shows the Transmission Loss factors applied at each connection point._
/// 
/// * Data Set Name: Market Config
/// * File Name: Transmissionlossfactor
/// * Data Version: 2
/// 
/// # Description
///  TRANSMISSIONLOSSFACTOR is public data, and is available to all participants. Source TRANSMISSIONLOSSFACTOR updates when new connection points are created or loss factors change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigTransmissionlossfactor2 {
    /// Transmission Loss Factor
    pub transmissionlossfactor: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record for given effective date
    pub versionno: rust_decimal::Decimal,
    /// Connection Point ID
    pub connectionpointid: String,
    /// &nbsp; 
    pub regionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Secondary transmission loss factor applied in settlements for generator purchases.
    pub secondary_tlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<MarketConfigTransmissionlossfactor2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "TRANSMISSIONLOSSFACTOR".into(),
                        version: 2,
                    }
                    
    }
}
