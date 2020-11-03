/// # Summary
/// 
/// ## TRADINGINTERCONNECT
///  _TRADINGINTERCONNECT shows the half-hourly summary of Interconnector flows based on 5-minute averages._
/// 
/// * Data Set Name: Trading
/// * File Name: Interconnectorres
/// * Data Version: 2
/// 
/// # Description
///  TRADINGINTERCONNECT is public data, and is available to all participants. Source TRADINGINTERCONNECT is updated half hourly.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERCONNECTORID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingInterconnectorres2 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// Period Identifier
    pub periodid: rust_decimal::Decimal,
    /// Average of the metered MW flow from the start of each dispatch interval.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Flow from SPD
    pub mwflow: Option<rust_decimal::Decimal>,
    /// MW losses at calculated MW flow
    pub mwlosses: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for TradingInterconnectorres2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: Some("INTERCONNECTORRES".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## TRADINGLOAD
///  _TRADINGLOAD shows half-hourly average dispatch levels, including fields to handle the Ancillary Services functionality._
/// 
/// * Data Set Name: Trading
/// * File Name: Unit Solution
/// * Data Version: 2
/// 
/// # Description
///  Source Own (confidential) TRADINGLOAD data updates half hourly, with public availability of all data on next day. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
/// * TRADETYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingUnitSolution2 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Not used
    pub tradetype: rust_decimal::Decimal,
    /// Period Identifier
    pub periodid: rust_decimal::Decimal,
    /// Average Initial MW at start of each period
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Average total MW dispatched over period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Average ramp down rate
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Average ramp up rate
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Average 5 min lower dispatch
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Average 60 sec lower dispatch
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Average60 sec lower dispatch
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Average 5 min raise dispatch
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Average 60 sec raise dispatch
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Average 6 sec raise dispatch
    pub raise6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    pub availability: Option<rust_decimal::Decimal>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for TradingUnitSolution2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: Some("UNIT_SOLUTION".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## TRADINGREGIONSUM
///  _TRADINGREGIONSUM sets out the half-hourly average regional demand and frequency control services. TRADINGREGIONSUM includes fields for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations._
/// 
/// * Data Set Name: Trading
/// * File Name: Regionsum
/// * Data Version: 4
/// 
/// # Description
///  TRADINGREGIONSUM is public data, and is available to all participants. Source TRADINGREGIONSUM is updated every 30 minutes.
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
pub struct TradingRegionsum4 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Trading interval identifier within settlement day.
    pub periodid: rust_decimal::Decimal,
    /// Total demand for region
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// The available generation in the Region for the interval
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Not used
    pub availableload: Option<rust_decimal::Decimal>,
    /// Forecast demand for region
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Averaged generation dispatched in region
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Averaged load dispatched in region
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Average energy transferred over interconnector
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Average excess generation in region
    pub excessgeneration: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    pub lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    pub lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    pub lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 5 min
    pub lower5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    pub lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 5 min
    pub lower5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    pub lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 5 min
    pub lower5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    pub lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    pub lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    pub lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 60 sec
    pub lower60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    pub lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 60 sec
    pub lower60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    pub lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 60 sec
    pub lower60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    pub lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    pub lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    pub lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 6 sec
    pub lower6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    pub lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 6 sec
    pub lower6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    pub lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 6 sec
    pub lower6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW dispatch
    pub raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 5 min
    pub raise5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    pub raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 5 min
    pub raise5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    pub raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 5 min
    pub raise5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    pub raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    pub raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 60 sec local dispatch
    pub raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 60 sec
    pub raise60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    pub raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 60 sec
    pub raise60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec total requirement
    pub raise60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 60 sec
    pub raise60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW dispatch
    pub raise6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW imported
    pub raise6secimport: Option<rust_decimal::Decimal>,
    /// Raise 6 sec local dispatch
    pub raise6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 6 sec
    pub raise6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec local requirement
    pub raise6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 6 sec
    pub raise6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec total requirement
    pub raise6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 6 sec
    pub raise6secsupplyprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of initial generation and import for region
    pub initialsupply: Option<rust_decimal::Decimal>,
    /// Sum of cleared generation and import for region
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation MW imported
    pub lowerregimport: Option<rust_decimal::Decimal>,
    /// Lower Regulation local dispatch
    pub lowerreglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation local requirement
    pub lowerreglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation total requirement
    pub lowerregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation MW imported
    pub raiseregimport: Option<rust_decimal::Decimal>,
    /// Raise Regulation local dispatch
    pub raisereglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation local requirement
    pub raisereglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation total requirement
    pub raiseregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min local requirement
    pub raise5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg local requirement
    pub raisereglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 sec local requirement
    pub raise60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 sec local requirement
    pub raise6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min local requirement
    pub lower5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg local requirement
    pub lowerreglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 sec local requirement
    pub lower60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 sec local requirement
    pub lower6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min requirement
    pub raise5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg requirement
    pub raiseregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 seconds requirement
    pub raise60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 seconds requirement
    pub raise6secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min requirement
    pub lower5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg requirement
    pub lowerregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 seconds requirement
    pub lower60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 seconds requirement
    pub lower6secviolation: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for TradingRegionsum4 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: Some("REGIONSUM".into()),
                        version: 4,
                    }
                    
    }
}
/// # Summary
/// 
/// ## TRADINGPRICE
///  _TRADINGPRICE sets out half-hourly spot market price, including fields to handle the Ancillary Services functionality. If prices are adjusted, the final price is recorded in the regional reference price (RRP) field with price before adjustment recorded in the regional original price (ROP) field._
/// 
/// * Data Set Name: Trading
/// * File Name: Price
/// * Data Version: 2
/// 
/// # Description
///  TRADINGPRICE data is public, so is available to all participants. Source TRADINGPRICE updates every 30 minutes. Notes INVALIDFLAG The INVALIDFLAG field is used to indicate whether the Trading interval price has been adjusted after the trading interval was completed. On a very restricted set of events, the market rules allow a dispatch price (5 min) to be adjusted on the next business day, and, when this occurs, the corresponding trading interval price for that region is also adjusted and marked as adjusted with INVALIDFLAG of 'A'. The INVALIDFLAG = 'Y' only applies to historical periods when not all six of the 5-minute dispatch intervals were run in the trading interval. System changes implemented on 30 September 2001 mean this situation no longer occurs since missing dispatch intervals are automatically populated from a previous interval. If the INVALIDFLAG field = '0', the price was not adjusted and all six dispatch intervals are present. Prices There is no field in the TRADINGPRICE table (or the MMS data model anywhere) telling you that the price is provisional or final. The only reliable method is to ensure that the trading date is at least 2 business days old.
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
pub struct TradingPrice2 {
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Run No
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Trading Interval Period
    pub periodid: rust_decimal::Decimal,
    /// Regional reference price for this dispatch period
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price where negative average
    pub eep: Option<rust_decimal::Decimal>,
    /// Indicates when the Trading interval price has been adjusted after the trading interval was completed
    pub invalidflag: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Regional Original Price. The price before any adjustments were made
    pub rop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// Status of regional prices for this dispatch interval "NOT FIRM" or "FIRM"
    pub price_status: Option<String>,
}
impl crate::GetTable for TradingPrice2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: Some("PRICE".into()),
                        version: 2,
                    }
                    
    }
}
