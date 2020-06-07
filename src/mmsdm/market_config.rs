/// Data Set Name: Market Config
/// File Name: Lossmodel
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossmodel1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date
    versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    interconnectorid: String,
    /// Not used
    periodid: Option<String>,
    /// Segment Identifier (1 to 80 at present)
    losssegment: rust_decimal::Decimal,
    /// MW Value for segment
    mwbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    lossfactor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Config
/// File Name: Regionstandingdata
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegionstandingdata1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version No of the standing data that should be effective on this date
    versionno: rust_decimal::Decimal,
    /// Differentiates this region from all other regions
    regionid: String,
    /// the unique identifier of the participant with responsibility for the region.
    rsoid: Option<String>,
    /// unique id of a connection point, being the reference point for this region
    regionalreferencepointid: Option<String>,
    /// Period identifier of the peak trading period of this connection point
    peaktradingperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    authorisedby: Option<String>,
    /// Scaling factor for regional FCAS requirement
    scalingfactor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Config
/// File Name: Lossfactormodel
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossfactormodel1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date of the status proposed
    versionno: rust_decimal::Decimal,
    /// The unique identifier for the interconnector.
    interconnectorid: String,
    /// The unique region identifier for a connection point of the interconnector
    regionid: String,
    /// The coefficient applied to the region demand in the calculation of the interconnector loss factor
    demandcoefficient: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Config
/// File Name: Interconnectorconstraint
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectorconstraint1 {
    /// SPD Factor
    reserveoverallloadfactor: Option<rust_decimal::Decimal>,
    /// Loss share attributable to from region
    fromregionlossshare: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version for this date
    versionno: rust_decimal::Decimal,
    /// Unique Id of this interconnector
    interconnectorid: String,
    /// Limit of energy flowing into the RegionFrom
    maxmwin: Option<rust_decimal::Decimal>,
    /// Limit of energy flowing out of the Region
    maxmwout: Option<rust_decimal::Decimal>,
    /// Constant Loss factor
    lossconstant: Option<rust_decimal::Decimal>,
    /// Linear coefficient of loss factor calculation
    lossflowcoefficient: Option<rust_decimal::Decimal>,
    /// Identifies the EMS entity that represents the interconnector flow
    emsmeasurand: Option<String>,
    /// User authorising record
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    dynamicrhs: Option<String>,
    /// Interconnector import limit
    importlimit: Option<rust_decimal::Decimal>,
    /// Interconnector export limit
    exportlimit: Option<rust_decimal::Decimal>,
    /// SPD Factor
    outagederationfactor: Option<rust_decimal::Decimal>,
    /// Factor for non-physical losses rerun
    nonphysicallossfactor: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 60 sec
    overloadfactor60sec: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 6 sec
    overloadfactor6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate that the interconnector cannot support FCAS Transfers
    fcassupportunavailable: Option<rust_decimal::Decimal>,
    /// Interconnector type - Currently either "REGULATED" or "MNSP"
    ictype: Option<String>,
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
/// Data Set Name: Market Config
/// File Name: Market Price Thresholds
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigMarketPriceThresholds1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// version no for the effective date
    versionno: rust_decimal::Decimal,
    /// value of lost load if total supply falls short of demand after load management then involuntary load
    voll: Option<rust_decimal::Decimal>,
    /// The floor price that the spot market price will not fall below.
    marketpricefloor: Option<rust_decimal::Decimal>,
    /// Threshold value beyond which Aggregate Prices per Region over 336 Trade Intervals (Energy), or 2016 Dispatch Intervals (FCAS), will result in an Administered Price declaration
    administered_price_threshold: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    /// user authorising
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Config
/// File Name: Transmissionlossfactor
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigTransmissionlossfactor2 {
    /// Transmission Loss Factor
    transmissionlossfactor: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version no of record for given effective date
    versionno: rust_decimal::Decimal,
    /// Connection Point ID
    connectionpointid: String,
    /// &nbsp; 
    regionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Secondary transmission loss factor applied in settlements for generator purchases.
    secondary_tlf: Option<rust_decimal::Decimal>,
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
/// Data Set Name: Market Config
/// File Name: Interconnectoralloc
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectoralloc1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    interconnectorid: String,
    /// Region Identifier
    regionid: String,
    /// Unique participant identifier
    participantid: String,
    /// Allocation % / 100
    allocation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Config
/// File Name: Intraregionalloc
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigIntraregionalloc1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    versionno: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Unique participant identifier
    participantid: String,
    /// Allocation Percent / 100
    allocation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Config
/// File Name: Interconnector
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnector1 {
    /// Unique Id of this interconnector
    interconnectorid: String,
    /// Starting region of the interconnect
    regionfrom: Option<String>,
    /// Not used
    rsoid: Option<String>,
    /// Ending region of the interconnect
    regionto: Option<String>,
    /// Description of interconnector
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
/// Data Set Name: Market Config
/// File Name: Bidtypestrk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypestrk1 {
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
impl crate::GetTable<MarketConfigBidtypestrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "MARKET_CONFIG".into(),
                        table_name: "BIDTYPESTRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Market Config
/// File Name: Bidtypes
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypes1 {
    /// Bid Type Identifier
    bidtype: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    /// Record version number
    versionno: rust_decimal::Decimal,
    /// Description of this Bid Type
    description: Option<String>,
    /// Number of active bands (1 to 10)
    numberofbands: Option<rust_decimal::Decimal>,
    /// Number of days prior to the Market Day when prices are locked from 12:30pm
    numdaysaheadpricelocked: Option<rust_decimal::Decimal>,
    /// ENERGY or AS validation rules to apply.
    validationrule: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Alias for this BIDTYPE used in the SPD Solver
    spdalias: Option<String>,
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
/// Data Set Name: Market Config
/// File Name: Region
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegion1 {
    /// Differentiates this region from all other regions
    regionid: String,
    /// Full description of region
    description: Option<String>,
    /// Status of the region e.g. working, inactive, archive.
    regionstatus: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
