use serde::{Deserialize, Serialize};
use crate::aemo::{self, Result, GetFromRawAemo, FileKeyable, RawAemoFile};


#[derive(Deserialize, Serialize, Debug, Clone)]
struct Price {
    #[serde(deserialize_with = "aemo::au_datetime_deserialize")]
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
    #[serde(deserialize_with = "aemo::au_datetime_deserialize")]
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
    fn key() -> aemo::FileKey {
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

// impl std::fmt::Display for Region {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     match self {
//         Self::MissingHeaderRecord =>  write!(f, "aemo file is missing the first `c` record"),
//     }

// }

// struct CaseSolution {}
// struct LocalPrice {}
// struct Price {}
// struct Regionsum {}
// struct Interconnectorres {}
// struct Constraint {}
// struct Interconnection {}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    header: aemo::AemoHeader,
    // case_solution: Vec<CaseSolution>,
    // local_price: Vec<LocalPrice>,
    price: Vec<Price>,
    // regionsum: Vec<Regionsum>,
    // interconnectorres: Vec<Interconnectorres>,
    // constraint: Vec<Constraint>,
    // interconection: Vec<Interconnection>,
}


impl File {
    pub fn from_raw(RawAemoFile { header, mut data }: RawAemoFile) -> Result<Self> {
        Ok(Self {
            header,
            price: Price::from_map(&mut data)?,
        }) 
    }

    pub fn to_block(self) -> clickhouse_rs::types::Block {
        clickhouse_rs::types::Block::new()
            .column("settlement_date", self.price.iter()
                .map(|pr| aemo::to_nem_date(&pr.settlement_date))
                .collect::<Vec<_>>() 
            )
            .column("run_no", self.price.iter().map(|pr| pr.run_no).collect::<Vec<_>>()  )
            .column("region_id", self.price.iter().map(|pr| serde_json::to_string(&pr.region_id).unwrap()).collect::<Vec<_>>() )
            .column("dispatch_interval", self.price.iter().map(|pr| pr.dispatch_interval).collect::<Vec<_>>() )
            .column("intervention", self.price.iter().map(|pr| pr.intervention).collect::<Vec<_>>() )
            .column("rrp", self.price.iter().map(|pr| pr.regional_reference_price).collect::<Vec<_>>() )
    }
}
