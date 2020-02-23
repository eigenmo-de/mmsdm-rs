use crate::{FileKeyable, GetFromRawAemo, RawAemoFile, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Price {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlement_date: chrono::NaiveDateTime,
    run_no: i32,
    region_id: Region,
    dispatch_interval: u64,
    intervention: i32,
    regional_reference_price: f64,
    eep: f64,
    rop: f64,
    apc_flag: i32,
    market_suspended_flag: i32,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    last_changed: chrono::NaiveDateTime,
    raise6sec_rrp: f64,
    raise6sec_rop: f64,
    raise6sec_apc_flag: i32,
    raise60sec_rrp: f64,
    raise60sec_rop: f64,
    raise60sec_apc_flag: i32,
    raise5min_rrp: f64,
    raise5min_rop: f64,
    raise5min_apc_flag: i32,
    raiseregsec_rrp: f64,
    raiseregsec_rop: f64,
    raiseregsec_apc_flag: i32,
    lower6sec_rrp: f64,
    lower6sec_rop: f64,
    lower6sec_apc_flag: i32,
    lower60sec_rrp: f64,
    lower60sec_rop: f64,
    lower60sec_apc_flag: i32,
    lower5min_rrp: f64,
    lower5min_rop: f64,
    lower5min_apc_flag: i32,
    lowerregsec_rrp: f64,
    lowerregsec_rop: f64,
    lowerregsec_apc_flag: i32,
    price_status: String,
    pre_ap_energy_price: f64,
    pre_ap_raise6_price: f64,
    pre_ap_raise60_price: f64,
    pre_ap_raise5min_price: f64,
    pre_ap_raisereg_price: f64,
    pre_ap_lower6_price: f64,
    pre_ap_lower60_price: f64,
    pre_ap_lower5min_price: f64,
    pre_ap_lowerreg_price: f64,
    cumul_pre_ap_energy_price: f64,
    cumul_pre_ap_raise6_price: f64,
    cumul_pre_ap_raise60_price: f64,
    cumul_pre_ap_raise5min_price: f64,
    cumul_pre_ap_raisereg_price: f64,
    cumul_pre_ap_lower6_price: f64,
    cumul_pre_ap_lower60_price: f64,
    cumul_pre_ap_lower5min_price: f64,
    cumul_pre_ap_lowerreg_price: f64,
    ocd_status: String,
    mii_status: String,
}

impl FileKeyable for Price {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "PRICE".into(), 4)
    }
}

impl GetFromRawAemo for Price {
    type Output = Self;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Region {
    TAS1,
    VIC1,
    QLD1,
    NSW1,
    SA1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    case_solution: Vec<CaseSolution>,
    // local_price: Vec<LocalPrice>,
    price: Vec<Price>,
    regionsum: Vec<Regionsum>,
    interconnectorres: Vec<Interconnectorres>,
    constraint: Vec<Constraint>,
    interconnection: Vec<Interconnection>,
}

impl crate::AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            case_solution: CaseSolution::from_map(&mut data)?,
            price: Price::from_map(&mut data)?,
            regionsum: Regionsum::from_map(&mut data)?,
            interconnectorres: Interconnectorres::from_map(&mut data)?,
            constraint: Constraint::from_map(&mut data)?,
            interconnection: Interconnection::from_map(&mut data)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CaseSolution {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime,
    runno: i32,
    intervention: i32,
    casesubtype: Option<String>,
    solutionstatus: Option<i32>,
    spdversion: Option<String>,
    nonphysicallosses: Option<i32>,
    totalobjective: Option<f64>,
    totalareagenviolation: Option<f64>,
    totalinterconnectorviolation: Option<f64>,
    totalgenericviolation: Option<f64>,
    totalramprateviolation: Option<f64>,
    totalunitmwcapacityviolation: Option<f64>,
    total5minviolation: Option<f64>,
    totalregviolation: Option<f64>,
    total6secviolation: Option<f64>,
    total60secviolation: Option<f64>,
    totalasprofileviolation: Option<f64>,
    totalfaststartviolation: Option<f64>,
    totalenergyofferviolation: Option<f64>,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
}

impl FileKeyable for CaseSolution {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "CASE_SOLUTION".into(), 2)
    }
}

impl GetFromRawAemo for CaseSolution {
    type Output = Self;
}

// struct LocalPrice {
//     settlementdate: chrono::NaiveDateTime,
//     duid: String,
//     local_price_adjustment:	f64,
//     locally_constrained: i32,
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Regionsum {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime,
    runno: i64,
    regionid: String,
    dispatchinterval: i64,
    intervention: i64,
    totaldemand: f64,
    availablegeneration: f64,
    availableload: f64,
    demandforecast: f64,
    dispatchablegeneration: f64,
    dispatchableload: Option<f64>,
    netinterchange: f64,
    excessgeneration: Option<f64>,
    lower5mindispatch: Option<f64>,
    lower5minimport: Option<f64>,
    lower5minlocaldispatch: f64,
    lower5minlocalprice: Option<f64>,
    lower5minlocalreq: Option<f64>,
    lower5minprice: Option<f64>,
    lower5minreq: Option<f64>,
    lower5minsupplyprice: Option<f64>,
    lower60secdispatch: Option<f64>,
    lower60secimport: Option<f64>,
    lower60seclocaldispatch: f64,
    lower60seclocalprice: Option<f64>,
    lower60seclocalreq: Option<f64>,
    lower60secprice: Option<f64>,
    lower60secreq: Option<f64>,
    lower60secsupplyprice: Option<f64>,
    lower6secdispatch: Option<f64>,
    lower6secimport: Option<f64>,
    lower6seclocaldispatch: f64,
    lower6seclocalprice: Option<f64>,
    lower6seclocalreq: Option<f64>,
    lower6secprice: Option<f64>,
    lower6secreq: Option<f64>,
    lower6secsupplyprice: Option<f64>,
    raise5mindispatch: Option<f64>,
    raise5minimport: Option<f64>,
    raise5minlocaldispatch: f64,
    raise5minlocalprice: Option<f64>,
    raise5minlocalreq: Option<f64>,
    raise5minprice: Option<f64>,
    raise5minreq: Option<f64>,
    raise5minsupplyprice: Option<f64>,
    raise60secdispatch: Option<f64>,
    raise60secimport: Option<f64>,
    raise60seclocaldispatch: f64,
    raise60seclocalprice: Option<f64>,
    raise60seclocalreq: Option<f64>,
    raise60secprice: Option<f64>,
    raise60secreq: Option<f64>,
    raise60secsupplyprice: Option<f64>,
    raise6secdispatch: Option<f64>,
    raise6secimport: Option<f64>,
    raise6seclocaldispatch: f64,
    raise6seclocalprice: Option<f64>,
    raise6seclocalreq: Option<f64>,
    raise6secprice: Option<f64>,
    raise6secreq: Option<f64>,
    raise6secsupplyprice: Option<f64>,
    aggegatedispatcherror: Option<f64>,
    aggregatedispatcherror: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    initialsupply: f64,
    clearedsupply: f64,
    lowerregimport: Option<f64>,
    lowerreglocaldispatch: f64,
    lowerreglocalreq: Option<f64>,
    lowerregreq: Option<f64>,
    raiseregimport: Option<f64>,
    raisereglocaldispatch: f64,
    raisereglocalreq: Option<f64>,
    raiseregreq: Option<f64>,
    raise5minlocalviolation: Option<f64>,
    raisereglocalviolation: Option<f64>,
    raise60seclocalviolation: Option<f64>,
    raise6seclocalviolation: Option<f64>,
    lower5minlocalviolation: Option<f64>,
    lowerreglocalviolation: Option<f64>,
    lower60seclocalviolation: Option<f64>,
    lower6seclocalviolation: Option<f64>,
    raise5minviolation: Option<f64>,
    raiseregviolation: Option<f64>,
    raise60secviolation: Option<f64>,
    raise6secviolation: Option<f64>,
    lower5minviolation: Option<f64>,
    lowerregviolation: Option<f64>,
    lower60secviolation: Option<f64>,
    lower6secviolation: Option<f64>,
    raise6secactualavailability: f64,
    raise60secactualavailability: f64,
    raise5minactualavailability: f64,
    raiseregactualavailability: f64,
    lower6secactualavailability: f64,
    lower60secactualavailability: f64,
    lower5minactualavailability: f64,
    lowerregactualavailability: f64,
    lorsurplus: Option<f64>,
    lrcsurplus: Option<f64>,
    totalintermittentgeneration: f64,
    demand_and_nonschedgen: f64,
    uigf: f64,
    semischedule_clearedmw: f64,
    semischedule_compliancemw: f64,
}

impl FileKeyable for Regionsum {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "REGIONSUM".into(), 4)
    }
}

impl GetFromRawAemo for Regionsum {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Interconnectorres {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime,
    runno: i64,
    interconnectorid: String,
    dispatchinterval: i64,
    intervention: i64,
    meteredmwflow: Option<f64>,
    mwflow: Option<f64>,
    mwlosses: Option<f64>,
    marginalvalue: Option<f64>,
    violationdegree: Option<f64>,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    exportlimit: Option<f64>,
    importlimit: Option<f64>,
    marginalloss: Option<f64>,
    exportgenconid: Option<String>,
    importgenconid: Option<String>,
    fcasexportlimit: Option<f64>,
    fcasimportlimit: Option<f64>,
    local_price_adjustment_export: Option<f64>,
    locally_constrained_export: Option<i32>,
    local_price_adjustment_import: Option<f64>,
    locally_constrained_import: Option<i32>,
}

impl FileKeyable for Interconnectorres {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "INTERCONNECTORRES".into(), 3)
    }
}

impl GetFromRawAemo for Interconnectorres {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Constraint {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime,
    runno: i64,
    constraintid: String,
    dispatchinterval: i64,
    intervention: i64,
    rhs: f64,
    marginalvalue: f64,
    violationdegree: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    duid: String,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    genconid_effectivedate: chrono::NaiveDateTime,
    genconid_versionno: i64,
    lhs: f64,
}

impl FileKeyable for Constraint {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "CONSTRAINT".into(), 5)
    }
}

impl GetFromRawAemo for Constraint {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Interconnection {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime,
    runno: i64,
    intervention: i64,
    from_regionid: String,
    to_regionid: String,
    dispatchinterval: i64,
    irlf: f64,
    mwflow: f64,
    meteredmwflow: f64,
    from_region_mw_losses: f64,
    to_region_mw_losses: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
}

impl FileKeyable for Interconnection {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "INTERCONNECTION".into(), 1)
    }
}

impl GetFromRawAemo for Interconnection {
    type Output = Self;
}
