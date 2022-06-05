#![deny(clippy::all)]
#![deny(warnings)]

pub mod data_model;

pub use mmsdm_core::*;

#[cfg(feature = "sql_server")]
pub mod sql_server;
