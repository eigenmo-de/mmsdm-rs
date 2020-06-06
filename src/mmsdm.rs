#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentClusterAvail1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    clusterid: String,
    periodid: rust_decimal::Decimal,
    elements_unavailable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandIntermittentClusterAvail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "INTERMITTENT_CLUSTER_AVAIL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct RooftopForecast1 {
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    powermean: Option<rust_decimal::Decimal>,
    powerpoe50: Option<rust_decimal::Decimal>,
    powerpoelow: Option<rust_decimal::Decimal>,
    powerpoehigh: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<RooftopForecast1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ROOFTOP".into(),
            table_name: "FORECAST".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentGenLimitDay1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    authorisedbyuser: Option<String>,
    authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable<DemandIntermittentGenLimitDay1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "INTERMITTENT_GEN_LIMIT_DAY".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentDsPred1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    origin: String,
    forecast_priority: rust_decimal::Decimal,
    forecast_mean: Option<rust_decimal::Decimal>,
    forecast_poe10: Option<rust_decimal::Decimal>,
    forecast_poe50: Option<rust_decimal::Decimal>,
    forecast_poe90: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandIntermittentDsPred1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "INTERMITTENT_DS_PRED".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandTrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DemandTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentClusterAvailDay1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    clusterid: String,
}
impl crate::GetTable<DemandIntermittentClusterAvailDay1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "INTERMITTENT_CLUSTER_AVAIL_DAY".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct RooftopActual2 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    #[serde(rename = "type")]
    type_: String,
    regionid: String,
    power: Option<rust_decimal::Decimal>,
    qi: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<RooftopActual2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ROOFTOP".into(),
            table_name: "ACTUAL".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForecastIntermittentGenData1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    powermean: Option<rust_decimal::Decimal>,
    powerpoe50: Option<rust_decimal::Decimal>,
    powerpoelow: Option<rust_decimal::Decimal>,
    powerpoehigh: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForecastIntermittentGenData1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORECAST".into(),
            table_name: "INTERMITTENT_GEN_DATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OperationalDemandActual1 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    regionid: String,
    operational_demand: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OperationalDemandActual1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OPERATIONAL_DEMAND".into(),
            table_name: "ACTUAL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandPeriod1 {
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    periodid: rust_decimal::Decimal,
    versionno: rust_decimal::Decimal,
    resdemand: Option<rust_decimal::Decimal>,
    demand90probability: Option<rust_decimal::Decimal>,
    demand10probability: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    mr_schedule: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandPeriod1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "PERIOD".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentDsRun1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    origin: String,
    forecast_priority: rust_decimal::Decimal,
    authorisedby: Option<String>,
    comments: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    model: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    participant_timestamp: Option<chrono::NaiveDateTime>,
    suppressed_aemo: Option<rust_decimal::Decimal>,
    suppressed_participant: Option<rust_decimal::Decimal>,
    transaction_id: Option<String>,
}
impl crate::GetTable<DemandIntermittentDsRun1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "INTERMITTENT_DS_RUN".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OperationalDemandForecast1 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    load_date: Option<chrono::NaiveDateTime>,
    operational_demand_poe10: Option<rust_decimal::Decimal>,
    operational_demand_poe50: Option<rust_decimal::Decimal>,
    operational_demand_poe90: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OperationalDemandForecast1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OPERATIONAL_DEMAND".into(),
            table_name: "FORECAST".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandMtpasaIntermittentAvail1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    clusterid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    elements_unavailable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DemandMtpasaIntermittentAvail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "MTPASA_INTERMITTENT_AVAIL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandMtpasaIntermittentLimit1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    uppermwlimit: Option<i64>,
    authorisedbyuser: Option<String>,
    authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable<DemandMtpasaIntermittentLimit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "MTPASA_INTERMITTENT_LIMIT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentGenLimit1 {
    #[serde(with = "crate::mms_datetime")]
    tradingdate: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    periodid: rust_decimal::Decimal,
    uppermwlimit: Option<i64>,
}
impl crate::GetTable<DemandIntermittentGenLimit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: "INTERMITTENT_GEN_LIMIT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForecastIntermittentGen1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    start_interval_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    end_interval_datetime: chrono::NaiveDateTime,
    versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForecastIntermittentGen1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORECAST".into(),
            table_name: "INTERMITTENT_GEN".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingUnitSolution2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    duid: String,
    tradetype: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    initialmw: Option<rust_decimal::Decimal>,
    totalcleared: Option<rust_decimal::Decimal>,
    rampdownrate: Option<rust_decimal::Decimal>,
    rampuprate: Option<rust_decimal::Decimal>,
    lower5min: Option<rust_decimal::Decimal>,
    lower60sec: Option<rust_decimal::Decimal>,
    lower6sec: Option<rust_decimal::Decimal>,
    raise5min: Option<rust_decimal::Decimal>,
    raise60sec: Option<rust_decimal::Decimal>,
    raise6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    lowerreg: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    availability: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingInterconnectorres2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    interconnectorid: String,
    periodid: rust_decimal::Decimal,
    meteredmwflow: Option<rust_decimal::Decimal>,
    mwflow: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingRegionsum4 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    regionid: String,
    periodid: rust_decimal::Decimal,
    totaldemand: Option<rust_decimal::Decimal>,
    availablegeneration: Option<rust_decimal::Decimal>,
    availableload: Option<rust_decimal::Decimal>,
    demandforecast: Option<rust_decimal::Decimal>,
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    dispatchableload: Option<rust_decimal::Decimal>,
    netinterchange: Option<rust_decimal::Decimal>,
    excessgeneration: Option<rust_decimal::Decimal>,
    lower5mindispatch: Option<rust_decimal::Decimal>,
    lower5minimport: Option<rust_decimal::Decimal>,
    lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    lower5minlocalprice: Option<rust_decimal::Decimal>,
    lower5minlocalreq: Option<rust_decimal::Decimal>,
    lower5minprice: Option<rust_decimal::Decimal>,
    lower5minreq: Option<rust_decimal::Decimal>,
    lower5minsupplyprice: Option<rust_decimal::Decimal>,
    lower60secdispatch: Option<rust_decimal::Decimal>,
    lower60secimport: Option<rust_decimal::Decimal>,
    lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    lower60seclocalprice: Option<rust_decimal::Decimal>,
    lower60seclocalreq: Option<rust_decimal::Decimal>,
    lower60secprice: Option<rust_decimal::Decimal>,
    lower60secreq: Option<rust_decimal::Decimal>,
    lower60secsupplyprice: Option<rust_decimal::Decimal>,
    lower6secdispatch: Option<rust_decimal::Decimal>,
    lower6secimport: Option<rust_decimal::Decimal>,
    lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    lower6seclocalprice: Option<rust_decimal::Decimal>,
    lower6seclocalreq: Option<rust_decimal::Decimal>,
    lower6secprice: Option<rust_decimal::Decimal>,
    lower6secreq: Option<rust_decimal::Decimal>,
    lower6secsupplyprice: Option<rust_decimal::Decimal>,
    raise5mindispatch: Option<rust_decimal::Decimal>,
    raise5minimport: Option<rust_decimal::Decimal>,
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    raise5minlocalprice: Option<rust_decimal::Decimal>,
    raise5minlocalreq: Option<rust_decimal::Decimal>,
    raise5minprice: Option<rust_decimal::Decimal>,
    raise5minreq: Option<rust_decimal::Decimal>,
    raise5minsupplyprice: Option<rust_decimal::Decimal>,
    raise60secdispatch: Option<rust_decimal::Decimal>,
    raise60secimport: Option<rust_decimal::Decimal>,
    raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    raise60seclocalprice: Option<rust_decimal::Decimal>,
    raise60seclocalreq: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingPrice2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    regionid: String,
    periodid: rust_decimal::Decimal,
    rrp: Option<rust_decimal::Decimal>,
    eep: Option<rust_decimal::Decimal>,
    invalidflag: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    rop: Option<rust_decimal::Decimal>,
    raise6secrrp: Option<rust_decimal::Decimal>,
    raise6secrop: Option<rust_decimal::Decimal>,
    raise60secrrp: Option<rust_decimal::Decimal>,
    raise60secrop: Option<rust_decimal::Decimal>,
    raise5minrrp: Option<rust_decimal::Decimal>,
    raise5minrop: Option<rust_decimal::Decimal>,
    raiseregrrp: Option<rust_decimal::Decimal>,
    raiseregrop: Option<rust_decimal::Decimal>,
    lower6secrrp: Option<rust_decimal::Decimal>,
    lower6secrop: Option<rust_decimal::Decimal>,
    lower60secrrp: Option<rust_decimal::Decimal>,
    lower60secrop: Option<rust_decimal::Decimal>,
    lower5minrrp: Option<rust_decimal::Decimal>,
    lower5minrop: Option<rust_decimal::Decimal>,
    lowerregrrp: Option<rust_decimal::Decimal>,
    lowerregrop: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecoveryRbf1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    service: String,
    contractid: String,
    paymenttype: String,
    regionid: String,
    rbf: Option<rust_decimal::Decimal>,
    payment_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicenergyflow5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsReallocations5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    participantid: String,
    reallocationid: String,
    reallocationvalue: Option<rust_decimal::Decimal>,
    energy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcCompensation1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: i64,
    apeventid: i64,
    claimid: i64,
    participantid: String,
    periodid: i64,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecovery2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    participantid: String,
    service: String,
    contractid: String,
    paymenttype: String,
    regionid: String,
    rbf: Option<rust_decimal::Decimal>,
    payment_amount: Option<rust_decimal::Decimal>,
    participant_energy: Option<rust_decimal::Decimal>,
    region_energy: Option<rust_decimal::Decimal>,
    recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    participant_generation: Option<rust_decimal::Decimal>,
    region_generation: Option<rust_decimal::Decimal>,
    recovery_amount_customer: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRpowerrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: Option<String>,
    periodid: rust_decimal::Decimal,
    regionid: String,
    availabilitypayment: Option<rust_decimal::Decimal>,
    enablingpayment: Option<rust_decimal::Decimal>,
    ccpayment: Option<rust_decimal::Decimal>,
    participantdemand: Option<rust_decimal::Decimal>,
    regiondemand: Option<rust_decimal::Decimal>,
    availabilityrecovery: Option<rust_decimal::Decimal>,
    enablingrecovery: Option<rust_decimal::Decimal>,
    ccrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    ccrecovery_gen: Option<rust_decimal::Decimal>,
    participantdemand_gen: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAncillarySummary5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    service: String,
    paymenttype: String,
    regionid: String,
    periodid: rust_decimal::Decimal,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasregionrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    bidtype: String,
    regionid: String,
    periodid: rust_decimal::Decimal,
    generatorregionenergy: Option<rust_decimal::Decimal>,
    customerregionenergy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRunParameter5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: i64,
    parameterid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    settlementrunno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    interconnectorid: String,
    regionid: String,
    mwflow: Option<rust_decimal::Decimal>,
    lossfactor: Option<rust_decimal::Decimal>,
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    csp_derogation_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasRecovery6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: String,
    participantid: String,
    regionid: String,
    periodid: rust_decimal::Decimal,
    lower6sec_recovery: Option<rust_decimal::Decimal>,
    raise6sec_recovery: Option<rust_decimal::Decimal>,
    lower60sec_recovery: Option<rust_decimal::Decimal>,
    raise60sec_recovery: Option<rust_decimal::Decimal>,
    lower5min_recovery: Option<rust_decimal::Decimal>,
    raise5min_recovery: Option<rust_decimal::Decimal>,
    lowerreg_recovery: Option<rust_decimal::Decimal>,
    raisereg_recovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    lower6sec_recovery_gen: Option<rust_decimal::Decimal>,
    raise6sec_recovery_gen: Option<rust_decimal::Decimal>,
    lower60sec_recovery_gen: Option<rust_decimal::Decimal>,
    raise60sec_recovery_gen: Option<rust_decimal::Decimal>,
    lower5min_recovery_gen: Option<rust_decimal::Decimal>,
    raise5min_recovery_gen: Option<rust_decimal::Decimal>,
    lowerreg_recovery_gen: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicenergyfigures5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    totalgenoutput: Option<rust_decimal::Decimal>,
    totalpcsd: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    tlr: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLuloadrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: Option<String>,
    periodid: rust_decimal::Decimal,
    regionid: String,
    enablingpayment: Option<rust_decimal::Decimal>,
    usagepayment: Option<rust_decimal::Decimal>,
    compensationpayment: Option<rust_decimal::Decimal>,
    participantdemand: Option<rust_decimal::Decimal>,
    regiondemand: Option<rust_decimal::Decimal>,
    enablingrecovery: Option<rust_decimal::Decimal>,
    usagerecovery: Option<rust_decimal::Decimal>,
    compensationrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    usagerecovery_gen: Option<rust_decimal::Decimal>,
    compensationrecovery_gen: Option<rust_decimal::Decimal>,
    participantdemand_gen: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsVicboundaryenergy5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    periodid: rust_decimal::Decimal,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrnspsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    settlementrunno: rust_decimal::Decimal,
    contractid: String,
    periodid: rust_decimal::Decimal,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    totalsurplus: Option<rust_decimal::Decimal>,
    contractallocation: Option<rust_decimal::Decimal>,
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    csp_derogation_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdata5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    participantid: String,
    tcpid: String,
    regionid: Option<String>,
    igenergy: Option<rust_decimal::Decimal>,
    xgenergy: Option<rust_decimal::Decimal>,
    inenergy: Option<rust_decimal::Decimal>,
    xnenergy: Option<rust_decimal::Decimal>,
    ipower: Option<rust_decimal::Decimal>,
    xpower: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    eep: Option<rust_decimal::Decimal>,
    tlf: Option<rust_decimal::Decimal>,
    cprrp: Option<rust_decimal::Decimal>,
    cpeep: Option<rust_decimal::Decimal>,
    ta: Option<rust_decimal::Decimal>,
    ep: Option<rust_decimal::Decimal>,
    apc: Option<rust_decimal::Decimal>,
    resc: Option<rust_decimal::Decimal>,
    resp: Option<rust_decimal::Decimal>,
    meterrunno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    hostdistributor: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsDaytrack5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    regionid: Option<String>,
    exanterunstatus: Option<String>,
    exanterunno: Option<rust_decimal::Decimal>,
    expostrunstatus: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSmallgendata1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    connectionpointid: String,
    periodid: rust_decimal::Decimal,
    participantid: String,
    regionid: Option<String>,
    importenergy: Option<rust_decimal::Decimal>,
    exportenergy: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    tlf: Option<rust_decimal::Decimal>,
    impenergycost: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIntervention5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    contractid: Option<String>,
    contractversion: Option<rust_decimal::Decimal>,
    participantid: Option<String>,
    regionid: Option<String>,
    duid: String,
    rcf: Option<char>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLunloadrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: Option<String>,
    periodid: rust_decimal::Decimal,
    regionid: String,
    enablingpayment: Option<rust_decimal::Decimal>,
    usagepayment: Option<rust_decimal::Decimal>,
    compensationpayment: Option<rust_decimal::Decimal>,
    participantdemand: Option<rust_decimal::Decimal>,
    regiondemand: Option<rust_decimal::Decimal>,
    enablingrecovery: Option<rust_decimal::Decimal>,
    usagerecovery: Option<rust_decimal::Decimal>,
    compensationrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    usagerecovery_gen: Option<rust_decimal::Decimal>,
    compensationrecovery_gen: Option<rust_decimal::Decimal>,
    participantdemand_gen: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIraucsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    settlementrunno: rust_decimal::Decimal,
    contractid: String,
    periodid: rust_decimal::Decimal,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    totalsurplus: Option<rust_decimal::Decimal>,
    contractallocation: Option<rust_decimal::Decimal>,
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    csp_derogation_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartpayment6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: String,
    periodid: rust_decimal::Decimal,
    regionid: Option<String>,
    restarttype: Option<rust_decimal::Decimal>,
    avaflag: Option<rust_decimal::Decimal>,
    availabilityprice: Option<rust_decimal::Decimal>,
    tcf: Option<rust_decimal::Decimal>,
    availabilitypayment: Option<rust_decimal::Decimal>,
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSetFcasRegulationTrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    constraintid: String,
    cmpf: Option<rust_decimal::Decimal>,
    crmpf: Option<rust_decimal::Decimal>,
    recovery_factor_cmpf: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLshedpayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: String,
    periodid: rust_decimal::Decimal,
    duid: Option<String>,
    regionid: Option<String>,
    tlf: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    lseprice: Option<rust_decimal::Decimal>,
    mcpprice: Option<rust_decimal::Decimal>,
    lscr: Option<rust_decimal::Decimal>,
    lsepayment: Option<rust_decimal::Decimal>,
    ccpayment: Option<rust_decimal::Decimal>,
    constrainedmw: Option<rust_decimal::Decimal>,
    unconstrainedmw: Option<rust_decimal::Decimal>,
    als: Option<rust_decimal::Decimal>,
    initialdemand: Option<rust_decimal::Decimal>,
    finaldemand: Option<rust_decimal::Decimal>,
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendataregion5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    regionid: String,
    genergy: Option<rust_decimal::Decimal>,
    aenergy: Option<rust_decimal::Decimal>,
    gpower: Option<rust_decimal::Decimal>,
    apower: Option<rust_decimal::Decimal>,
    netenergy: Option<rust_decimal::Decimal>,
    energycost: Option<rust_decimal::Decimal>,
    excessenergycost: Option<rust_decimal::Decimal>,
    expenergy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasPayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: Option<String>,
    duid: String,
    regionid: Option<String>,
    periodid: rust_decimal::Decimal,
    lower6sec_payment: Option<rust_decimal::Decimal>,
    raise6sec_payment: Option<rust_decimal::Decimal>,
    lower60sec_payment: Option<rust_decimal::Decimal>,
    raise60sec_payment: Option<rust_decimal::Decimal>,
    lower5min_payment: Option<rust_decimal::Decimal>,
    raise5min_payment: Option<rust_decimal::Decimal>,
    lowerreg_payment: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrfmrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    irfmid: String,
    irmfversion: Option<rust_decimal::Decimal>,
    participantid: String,
    participantdemand: Option<rust_decimal::Decimal>,
    totaltcd: Option<rust_decimal::Decimal>,
    totaltfd: Option<rust_decimal::Decimal>,
    irfmamount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMarketfees5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    participantid: String,
    periodid: rust_decimal::Decimal,
    marketfeeid: String,
    marketfeevalue: Option<rust_decimal::Decimal>,
    energy: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRpowerpayment6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: String,
    periodid: rust_decimal::Decimal,
    duid: Option<String>,
    regionid: Option<String>,
    tlf: Option<rust_decimal::Decimal>,
    ebp: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    mvaraprice: Option<rust_decimal::Decimal>,
    mvareprice: Option<rust_decimal::Decimal>,
    mvargprice: Option<rust_decimal::Decimal>,
    ccprice: Option<rust_decimal::Decimal>,
    synccompensation: Option<rust_decimal::Decimal>,
    mta: Option<rust_decimal::Decimal>,
    mtg: Option<rust_decimal::Decimal>,
    blocksize: Option<rust_decimal::Decimal>,
    avaflag: Option<rust_decimal::Decimal>,
    clearedmw: Option<rust_decimal::Decimal>,
    unconstrainedmw: Option<rust_decimal::Decimal>,
    availabilitypayment: Option<rust_decimal::Decimal>,
    enablingpayment: Option<rust_decimal::Decimal>,
    ccpayment: Option<rust_decimal::Decimal>,
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrPayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    regionid: String,
    participantid: Option<String>,
    duid: String,
    periodid: rust_decimal::Decimal,
    mr_capacity: Option<rust_decimal::Decimal>,
    uncapped_payment: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrpartsurplus6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    settlementrunno: rust_decimal::Decimal,
    contractid: String,
    periodid: rust_decimal::Decimal,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    totalsurplus: Option<rust_decimal::Decimal>,
    contractallocation: Option<rust_decimal::Decimal>,
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    csp_derogation_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMrRecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    regionid: String,
    participantid: Option<String>,
    duid: String,
    periodid: rust_decimal::Decimal,
    arodef: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIntraregionresidues5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: i64,
    periodid: i64,
    regionid: String,
    ep: Option<rust_decimal::Decimal>,
    ec: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    exp: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcRecovery1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: i64,
    apeventid: i64,
    claimid: i64,
    participantid: String,
    periodid: i64,
    regionid: String,
    recovery_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdataregion5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    regionid: String,
    sumigenergy: Option<rust_decimal::Decimal>,
    sumxgenergy: Option<rust_decimal::Decimal>,
    suminenergy: Option<rust_decimal::Decimal>,
    sumxnenergy: Option<rust_decimal::Decimal>,
    sumipower: Option<rust_decimal::Decimal>,
    sumxpower: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendata6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    participantid: Option<String>,
    stationid: String,
    duid: String,
    gensetid: String,
    regionid: String,
    genergy: Option<rust_decimal::Decimal>,
    aenergy: Option<rust_decimal::Decimal>,
    gpower: Option<rust_decimal::Decimal>,
    apower: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    eep: Option<rust_decimal::Decimal>,
    tlf: Option<rust_decimal::Decimal>,
    cprrp: Option<rust_decimal::Decimal>,
    cpeep: Option<rust_decimal::Decimal>,
    netenergy: Option<rust_decimal::Decimal>,
    energycost: Option<rust_decimal::Decimal>,
    excessenergycost: Option<rust_decimal::Decimal>,
    apc: Option<rust_decimal::Decimal>,
    resc: Option<rust_decimal::Decimal>,
    resp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    expenergy: Option<rust_decimal::Decimal>,
    expenergycost: Option<rust_decimal::Decimal>,
    meterrunno: Option<rust_decimal::Decimal>,
    mda: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsInterventionrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    contractid: String,
    rcf: Option<char>,
    participantid: String,
    participantdemand: Option<rust_decimal::Decimal>,
    totaldemand: Option<rust_decimal::Decimal>,
    interventionpayment: Option<rust_decimal::Decimal>,
    interventionamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLunloadpayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: String,
    periodid: rust_decimal::Decimal,
    duid: Option<String>,
    regionid: Option<String>,
    tlf: Option<rust_decimal::Decimal>,
    ebp: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    enablingprice: Option<rust_decimal::Decimal>,
    usageprice: Option<rust_decimal::Decimal>,
    ccprice: Option<rust_decimal::Decimal>,
    clearedmw: Option<rust_decimal::Decimal>,
    unconstrainedmw: Option<rust_decimal::Decimal>,
    controlrange: Option<rust_decimal::Decimal>,
    enablingpayment: Option<rust_decimal::Decimal>,
    usagepayment: Option<rust_decimal::Decimal>,
    compensationpayment: Option<rust_decimal::Decimal>,
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAgcrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: Option<String>,
    periodid: rust_decimal::Decimal,
    regionid: String,
    enablingpayment: Option<rust_decimal::Decimal>,
    participantdemand: Option<rust_decimal::Decimal>,
    regiondemand: Option<rust_decimal::Decimal>,
    enablingrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    enablingrecovery_gen: Option<rust_decimal::Decimal>,
    participantdemand_gen: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAgcpayment5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: String,
    periodid: rust_decimal::Decimal,
    duid: Option<String>,
    regionid: Option<String>,
    tlf: Option<rust_decimal::Decimal>,
    ebp: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    clearedmw: Option<rust_decimal::Decimal>,
    initialmw: Option<rust_decimal::Decimal>,
    enablingpayment: Option<rust_decimal::Decimal>,
    contractversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLshedrecovery5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: Option<String>,
    periodid: rust_decimal::Decimal,
    regionid: String,
    lsepayment: Option<rust_decimal::Decimal>,
    ccpayment: Option<rust_decimal::Decimal>,
    participantdemand: Option<rust_decimal::Decimal>,
    regiondemand: Option<rust_decimal::Decimal>,
    lserecovery: Option<rust_decimal::Decimal>,
    ccrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    lserecovery_gen: Option<rust_decimal::Decimal>,
    ccrecovery_gen: Option<rust_decimal::Decimal>,
    participantdemand_gen: Option<rust_decimal::Decimal>,
    regiondemand_gen: Option<rust_decimal::Decimal>,
    availabilityrecovery: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcascomp5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    duid: String,
    regionid: Option<String>,
    periodid: rust_decimal::Decimal,
    ccprice: Option<rust_decimal::Decimal>,
    clearedmw: Option<rust_decimal::Decimal>,
    unconstrainedmw: Option<rust_decimal::Decimal>,
    ebp: Option<rust_decimal::Decimal>,
    tlf: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    excessgen: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartrecovery6 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    contractid: Option<String>,
    periodid: rust_decimal::Decimal,
    regionid: String,
    availabilitypayment: Option<rust_decimal::Decimal>,
    participantdemand: Option<rust_decimal::Decimal>,
    regiondemand: Option<rust_decimal::Decimal>,
    availabilityrecovery: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    participantdemand_gen: Option<rust_decimal::Decimal>,
    regiondemand_gen: Option<rust_decimal::Decimal>,
    enablingpayment: Option<rust_decimal::Decimal>,
    enablingrecovery: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccCasesolution1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
}
impl crate::GetTable<MccCasesolution1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MCC".into(),
            table_name: "CASESOLUTION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccConstraintsolution1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    constraintid: String,
    rhs: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<MccConstraintsolution1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MCC".into(),
            table_name: "CONSTRAINTSOLUTION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemandTrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: i64,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchScenarioDemandTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "SCENARIO_DEMAND_TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchCaseSolution1 {
    predispatchseqno: String,
    runno: rust_decimal::Decimal,
    solutionstatus: Option<rust_decimal::Decimal>,
    spdversion: Option<String>,
    nonphysicallosses: Option<rust_decimal::Decimal>,
    totalobjective: Option<rust_decimal::Decimal>,
    totalareagenviolation: Option<rust_decimal::Decimal>,
    totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    totalgenericviolation: Option<rust_decimal::Decimal>,
    totalramprateviolation: Option<rust_decimal::Decimal>,
    totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    total5minviolation: Option<rust_decimal::Decimal>,
    totalregviolation: Option<rust_decimal::Decimal>,
    total6secviolation: Option<rust_decimal::Decimal>,
    total60secviolation: Option<rust_decimal::Decimal>,
    totalasprofileviolation: Option<rust_decimal::Decimal>,
    totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    totalenergyofferviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchCaseSolution1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "CASE_SOLUTION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectrSens1 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    interconnectorid: String,
    periodid: Option<String>,
    intervention: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    intervention_active: Option<rust_decimal::Decimal>,
    mwflow1: Option<rust_decimal::Decimal>,
    mwflow2: Option<rust_decimal::Decimal>,
    mwflow3: Option<rust_decimal::Decimal>,
    mwflow4: Option<rust_decimal::Decimal>,
    mwflow5: Option<rust_decimal::Decimal>,
    mwflow6: Option<rust_decimal::Decimal>,
    mwflow7: Option<rust_decimal::Decimal>,
    mwflow8: Option<rust_decimal::Decimal>,
    mwflow9: Option<rust_decimal::Decimal>,
    mwflow10: Option<rust_decimal::Decimal>,
    mwflow11: Option<rust_decimal::Decimal>,
    mwflow12: Option<rust_decimal::Decimal>,
    mwflow13: Option<rust_decimal::Decimal>,
    mwflow14: Option<rust_decimal::Decimal>,
    mwflow15: Option<rust_decimal::Decimal>,
    mwflow16: Option<rust_decimal::Decimal>,
    mwflow17: Option<rust_decimal::Decimal>,
    mwflow18: Option<rust_decimal::Decimal>,
    mwflow19: Option<rust_decimal::Decimal>,
    mwflow20: Option<rust_decimal::Decimal>,
    mwflow21: Option<rust_decimal::Decimal>,
    mwflow22: Option<rust_decimal::Decimal>,
    mwflow23: Option<rust_decimal::Decimal>,
    mwflow24: Option<rust_decimal::Decimal>,
    mwflow25: Option<rust_decimal::Decimal>,
    mwflow26: Option<rust_decimal::Decimal>,
    mwflow27: Option<rust_decimal::Decimal>,
    mwflow28: Option<rust_decimal::Decimal>,
    mwflow29: Option<rust_decimal::Decimal>,
    mwflow30: Option<rust_decimal::Decimal>,
    mwflow31: Option<rust_decimal::Decimal>,
    mwflow32: Option<rust_decimal::Decimal>,
    mwflow33: Option<rust_decimal::Decimal>,
    mwflow34: Option<rust_decimal::Decimal>,
    mwflow35: Option<rust_decimal::Decimal>,
    mwflow36: Option<rust_decimal::Decimal>,
    mwflow37: Option<rust_decimal::Decimal>,
    mwflow38: Option<rust_decimal::Decimal>,
    mwflow39: Option<rust_decimal::Decimal>,
    mwflow40: Option<rust_decimal::Decimal>,
    mwflow41: Option<rust_decimal::Decimal>,
    mwflow42: Option<rust_decimal::Decimal>,
    mwflow43: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchInterconnectrSens1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "INTERCONNECTR_SENS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchUnitSolution2 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    duid: String,
    tradetype: Option<rust_decimal::Decimal>,
    periodid: Option<String>,
    intervention: Option<rust_decimal::Decimal>,
    connectionpointid: Option<String>,
    agcstatus: Option<rust_decimal::Decimal>,
    dispatchmode: Option<rust_decimal::Decimal>,
    initialmw: Option<rust_decimal::Decimal>,
    totalcleared: Option<rust_decimal::Decimal>,
    lower5min: Option<rust_decimal::Decimal>,
    lower60sec: Option<rust_decimal::Decimal>,
    lower6sec: Option<rust_decimal::Decimal>,
    raise5min: Option<rust_decimal::Decimal>,
    raise60sec: Option<rust_decimal::Decimal>,
    raise6sec: Option<rust_decimal::Decimal>,
    rampdownrate: Option<rust_decimal::Decimal>,
    rampuprate: Option<rust_decimal::Decimal>,
    downepf: Option<rust_decimal::Decimal>,
    upepf: Option<rust_decimal::Decimal>,
    marginal5minvalue: Option<rust_decimal::Decimal>,
    marginal60secvalue: Option<rust_decimal::Decimal>,
    marginal6secvalue: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violation5mindegree: Option<rust_decimal::Decimal>,
    violation60secdegree: Option<rust_decimal::Decimal>,
    violation6secdegree: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    lowerreg: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    availability: Option<rust_decimal::Decimal>,
    raise6secflags: Option<rust_decimal::Decimal>,
    raise60secflags: Option<rust_decimal::Decimal>,
    raise5minflags: Option<rust_decimal::Decimal>,
    raiseregflags: Option<rust_decimal::Decimal>,
    lower6secflags: Option<rust_decimal::Decimal>,
    lower60secflags: Option<rust_decimal::Decimal>,
    lower5minflags: Option<rust_decimal::Decimal>,
    lowerregflags: Option<rust_decimal::Decimal>,
    raise6secactualavailability: Option<rust_decimal::Decimal>,
    raise60secactualavailability: Option<rust_decimal::Decimal>,
    raise5minactualavailability: Option<rust_decimal::Decimal>,
    raiseregactualavailability: Option<rust_decimal::Decimal>,
    lower6secactualavailability: Option<rust_decimal::Decimal>,
    lower60secactualavailability: Option<rust_decimal::Decimal>,
    lower5minactualavailability: Option<rust_decimal::Decimal>,
    lowerregactualavailability: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchUnitSolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "UNIT_SOLUTION".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchPricesensitivities1 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    regionid: String,
    periodid: Option<String>,
    intervention: Option<rust_decimal::Decimal>,
    rrpeep1: Option<rust_decimal::Decimal>,
    rrpeep2: Option<rust_decimal::Decimal>,
    rrpeep3: Option<rust_decimal::Decimal>,
    rrpeep4: Option<rust_decimal::Decimal>,
    rrpeep5: Option<rust_decimal::Decimal>,
    rrpeep6: Option<rust_decimal::Decimal>,
    rrpeep7: Option<rust_decimal::Decimal>,
    rrpeep8: Option<rust_decimal::Decimal>,
    rrpeep9: Option<rust_decimal::Decimal>,
    rrpeep10: Option<rust_decimal::Decimal>,
    rrpeep11: Option<rust_decimal::Decimal>,
    rrpeep12: Option<rust_decimal::Decimal>,
    rrpeep13: Option<rust_decimal::Decimal>,
    rrpeep14: Option<rust_decimal::Decimal>,
    rrpeep15: Option<rust_decimal::Decimal>,
    rrpeep16: Option<rust_decimal::Decimal>,
    rrpeep17: Option<rust_decimal::Decimal>,
    rrpeep18: Option<rust_decimal::Decimal>,
    rrpeep19: Option<rust_decimal::Decimal>,
    rrpeep20: Option<rust_decimal::Decimal>,
    rrpeep21: Option<rust_decimal::Decimal>,
    rrpeep22: Option<rust_decimal::Decimal>,
    rrpeep23: Option<rust_decimal::Decimal>,
    rrpeep24: Option<rust_decimal::Decimal>,
    rrpeep25: Option<rust_decimal::Decimal>,
    rrpeep26: Option<rust_decimal::Decimal>,
    rrpeep27: Option<rust_decimal::Decimal>,
    rrpeep28: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    rrpeep29: Option<rust_decimal::Decimal>,
    rrpeep30: Option<rust_decimal::Decimal>,
    rrpeep31: Option<rust_decimal::Decimal>,
    rrpeep32: Option<rust_decimal::Decimal>,
    rrpeep33: Option<rust_decimal::Decimal>,
    rrpeep34: Option<rust_decimal::Decimal>,
    rrpeep35: Option<rust_decimal::Decimal>,
    intervention_active: Option<rust_decimal::Decimal>,
    rrpeep36: Option<rust_decimal::Decimal>,
    rrpeep37: Option<rust_decimal::Decimal>,
    rrpeep38: Option<rust_decimal::Decimal>,
    rrpeep39: Option<rust_decimal::Decimal>,
    rrpeep40: Option<rust_decimal::Decimal>,
    rrpeep41: Option<rust_decimal::Decimal>,
    rrpeep42: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchPricesensitivities1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "PRICESENSITIVITIES".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionfcasrequirement2 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    intervention: Option<rust_decimal::Decimal>,
    periodid: Option<String>,
    genconid: String,
    regionid: String,
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    genconeffectivedate: Option<chrono::NaiveDateTime>,
    genconversionno: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    base_cost: Option<rust_decimal::Decimal>,
    adjusted_cost: Option<rust_decimal::Decimal>,
    estimated_cmpf: Option<rust_decimal::Decimal>,
    estimated_crmpf: Option<rust_decimal::Decimal>,
    recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    recovery_factor_crmpf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchRegionfcasrequirement2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "REGIONFCASREQUIREMENT".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchConstraintSolution5 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    constraintid: String,
    periodid: Option<String>,
    intervention: Option<rust_decimal::Decimal>,
    rhs: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    genconid_effectivedate: Option<chrono::NaiveDateTime>,
    genconid_versionno: Option<rust_decimal::Decimal>,
    lhs: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchConstraintSolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "CONSTRAINT_SOLUTION".into(),
            version: 5,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionPrices1 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    regionid: String,
    periodid: Option<String>,
    intervention: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    eep: Option<rust_decimal::Decimal>,
    rrp1: Option<rust_decimal::Decimal>,
    eep1: Option<rust_decimal::Decimal>,
    rrp2: Option<rust_decimal::Decimal>,
    eep2: Option<rust_decimal::Decimal>,
    rrp3: Option<rust_decimal::Decimal>,
    eep3: Option<rust_decimal::Decimal>,
    rrp4: Option<rust_decimal::Decimal>,
    eep4: Option<rust_decimal::Decimal>,
    rrp5: Option<rust_decimal::Decimal>,
    eep5: Option<rust_decimal::Decimal>,
    rrp6: Option<rust_decimal::Decimal>,
    eep6: Option<rust_decimal::Decimal>,
    rrp7: Option<rust_decimal::Decimal>,
    eep7: Option<rust_decimal::Decimal>,
    rrp8: Option<rust_decimal::Decimal>,
    eep8: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    raise6secrrp: Option<rust_decimal::Decimal>,
    raise60secrrp: Option<rust_decimal::Decimal>,
    raise5minrrp: Option<rust_decimal::Decimal>,
    raiseregrrp: Option<rust_decimal::Decimal>,
    lower6secrrp: Option<rust_decimal::Decimal>,
    lower60secrrp: Option<rust_decimal::Decimal>,
    lower5minrrp: Option<rust_decimal::Decimal>,
    lowerregrrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchRegionPrices1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "REGION_PRICES".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchBlockedConstraints1 {
    predispatchseqno: String,
    constraintid: String,
}
impl crate::GetTable<PredispatchBlockedConstraints1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "BLOCKED_CONSTRAINTS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchMnspbidtrk1 {
    predispatchseqno: String,
    linkid: String,
    periodid: String,
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    settlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    versionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchMnspbidtrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "MNSPBIDTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchOffertrk1 {
    predispatchseqno: String,
    duid: String,
    bidtype: String,
    periodid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    bidofferdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchOffertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "OFFERTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectorSoln3 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    interconnectorid: String,
    periodid: Option<String>,
    intervention: Option<rust_decimal::Decimal>,
    meteredmwflow: Option<rust_decimal::Decimal>,
    mwflow: Option<rust_decimal::Decimal>,
    mwlosses: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    exportlimit: Option<rust_decimal::Decimal>,
    importlimit: Option<rust_decimal::Decimal>,
    marginalloss: Option<rust_decimal::Decimal>,
    exportgenconid: Option<String>,
    importgenconid: Option<String>,
    fcasexportlimit: Option<rust_decimal::Decimal>,
    fcasimportlimit: Option<rust_decimal::Decimal>,
    local_price_adjustment_export: Option<rust_decimal::Decimal>,
    locally_constrained_export: Option<rust_decimal::Decimal>,
    local_price_adjustment_import: Option<rust_decimal::Decimal>,
    locally_constrained_import: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchInterconnectorSoln3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "INTERCONNECTOR_SOLN".into(),
            version: 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionSolution4 {
    predispatchseqno: Option<String>,
    runno: Option<rust_decimal::Decimal>,
    regionid: String,
    periodid: Option<String>,
    intervention: Option<rust_decimal::Decimal>,
    totaldemand: Option<rust_decimal::Decimal>,
    availablegeneration: Option<rust_decimal::Decimal>,
    availableload: Option<rust_decimal::Decimal>,
    demandforecast: Option<rust_decimal::Decimal>,
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    dispatchableload: Option<rust_decimal::Decimal>,
    netinterchange: Option<rust_decimal::Decimal>,
    excessgeneration: Option<rust_decimal::Decimal>,
    lower5mindispatch: Option<rust_decimal::Decimal>,
    lower5minimport: Option<rust_decimal::Decimal>,
    lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    lower5minlocalprice: Option<rust_decimal::Decimal>,
    lower5minlocalreq: Option<rust_decimal::Decimal>,
    lower5minprice: Option<rust_decimal::Decimal>,
    lower5minreq: Option<rust_decimal::Decimal>,
    lower5minsupplyprice: Option<rust_decimal::Decimal>,
    lower60secdispatch: Option<rust_decimal::Decimal>,
    lower60secimport: Option<rust_decimal::Decimal>,
    lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    lower60seclocalprice: Option<rust_decimal::Decimal>,
    lower60seclocalreq: Option<rust_decimal::Decimal>,
    lower60secprice: Option<rust_decimal::Decimal>,
    lower60secreq: Option<rust_decimal::Decimal>,
    lower60secsupplyprice: Option<rust_decimal::Decimal>,
    lower6secdispatch: Option<rust_decimal::Decimal>,
    lower6secimport: Option<rust_decimal::Decimal>,
    lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    lower6seclocalprice: Option<rust_decimal::Decimal>,
    lower6seclocalreq: Option<rust_decimal::Decimal>,
    lower6secprice: Option<rust_decimal::Decimal>,
    lower6secreq: Option<rust_decimal::Decimal>,
    lower6secsupplyprice: Option<rust_decimal::Decimal>,
    raise5mindispatch: Option<rust_decimal::Decimal>,
    raise5minimport: Option<rust_decimal::Decimal>,
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    raise5minlocalprice: Option<rust_decimal::Decimal>,
    raise5minlocalreq: Option<rust_decimal::Decimal>,
    raise5minprice: Option<rust_decimal::Decimal>,
    raise5minreq: Option<rust_decimal::Decimal>,
    raise5minsupplyprice: Option<rust_decimal::Decimal>,
    raise60secdispatch: Option<rust_decimal::Decimal>,
    raise60secimport: Option<rust_decimal::Decimal>,
    raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    raise60seclocalprice: Option<rust_decimal::Decimal>,
    raise60seclocalreq: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PredispatchRegionSolution4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "REGION_SOLUTION".into(),
            version: 4,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemand1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: i64,
    scenario: i64,
    regionid: String,
    deltamw: Option<i64>,
}
impl crate::GetTable<PredispatchScenarioDemand1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "SCENARIO_DEMAND".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchLocalPrice1 {
    predispatchseqno: String,
    #[serde(with = "crate::mms_datetime")]
    datetime: chrono::NaiveDateTime,
    duid: String,
    periodid: Option<String>,
    local_price_adjustment: Option<rust_decimal::Decimal>,
    locally_constrained: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PredispatchLocalPrice1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: "LOCAL_PRICE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaCasesolution3 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    pasaversion: Option<String>,
    reservecondition: Option<rust_decimal::Decimal>,
    lorcondition: Option<rust_decimal::Decimal>,
    capacityobjfunction: Option<rust_decimal::Decimal>,
    capacityoption: Option<rust_decimal::Decimal>,
    maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    maxsparecapacityoption: Option<rust_decimal::Decimal>,
    interconnectorflowpenalty: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    reliabilitylrcdemandoption: Option<rust_decimal::Decimal>,
    outagelrcdemandoption: Option<rust_decimal::Decimal>,
    lordemandoption: Option<rust_decimal::Decimal>,
    reliabilitylrccapacityoption: Option<String>,
    outagelrccapacityoption: Option<String>,
    lorcapacityoption: Option<String>,
    loruigf_option: Option<rust_decimal::Decimal>,
    reliability_lrcuigf_option: Option<rust_decimal::Decimal>,
    outage_lrcuigf_option: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<StpasaCasesolution3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: "CASESOLUTION".into(),
            version: 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaConstraintsolution2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    constraintid: String,
    capacityrhs: Option<rust_decimal::Decimal>,
    capacitymarginalvalue: Option<rust_decimal::Decimal>,
    capacityviolationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    runtype: String,
}
impl crate::GetTable<StpasaConstraintsolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: "CONSTRAINTSOLUTION".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaInterconnectorsoln2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    interconnectorid: String,
    capacitymwflow: Option<rust_decimal::Decimal>,
    capacitymarginalvalue: Option<rust_decimal::Decimal>,
    capacityviolationdegree: Option<rust_decimal::Decimal>,
    calculatedexportlimit: Option<rust_decimal::Decimal>,
    calculatedimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    runtype: String,
    exportlimitconstraintid: Option<String>,
    importlimitconstraintid: Option<String>,
}
impl crate::GetTable<StpasaInterconnectorsoln2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: "INTERCONNECTORSOLN".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StpasaRegionsolution5 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    regionid: String,
    demand10: Option<rust_decimal::Decimal>,
    demand50: Option<rust_decimal::Decimal>,
    demand90: Option<rust_decimal::Decimal>,
    reservereq: Option<rust_decimal::Decimal>,
    capacityreq: Option<rust_decimal::Decimal>,
    energyreqdemand50: Option<rust_decimal::Decimal>,
    unconstrainedcapacity: Option<rust_decimal::Decimal>,
    constrainedcapacity: Option<rust_decimal::Decimal>,
    netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    surpluscapacity: Option<rust_decimal::Decimal>,
    surplusreserve: Option<rust_decimal::Decimal>,
    reservecondition: Option<rust_decimal::Decimal>,
    maxsurplusreserve: Option<rust_decimal::Decimal>,
    maxsparecapacity: Option<rust_decimal::Decimal>,
    lorcondition: Option<rust_decimal::Decimal>,
    aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    aggregatescheduledload: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    aggregatepasaavailability: Option<rust_decimal::Decimal>,
    runtype: String,
    energyreqdemand10: Option<rust_decimal::Decimal>,
    calculatedlor1level: Option<rust_decimal::Decimal>,
    calculatedlor2level: Option<rust_decimal::Decimal>,
    msrnetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    lornetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    totalintermittentgeneration: Option<rust_decimal::Decimal>,
    demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    uigf: Option<rust_decimal::Decimal>,
    semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    lor_semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    lcr: Option<rust_decimal::Decimal>,
    lcr2: Option<rust_decimal::Decimal>,
    fum: Option<rust_decimal::Decimal>,
    ss_solar_uigf: Option<rust_decimal::Decimal>,
    ss_wind_uigf: Option<rust_decimal::Decimal>,
    ss_solar_capacity: Option<rust_decimal::Decimal>,
    ss_wind_capacity: Option<rust_decimal::Decimal>,
    ss_solar_cleared: Option<rust_decimal::Decimal>,
    ss_wind_cleared: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<StpasaRegionsolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "STPASA".into(),
            table_name: "REGIONSOLUTION".into(),
            version: 5,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEvent1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_cut_off_time: Option<chrono::NaiveDateTime>,
    settlement_complete: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrEvent1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: "EVENT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrPerofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    stack_position: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    duid: Option<String>,
    accepted_capacity: Option<rust_decimal::Decimal>,
    deducted_capacity: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrPerofferStack1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: "PEROFFER_STACK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrDayofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    stack_position: rust_decimal::Decimal,
    duid: Option<String>,
    authorised: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_settlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_offerdate: Option<chrono::NaiveDateTime>,
    offer_versionno: Option<rust_decimal::Decimal>,
    offer_type: Option<String>,
    laof: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrDayofferStack1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: "DAYOFFER_STACK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEventSchedule1 {
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    demand_effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    demand_offerdate: Option<chrono::NaiveDateTime>,
    demand_versionno: Option<rust_decimal::Decimal>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MrEventSchedule1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: "EVENT_SCHEDULE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ReserveDataReserve1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    regionid: String,
    periodid: rust_decimal::Decimal,
    lower5min: Option<rust_decimal::Decimal>,
    lower60sec: Option<rust_decimal::Decimal>,
    lower6sec: Option<rust_decimal::Decimal>,
    raise5min: Option<rust_decimal::Decimal>,
    raise60sec: Option<rust_decimal::Decimal>,
    raise6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    pasareserve: Option<rust_decimal::Decimal>,
    loadrejectionreservereq: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    lowerreg: Option<rust_decimal::Decimal>,
    lor1level: Option<rust_decimal::Decimal>,
    lor2level: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ReserveDataReserve1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "RESERVE_DATA".into(),
            table_name: "RESERVE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimitSet1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    reservelimit_set_id: Option<String>,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MtpasaReservelimitSet1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: "RESERVELIMIT_SET".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimitRegion1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    reservelimitid: String,
    regionid: String,
    coef: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MtpasaReservelimitRegion1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: "RESERVELIMIT_REGION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimit1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    reservelimitid: String,
    description: Option<String>,
    rhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MtpasaReservelimit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: "RESERVELIMIT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigSecdepositInterestRate1 {
    interest_acct_id: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    interest_rate: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BillingConfigSecdepositInterestRate1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: "SECDEPOSIT_INTEREST_RATE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstBasClass1 {
    bas_class: String,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstBasClass1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: "GST_BAS_CLASS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigBillingcalendar2 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    preliminarystatementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    finalstatementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    paymentdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    revision1_statementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    revision2_statementdate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigBillingcalendar2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: "BILLINGCALENDAR".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstTransactionClass1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    transaction_type: String,
    bas_class: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstTransactionClass1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: "GST_TRANSACTION_CLASS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstRate1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    bas_class: String,
    gst_rate: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstRate1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: "GST_RATE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigSecdepositProvision1 {
    security_deposit_id: String,
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    transaction_date: Option<chrono::NaiveDateTime>,
    maturity_contractyear: Option<rust_decimal::Decimal>,
    maturity_weekno: Option<rust_decimal::Decimal>,
    amount: Option<rust_decimal::Decimal>,
    interest_rate: Option<rust_decimal::Decimal>,
    interest_calc_type: Option<String>,
    interest_acct_id: Option<String>,
}
impl crate::GetTable<BillingConfigSecdepositProvision1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: "SECDEPOSIT_PROVISION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingConfigGstTransactionType1 {
    transaction_type: String,
    description: Option<String>,
    gl_financialcode: Option<String>,
    gl_tcode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BillingConfigGstTransactionType1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BILLING_CONFIG".into(),
            table_name: "GST_TRANSACTION_TYPE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFees5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    marketfeeid: String,
    rate: Option<rust_decimal::Decimal>,
    energy: Option<rust_decimal::Decimal>,
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublication1 {
    contractyear: i64,
    weekno: i64,
    billrunno: i64,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    regionid: String,
    sentoutenergy: Option<rust_decimal::Decimal>,
    generatoremissions: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecovery1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    service: String,
    contractid: String,
    regionid: String,
    rbf: Option<rust_decimal::Decimal>,
    test_payment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    recovery_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    recovery_end_date: Option<chrono::NaiveDateTime>,
    participant_energy: Option<rust_decimal::Decimal>,
    region_energy: Option<rust_decimal::Decimal>,
    nem_energy: Option<rust_decimal::Decimal>,
    customer_proportion: Option<rust_decimal::Decimal>,
    generator_proportion: Option<rust_decimal::Decimal>,
    participant_generation: Option<rust_decimal::Decimal>,
    nem_generation: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrSummary5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    total_payments: Option<rust_decimal::Decimal>,
    total_recovery: Option<rust_decimal::Decimal>,
    total_rsa: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplus5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    residueyear: Option<rust_decimal::Decimal>,
    quarter: Option<rust_decimal::Decimal>,
    billrunno: rust_decimal::Decimal,
    contractid: String,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    totalresidues: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestRate1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    interest_acct_id: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingDirectionReconOther1 {
    contractyear: i64,
    weekno: i64,
    billrunno: i64,
    direction_id: String,
    regionid: String,
    direction_desc: Option<String>,
    direction_type_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_end_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_start_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_end_interval: Option<chrono::NaiveDateTime>,
    compensation_amount: Option<rust_decimal::Decimal>,
    interest_amount: Option<rust_decimal::Decimal>,
    independent_expert_fee: Option<rust_decimal::Decimal>,
    cra: Option<rust_decimal::Decimal>,
    regional_customer_energy: Option<rust_decimal::Decimal>,
    regional_generator_energy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingCpdata5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    connectionpointid: String,
    aggregateenergy: Option<rust_decimal::Decimal>,
    purchases: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstPayments1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    service: String,
    contractid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepositApplication1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstDetail5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    bas_class: String,
    transaction_type: String,
    gst_exclusive_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDaytrk5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplussum6 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    residueyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
    participantid: String,
    totalsurplus: Option<rust_decimal::Decimal>,
    auctionfees: Option<rust_decimal::Decimal>,
    auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    csp_derogation_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrfm5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublicationTrk1 {
    contractyear: i64,
    weekno: i64,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryTrk1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    recovery_contractyear: rust_decimal::Decimal,
    recovery_weekno: rust_decimal::Decimal,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingReallocDetail5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    counterparty: String,
    reallocationid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrRecovery5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    participantid: Option<String>,
    duid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplussum7 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    residueyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
    participantid: String,
    totalsurplus: Option<rust_decimal::Decimal>,
    auctionfees: Option<rust_decimal::Decimal>,
    actualpayment: Option<rust_decimal::Decimal>,
    auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    unadjusted_irsr: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingInterresidues5 {
    allocation: Option<rust_decimal::Decimal>,
    totalsurplus: Option<rust_decimal::Decimal>,
    interconnectorid: String,
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirectionReconciliatn1 {
    contractyear: i64,
    weekno: i64,
    billrunno: i64,
    direction_id: String,
    direction_desc: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    direction_end_date: Option<chrono::NaiveDateTime>,
    compensation_amount: Option<rust_decimal::Decimal>,
    independent_expert_fee: Option<rust_decimal::Decimal>,
    interest_amount: Option<rust_decimal::Decimal>,
    cra: Option<rust_decimal::Decimal>,
    nem_fee_id: Option<String>,
    nem_fixed_fee_amount: Option<rust_decimal::Decimal>,
    mkt_customer_perc: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryRbf1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    service: String,
    contractid: String,
    regionid: String,
    rbf: Option<rust_decimal::Decimal>,
    payment_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallDetail1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    transaction_type: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcCompensation2 {
    contractyear: i64,
    weekno: i64,
    billrunno: i64,
    apeventid: i64,
    claimid: i64,
    participantid: Option<String>,
    compensation_amount: Option<rust_decimal::Decimal>,
    event_type: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingPrioradjustments5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    adjcontractyear: rust_decimal::Decimal,
    adjweekno: rust_decimal::Decimal,
    adjbillrunno: rust_decimal::Decimal,
    participantid: String,
    prevamount: Option<rust_decimal::Decimal>,
    adjamount: Option<rust_decimal::Decimal>,
    irn: Option<rust_decimal::Decimal>,
    irp: Option<rust_decimal::Decimal>,
    interestamount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    irsr_prevamount: Option<rust_decimal::Decimal>,
    irsr_adjamount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionexports5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    regionid: String,
    exportto: String,
    energy: Option<rust_decimal::Decimal>,
    value: Option<rust_decimal::Decimal>,
    surplusenergy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionfigures5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    regionid: String,
    energyout: Option<rust_decimal::Decimal>,
    valueout: Option<rust_decimal::Decimal>,
    energypurchased: Option<rust_decimal::Decimal>,
    valuepurchased: Option<rust_decimal::Decimal>,
    excessgen: Option<rust_decimal::Decimal>,
    reservetrading: Option<rust_decimal::Decimal>,
    intcompo: Option<rust_decimal::Decimal>,
    adminpricecompo: Option<rust_decimal::Decimal>,
    settsurplus: Option<rust_decimal::Decimal>,
    aspayment: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingResTraderPayment1 {
    contractyear: i64,
    weekno: i64,
    billrunno: i64,
    contractid: String,
    payment_type: String,
    participantid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGendata5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    connectionpointid: String,
    stationid: Option<String>,
    duid: Option<String>,
    aggregateenergy: Option<rust_decimal::Decimal>,
    sales: Option<rust_decimal::Decimal>,
    purchases: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    purchasedenergy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcRecovery2 {
    contractyear: i64,
    weekno: i64,
    billrunno: i64,
    apeventid: i64,
    claimid: i64,
    participantid: String,
    regionid: String,
    recovery_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    eligibility_start_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    eligibility_end_interval: Option<chrono::NaiveDateTime>,
    participant_demand: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstSummary5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    bas_class: String,
    gst_exclusive_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingWhitehole5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    nl: Option<rust_decimal::Decimal>,
    participantdemand: Option<rust_decimal::Decimal>,
    regiondemand: Option<rust_decimal::Decimal>,
    whiteholepayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplussum7 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    residueyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
    participantid: String,
    totalsurplus: Option<rust_decimal::Decimal>,
    auctionfees: Option<rust_decimal::Decimal>,
    actualpayment: Option<rust_decimal::Decimal>,
    auctionfees_gst: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    csp_derogation_amount: Option<rust_decimal::Decimal>,
    unadjusted_irsr: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestPay1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    security_deposit_id: String,
    participantid: String,
    interest_amount: Option<rust_decimal::Decimal>,
    interest_calc_type: Option<String>,
    interest_acct_id: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSmelterreduction5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    rate1: Option<rust_decimal::Decimal>,
    ra1: Option<rust_decimal::Decimal>,
    rate2: Option<rust_decimal::Decimal>,
    ra2: Option<rust_decimal::Decimal>,
    te: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingResTraderRecovery1 {
    contractyear: i64,
    weekno: i64,
    billrunno: i64,
    regionid: String,
    participantid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrPayment5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    participantid: Option<String>,
    duid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDailyEnergySummary1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    participantid: String,
    regionid: String,
    customer_energy_purchased: Option<rust_decimal::Decimal>,
    generator_energy_sold: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingMrShortfall5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    mr_date: chrono::NaiveDateTime,
    regionid: String,
    participantid: String,
    age: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFinancialadjustments5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    participanttype: Option<String>,
    adjustmentitem: String,
    amount: Option<rust_decimal::Decimal>,
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    financialcode: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRealloc5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    counterparty: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplus5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    residueyear: Option<rust_decimal::Decimal>,
    quarter: Option<rust_decimal::Decimal>,
    billrunno: rust_decimal::Decimal,
    contractid: String,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    totalresidues: Option<rust_decimal::Decimal>,
    adjustment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionimports5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    regionid: String,
    importfrom: String,
    energy: Option<rust_decimal::Decimal>,
    value: Option<rust_decimal::Decimal>,
    surplusenergy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplus5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    residueyear: Option<rust_decimal::Decimal>,
    quarter: Option<rust_decimal::Decimal>,
    billrunno: rust_decimal::Decimal,
    contractid: String,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    totalresidues: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAsrecovery7 {
    regionid: String,
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    raise6sec: Option<rust_decimal::Decimal>,
    lower6sec: Option<rust_decimal::Decimal>,
    raise60sec: Option<rust_decimal::Decimal>,
    lower60sec: Option<rust_decimal::Decimal>,
    agc: Option<rust_decimal::Decimal>,
    fcascomp: Option<rust_decimal::Decimal>,
    loadshed: Option<rust_decimal::Decimal>,
    rgul: Option<rust_decimal::Decimal>,
    rguu: Option<rust_decimal::Decimal>,
    reactivepower: Option<rust_decimal::Decimal>,
    systemrestart: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    raise6sec_gen: Option<rust_decimal::Decimal>,
    lower6sec_gen: Option<rust_decimal::Decimal>,
    raise60sec_gen: Option<rust_decimal::Decimal>,
    lower60sec_gen: Option<rust_decimal::Decimal>,
    agc_gen: Option<rust_decimal::Decimal>,
    fcascomp_gen: Option<rust_decimal::Decimal>,
    loadshed_gen: Option<rust_decimal::Decimal>,
    rgul_gen: Option<rust_decimal::Decimal>,
    rguu_gen: Option<rust_decimal::Decimal>,
    reactivepower_gen: Option<rust_decimal::Decimal>,
    systemrestart_gen: Option<rust_decimal::Decimal>,
    lower5min: Option<rust_decimal::Decimal>,
    raise5min: Option<rust_decimal::Decimal>,
    lowerreg: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    lower5min_gen: Option<rust_decimal::Decimal>,
    raise5min_gen: Option<rust_decimal::Decimal>,
    lowerreg_gen: Option<rust_decimal::Decimal>,
    raisereg_gen: Option<rust_decimal::Decimal>,
    availability_reactive: Option<rust_decimal::Decimal>,
    availability_reactive_rbt: Option<rust_decimal::Decimal>,
    availability_reactive_gen: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRuntrk5 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    status: Option<String>,
    adj_cleared: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    postdate: Option<chrono::NaiveDateTime>,
    postby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    receiptpostdate: Option<chrono::NaiveDateTime>,
    receiptpostby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    paymentpostdate: Option<chrono::NaiveDateTime>,
    paymentpostby: Option<String>,
    shortfall: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIntraresidues5 {
    allocation: Option<rust_decimal::Decimal>,
    totalsurplus: Option<rust_decimal::Decimal>,
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    surplusvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAspayments6 {
    regionid: Option<String>,
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    connectionpointid: String,
    raise6sec: Option<rust_decimal::Decimal>,
    lower6sec: Option<rust_decimal::Decimal>,
    raise60sec: Option<rust_decimal::Decimal>,
    lower60sec: Option<rust_decimal::Decimal>,
    agc: Option<rust_decimal::Decimal>,
    fcascomp: Option<rust_decimal::Decimal>,
    loadshed: Option<rust_decimal::Decimal>,
    rgul: Option<rust_decimal::Decimal>,
    rguu: Option<rust_decimal::Decimal>,
    reactivepower: Option<rust_decimal::Decimal>,
    systemrestart: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    lower5min: Option<rust_decimal::Decimal>,
    raise5min: Option<rust_decimal::Decimal>,
    lowerreg: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    availability_reactive: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallAmount1 {
    contractyear: rust_decimal::Decimal,
    weekno: rust_decimal::Decimal,
    billrunno: rust_decimal::Decimal,
    participantid: String,
    shortfall_amount: Option<rust_decimal::Decimal>,
    shortfall: Option<rust_decimal::Decimal>,
    shortfall_company_id: Option<String>,
    company_shortfall_amount: Option<rust_decimal::Decimal>,
    participant_net_energy: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialCompanyPosition1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    runno: i64,
    company_id: String,
    mcl: Option<rust_decimal::Decimal>,
    credit_support: Option<rust_decimal::Decimal>,
    trading_limit: Option<rust_decimal::Decimal>,
    current_amount_balance: Option<rust_decimal::Decimal>,
    security_deposit_provision: Option<rust_decimal::Decimal>,
    security_deposit_offset: Option<rust_decimal::Decimal>,
    security_deposit_balance: Option<rust_decimal::Decimal>,
    expost_realloc_balance: Option<rust_decimal::Decimal>,
    default_balance: Option<rust_decimal::Decimal>,
    outstandings: Option<rust_decimal::Decimal>,
    trading_margin: Option<rust_decimal::Decimal>,
    typical_accrual: Option<rust_decimal::Decimal>,
    prudential_margin: Option<rust_decimal::Decimal>,
    early_payment_amount: Option<rust_decimal::Decimal>,
    percentage_outstandings: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PrudentialCompanyPosition1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRUDENTIAL".into(),
            table_name: "COMPANY_POSITION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialRuntrk1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    runno: i64,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PrudentialRuntrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRUDENTIAL".into(),
            table_name: "RUNTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrestartdata1 {
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    availability: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    periodid: rust_decimal::Decimal,
}
impl crate::GetTable<AsofferOfferrestartdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: "OFFERRESTARTDATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferastrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<AsofferOfferastrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: "OFFERASTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferagcdata1 {
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    availability: Option<rust_decimal::Decimal>,
    upperlimit: Option<rust_decimal::Decimal>,
    lowerlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    periodid: rust_decimal::Decimal,
    agcup: Option<rust_decimal::Decimal>,
    agcdown: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<AsofferOfferagcdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: "OFFERAGCDATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferlsheddata1 {
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    availableload: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    periodid: rust_decimal::Decimal,
}
impl crate::GetTable<AsofferOfferlsheddata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: "OFFERLSHEDDATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrpowerdata1 {
    contractid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    availability: Option<rust_decimal::Decimal>,
    mta: Option<rust_decimal::Decimal>,
    mtg: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    filename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<AsofferOfferrpowerdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: "OFFERRPOWERDATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionTranche1 {
    contractyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    versionno: rust_decimal::Decimal,
    tranche: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    auctiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifydate: Option<chrono::NaiveDateTime>,
    unitallocation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    changedate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProfile1 {
    auctionid: String,
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    filename: Option<String>,
    ackfilename: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraCashSecurity1 {
    cash_security_id: String,
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    provision_date: Option<chrono::NaiveDateTime>,
    cash_amount: Option<rust_decimal::Decimal>,
    interest_acct_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    finalreturndate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionValuationid1 {
    valuationid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucReceipts1 {
    sra_year: i64,
    sra_quarter: i64,
    sra_runno: i64,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    contractid: String,
    units_purchased: Option<rust_decimal::Decimal>,
    clearing_price: Option<rust_decimal::Decimal>,
    receipt_amount: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    proceeds_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueContracts1 {
    contractyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    tranche: rust_decimal::Decimal,
    contractid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    auctiondate: Option<chrono::NaiveDateTime>,
    calcmethod: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifypostdate: Option<chrono::NaiveDateTime>,
    notifyby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    postdate: Option<chrono::NaiveDateTime>,
    postedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    description: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigResiduecontractpayments1 {
    contractid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConFunds1 {
    contractid: String,
    interconnectorid: String,
    fromregionid: String,
    defaultunits: Option<rust_decimal::Decimal>,
    rolloverunits: Option<rust_decimal::Decimal>,
    reallocatedunits: Option<rust_decimal::Decimal>,
    unitsoffered: Option<rust_decimal::Decimal>,
    meanreserveprice: Option<rust_decimal::Decimal>,
    scalefactor: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMardetail1 {
    sra_year: i64,
    sra_quarter: i64,
    sra_runno: i64,
    participantid: String,
    cash_security_id: String,
    returned_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialRuntrk1 {
    sra_year: i64,
    sra_quarter: i64,
    sra_runno: i64,
    runtype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    rundate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    posteddate: Option<chrono::NaiveDateTime>,
    interest_versionno: Option<i64>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpayDetail1 {
    sra_year: i64,
    sra_quarter: i64,
    sra_runno: i64,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    contractid: String,
    maximum_units: Option<rust_decimal::Decimal>,
    units_sold: Option<rust_decimal::Decimal>,
    shortfall_units: Option<rust_decimal::Decimal>,
    reserve_price: Option<rust_decimal::Decimal>,
    clearing_price: Option<rust_decimal::Decimal>,
    payment_amount: Option<rust_decimal::Decimal>,
    shortfall_amount: Option<rust_decimal::Decimal>,
    allocation: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialExposure1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    prudential_runno: i64,
    participantid: String,
    sra_year: i64,
    sra_quarter: i64,
    interconnectorid: String,
    fromregionid: String,
    max_tranche: Option<i64>,
    auctionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    offer_submissiontime: Option<chrono::NaiveDateTime>,
    average_purchase_price: Option<rust_decimal::Decimal>,
    average_cancellation_price: Option<rust_decimal::Decimal>,
    cancellation_volume: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpaySum1 {
    sra_year: i64,
    sra_quarter: i64,
    sra_runno: i64,
    participantid: String,
    gross_proceeds_amount: Option<rust_decimal::Decimal>,
    total_gross_proceeds_amount: Option<rust_decimal::Decimal>,
    shortfall_amount: Option<rust_decimal::Decimal>,
    total_shortfall_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFundsBid1 {
    contractid: String,
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    optionid: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFileTrk1 {
    contractid: Option<String>,
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    filename: Option<String>,
    ackfilename: Option<String>,
    status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueTrk1 {
    contractid: Option<String>,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    rundate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    postdate: Option<chrono::NaiveDateTime>,
    postedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    status: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConData2 {
    contractid: String,
    versionno: rust_decimal::Decimal,
    participantid: String,
    interconnectorid: String,
    fromregionid: String,
    unitspurchased: Option<rust_decimal::Decimal>,
    linkpayment: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePublicData1 {
    contractid: String,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
    unitsoffered: Option<rust_decimal::Decimal>,
    unitssold: Option<rust_decimal::Decimal>,
    clearingprice: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueEstimate1 {
    contractyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    valuationid: String,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
    monthno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuction1 {
    auctionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    auctiondate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    notifydate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialRun1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueTrack1 {
    contractyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    valuationid: String,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    status: Option<String>,
    documentref: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueBidTrk1 {
    contractid: Option<String>,
    versionno: rust_decimal::Decimal,
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidloaddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCashSecurity1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    prudential_runno: i64,
    participantid: String,
    cash_security_id: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsPriceBid1 {
    contractid: Option<String>,
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    optionid: rust_decimal::Decimal,
    bidprice: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRpEstimate1 {
    contractyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    valuationid: String,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConEstimatesTrk1 {
    contractid: String,
    contractyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    valuationid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProduct1 {
    auctionid: String,
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    loaddate: chrono::NaiveDateTime,
    optionid: i64,
    interconnectorid: Option<String>,
    fromregionid: Option<String>,
    offer_quantity: Option<i64>,
    offer_price: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionCalendar2 {
    contractyear: rust_decimal::Decimal,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMargin1 {
    sra_year: i64,
    sra_quarter: i64,
    sra_runno: i64,
    participantid: String,
    total_cash_security: Option<rust_decimal::Decimal>,
    required_margin: Option<rust_decimal::Decimal>,
    returned_margin: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionIcAllocations2 {
    contractyear: rust_decimal::Decimal,
    quarter: rust_decimal::Decimal,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    fromregionid: String,
    maximumunits: Option<rust_decimal::Decimal>,
    proportion: Option<rust_decimal::Decimal>,
    auctionfee: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    changedate: Option<chrono::NaiveDateTime>,
    changedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePriceFundsBid1 {
    contractid: String,
    interconnectorid: String,
    fromregionid: String,
    units: Option<rust_decimal::Decimal>,
    bidprice: Option<rust_decimal::Decimal>,
    linkedbidflag: rust_decimal::Decimal,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCompPosition1 {
    #[serde(with = "crate::mms_datetime")]
    prudential_date: chrono::NaiveDateTime,
    prudential_runno: i64,
    participantid: String,
    trading_limit: Option<rust_decimal::Decimal>,
    prudential_exposure_amount: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataInterconnector1 {
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    interconnectorid: String,
    periodid: rust_decimal::Decimal,
    importvalue: Option<rust_decimal::Decimal>,
    exportvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataInterconnector1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: "INTERCONNECTOR".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataTrk1 {
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    aggregate_reads_load_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    individual_reads_load_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: "TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataAggregateReads1 {
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    connectionpointid: String,
    meter_type: String,
    frmp: String,
    lr: String,
    periodid: rust_decimal::Decimal,
    importvalue: rust_decimal::Decimal,
    exportvalue: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataAggregateReads1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: "AGGREGATE_READS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataIndividualReads1 {
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    meter_id: String,
    meter_id_suffix: String,
    frmp: String,
    lr: String,
    periodid: rust_decimal::Decimal,
    connectionpointid: String,
    meter_type: String,
    importvalue: rust_decimal::Decimal,
    exportvalue: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataIndividualReads1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: "INDIVIDUAL_READS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStation1 {
    stationid: String,
    stationname: Option<String>,
    address1: Option<String>,
    address2: Option<String>,
    address3: Option<String>,
    address4: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postcode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    connectionpointid: Option<String>,
}
impl crate::GetTable<ParticipantRegistrationStation1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "STATION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspInterconnector2 {
    linkid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    interconnectorid: Option<String>,
    fromregion: Option<String>,
    toregion: Option<String>,
    maxcapacity: Option<rust_decimal::Decimal>,
    tlf: Option<rust_decimal::Decimal>,
    lhsfactor: Option<rust_decimal::Decimal>,
    meterflowconstant: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    from_region_tlf: Option<rust_decimal::Decimal>,
    to_region_tlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ParticipantRegistrationMnspInterconnector2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "MNSP_INTERCONNECTOR".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcreditdetail1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    participantid: String,
    creditlimit: Option<rust_decimal::Decimal>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantcreditdetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "PARTICIPANTCREDITDETAIL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationowner1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    participantid: String,
    stationid: String,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStationowner1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "STATIONOWNER".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationoperatingstatus1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    stationid: String,
    versionno: rust_decimal::Decimal,
    status: Option<String>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStationoperatingstatus1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "STATIONOPERATINGSTATUS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspParticipant1 {
    interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationMnspParticipant1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "MNSP_PARTICIPANT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationownertrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    participantid: String,
    versionno: rust_decimal::Decimal,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStationownertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "STATIONOWNERTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenmeter1 {
    meterid: String,
    gensetid: Option<String>,
    connectionpointid: Option<String>,
    stationid: Option<String>,
    metertype: Option<String>,
    meterclass: Option<String>,
    voltagelevel: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    applydate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    comdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    decomdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationGenmeter1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "GENMETER".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetails1 {
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    bidtype: String,
    maxcapacity: Option<rust_decimal::Decimal>,
    minenablementlevel: Option<rust_decimal::Decimal>,
    maxenablementlevel: Option<rust_decimal::Decimal>,
    maxlowerangle: Option<rust_decimal::Decimal>,
    maxupperangle: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationBidduiddetails1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "BIDDUIDDETAILS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunitsUnit1 {
    gensetid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    unit_grouping_label: String,
    unit_count: Option<rust_decimal::Decimal>,
    unit_size: Option<rust_decimal::Decimal>,
    unit_max_size: Option<rust_decimal::Decimal>,
    aggregation_flag: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationGenunitsUnit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "GENUNITS_UNIT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategory1 {
    participantcategoryid: String,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantcategory1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "PARTICIPANTCATEGORY".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStadualloc1 {
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    stationid: String,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationStadualloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "STADUALLOC".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetail3 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    duid: String,
    versionno: rust_decimal::Decimal,
    connectionpointid: Option<String>,
    voltlevel: Option<String>,
    registeredcapacity: Option<rust_decimal::Decimal>,
    agccapability: Option<String>,
    dispatchtype: Option<String>,
    maxcapacity: Option<rust_decimal::Decimal>,
    starttype: Option<String>,
    normallyonflag: Option<String>,
    physicaldetailsflag: Option<String>,
    spinningreserveflag: Option<String>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    intermittentflag: Option<String>,
    semi_schedule_flag: Option<String>,
    maxrateofchangeup: Option<rust_decimal::Decimal>,
    maxrateofchangedown: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ParticipantRegistrationDudetail3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "DUDETAIL".into(),
            version: 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetailstrk1 {
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationBidduiddetailstrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "BIDDUIDDETAILSTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetailsummary4 {
    duid: String,
    #[serde(with = "crate::mms_datetime")]
    start_date: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    end_date: chrono::NaiveDateTime,
    dispatchtype: Option<String>,
    connectionpointid: Option<String>,
    regionid: Option<String>,
    stationid: Option<String>,
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    transmissionlossfactor: Option<rust_decimal::Decimal>,
    starttype: Option<String>,
    distributionlossfactor: Option<rust_decimal::Decimal>,
    minimum_energy_price: Option<rust_decimal::Decimal>,
    maximum_energy_price: Option<rust_decimal::Decimal>,
    schedule_type: Option<String>,
    min_ramp_rate_up: Option<rust_decimal::Decimal>,
    min_ramp_rate_down: Option<rust_decimal::Decimal>,
    max_ramp_rate_up: Option<rust_decimal::Decimal>,
    max_ramp_rate_down: Option<rust_decimal::Decimal>,
    is_aggregated: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ParticipantRegistrationDudetailsummary4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "DUDETAILSUMMARY".into(),
            version: 4,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunits2 {
    gensetid: String,
    stationid: Option<String>,
    setlossfactor: Option<rust_decimal::Decimal>,
    cdindicator: Option<String>,
    agcflag: Option<String>,
    spinningflag: Option<String>,
    voltlevel: Option<rust_decimal::Decimal>,
    registeredcapacity: Option<rust_decimal::Decimal>,
    dispatchtype: Option<String>,
    starttype: Option<String>,
    mktgeneratorind: Option<String>,
    normalstatus: Option<String>,
    maxcapacity: Option<rust_decimal::Decimal>,
    gensettype: Option<String>,
    gensetname: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    co2e_emissions_factor: Option<rust_decimal::Decimal>,
    co2e_energy_source: Option<String>,
    co2e_data_source: Option<String>,
}
impl crate::GetTable<ParticipantRegistrationGenunits2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "GENUNITS".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategoryalloc1 {
    participantcategoryid: String,
    participantid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantcategoryalloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "PARTICIPANTCATEGORYALLOC".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDualloc1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    duid: String,
    gensetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationDualloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "DUALLOC".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDispatchableunit1 {
    duid: String,
    duname: Option<String>,
    unittype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationDispatchableunit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "DISPATCHABLEUNIT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipant1 {
    participantid: String,
    participantclassid: Option<String>,
    name: Option<String>,
    description: Option<String>,
    acn: Option<String>,
    primarybusiness: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipant1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "PARTICIPANT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantaccount1 {
    accountname: Option<String>,
    participantid: String,
    accountnumber: Option<String>,
    bankname: Option<String>,
    banknumber: Option<rust_decimal::Decimal>,
    branchname: Option<String>,
    branchnumber: Option<rust_decimal::Decimal>,
    bsbnumber: Option<String>,
    nemmcocreditaccountnumber: Option<rust_decimal::Decimal>,
    nemmcodebitaccountnumber: Option<rust_decimal::Decimal>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    abn: Option<String>,
}
impl crate::GetTable<ParticipantRegistrationParticipantaccount1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "PARTICIPANTACCOUNT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantclass1 {
    participantclassid: String,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ParticipantRegistrationParticipantclass1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: "PARTICIPANTCLASS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMnspDayoffer2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    linkid: String,
    entrytype: Option<String>,
    rebidexplanation: Option<String>,
    priceband1: Option<rust_decimal::Decimal>,
    priceband2: Option<rust_decimal::Decimal>,
    priceband3: Option<rust_decimal::Decimal>,
    priceband4: Option<rust_decimal::Decimal>,
    priceband5: Option<rust_decimal::Decimal>,
    priceband6: Option<rust_decimal::Decimal>,
    priceband7: Option<rust_decimal::Decimal>,
    priceband8: Option<rust_decimal::Decimal>,
    priceband9: Option<rust_decimal::Decimal>,
    priceband10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    mr_factor: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<OfferMnspDayoffer2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "MNSP_DAYOFFER".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMnspPeroffer1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    linkid: String,
    periodid: rust_decimal::Decimal,
    maxavail: Option<rust_decimal::Decimal>,
    bandavail1: Option<rust_decimal::Decimal>,
    bandavail2: Option<rust_decimal::Decimal>,
    bandavail3: Option<rust_decimal::Decimal>,
    bandavail4: Option<rust_decimal::Decimal>,
    bandavail5: Option<rust_decimal::Decimal>,
    bandavail6: Option<rust_decimal::Decimal>,
    bandavail7: Option<rust_decimal::Decimal>,
    bandavail8: Option<rust_decimal::Decimal>,
    bandavail9: Option<rust_decimal::Decimal>,
    bandavail10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    fixedload: Option<rust_decimal::Decimal>,
    rampuprate: Option<rust_decimal::Decimal>,
    pasaavailability: Option<rust_decimal::Decimal>,
    mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<OfferMnspPeroffer1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "MNSP_PEROFFER".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMtpasaOfferdata1 {
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    unitid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    energy: Option<i64>,
    capacity1: Option<i64>,
    capacity2: Option<i64>,
    capacity3: Option<i64>,
    capacity4: Option<i64>,
    capacity5: Option<i64>,
    capacity6: Option<i64>,
    capacity7: Option<i64>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OfferMtpasaOfferdata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "MTPASA_OFFERDATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMtpasaOfferfiletrk1 {
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdatetime: chrono::NaiveDateTime,
    filename: Option<String>,
}
impl crate::GetTable<OfferMtpasaOfferfiletrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "MTPASA_OFFERFILETRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidBidperofferD2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    periodid: Option<rust_decimal::Decimal>,
    versionno: Option<rust_decimal::Decimal>,
    maxavail: Option<rust_decimal::Decimal>,
    fixedload: Option<rust_decimal::Decimal>,
    rocup: Option<rust_decimal::Decimal>,
    rocdown: Option<rust_decimal::Decimal>,
    enablementmin: Option<rust_decimal::Decimal>,
    enablementmax: Option<rust_decimal::Decimal>,
    lowbreakpoint: Option<rust_decimal::Decimal>,
    highbreakpoint: Option<rust_decimal::Decimal>,
    bandavail1: Option<rust_decimal::Decimal>,
    bandavail2: Option<rust_decimal::Decimal>,
    bandavail3: Option<rust_decimal::Decimal>,
    bandavail4: Option<rust_decimal::Decimal>,
    bandavail5: Option<rust_decimal::Decimal>,
    bandavail6: Option<rust_decimal::Decimal>,
    bandavail7: Option<rust_decimal::Decimal>,
    bandavail8: Option<rust_decimal::Decimal>,
    bandavail9: Option<rust_decimal::Decimal>,
    bandavail10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    pasaavailability: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<BidBidperofferD2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: "BIDPEROFFER_D".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidMnspFiletrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    participantid: String,
    filename: String,
    status: Option<String>,
    ackfilename: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<BidMnspFiletrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: "MNSP_FILETRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferMnspOffertrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantid: String,
    filename: String,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OfferMnspOffertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "MNSP_OFFERTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferBiddayoffer2 {
    duid: String,
    bidtype: String,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    versionno: Option<rust_decimal::Decimal>,
    participantid: Option<String>,
    dailyenergyconstraint: Option<rust_decimal::Decimal>,
    rebidexplanation: Option<String>,
    priceband1: Option<rust_decimal::Decimal>,
    priceband2: Option<rust_decimal::Decimal>,
    priceband3: Option<rust_decimal::Decimal>,
    priceband4: Option<rust_decimal::Decimal>,
    priceband5: Option<rust_decimal::Decimal>,
    priceband6: Option<rust_decimal::Decimal>,
    priceband7: Option<rust_decimal::Decimal>,
    priceband8: Option<rust_decimal::Decimal>,
    priceband9: Option<rust_decimal::Decimal>,
    priceband10: Option<rust_decimal::Decimal>,
    minimumload: Option<rust_decimal::Decimal>,
    t1: Option<rust_decimal::Decimal>,
    t2: Option<rust_decimal::Decimal>,
    t3: Option<rust_decimal::Decimal>,
    t4: Option<rust_decimal::Decimal>,
    normalstatus: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    mr_factor: Option<rust_decimal::Decimal>,
    entrytype: Option<String>,
}
impl crate::GetTable<OfferBiddayoffer2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "BIDDAYOFFER".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BidBiddayofferD2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdate: Option<chrono::NaiveDateTime>,
    versionno: Option<rust_decimal::Decimal>,
    participantid: Option<String>,
    dailyenergyconstraint: Option<rust_decimal::Decimal>,
    rebidexplanation: Option<String>,
    priceband1: Option<rust_decimal::Decimal>,
    priceband2: Option<rust_decimal::Decimal>,
    priceband3: Option<rust_decimal::Decimal>,
    priceband4: Option<rust_decimal::Decimal>,
    priceband5: Option<rust_decimal::Decimal>,
    priceband6: Option<rust_decimal::Decimal>,
    priceband7: Option<rust_decimal::Decimal>,
    priceband8: Option<rust_decimal::Decimal>,
    priceband9: Option<rust_decimal::Decimal>,
    priceband10: Option<rust_decimal::Decimal>,
    minimumload: Option<rust_decimal::Decimal>,
    t1: Option<rust_decimal::Decimal>,
    t2: Option<rust_decimal::Decimal>,
    t3: Option<rust_decimal::Decimal>,
    t4: Option<rust_decimal::Decimal>,
    normalstatus: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    mr_factor: Option<rust_decimal::Decimal>,
    entrytype: Option<String>,
}
impl crate::GetTable<BidBiddayofferD2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "BID".into(),
            table_name: "BIDDAYOFFER_D".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferBidofferfiletrk1 {
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    filename: String,
    status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<OfferBidofferfiletrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "BIDOFFERFILETRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OfferBidperoffer1 {
    duid: String,
    bidtype: String,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    offerdate: chrono::NaiveDateTime,
    periodid: rust_decimal::Decimal,
    versionno: Option<rust_decimal::Decimal>,
    maxavail: Option<rust_decimal::Decimal>,
    fixedload: Option<rust_decimal::Decimal>,
    rocup: Option<rust_decimal::Decimal>,
    rocdown: Option<rust_decimal::Decimal>,
    enablementmin: Option<rust_decimal::Decimal>,
    enablementmax: Option<rust_decimal::Decimal>,
    lowbreakpoint: Option<rust_decimal::Decimal>,
    highbreakpoint: Option<rust_decimal::Decimal>,
    bandavail1: Option<rust_decimal::Decimal>,
    bandavail2: Option<rust_decimal::Decimal>,
    bandavail3: Option<rust_decimal::Decimal>,
    bandavail4: Option<rust_decimal::Decimal>,
    bandavail5: Option<rust_decimal::Decimal>,
    bandavail6: Option<rust_decimal::Decimal>,
    bandavail7: Option<rust_decimal::Decimal>,
    bandavail8: Option<rust_decimal::Decimal>,
    bandavail9: Option<rust_decimal::Decimal>,
    bandavail10: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    pasaavailability: Option<rust_decimal::Decimal>,
    mr_capacity: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<OfferBidperoffer1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OFFER".into(),
            table_name: "BIDPEROFFER".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GcrhsNull1 {
    genconid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    scope: String,
    termid: rust_decimal::Decimal,
    groupid: Option<rust_decimal::Decimal>,
    spd_id: Option<String>,
    spd_type: Option<String>,
    factor: Option<rust_decimal::Decimal>,
    operation: Option<String>,
    defaultvalue: Option<rust_decimal::Decimal>,
    parameterterm1: Option<String>,
    parameterterm2: Option<String>,
    parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GcrhsNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GCRHS".into(),
            table_name: "NULL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdrcNull2 {
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    genconid: String,
    factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    bidtype: String,
}
impl crate::GetTable<SpdrcNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SPDRC".into(),
            table_name: "NULL".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdcpcNull2 {
    connectionpointid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    genconid: String,
    factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    bidtype: String,
}
impl crate::GetTable<SpdcpcNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SPDCPC".into(),
            table_name: "NULL".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqdescNull2 {
    equationid: String,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    impact: Option<String>,
    source: Option<String>,
    limittype: Option<String>,
    reason: Option<String>,
    modifications: Option<String>,
    additionalnotes: Option<String>,
}
impl crate::GetTable<GeqdescNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GEQDESC".into(),
            table_name: "NULL".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsettrkNull2 {
    genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    description: Option<String>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    coverage: Option<String>,
    modifications: Option<String>,
    systemnormal: Option<String>,
    outage: Option<String>,
}
impl crate::GetTable<GenconsettrkNull2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENCONSETTRK".into(),
            table_name: "NULL".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GencondataNull6 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    genconid: String,
    constrainttype: Option<String>,
    constraintvalue: Option<rust_decimal::Decimal>,
    description: Option<String>,
    status: Option<String>,
    genericconstraintweight: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    dynamicrhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    dispatch: Option<String>,
    predispatch: Option<String>,
    stpasa: Option<String>,
    mtpasa: Option<String>,
    impact: Option<String>,
    source: Option<String>,
    limittype: Option<String>,
    reason: Option<String>,
    modifications: Option<String>,
    additionalnotes: Option<String>,
    p5min_scope_override: Option<String>,
    lrc: Option<String>,
    lor: Option<String>,
    force_scada: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<GencondataNull6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENCONDATA".into(),
            table_name: "NULL".into(),
            version: 6,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsetNull1 {
    genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    genconid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    genconeffdate: Option<chrono::NaiveDateTime>,
    genconversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GenconsetNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENCONSET".into(),
            table_name: "NULL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqrhsNull1 {
    equationid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    termid: rust_decimal::Decimal,
    groupid: Option<rust_decimal::Decimal>,
    spd_id: Option<String>,
    spd_type: Option<String>,
    factor: Option<rust_decimal::Decimal>,
    operation: Option<String>,
    defaultvalue: Option<rust_decimal::Decimal>,
    parameterterm1: Option<String>,
    parameterterm2: Option<String>,
    parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GeqrhsNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GEQRHS".into(),
            table_name: "NULL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdiccNull1 {
    interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    genconid: String,
    factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SpdiccNull1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SPDICC".into(),
            table_name: "NULL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintGenconsetinvoke2 {
    invocation_id: i64,
    #[serde(with = "crate::mms_datetime")]
    startdate: chrono::NaiveDateTime,
    startperiod: rust_decimal::Decimal,
    genconsetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    endperiod: Option<rust_decimal::Decimal>,
    startauthorisedby: Option<String>,
    endauthorisedby: Option<String>,
    intervention: Option<String>,
    asconstrainttype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startintervaldatetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    endintervaldatetime: Option<chrono::NaiveDateTime>,
    systemnormal: Option<String>,
}
impl crate::GetTable<GenericConstraintGenconsetinvoke2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENERIC_CONSTRAINT".into(),
            table_name: "GENCONSETINVOKE".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintEmsmaster1 {
    spd_id: String,
    spd_type: String,
    description: Option<String>,
    grouping_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GenericConstraintEmsmaster1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENERIC_CONSTRAINT".into(),
            table_name: "EMSMASTER".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMnspbidtrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    participantid: String,
    linkid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    offersettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    offereffectivedate: Option<chrono::NaiveDateTime>,
    offerversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchMnspbidtrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "MNSPBIDTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchRegionsum4 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    regionid: String,
    dispatchinterval: rust_decimal::Decimal,
    intervention: rust_decimal::Decimal,
    totaldemand: Option<rust_decimal::Decimal>,
    availablegeneration: Option<rust_decimal::Decimal>,
    availableload: Option<rust_decimal::Decimal>,
    demandforecast: Option<rust_decimal::Decimal>,
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    dispatchableload: Option<rust_decimal::Decimal>,
    netinterchange: Option<rust_decimal::Decimal>,
    excessgeneration: Option<rust_decimal::Decimal>,
    lower5mindispatch: Option<rust_decimal::Decimal>,
    lower5minimport: Option<rust_decimal::Decimal>,
    lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    lower5minlocalprice: Option<rust_decimal::Decimal>,
    lower5minlocalreq: Option<rust_decimal::Decimal>,
    lower5minprice: Option<rust_decimal::Decimal>,
    lower5minreq: Option<rust_decimal::Decimal>,
    lower5minsupplyprice: Option<rust_decimal::Decimal>,
    lower60secdispatch: Option<rust_decimal::Decimal>,
    lower60secimport: Option<rust_decimal::Decimal>,
    lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    lower60seclocalprice: Option<rust_decimal::Decimal>,
    lower60seclocalreq: Option<rust_decimal::Decimal>,
    lower60secprice: Option<rust_decimal::Decimal>,
    lower60secreq: Option<rust_decimal::Decimal>,
    lower60secsupplyprice: Option<rust_decimal::Decimal>,
    lower6secdispatch: Option<rust_decimal::Decimal>,
    lower6secimport: Option<rust_decimal::Decimal>,
    lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    lower6seclocalprice: Option<rust_decimal::Decimal>,
    lower6seclocalreq: Option<rust_decimal::Decimal>,
    lower6secprice: Option<rust_decimal::Decimal>,
    lower6secreq: Option<rust_decimal::Decimal>,
    lower6secsupplyprice: Option<rust_decimal::Decimal>,
    raise5mindispatch: Option<rust_decimal::Decimal>,
    raise5minimport: Option<rust_decimal::Decimal>,
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    raise5minlocalprice: Option<rust_decimal::Decimal>,
    raise5minlocalreq: Option<rust_decimal::Decimal>,
    raise5minprice: Option<rust_decimal::Decimal>,
    raise5minreq: Option<rust_decimal::Decimal>,
    raise5minsupplyprice: Option<rust_decimal::Decimal>,
    raise60secdispatch: Option<rust_decimal::Decimal>,
    raise60secimport: Option<rust_decimal::Decimal>,
    raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    raise60seclocalprice: Option<rust_decimal::Decimal>,
    raise60seclocalreq: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchRegionsum4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "REGIONSUM".into(),
            version: 4,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchLocalPrice1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    local_price_adjustment: Option<rust_decimal::Decimal>,
    locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchLocalPrice1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "LOCAL_PRICE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitSolution2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    duid: String,
    tradetype: Option<rust_decimal::Decimal>,
    dispatchinterval: Option<rust_decimal::Decimal>,
    intervention: rust_decimal::Decimal,
    connectionpointid: Option<String>,
    dispatchmode: Option<rust_decimal::Decimal>,
    agcstatus: Option<rust_decimal::Decimal>,
    initialmw: Option<rust_decimal::Decimal>,
    totalcleared: Option<rust_decimal::Decimal>,
    rampdownrate: Option<rust_decimal::Decimal>,
    rampuprate: Option<rust_decimal::Decimal>,
    lower5min: Option<rust_decimal::Decimal>,
    lower60sec: Option<rust_decimal::Decimal>,
    lower6sec: Option<rust_decimal::Decimal>,
    raise5min: Option<rust_decimal::Decimal>,
    raise60sec: Option<rust_decimal::Decimal>,
    raise6sec: Option<rust_decimal::Decimal>,
    downepf: Option<rust_decimal::Decimal>,
    upepf: Option<rust_decimal::Decimal>,
    marginal5minvalue: Option<rust_decimal::Decimal>,
    marginal60secvalue: Option<rust_decimal::Decimal>,
    marginal6secvalue: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violation5mindegree: Option<rust_decimal::Decimal>,
    violation60secdegree: Option<rust_decimal::Decimal>,
    violation6secdegree: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    lowerreg: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    availability: Option<rust_decimal::Decimal>,
    raise6secflags: Option<rust_decimal::Decimal>,
    raise60secflags: Option<rust_decimal::Decimal>,
    raise5minflags: Option<rust_decimal::Decimal>,
    raiseregflags: Option<rust_decimal::Decimal>,
    lower6secflags: Option<rust_decimal::Decimal>,
    lower60secflags: Option<rust_decimal::Decimal>,
    lower5minflags: Option<rust_decimal::Decimal>,
    lowerregflags: Option<rust_decimal::Decimal>,
    raiseregavailability: Option<rust_decimal::Decimal>,
    raiseregenablementmax: Option<rust_decimal::Decimal>,
    raiseregenablementmin: Option<rust_decimal::Decimal>,
    lowerregavailability: Option<rust_decimal::Decimal>,
    lowerregenablementmax: Option<rust_decimal::Decimal>,
    lowerregenablementmin: Option<rust_decimal::Decimal>,
    raise6secactualavailability: Option<rust_decimal::Decimal>,
    raise60secactualavailability: Option<rust_decimal::Decimal>,
    raise5minactualavailability: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchUnitSolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "UNIT_SOLUTION".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnectorres3 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    interconnectorid: String,
    dispatchinterval: rust_decimal::Decimal,
    intervention: rust_decimal::Decimal,
    meteredmwflow: Option<rust_decimal::Decimal>,
    mwflow: Option<rust_decimal::Decimal>,
    mwlosses: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    exportlimit: Option<rust_decimal::Decimal>,
    importlimit: Option<rust_decimal::Decimal>,
    marginalloss: Option<rust_decimal::Decimal>,
    exportgenconid: Option<String>,
    importgenconid: Option<String>,
    fcasexportlimit: Option<rust_decimal::Decimal>,
    fcasimportlimit: Option<rust_decimal::Decimal>,
    local_price_adjustment_export: Option<rust_decimal::Decimal>,
    locally_constrained_export: Option<rust_decimal::Decimal>,
    local_price_adjustment_import: Option<rust_decimal::Decimal>,
    locally_constrained_import: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchInterconnectorres3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "INTERCONNECTORRES".into(),
            version: 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchOffertrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    bidsettlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    bidofferdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchOffertrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "OFFERTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchBlockedConstraints1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    constraintid: String,
}
impl crate::GetTable<DispatchBlockedConstraints1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "BLOCKED_CONSTRAINTS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintrelaxation1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    constraintid: String,
    rhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    versionno: rust_decimal::Decimal,
}
impl crate::GetTable<PriceloadConstraintrelaxation1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRICELOAD".into(),
            table_name: "CONSTRAINTRELAXATION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnection1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    intervention: rust_decimal::Decimal,
    from_regionid: String,
    to_regionid: String,
    dispatchinterval: Option<rust_decimal::Decimal>,
    irlf: Option<rust_decimal::Decimal>,
    mwflow: Option<rust_decimal::Decimal>,
    meteredmwflow: Option<rust_decimal::Decimal>,
    from_region_mw_losses: Option<rust_decimal::Decimal>,
    to_region_mw_losses: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchInterconnection1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "INTERCONNECTION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitConformance1 {
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    duid: String,
    totalcleared: Option<rust_decimal::Decimal>,
    actualmw: Option<rust_decimal::Decimal>,
    roc: Option<rust_decimal::Decimal>,
    availability: Option<rust_decimal::Decimal>,
    lowerreg: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    striglm: Option<rust_decimal::Decimal>,
    ltriglm: Option<rust_decimal::Decimal>,
    mwerror: Option<rust_decimal::Decimal>,
    max_mwerror: Option<rust_decimal::Decimal>,
    lecount: Option<i64>,
    secount: Option<i64>,
    status: Option<String>,
    participant_status_action: Option<String>,
    operating_mode: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchUnitConformance1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "UNIT_CONFORMANCE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchConstraint5 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    constraintid: String,
    dispatchinterval: rust_decimal::Decimal,
    intervention: rust_decimal::Decimal,
    rhs: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    genconid_effectivedate: Option<chrono::NaiveDateTime>,
    genconid_versionno: Option<rust_decimal::Decimal>,
    lhs: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchConstraint5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "CONSTRAINT".into(),
            version: 5,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadPriceRevision1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    intervention: rust_decimal::Decimal,
    regionid: String,
    bidtype: String,
    versionno: i64,
    rrp_new: Option<rust_decimal::Decimal>,
    rrp_old: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<PriceloadPriceRevision1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRICELOAD".into(),
            table_name: "PRICE_REVISION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitScada1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    scadavalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchUnitScada1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "UNIT_SCADA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintFcasOcd1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: i64,
    intervention: i64,
    constraintid: String,
    versionno: i64,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    rhs: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PriceloadConstraintFcasOcd1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRICELOAD".into(),
            table_name: "CONSTRAINT_FCAS_OCD".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchCaseSolution2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    intervention: rust_decimal::Decimal,
    casesubtype: Option<String>,
    solutionstatus: Option<rust_decimal::Decimal>,
    spdversion: Option<String>,
    nonphysicallosses: Option<rust_decimal::Decimal>,
    totalobjective: Option<rust_decimal::Decimal>,
    totalareagenviolation: Option<rust_decimal::Decimal>,
    totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    totalgenericviolation: Option<rust_decimal::Decimal>,
    totalramprateviolation: Option<rust_decimal::Decimal>,
    totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    total5minviolation: Option<rust_decimal::Decimal>,
    totalregviolation: Option<rust_decimal::Decimal>,
    total6secviolation: Option<rust_decimal::Decimal>,
    total60secviolation: Option<rust_decimal::Decimal>,
    totalasprofileviolation: Option<rust_decimal::Decimal>,
    totalfaststartviolation: Option<rust_decimal::Decimal>,
    totalenergyofferviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    switchruninitialstatus: Option<rust_decimal::Decimal>,
    switchrunbeststatus: Option<rust_decimal::Decimal>,
    switchrunbeststatus_int: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchCaseSolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "CASE_SOLUTION".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchIntermittentForecastTrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    origin: Option<String>,
    forecast_priority: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    offerdatetime: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchIntermittentForecastTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "INTERMITTENT_FORECAST_TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchPrice4 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    regionid: String,
    dispatchinterval: String,
    intervention: rust_decimal::Decimal,
    rrp: Option<rust_decimal::Decimal>,
    eep: Option<rust_decimal::Decimal>,
    rop: Option<rust_decimal::Decimal>,
    apcflag: Option<rust_decimal::Decimal>,
    marketsuspendedflag: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    raise6secrrp: Option<rust_decimal::Decimal>,
    raise6secrop: Option<rust_decimal::Decimal>,
    raise6secapcflag: Option<rust_decimal::Decimal>,
    raise60secrrp: Option<rust_decimal::Decimal>,
    raise60secrop: Option<rust_decimal::Decimal>,
    raise60secapcflag: Option<rust_decimal::Decimal>,
    raise5minrrp: Option<rust_decimal::Decimal>,
    raise5minrop: Option<rust_decimal::Decimal>,
    raise5minapcflag: Option<rust_decimal::Decimal>,
    raiseregrrp: Option<rust_decimal::Decimal>,
    raiseregrop: Option<rust_decimal::Decimal>,
    raiseregapcflag: Option<rust_decimal::Decimal>,
    lower6secrrp: Option<rust_decimal::Decimal>,
    lower6secrop: Option<rust_decimal::Decimal>,
    lower6secapcflag: Option<rust_decimal::Decimal>,
    lower60secrrp: Option<rust_decimal::Decimal>,
    lower60secrop: Option<rust_decimal::Decimal>,
    lower60secapcflag: Option<rust_decimal::Decimal>,
    lower5minrrp: Option<rust_decimal::Decimal>,
    lower5minrop: Option<rust_decimal::Decimal>,
    lower5minapcflag: Option<rust_decimal::Decimal>,
    lowerregrrp: Option<rust_decimal::Decimal>,
    lowerregrop: Option<rust_decimal::Decimal>,
    lowerregapcflag: Option<rust_decimal::Decimal>,
    price_status: Option<String>,
    pre_ap_energy_price: Option<rust_decimal::Decimal>,
    pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    cumul_pre_ap_energy_price: Option<rust_decimal::Decimal>,
    cumul_pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    cumul_pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    cumul_pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    cumul_pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchPrice4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "PRICE".into(),
            version: 4,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMrScheduleTrk1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    mr_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    version_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<DispatchMrScheduleTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "MR_SCHEDULE_TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchFcasReq2 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    runno: rust_decimal::Decimal,
    intervention: rust_decimal::Decimal,
    genconid: String,
    regionid: String,
    bidtype: String,
    #[serde(with = "crate::mms_datetime_opt")]
    genconeffectivedate: Option<chrono::NaiveDateTime>,
    genconversionno: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    base_cost: Option<rust_decimal::Decimal>,
    adjusted_cost: Option<rust_decimal::Decimal>,
    estimated_cmpf: Option<rust_decimal::Decimal>,
    estimated_crmpf: Option<rust_decimal::Decimal>,
    recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    recovery_factor_crmpf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchFcasReq2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "FCAS_REQ".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchNegativeResidue1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    nrm_datetime: chrono::NaiveDateTime,
    directional_interconnectorid: String,
    nrm_activated_flag: Option<rust_decimal::Decimal>,
    cumul_negresidue_amount: Option<rust_decimal::Decimal>,
    cumul_negresidue_prev_ti: Option<rust_decimal::Decimal>,
    negresidue_current_ti: Option<rust_decimal::Decimal>,
    negresidue_pd_next_ti: Option<rust_decimal::Decimal>,
    price_revision: Option<String>,
    predispatchseqno: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    event_activated_di: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    event_deactivated_di: Option<chrono::NaiveDateTime>,
    di_notbinding_count: Option<rust_decimal::Decimal>,
    di_violated_count: Option<rust_decimal::Decimal>,
    nrmconstraint_blocked_flag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<DispatchNegativeResidue1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: "NEGATIVE_RESIDUE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaRegionsolution5 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    regionid: String,
    demand10: Option<rust_decimal::Decimal>,
    demand50: Option<rust_decimal::Decimal>,
    demand90: Option<rust_decimal::Decimal>,
    reservereq: Option<rust_decimal::Decimal>,
    capacityreq: Option<rust_decimal::Decimal>,
    energyreqdemand50: Option<rust_decimal::Decimal>,
    unconstrainedcapacity: Option<rust_decimal::Decimal>,
    constrainedcapacity: Option<rust_decimal::Decimal>,
    netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    surpluscapacity: Option<rust_decimal::Decimal>,
    surplusreserve: Option<rust_decimal::Decimal>,
    reservecondition: Option<rust_decimal::Decimal>,
    maxsurplusreserve: Option<rust_decimal::Decimal>,
    maxsparecapacity: Option<rust_decimal::Decimal>,
    lorcondition: Option<rust_decimal::Decimal>,
    aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    aggregatescheduledload: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    aggregatepasaavailability: Option<rust_decimal::Decimal>,
    runtype: String,
    energyreqdemand10: Option<rust_decimal::Decimal>,
    calculatedlor1level: Option<rust_decimal::Decimal>,
    calculatedlor2level: Option<rust_decimal::Decimal>,
    msrnetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    lornetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    totalintermittentgeneration: Option<rust_decimal::Decimal>,
    demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    uigf: Option<rust_decimal::Decimal>,
    semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    lor_semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    lcr: Option<rust_decimal::Decimal>,
    lcr2: Option<rust_decimal::Decimal>,
    fum: Option<rust_decimal::Decimal>,
    ss_solar_uigf: Option<rust_decimal::Decimal>,
    ss_wind_uigf: Option<rust_decimal::Decimal>,
    ss_solar_capacity: Option<rust_decimal::Decimal>,
    ss_wind_capacity: Option<rust_decimal::Decimal>,
    ss_solar_cleared: Option<rust_decimal::Decimal>,
    ss_wind_cleared: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PdpasaRegionsolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PDPASA".into(),
            table_name: "REGIONSOLUTION".into(),
            version: 5,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaCasesolution3 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    pasaversion: Option<String>,
    reservecondition: Option<rust_decimal::Decimal>,
    lorcondition: Option<rust_decimal::Decimal>,
    capacityobjfunction: Option<rust_decimal::Decimal>,
    capacityoption: Option<rust_decimal::Decimal>,
    maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    maxsparecapacityoption: Option<rust_decimal::Decimal>,
    interconnectorflowpenalty: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    reliabilitylrcdemandoption: Option<rust_decimal::Decimal>,
    outagelrcdemandoption: Option<rust_decimal::Decimal>,
    lordemandoption: Option<rust_decimal::Decimal>,
    reliabilitylrccapacityoption: Option<String>,
    outagelrccapacityoption: Option<String>,
    lorcapacityoption: Option<String>,
    loruigf_option: Option<rust_decimal::Decimal>,
    reliability_lrcuigf_option: Option<rust_decimal::Decimal>,
    outage_lrcuigf_option: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<PdpasaCasesolution3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PDPASA".into(),
            table_name: "CASESOLUTION".into(),
            version: 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigMarketPriceThresholds1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    voll: Option<rust_decimal::Decimal>,
    marketpricefloor: Option<rust_decimal::Decimal>,
    administered_price_threshold: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegionstandingdata1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    regionid: String,
    rsoid: Option<String>,
    regionalreferencepointid: Option<String>,
    peaktradingperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnector1 {
    interconnectorid: String,
    regionfrom: Option<String>,
    rsoid: Option<String>,
    regionto: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectoralloc1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    regionid: String,
    participantid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectorconstraint1 {
    reserveoverallloadfactor: Option<rust_decimal::Decimal>,
    fromregionlossshare: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    maxmwin: Option<rust_decimal::Decimal>,
    maxmwout: Option<rust_decimal::Decimal>,
    lossconstant: Option<rust_decimal::Decimal>,
    lossflowcoefficient: Option<rust_decimal::Decimal>,
    emsmeasurand: Option<String>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    dynamicrhs: Option<String>,
    importlimit: Option<rust_decimal::Decimal>,
    exportlimit: Option<rust_decimal::Decimal>,
    outagederationfactor: Option<rust_decimal::Decimal>,
    nonphysicallossfactor: Option<rust_decimal::Decimal>,
    overloadfactor60sec: Option<rust_decimal::Decimal>,
    overloadfactor6sec: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    fcassupportunavailable: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossmodel1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    periodid: Option<String>,
    losssegment: rust_decimal::Decimal,
    mwbreakpoint: Option<rust_decimal::Decimal>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypestrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossfactormodel1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    interconnectorid: String,
    regionid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigIntraregionalloc1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    regionid: String,
    participantid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigTransmissionlossfactor2 {
    transmissionlossfactor: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    connectionpointid: String,
    regionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegion1 {
    regionid: String,
    description: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypes1 {
    bidtype: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    description: Option<String>,
    numberofbands: Option<rust_decimal::Decimal>,
    numdaysaheadpricelocked: Option<rust_decimal::Decimal>,
    validationrule: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagestatuscode1 {
    outagestatuscode: String,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkOutagestatuscode1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "OUTAGESTATUSCODE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkSubstationdetail1 {
    substationid: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    description: Option<String>,
    regionid: Option<String>,
    ownerid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkSubstationdetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "SUBSTATIONDETAIL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkStaticrating1 {
    substationid: String,
    equipmenttype: String,
    equipmentid: String,
    ratinglevel: String,
    applicationid: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    ratingvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkStaticrating1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "STATICRATING".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutageconstraintset1 {
    outageid: rust_decimal::Decimal,
    genconsetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    startinterval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    endinterval: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkOutageconstraintset1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "OUTAGECONSTRAINTSET".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagedetail3 {
    outageid: rust_decimal::Decimal,
    substationid: String,
    equipmenttype: String,
    equipmentid: String,
    #[serde(with = "crate::mms_datetime")]
    starttime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    endtime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    submitteddate: Option<chrono::NaiveDateTime>,
    outagestatuscode: Option<String>,
    resubmitreason: Option<String>,
    resubmitoutageid: Option<rust_decimal::Decimal>,
    recalltimeday: Option<rust_decimal::Decimal>,
    recalltimenight: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    reason: Option<String>,
    issecondary: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    actual_starttime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    actual_endtime: Option<chrono::NaiveDateTime>,
    companyrefcode: Option<String>,
}
impl crate::GetTable<NetworkOutagedetail3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "OUTAGEDETAIL".into(),
            version: 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRealtimerating1 {
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    spd_id: String,
    ratingvalue: rust_decimal::Decimal,
}
impl crate::GetTable<NetworkRealtimerating1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "REALTIMERATING".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkEquipmentdetail1 {
    substationid: String,
    equipmenttype: String,
    equipmentid: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    voltage: Option<String>,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkEquipmentdetail1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "EQUIPMENTDETAIL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRating1 {
    spd_id: String,
    #[serde(with = "crate::mms_datetime")]
    validfrom: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    validto: Option<chrono::NaiveDateTime>,
    regionid: Option<String>,
    substationid: Option<String>,
    equipmenttype: Option<String>,
    equipmentid: Option<String>,
    ratinglevel: Option<String>,
    isdynamic: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<NetworkRating1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: "RATING".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocation2 {
    reallocationid: String,
    creditparticipantid: Option<String>,
    debitparticipantid: Option<String>,
    regionid: Option<String>,
    agreementtype: Option<String>,
    creditreference: Option<String>,
    debitreference: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    current_stepid: Option<String>,
    daytype: Option<String>,
    reallocation_type: Option<String>,
    calendarid: Option<String>,
    intervallength: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SetcfgReallocation2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETCFG".into(),
            table_name: "REALLOCATION".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfee1 {
    marketfeeid: String,
    marketfeeperiod: Option<String>,
    marketfeetype: Option<String>,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    gl_tcode: Option<String>,
    gl_financialcode: Option<String>,
    fee_class: Option<String>,
}
impl crate::GetTable<SettlementConfigMarketfee1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "MARKETFEE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExclTrk1 {
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketFeeCatExclTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "MARKET_FEE_CAT_EXCL_TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeedata1 {
    marketfeeid: String,
    marketfeeversionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    marketfeevalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketfeedata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "MARKETFEEDATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusion1 {
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketFeeExclusion1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "MARKET_FEE_EXCLUSION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeetrk1 {
    marketfeeversionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketfeetrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "MARKETFEETRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusionTrk1 {
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigMarketFeeExclusionTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "MARKET_FEE_EXCLUSION_TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigAncillaryRecoverySplit1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    service: String,
    paymenttype: String,
    customer_portion: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigAncillaryRecoverySplit1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "ANCILLARY_RECOVERY_SPLIT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocationinterval1 {
    reallocationid: String,
    periodid: i64,
    value: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    nrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<SetcfgReallocationinterval1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETCFG".into(),
            table_name: "REALLOCATIONINTERVAL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpf1 {
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantcategoryid: String,
    connectionpointid: String,
    mpf: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigSetcfgParticipantMpf1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "SETCFG_PARTICIPANT_MPF".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpftrk1 {
    participantid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigSetcfgParticipantMpftrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "SETCFG_PARTICIPANT_MPFTRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExcl1 {
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    participant_categoryid: String,
}
impl crate::GetTable<SettlementConfigMarketFeeCatExcl1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "MARKET_FEE_CAT_EXCL".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigParticipantBandfeeAlloc1 {
    participantid: String,
    marketfeeid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    participantcategoryid: String,
    marketfeevalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<SettlementConfigParticipantBandfeeAlloc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: "PARTICIPANT_BANDFEE_ALLOC".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct VoltageInstructionInstruction2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    ems_id: String,
    participantid: Option<String>,
    station_id: Option<String>,
    device_id: Option<String>,
    device_type: Option<String>,
    control_type: Option<String>,
    target: Option<rust_decimal::Decimal>,
    conforming: Option<rust_decimal::Decimal>,
    instruction_summary: Option<String>,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    instruction_sequence: Option<rust_decimal::Decimal>,
    additional_notes: Option<String>,
}
impl crate::GetTable<VoltageInstructionInstruction2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "VOLTAGE_INSTRUCTION".into(),
            table_name: "INSTRUCTION".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct VoltageInstructionTrack2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    file_type: Option<String>,
    #[serde(with = "crate::mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    se_datetime: Option<chrono::NaiveDateTime>,
    solution_category: Option<String>,
    solution_status: Option<String>,
    operating_mode: Option<String>,
    operating_status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    est_expiry: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    est_next_instruction: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<VoltageInstructionTrack2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "VOLTAGE_INSTRUCTION".into(),
            table_name: "TRACK".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructGdinstruct1 {
    duid: Option<String>,
    stationid: Option<String>,
    regionid: Option<String>,
    id: rust_decimal::Decimal,
    instructiontypeid: Option<String>,
    instructionsubtypeid: Option<String>,
    instructionclassid: Option<String>,
    reason: Option<String>,
    instlevel: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    participantid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    issuedtime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    targettime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GdInstructGdinstruct1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GD_INSTRUCT".into(),
            table_name: "GDINSTRUCT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructiontype1 {
    instructiontypeid: String,
    description: Option<String>,
    regionid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GdInstructInstructiontype1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GD_INSTRUCT".into(),
            table_name: "INSTRUCTIONTYPE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructionsubtype1 {
    instructiontypeid: String,
    instructionsubtypeid: String,
    description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<GdInstructInstructionsubtype1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GD_INSTRUCT".into(),
            table_name: "INSTRUCTIONSUBTYPE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticedata1 {
    noticeid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    typeid: Option<String>,
    noticetype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    reason: Option<String>,
    externalreference: Option<String>,
}
impl crate::GetTable<MarketNoticeMarketnoticedata1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MARKET_NOTICE".into(),
            table_name: "MARKETNOTICEDATA".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeParticipantnoticetrk1 {
    participantid: String,
    noticeid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketNoticeParticipantnoticetrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MARKET_NOTICE".into(),
            table_name: "PARTICIPANTNOTICETRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticetype1 {
    typeid: String,
    description: Option<String>,
    raisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MarketNoticeMarketnoticetype1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MARKET_NOTICE".into(),
            table_name: "MARKETNOTICETYPE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minCasesolution2 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    startinterval_datetime: Option<String>,
    totalobjective: Option<rust_decimal::Decimal>,
    nonphysicallosses: Option<rust_decimal::Decimal>,
    totalareagenviolation: Option<rust_decimal::Decimal>,
    totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    totalgenericviolation: Option<rust_decimal::Decimal>,
    totalramprateviolation: Option<rust_decimal::Decimal>,
    totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    total5minviolation: Option<rust_decimal::Decimal>,
    totalregviolation: Option<rust_decimal::Decimal>,
    total6secviolation: Option<rust_decimal::Decimal>,
    total60secviolation: Option<rust_decimal::Decimal>,
    totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    totalenergyofferviolation: Option<rust_decimal::Decimal>,
    totalasprofileviolation: Option<rust_decimal::Decimal>,
    totalfaststartviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minCasesolution2> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: "CASESOLUTION".into(),
            version: 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minUnitsolution3 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    duid: String,
    connectionpointid: Option<String>,
    tradetype: Option<rust_decimal::Decimal>,
    agcstatus: Option<rust_decimal::Decimal>,
    initialmw: Option<rust_decimal::Decimal>,
    totalcleared: Option<rust_decimal::Decimal>,
    rampdownrate: Option<rust_decimal::Decimal>,
    rampuprate: Option<rust_decimal::Decimal>,
    lower5min: Option<rust_decimal::Decimal>,
    lower60sec: Option<rust_decimal::Decimal>,
    lower6sec: Option<rust_decimal::Decimal>,
    raise5min: Option<rust_decimal::Decimal>,
    raise60sec: Option<rust_decimal::Decimal>,
    raise6sec: Option<rust_decimal::Decimal>,
    lowerreg: Option<rust_decimal::Decimal>,
    raisereg: Option<rust_decimal::Decimal>,
    availability: Option<rust_decimal::Decimal>,
    raise6secflags: Option<rust_decimal::Decimal>,
    raise60secflags: Option<rust_decimal::Decimal>,
    raise5minflags: Option<rust_decimal::Decimal>,
    raiseregflags: Option<rust_decimal::Decimal>,
    lower6secflags: Option<rust_decimal::Decimal>,
    lower60secflags: Option<rust_decimal::Decimal>,
    lower5minflags: Option<rust_decimal::Decimal>,
    lowerregflags: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    semidispatchcap: Option<rust_decimal::Decimal>,
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minUnitsolution3> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: "UNITSOLUTION".into(),
            version: 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minInterconnectorsoln4 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    meteredmwflow: Option<rust_decimal::Decimal>,
    mwflow: Option<rust_decimal::Decimal>,
    mwlosses: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    mnsp: Option<rust_decimal::Decimal>,
    exportlimit: Option<rust_decimal::Decimal>,
    importlimit: Option<rust_decimal::Decimal>,
    marginalloss: Option<rust_decimal::Decimal>,
    exportgenconid: Option<String>,
    importgenconid: Option<String>,
    fcasexportlimit: Option<rust_decimal::Decimal>,
    fcasimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    local_price_adjustment_export: Option<rust_decimal::Decimal>,
    locally_constrained_export: Option<rust_decimal::Decimal>,
    local_price_adjustment_import: Option<rust_decimal::Decimal>,
    locally_constrained_import: Option<rust_decimal::Decimal>,
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minInterconnectorsoln4> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: "INTERCONNECTORSOLN".into(),
            version: 4,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minLocalPrice1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    duid: String,
    local_price_adjustment: Option<rust_decimal::Decimal>,
    locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minLocalPrice1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: "LOCAL_PRICE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minRegionsolution5 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    regionid: String,
    rrp: Option<rust_decimal::Decimal>,
    rop: Option<rust_decimal::Decimal>,
    excessgeneration: Option<rust_decimal::Decimal>,
    raise6secrrp: Option<rust_decimal::Decimal>,
    raise6secrop: Option<rust_decimal::Decimal>,
    raise60secrrp: Option<rust_decimal::Decimal>,
    raise60secrop: Option<rust_decimal::Decimal>,
    raise5minrrp: Option<rust_decimal::Decimal>,
    raise5minrop: Option<rust_decimal::Decimal>,
    raiseregrrp: Option<rust_decimal::Decimal>,
    raiseregrop: Option<rust_decimal::Decimal>,
    lower6secrrp: Option<rust_decimal::Decimal>,
    lower6secrop: Option<rust_decimal::Decimal>,
    lower60secrrp: Option<rust_decimal::Decimal>,
    lower60secrop: Option<rust_decimal::Decimal>,
    lower5minrrp: Option<rust_decimal::Decimal>,
    lower5minrop: Option<rust_decimal::Decimal>,
    lowerregrrp: Option<rust_decimal::Decimal>,
    lowerregrop: Option<rust_decimal::Decimal>,
    totaldemand: Option<rust_decimal::Decimal>,
    availablegeneration: Option<rust_decimal::Decimal>,
    availableload: Option<rust_decimal::Decimal>,
    demandforecast: Option<rust_decimal::Decimal>,
    dispatchablegeneration: Option<rust_decimal::Decimal>,
    dispatchableload: Option<rust_decimal::Decimal>,
    netinterchange: Option<rust_decimal::Decimal>,
    lower5mindispatch: Option<rust_decimal::Decimal>,
    lower5minimport: Option<rust_decimal::Decimal>,
    lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    lower5minlocalreq: Option<rust_decimal::Decimal>,
    lower5minreq: Option<rust_decimal::Decimal>,
    lower60secdispatch: Option<rust_decimal::Decimal>,
    lower60secimport: Option<rust_decimal::Decimal>,
    lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    lower60seclocalreq: Option<rust_decimal::Decimal>,
    lower60secreq: Option<rust_decimal::Decimal>,
    lower6secdispatch: Option<rust_decimal::Decimal>,
    lower6secimport: Option<rust_decimal::Decimal>,
    lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    lower6seclocalreq: Option<rust_decimal::Decimal>,
    lower6secreq: Option<rust_decimal::Decimal>,
    raise5mindispatch: Option<rust_decimal::Decimal>,
    raise5minimport: Option<rust_decimal::Decimal>,
    raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    raise5minlocalreq: Option<rust_decimal::Decimal>,
    raise5minreq: Option<rust_decimal::Decimal>,
    raise60secdispatch: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minRegionsolution5> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: "REGIONSOLUTION".into(),
            version: 5,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minBlockedConstraints1 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    constraintid: String,
}
impl crate::GetTable<P5minBlockedConstraints1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: "BLOCKED_CONSTRAINTS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minConstraintsolution6 {
    #[serde(with = "crate::mms_datetime")]
    run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    constraintid: String,
    rhs: Option<rust_decimal::Decimal>,
    marginalvalue: Option<rust_decimal::Decimal>,
    violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    genconid_effectivedate: Option<chrono::NaiveDateTime>,
    genconid_versionno: Option<rust_decimal::Decimal>,
    lhs: Option<rust_decimal::Decimal>,
    intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<P5minConstraintsolution6> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: "CONSTRAINTSOLUTION".into(),
            version: 6,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendSchedule1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    day_type: String,
    regionid: String,
    periodid: rust_decimal::Decimal,
    energy_rrp: Option<rust_decimal::Decimal>,
    r6_rrp: Option<rust_decimal::Decimal>,
    r60_rrp: Option<rust_decimal::Decimal>,
    r5_rrp: Option<rust_decimal::Decimal>,
    rreg_rrp: Option<rust_decimal::Decimal>,
    l6_rrp: Option<rust_decimal::Decimal>,
    l60_rrp: Option<rust_decimal::Decimal>,
    l5_rrp: Option<rust_decimal::Decimal>,
    lreg_rrp: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendSchedule1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: "MARKET_SUSPEND_SCHEDULE".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapcintervals1 {
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    apcvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    apctype: Option<rust_decimal::Decimal>,
    fcasapcvalue: Option<rust_decimal::Decimal>,
    apfvalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ApRegionapcintervals1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: "REGIONAPCINTERVALS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmevents1 {
    irfmid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    startperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    endperiod: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureIrfmevents1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: "IRFMEVENTS".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegimeSum1 {
    suspension_id: String,
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    start_interval: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    end_interval: Option<chrono::NaiveDateTime>,
    pricing_regime: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendRegimeSum1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: "MARKET_SUSPEND_REGIME_SUM".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmamount1 {
    irfmid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivedate: Option<chrono::NaiveDateTime>,
    versionno: rust_decimal::Decimal,
    periodid: rust_decimal::Decimal,
    amount: Option<rust_decimal::Decimal>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureIrfmamount1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: "IRFMAMOUNT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApeventregion1 {
    apeventid: rust_decimal::Decimal,
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
    energyapflag: Option<rust_decimal::Decimal>,
    raise6secapflag: Option<rust_decimal::Decimal>,
    raise60secapflag: Option<rust_decimal::Decimal>,
    raise5minapflag: Option<rust_decimal::Decimal>,
    raiseregapflag: Option<rust_decimal::Decimal>,
    lower6secapflag: Option<rust_decimal::Decimal>,
    lower60secapflag: Option<rust_decimal::Decimal>,
    lower5minapflag: Option<rust_decimal::Decimal>,
    lowerregapflag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable<ApApeventregion1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: "APEVENTREGION".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureOverriderrp1 {
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    startdate: chrono::NaiveDateTime,
    startperiod: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    endperiod: Option<rust_decimal::Decimal>,
    rrp: Option<rust_decimal::Decimal>,
    description: Option<String>,
    authorisestart: Option<String>,
    authoriseend: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureOverriderrp1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: "OVERRIDERRP".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApevent1 {
    apeventid: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivefrominterval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    effectivetointerval: Option<chrono::NaiveDateTime>,
    reason: Option<String>,
    startauthorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    startauthoriseddate: Option<chrono::NaiveDateTime>,
    endauthorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    endauthoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ApApevent1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: "APEVENT".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendScheduleTrk1 {
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    source_start_date: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    source_end_date: Option<chrono::NaiveDateTime>,
    comments: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendScheduleTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: "MARKET_SUSPEND_SCHEDULE_TRK".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegionSum1 {
    suspension_id: String,
    regionid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    initial_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    end_region_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    end_suspension_interval: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ForceMajeureMarketSuspendRegionSum1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: "MARKET_SUSPEND_REGION_SUM".into(),
            version: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapc1 {
    regionid: String,
    #[serde(with = "crate::mms_datetime")]
    effectivedate: chrono::NaiveDateTime,
    versionno: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    authoriseddate: Option<chrono::NaiveDateTime>,
    authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<ApRegionapc1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: "REGIONAPC".into(),
            version: 1,
        }
    }
}
