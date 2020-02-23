use crate::{AemoFile, FileKeyable, GetFromRawAemo, RawAemoFile, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    price_sensitivities: Vec<PriceSensitivities>,
    interconnector_sensitivities: Vec<InterconnectorSensitivities>,
}

impl AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            price_sensitivities: PriceSensitivities::from_map(&mut data)?,
            interconnector_sensitivities: InterconnectorSensitivities::from_map(&mut data)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InterconnectorSensitivities {
    predispatchseqno: String,
    runno: i32,
    interconnectorid: String,
    periodid: String,
    intervention: i32,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    datetime: chrono::NaiveDateTime,
    intervention_active: i32,
    mwflow1: f64,
    mwflow2: f64,
    mwflow3: f64,
    mwflow4: f64,
    mwflow5: f64,
    mwflow6: f64,
    mwflow7: f64,
    mwflow8: f64,
    mwflow9: f64,
    mwflow10: f64,
    mwflow11: f64,
    mwflow12: f64,
    mwflow13: f64,
    mwflow14: f64,
    mwflow15: f64,
    mwflow16: f64,
    mwflow17: f64,
    mwflow18: f64,
    mwflow19: f64,
    mwflow20: f64,
    mwflow21: f64,
    mwflow22: f64,
    mwflow23: f64,
    mwflow24: f64,
    mwflow25: f64,
    mwflow26: f64,
    mwflow27: f64,
    mwflow28: f64,
    mwflow29: f64,
    mwflow30: f64,
    mwflow31: f64,
    mwflow32: f64,
    mwflow33: f64,
    mwflow34: f64,
    mwflow35: f64,
    mwflow36: f64,
    mwflow37: f64,
    mwflow38: f64,
    mwflow39: f64,
    mwflow40: f64,
    mwflow41: f64,
    mwflow42: f64,
    mwflow43: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
}

impl FileKeyable for InterconnectorSensitivities {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "INTERCONNECTR_SENS".into(), 1)
    }
}

impl GetFromRawAemo for InterconnectorSensitivities {
    type Output = Self;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PriceSensitivities {
    predispatchseqno: String,
    runno: i32,
    regionid: String,
    periodid: String,
    intervention: i32,
    rrpeep1: f64,
    rrpeep2: f64,
    rrpeep3: f64,
    rrpeep4: f64,
    rrpeep5: f64,
    rrpeep6: f64,
    rrpeep7: f64,
    rrpeep8: f64,
    rrpeep9: f64,
    rrpeep10: f64,
    rrpeep11: f64,
    rrpeep12: f64,
    rrpeep13: f64,
    rrpeep14: f64,
    rrpeep15: f64,
    rrpeep16: f64,
    rrpeep17: f64,
    rrpeep18: f64,
    rrpeep19: f64,
    rrpeep20: f64,
    rrpeep21: f64,
    rrpeep22: f64,
    rrpeep23: f64,
    rrpeep24: f64,
    rrpeep25: f64,
    rrpeep26: f64,
    rrpeep27: f64,
    rrpeep28: f64,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    datetime: chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    lastchanged: chrono::NaiveDateTime,
    rrpeep29: f64,
    rrpeep30: f64,
    rrpeep31: f64,
    rrpeep32: f64,
    rrpeep33: f64,
    rrpeep34: f64,
    rrpeep35: f64,
    intervention_active: i32,
    rrpeep36: f64,
    rrpeep37: f64,
    rrpeep38: f64,
    rrpeep39: f64,
    rrpeep40: f64,
    rrpeep41: f64,
    rrpeep42: f64,
    rrpeep43: f64,
}

impl FileKeyable for PriceSensitivities {
    fn key() -> crate::FileKey {
        ("PREDISPATCH".into(), "PRICESENSITIVITIES".into(), 1)
    }
}

impl GetFromRawAemo for PriceSensitivities {
    type Output = Self;
}
