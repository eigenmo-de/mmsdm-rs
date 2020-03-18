use crate::{AemoFile, FileKeyable, GetFromRawAemo, RawAemoFile, Result, mms_datetime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    forecast: Vec<RooftopForecast>,
}

impl AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            forecast: RooftopForecast::from_map(&mut data)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RooftopForecast {
    #[serde(with = "mms_datetime")]
    version_datetime: chrono::NaiveDateTime,
    regionid: String,
    #[serde(with = "mms_datetime")]
    interval_datetime: chrono::NaiveDateTime,
    powermean: f64,
    powerpoe50: f64,
    powerpoelow: f64,
    powerpoehigh: f64,
    #[serde(with = "mms_datetime")]
    lastchanged: chrono::NaiveDateTime,
}

impl FileKeyable for RooftopForecast {
    fn key() -> crate::FileKey {
        ("ROOFTOP".into(), "FORECAST".into(), 1)
    }
}

impl GetFromRawAemo for RooftopForecast {
    type Output = Self;
}
