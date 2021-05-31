use heck::{CamelCase, ShoutySnakeCase, TitleCase};
use std::{collections, fs, str};

use crate::{mms, pdr};

impl mms::DataType {
    fn as_rust_type(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } => "String",
            mms::DataType::Char => "char",
            mms::DataType::Date => "chrono::NaiveDateTime",
            mms::DataType::DateTime => "chrono::NaiveDateTime",
            mms::DataType::Decimal { .. } => "rust_decimal::Decimal",
            mms::DataType::Integer { .. } => "i64",
        }
        .into()
    }
}

impl mms::TableColumn {
    fn to_rust_type(&self) -> String {
        if self.mandatory {
            format!("{}", self.data_type.as_rust_type())
        } else {
            format!("Option<{}>", self.data_type.as_rust_type())
        }
    }
    fn get_comment(&self) -> &str {
        &self.comment
    }
}

impl mms::TableNote {
    fn get_doc(&self) -> String {
        format!("* ({}) {} {}", self.name, self.comment, self.value)
    }
}

impl mms::TableNotes {
    fn get_doc(&self) -> String {
        format!(
            "# Notes\n {}",
            self.notes
                .iter()
                .map(|n| n.get_doc())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl mms::Description {
    fn get_doc(&self) -> String {
        format!("# Description\n {}", self.inner)
    }
}

impl mms::PkColumns {
    fn get_doc(&self) -> String {
        self.cols
            .iter()
            .map(|c| format!("* {}", c))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl mms::TableSummary {
    fn get_doc(&self) -> String {
        format!("## {}\n _{}_", self.name, self.comment)
    }
}

impl mms::TablePage {
    pub fn get_doc(&self, report: &pdr::Report) -> String {
        //use heck::TitleCase;
        format!(
            r#"# Summary

{summary}

{pdr_report}

{description_opt}

{notes_opt}

# Primary Key Columns

{primary_key}
"#,
            summary = self.summary.get_doc(),
            pdr_report = report.get_doc(),
            description_opt = self
                .description
                .as_ref()
                .map(|d| d.get_doc())
                .unwrap_or_else(|| "".into()),
            notes_opt = self
                .notes
                .as_ref()
                .map(|n| n.get_doc())
                .unwrap_or_else(|| "".into()),
            primary_key = self.primary_key_columns.get_doc(),
        )
    }
}

impl pdr::Report {
    fn get_doc(&self) -> String {
        format!(
            r#"* Data Set Name: {mms_data_set_name}
* File Name: {mms_file_name}
* Data Version: {version}"#,
            mms_data_set_name = self.name.to_title_case(),
            mms_file_name = if let Some(sub_type) = &self.sub_type {
                sub_type
            } else {
                ""
            }
            .to_title_case(),
            version = self.version,
        )
    }
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
    pub fn struct_name(&self) -> String {
        if let Some(sub_type) = &self.sub_type {
            format!(
                "{}{}{}",
                self.name.to_camel_case(),
                sub_type.to_camel_case(),
                self.version
            )
        } else {
            format!("{}{}", self.name.to_camel_case(), self.version)
        }
    }
    pub fn file_key_literal(&self) -> String {
        format!(
            r#"
            crate::FileKey {{
                data_set_name: "{data_set}".into(),
                table_name: Some("{table}".into()),
                version: {version},
            }}
            "#,
            data_set = self.name.to_shouty_snake_case(),
            table = if let Some(sub_type) = &self.sub_type {
                sub_type
            } else {
                ""
            }
            .to_shouty_snake_case(),
            version = self.version,
        )
    }
}

fn should_skip(report: &mms::Report) -> bool {
    // skip historical dataset - there are no table definitions anyway
    (report.name == "HISTORICAL")
    || (report.name == "CONFIGURATION")

    // temporary
    || (report.name == "BIDS" && report.sub_type == "BIDPEROFFER_D")
    || (report.name == "BIDS" && report.sub_type == "BIDDAYOFFER_D")
    || (report.name == "DEMAND_FORECASTS" && report.sub_type == "INTERMITTENT_P5_RUN")
    || (report.name == "DEMAND_FORECASTS" && report.sub_type == "INTERMITTENT_P5_PRED")
    || (report.name == "BILLING_RUN" && report.sub_type == "BILLINGAPCCOMPENSATION")
    || (report.name == "BILLING_RUN" && report.sub_type == "BILLINGAPCRECOVERY")
    || (report.name == "BILLING_RUN" && report.sub_type == "BILLING_RES_TRADER_RECOVERY")
    || (report.name == "BILLING_RUN" && report.sub_type == "BILLING_RES_TRADER_PAYMENT")
    || (report.name == "SETTLEMENT_DATA" && report.sub_type == "SETRESERVERECOVERY")
}

pub fn run() -> anyhow::Result<()> {
    let rdr = fs::File::open("mmsdm.json")?;
    let mapping = fs::read_to_string("table_mapping.csv").unwrap();
    let mut map = collections::HashMap::new();
    for row in mapping.split("\n").skip(1) {
        if row.contains(',') {
            let pieces = row.split(",").collect::<Vec<&str>>();
            let mms = mms::Report {
                name: pieces[0].to_string(),
                sub_type: pieces[1].to_string(),
            };
            let pdr = pdr::Report {
                name: pieces[2].to_string(),
                sub_type: match pieces[3] {
                    "" => None,
                    otherwise => Some(otherwise.to_string()),
                },
                version: pieces[4].parse().unwrap(),
                transaction_type: pieces[5].to_string(),
                row_filter: pieces[6].to_string(),
            };
            map.insert(mms, pdr);
        }
    }
    // abv
    let local_info: mms::Packages = serde_json::from_reader(rdr).unwrap();
    for (data_set, tables) in local_info.into_iter() {
        let mut fmt_str = String::new();
        let mut fmtr = codegen::Formatter::new(&mut fmt_str);
        for (table_key, table) in tables.into_iter() {
            let mms_report = mms::Report {
                name: data_set.clone(),
                sub_type: table_key,
            };

            if should_skip(&mms_report) {
                continue;
            }

            if let Some(pdr_report) = map.get(&mms_report) {
                let mut current_struct = codegen::Struct::new(&pdr_report.struct_name());
                current_struct
                    .vis("pub")
                    .doc(&table.get_doc(&pdr_report))
                    .derive("Debug")
                    .derive("Clone")
                    .derive("PartialEq")
                    .derive("Eq")
                    .derive("serde::Deserialize")
                    .derive("serde::Serialize");
                for col in table.columns.columns {
                    if &col.field_name() == "type" {
                        let mut field = codegen::Field::new("pub type_", &col.to_rust_type());
                        field.annotation(vec!["#[serde(rename = \"type\")]"]);
                        current_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPPP") {
                        // parse as DispatchPeriod
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::DispatchPeriod",
                        );
                        field.annotation(vec!["#[serde(with = \"crate::dispatch_period\")]"]);
                        current_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPP") {
                        // parse as TradingPeriod
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::TradingPeriod",
                        );
                        field.annotation(vec!["#[serde(with = \"crate::trading_period\")]"]);
                        current_struct.push_field(field);
                    } else if col.data_type == mms::DataType::Date {
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            &col.to_rust_type(),
                        );
                        if col.mandatory {
                            field.annotation(vec!["#[serde(with = \"crate::mms_datetime\")]"]);
                        } else {
                            field.annotation(vec!["#[serde(with = \"crate::mms_datetime_opt\")]"]);
                        };
                        current_struct.push_field(field);
                    } else {
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            &col.to_rust_type(),
                        );
                        field.doc(vec![&col.get_comment()]);
                        current_struct.push_field(field);
                    };
                }
                current_struct.fmt(&mut fmtr)?;

                let mut current_impl = codegen::Impl::new(pdr_report.struct_name());
                current_impl.impl_trait("crate::GetTable");

                let mut get_file_key = codegen::Function::new("get_file_key");
                get_file_key.ret("crate::FileKey");

                get_file_key.line(&pdr_report.file_key_literal());

                current_impl.push_fn(get_file_key);
                current_impl.fmt(&mut fmtr)?;
            } else {
                println!("Cannot find:");
                dbg!(mms_report);
            }
        }
        use heck::SnakeCase;
        fs::write(
            format!("src/data_model/{}.rs", data_set.to_snake_case()),
            fmt_str,
        )?;
    }

    Ok(())
}
