use serde::{Deserialize, Serialize};
use heck::{TitleCase, CamelCase, ShoutySnakeCase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    name: String,
    sub_type: String,
    version: u8,
    transaction_type: String,
    row_filter: String,
}

impl Report {
    pub fn get_doc(&self) -> String {
        format!(
            r#"* Data Set Name: {mms_data_set_name}
* File Name: {mms_file_name}
* Data Version: {version}"#,
            mms_data_set_name = self.name.to_title_case(),
            mms_file_name = self.sub_type.to_title_case(),
            version = self.version,
        )
    }
    pub fn struct_name(&self) -> String {
        format!(
            "{}{}{}",
            self.name.to_camel_case(),
            self.sub_type.to_camel_case(),
            self.version
        )
    }
    pub fn file_key_literal(&self) -> String {
        format!(
            r#"
            crate::FileKey {{
                data_set_name: "{data_set}".into(),
                table_name: "{table}".into(),
                version: {version},
            }}
            "#,
            data_set = self.name.to_shouty_snake_case(),
            table = self.sub_type.to_shouty_snake_case(),
            version = self.version,
        )
    }
}
