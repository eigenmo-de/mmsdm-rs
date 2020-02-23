use crate::{AemoFile, FileKeyable, GetFromRawAemo, RawAemoFile, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    case_solution: Vec<CaseSolution>,
    local_price: Vec<LocalPrice>,
    region_prices: Vec<RegionPrices>,
    region_solution: Vec<RegionSolution>,
    interconnector_soln: Vec<InterconnectorSolution>,
    constraint_solution: Vec<ConstraintSolution>,
}

impl AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            case_solution: CaseSolution::from_map(&mut data)?,
            local_price: LocalPrice::from_map(&mut data)?,
            region_prices: RegionPrices::from_map(&mut data)?,
            region_solution: RegionSolution::from_map(&mut data)?,
            interconnector_soln: InterconnectorSolution::from_map(&mut data)?,
            constraint_solution: ConstraintSolution::from_map(&mut data)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CaseSolution {
    predispatchseqno: String,
    runno: i32,
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
    totalenergyconstrviolation: Option<f64>,
    totalenergyofferviolation: Option<f64>,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    intervention: Option<i32>,
}

impl FileKeyable for CaseSolution {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "CASE_SOLUTION".into(), 1)
    }
}

impl GetFromRawAemo for CaseSolution {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocalPrice {
    predispatchseqno: String,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    datetime: chrono::NaiveDateTime,
    duid: String,
    periodid: i32,
    local_price_adjustment: f64,
    locally_constrained: i32,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
}

impl FileKeyable for LocalPrice {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "LOCAL_PRICE".into(), 1)
    }
}

impl GetFromRawAemo for LocalPrice {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RegionPrices {
    predispatchseqno: String,
    runno: i32,
    regionid: String,
    periodid: String,
    intervention: i32,
    rrp: f64,
    eep: f64,
    rrp1: Option<f64>,
    eep1: Option<f64>,
    rrp2: Option<f64>,
    eep2: Option<f64>,
    rrp3: Option<f64>,
    eep3: Option<f64>,
    rrp4: Option<f64>,
    eep4: Option<f64>,
    rrp5: Option<f64>,
    eep5: Option<f64>,
    rrp6: Option<f64>,
    eep6: Option<f64>,
    rrp7: Option<f64>,
    eep7: Option<f64>,
    rrp8: Option<f64>,
    eep8: Option<f64>,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    datetime: chrono::NaiveDateTime,
    raise6secrrp: f64,
    raise60secrrp: f64,
    raise5minrrp: f64,
    raiseregrrp: f64,
    lower6secrrp: f64,
    lower60secrrp: f64,
    lower5minrrp: f64,
    lowerregrrp: f64,
}

impl FileKeyable for RegionPrices {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "REGION_PRICES".into(), 1)
    }
}

impl GetFromRawAemo for RegionPrices {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RegionSolution {
    predispatchseqno: String,
    runno: i32,
    regionid: String,
    periodid: String,
    intervention: i32,
    totaldemand: f64,
    availablegeneration: f64,
    availableload: f64,
    demandforecast: f64,
    dispatchablegeneration: f64,
    dispatchableload: f64,
    netinterchange: f64,
    excessgeneration: f64,
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
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    datetime: chrono::NaiveDateTime,
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
    decavailability: f64,
    lorsurplus: Option<f64>,
    lrcsurplus: Option<f64>,
    totalintermittentgeneration: f64,
    demand_and_nonschedgen: f64,
    uigf: f64,
    semischedule_clearedmw: f64,
    semischedule_compliancemw: f64,
}

impl FileKeyable for RegionSolution {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "REGION_SOLUTION".into(), 4)
    }
}

impl GetFromRawAemo for RegionSolution {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InterconnectorSolution {
    predispatchseqno: String,
    runno: i32,
    interconnectorid: String,
    periodid: String,
    intervention: i32,
    meteredmwflow: f64,
    mwflow: f64,
    mwlosses: f64,
    marginalvalue: f64,
    violationdegree: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    datetime: chrono::NaiveDateTime,
    exportlimit: f64,
    importlimit: f64,
    marginalloss: f64,
    exportgenconid: String,
    importgenconid: String,
    fcasexportlimit: f64,
    fcasimportlimit: f64,
    local_price_adjustment_export: f64,
    locally_constrained_export: i32,
    local_price_adjustment_import: f64,
    locally_constrained_import: i32,
}

impl FileKeyable for InterconnectorSolution {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "INTERCONNECTOR_SOLN".into(), 3)
    }
}

impl GetFromRawAemo for InterconnectorSolution {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConstraintSolution {
    predispatchseqno: String,
    runno: i32,
    constraintid: String,
    periodid: String,
    intervention: i32,
    rhs: f64,
    marginalvalue: f64,
    violationdegree: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    datetime: chrono::NaiveDateTime,
    duid: Option<String>,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    genconid_effectivedate: chrono::NaiveDateTime,
    genconid_versionno: i64,
    lhs: f64,
}

impl FileKeyable for ConstraintSolution {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "CONSTRAINT_SOLUTION".into(), 5)
    }
}

impl GetFromRawAemo for ConstraintSolution {
    type Output = Self;
}
