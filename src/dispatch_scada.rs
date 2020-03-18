use crate::{AemoFile, FileKeyable, GetFromRawAemo, RawAemoFile, Result, mms_datetime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    scada: Vec<UnitScada>,
}

impl AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            scada: UnitScada::from_map(&mut data)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UnitScada {
    #[serde(with = "mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    scadavalue: f64,
}

impl FileKeyable for UnitScada {
    fn key() -> crate::FileKey {
        ("DISPATCH".into(), "UNIT_SCADA".into(), 1)
    }
}

impl GetFromRawAemo for UnitScada {
    type Output = Self;
}
