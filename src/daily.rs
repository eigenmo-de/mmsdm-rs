use serde::{Deserialize, Serialize};
use crate::{Result, GetFromRawAemo, FileKeyable, RawAemoFile, AemoFile};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    case_solution: Vec<CaseSolution>,
}


impl AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            case_solution: CaseSolution::from_map(&mut data)?,
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
    solutionstatus: i32, 
    spdversion: Option<String>, 							
    nonphysicallosses: i32, 
    totalobjective: f64, 
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
        ("DISPATCH".into(), "CASESOLUTION".into(), 1)
    }
}

impl GetFromRawAemo for CaseSolution {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DRegion {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime, 
    runno: i32, 
    regionid: String, 
    intervention: i32, 
    rrp: f64, 
    eep: f64, 
    rop: f64, 
    apcflag: i32, 
    marketsuspendedflag: i32, 
    totaldemand: f64,
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
    aggregatedispatcherror: f64, 
    availablegeneration: f64, 
    availableload: f64, 
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
    raise6secrrp: f64, 
    raise6secrop: f64, 
    raise6secapcflag: i32, 
    raise60secrrp: f64, 
    raise60secrop: f64, 
    raise60secapcflag: i32,
    raise5minrrp: f64, 
    raise5minrop: f64, 
    raise5minapcflag: i32, 
    raiseregrrp: f64, 
    raiseregrop: f64, 
    raiseregapcflag: i32, 
    lower6secrrp: f64, 
    lower6secrop: f64, 
    lower6secapcflag: i32, 
    lower60secrrp:f64, 
    lower60secrop:f64, 
    lower60secapcflag: i32, 
    lower5minrrp: f64, 
    lower5minrop: f64, 
    lower5minapcflag: i32, 
    lowerregrrp: f64, 
    lowerregrop: f64, 
    lowerregapcflag: i32, 
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
}

impl FileKeyable for DRegion {
    fn key() -> crate::FileKey {
        ("DREGION".into(), "".into(), 3)
    }
}

impl GetFromRawAemo for DRegion {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DUnit {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime, 
    runno: i32,
    duid: String, 
    intervention: i32, 
    dispatchmode: i32, 
    agcstatus: i32, 
    initialmw: f64, 
    totalcleared: f64, 
    rampdownrate: f64, 
    rampuprate: f64, 
    lower5min: f64, 
    lower60sec: f64, 
    lower6sec: f64, 
    raise5min: f64, 
    raise60sec: f64, 
    raise6sec: f64, 
    marginal5minvalue: Option<f64>, 				
    marginal60secvalue: Option<f64>, 				
    marginal6secvalue: Option<f64>, 				
    marginalvalue: Option<f64>, 				
    violation5mindegree: Option<f64>,				
    violation60secdegree: Option<f64>,				
    violation6secdegree: Option<f64>,				
    violationdegree: Option<f64>,				
    lowerreg: f64, 
    raisereg: f64, 
    availability: f64, 
    raise6secflags: i32, 	
    raise60secflags: i32, 	
    raise5minflags: i32, 	
    raiseregflags: i32, 	
    lower6secflags: i32, 	
    lower60secflags: i32, 	
    lower5minflags: i32, 	
    lowerregflags: i32, 	
    raiseregavailability: f64, 
    raiseregenablementmax: f64, 
    raiseregenablementmin: f64, 
    lowerregavailability: f64, 
    lowerregenablementmax: f64, 
    lowerregenablementmin: f64, 
    raise6secactualavailability: f64, 
    raise60secactualavailability: f64, 
    raise5minactualavailability: f64, 
    raiseregactualavailability: f64, 
    lower6secactualavailability: f64, 
    lower60secactualavailability: f64, 
    lower5minactualavailability: f64, 
    lowerregactualavailability: f64, 
    semidispatchcap: i32, 
}

impl FileKeyable for DUnit {
    fn key() -> crate::FileKey {
        ("DUNIT".into(), "".into(), 3)
    }
}

impl GetFromRawAemo for DUnit {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RegionFcasRequirement {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime,
    runno: i32,
    intervention: i32,
    genconid: String, 
    regionid: String,
    bidtype: String,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    genconeffectivedate: chrono::NaiveDateTime,
    genconversionno: i32, 
    marginalvalue: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
}

impl FileKeyable for RegionFcasRequirement {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "REGIONFCASREQUIREMENT".into(), 1)
    }
}

impl GetFromRawAemo for RegionFcasRequirement {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TRegion {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime, 
    runno: i32, 
    regionid: String, 
    rrp: f64, 
    eep: f64, 
    invalidflag: String, 
    totaldemand: f64, 
    demandforecast: f64, 
    dispatchablegeneration: f64, 
    dispatchableload: f64, 
    netinterchange: f64, 
    excessgeneration:f64,
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
    availablegeneration: f64, 
    availableload: f64, 
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
    raise6secrrp: f64, 
    raise60secrrp: f64, 
    raise5minrrp: f64, 
    raiseregrrp: f64, 
    lower6secrrp: f64, 
    lower60secrrp: f64, 
    lower5minrrp: f64, 
    lowerregrrp: f64, 
}

impl FileKeyable for TRegion {
    fn key() -> crate::FileKey {
        ("TREGION".into(), "".into(), 2)
    }
}

impl GetFromRawAemo for TRegion {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TUnit {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlementdate: chrono::NaiveDateTime, 
    runno: i32, 
    duid: String, 
    initialmw: f64, 
    totalcleared: f64, 
    rampdownrate: f64, 
    rampuprate: f64, 
    lower5min: f64, 
    lower60sec: f64, 
    lower6sec: f64, 
    raise5min: f64, 
    raise60sec: f64, 
    raise6sec: f64, 
    lowerreg: f64, 
    raisereg: f64, 
    availability: f64, 
}

impl FileKeyable for TUnit {
    fn key() -> crate::FileKey {
        ("TUNIT".into(), "".into(), 2)
    }
}

impl GetFromRawAemo for TUnit {
    type Output = Self;
}

