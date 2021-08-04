use serde::{Deserialize, Serialize};
use heck::CamelCase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub name: String,
    pub sub_type: Option<String>,
    pub version: u8,
    pub transaction_type: String,
    pub row_filter: String,
}
impl Report {
    pub fn sql_table_name(&self) -> String {
        format!(
            "{}{}{}",
            self.name.to_camel_case(),
            if let Some(sub_type) = &self.sub_type {
                sub_type
            } else {
                ""
            }
            .to_camel_case(),
            self.version
        )
    }
}
