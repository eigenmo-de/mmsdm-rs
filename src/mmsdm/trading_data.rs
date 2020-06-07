/// Data Set Name: Trading
/// File Name: Price
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingPrice2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Run No
    runno: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Trading Interval Period
    periodid: rust_decimal::Decimal,
    /// Regional reference price for this dispatch period
    rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price where negative average
    eep: Option<rust_decimal::Decimal>,
    /// Indicates when the Trading interval price has been adjusted after the trading interval was completed
    invalidflag: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Regional Original Price. The price before any adjustments were made
    rop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    raise6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    raise6secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    raise60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    raise60secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    raise5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    raise5minrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    raiseregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    raiseregrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lower6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    lower6secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lower60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    lower60secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lower5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    lower5minrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    lowerregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    lowerregrop: Option<rust_decimal::Decimal>,
    /// Status of regional prices for this dispatch interval "NOT FIRM" or "FIRM"
    price_status: Option<String>,
}
impl crate::GetTable<TradingPrice2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: "PRICE".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Trading
/// File Name: Interconnectorres
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingInterconnectorres2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    runno: rust_decimal::Decimal,
    /// Interconnector identifier
    interconnectorid: String,
    /// Period Identifier
    periodid: rust_decimal::Decimal,
    /// Average of the metered MW flow from the start of each dispatch interval.
    meteredmwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Flow from SPD
    mwflow: Option<rust_decimal::Decimal>,
    /// MW losses at calculated MW flow
    mwlosses: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<TradingInterconnectorres2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: "INTERCONNECTORRES".into(),
                        version: 2,
                    }
                    
    }
}
/// Data Set Name: Trading
/// File Name: Regionsum
/// Data Version: 4
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingRegionsum4 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    runno: rust_decimal::Decimal,
    /// Region Identifier
    regionid: String,
    /// Trading interval identifier within settlement day.
    periodid: rust_decimal::Decimal,
    /// Total demand for region
    totaldemand: Option<rust_decimal::Decimal>,
    /// The available generation in the Region for the interval
    availablegeneration: Option<rust_decimal::Decimal>,
    /// Not used
    availableload: Option<rust_decimal::Decimal>,
    /// Forecast demand for region
    demandforecast: Option<rust_decimal::Decimal>,
    /// Averaged generation dispatched in region
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Averaged load dispatched in region
    dispatchableload: Option<rust_decimal::Decimal>,
    /// Average energy transferred over interconnector
    netinterchange: Option<rust_decimal::Decimal>,
    /// Average excess generation in region
    excessgeneration: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 5 min
    lower5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 5 min
    lower5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 5 min
    lower5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 60 sec
    lower60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 60 sec
    lower60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 60 sec
    lower60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 6 sec
    lower6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 6 sec
    lower6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 6 sec
    lower6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW dispatch
    raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 5 min
    raise5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 5 min
    raise5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 5 min
    raise5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 60 sec local dispatch
    raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 60 sec
    raise60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 60 sec
    raise60secprice: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<TradingRegionsum4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: "REGIONSUM".into(),
                        version: 4,
                    }
                    
    }
}
/// Data Set Name: Trading
/// File Name: Unit Solution
/// Data Version: 2
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingUnitSolution2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    runno: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    duid: String,
    /// Not used
    tradetype: rust_decimal::Decimal,
    /// Period Identifier
    periodid: rust_decimal::Decimal,
    /// Average Initial MW at start of each period
    initialmw: Option<rust_decimal::Decimal>,
    /// Average total MW dispatched over period
    totalcleared: Option<rust_decimal::Decimal>,
    /// Average ramp down rate
    rampdownrate: Option<rust_decimal::Decimal>,
    /// Average ramp up rate
    rampuprate: Option<rust_decimal::Decimal>,
    /// Average 5 min lower dispatch
    lower5min: Option<rust_decimal::Decimal>,
    /// Average 60 sec lower dispatch
    lower60sec: Option<rust_decimal::Decimal>,
    /// Average60 sec lower dispatch
    lower6sec: Option<rust_decimal::Decimal>,
    /// Average 5 min raise dispatch
    raise5min: Option<rust_decimal::Decimal>,
    /// Average 60 sec raise dispatch
    raise60sec: Option<rust_decimal::Decimal>,
    /// Average 6 sec raise dispatch
    raise6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower Regulation reserve target
    lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    availability: Option<rust_decimal::Decimal>,
    /// Boolean representation flagging if the Target is Capped
    semidispatchcap: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<TradingUnitSolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "TRADING".into(),
                        table_name: "UNIT_SOLUTION".into(),
                        version: 2,
                    }
                    
    }
}
