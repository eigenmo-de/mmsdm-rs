use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub name: String,
    pub sub_type: Option<String>,
    pub version: u8,
    pub transaction_type: String,
    pub row_filter: String,
}
