#![no_std]
#![deny(clippy::all)]
//#![deny(warnings)]

pub mod data_model;

pub use mmsdm_core::*;

#[cfg(feature = "chrono-tz")]
// this is useful to get the datetime part of nem settlementdate / lastchanged fields
pub fn to_nem_datetime(ndt: &chrono::NaiveDateTime) -> chrono::DateTime<chrono_tz::Tz> {
    chrono_tz::Australia::Brisbane
        .from_local_datetime(ndt)
        .unwrap()
}

#[cfg(feature = "chrono-tz")]
// this is useful to get the date part of nem settlementdate / lastchanged fields
pub fn to_nem_date(ndt: &chrono::NaiveDateTime) -> chrono::NaiveDate {
    to_nem_datetime(ndt).date_naive()
}
