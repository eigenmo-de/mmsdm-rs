use serde::{Deserialize, Serialize};
use crate::{Result, GetFromRawAemo, FileKeyable, RawAemoFile};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: crate::AemoHeader,
    price: Vec<BidDayOffer>,
    volume: Vec<BidPerOffer>,
}


impl crate::AemoFile for File {
    fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            price: BidDayOffer::from_map(&mut data)?,
            volume: BidPerOffer::from_map(&mut data)?,
        }) 
    }
}

impl File {
    pub fn get_price(&self) -> &'_ Vec<BidDayOffer> {
        &self.price
    }
}



#[derive(Clone, Debug, Deserialize, Serialize)]
enum BidType {
    RAISEREG,
    RAISE6SEC,
    RAISE60SEC,
    RAISE5MIN,
    ENERGY,
    LOWERREG,
    LOWER6SEC,
    LOWER60SEC,
    LOWER5MIN,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum EntryType {
    DAILY,
    REBID,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BidDayOffer {
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlement_date: chrono::NaiveDateTime,
    duid: String,
    bid_type: Option<BidType>,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    bid_settlement_date:  chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    bid_offer_date: chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::opt_au_datetime_deserialize")]
    first_dispatch: Option<chrono::NaiveDateTime>,
    #[serde(deserialize_with = "crate::opt_au_datetime_deserialize")]
    first_predispatch: Option<chrono::NaiveDateTime>,
    daily_energy_constraint: Option<i32>,
    rebid_explanation: String,
    pub price_band_1: f64,
    price_band_2: f64,
    price_band_3: f64,
    price_band_4: f64,
    price_band_5: f64,
    price_band_6: f64,
    price_band_7: f64,
    price_band_8: f64,
    price_band_9: f64,
    price_band_10: f64,
    minimum_load: Option<i32>,
    t1: Option<i32>,
    t2: Option<i32>,
    t3: Option<i32>,
    t4: Option<i32>,
    normal_status: Option<i32>,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    last_changed: chrono::NaiveDateTime,
    bid_version_no: i32,
    mr_factor: Option<i32>,
    entry_type: Option<EntryType>,
}

impl FileKeyable for BidDayOffer {
    fn key() -> crate::FileKey {
        ("YESTBID".into(), "BIDDAYOFFER".into(), 5)
    }
}

impl GetFromRawAemo for BidDayOffer {
    type Output = Self;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BidPerOffer {
#[serde(deserialize_with = "crate::au_datetime_deserialize")]
    settlement_date: chrono::NaiveDateTime,
    duid: String,
    bid_type: BidType,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    bid_settlement_date:  chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    bid_offer_date: chrono::NaiveDateTime,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    trading_period: chrono::NaiveDateTime,
    max_availability: i32,
    fixed_load: Option<i32>,
    roc_up: Option<i32>,
    roc_down: Option<i32>,
    enablement_min: i32,
    enablement_max: i32,
    low_break_point: i32,
    high_break_point: i32,
    band_availability_1: i32,
    band_availability_2: i32,
    band_availability_3: i32,
    band_availability_4: i32,
    band_availability_5: i32,
    band_availability_6: i32,
    band_availability_7: i32,
    band_availability_8: i32,
    band_availability_9: i32,
    band_availability_10: i32,
    pasa_availability: Option<i32>,
    period_id: i32,
    #[serde(deserialize_with = "crate::au_datetime_deserialize")]
    last_changed: chrono::NaiveDateTime,
    bid_version_no: i32,
    mr_capacity: Option<i32>,
}

impl FileKeyable for BidPerOffer {
    fn key() -> crate::FileKey {
        ("YESTBID".into(), "BIDPEROFFER".into(), 3)
    }
}

impl GetFromRawAemo for BidPerOffer {
    type Output = Self;
}
