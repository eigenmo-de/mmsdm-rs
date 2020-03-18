use crate::{FileKeyable, GetFromRawAemo, RawAemoFile, Result, mms_datetime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    setcpdata: Vec<SetCpData>,
}

impl crate::AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            setcpdata: SetCpData::from_map(&mut data)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SetCpData {
    #[serde(with = "mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    versionno: i32,
    periodid: i32,
    participantid: String,
    tcpid: String,
    regoinid: String,
    igenergy: f64,
    xgenergy: f64,
    inenergy: f64,
    xnenergy: f64,
    ipower: f64,
    xpower: f64,
    rrp: f64,
    eep: f64,
    tlf: f64,
    cprrp: f64,
    cpeep: f64,
    ta: f64,
    ep: f64,
    apc: Option<f64>,
    resc: Option<f64>,
    resp: Option<f64>,
    meterrunno: i32,
    #[serde(with = "mms_datetime")]
    lastchanged: chrono::NaiveDateTime,
    hostdistributor: Option<String>,
    mda: String,
}

impl FileKeyable for SetCpData {
    fn key() -> crate::FileKey {
        ("SETTLEMENTS".into(), "SETCPDATA".into(), 1)
    }
}

impl GetFromRawAemo for SetCpData {
    type Output = Self;
}
