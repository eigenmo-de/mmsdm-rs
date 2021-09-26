use heck::{CamelCase, ShoutySnakeCase, SnakeCase, TitleCase};
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

}

impl mms::TableNote {
    fn get_rust_doc(&self) -> String {
        format!("* ({}) {} {}", self.name, self.comment, self.value)
    }
}

impl mms::TableNotes {
    fn get_rust_doc(&self) -> String {
        format!(
            "# Notes\n {}",
            self.notes
                .iter()
                .map(|n| n.get_rust_doc())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl mms::Description {
    fn get_rust_doc(&self) -> String {
        format!("# Description\n {}", self.inner)
    }
}

impl mms::PkColumns {
    fn get_rust_doc(&self) -> String {
        self.cols
            .iter()
            .map(|c| format!("* {}", c))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl mms::TableSummary {
    fn get_rust_doc(&self) -> String {
        format!("## {}\n _{}_", self.name, self.comment)
    }
}

impl mms::TablePage {
    pub fn get_rust_doc(&self, report: &pdr::Report) -> String {
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
            summary = self.summary.get_rust_doc(),
            pdr_report = report.get_rust_doc(),
            description_opt = self
                .description
                .as_ref()
                .map(|d| d.get_rust_doc())
                .unwrap_or_else(|| "".into()),
            notes_opt = self
                .notes
                .as_ref()
                .map(|n| n.get_rust_doc())
                .unwrap_or_else(|| "".into()),
            primary_key = self.primary_key_columns.get_rust_doc(),
        )
    }
}

impl pdr::Report {
    fn get_rust_doc(&self) -> String {
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

    pub fn get_rust_struct_name(&self) -> String {
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
    pub fn get_rust_pk_name(&self) -> String {
        format!("{}PrimaryKey", self.get_rust_struct_name())
    }
    pub fn get_partition_base(&self) -> String {
        if let Some(sub_type) = &self.sub_type {
            format!("{}_{}_v{}", 
                self.name.to_snake_case(),
                sub_type.to_snake_case(),
                self.version,
            )
        } else {
            format!("{}_v{}", 
                self.name.to_snake_case(),
                self.version,
            )
        }
    }
    pub fn get_rust_file_key_literal(&self) -> String {
        format!(
            r#"crate::FileKey {{
    data_set_name: "{data_set}".into(),
    table_name: Some("{table}".into()),
    version: {version},
}}"#,
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

            if mms_report.should_skip() {
                continue;
            }

            if let Some(pdr_report) = map.get(&mms_report) {
                let mut current_struct = codegen::Struct::new(&pdr_report.get_rust_struct_name());
                current_struct
                    .vis("pub")
                    .doc(&table.get_rust_doc(&pdr_report))
                    .derive("Debug")
                    .derive("Clone")
                    .derive("PartialEq")
                    .derive("Eq")
                    .derive("serde::Deserialize")
                    .derive("serde::Serialize");
                for col in table.columns.columns.iter() {
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
                        field.doc(vec![&col.comment]);
                        current_struct.push_field(field);
                    };
                }
                current_struct.fmt(&mut fmtr)?;

                let mut current_impl = codegen::Impl::new(pdr_report.get_rust_struct_name());
                current_impl.impl_trait("crate::GetTable");

                let mut get_file_key = codegen::Function::new("get_file_key");
                get_file_key.ret("crate::FileKey");

                get_file_key.line(&pdr_report.get_rust_file_key_literal());

                current_impl.push_fn(get_file_key);


                let mut primary_key = codegen::Function::new("primary_key");
                primary_key.ret(&pdr_report.get_rust_pk_name());
                primary_key.arg_ref_self();
                primary_key.line(&format!(
                    r#"{pk_name} {{
{pk_fields}
}}"#,
                    pk_name = pdr_report.get_rust_pk_name(),
                    pk_fields = table.primary_key_columns.cols.iter()
                        .map(|name| format!("    {0}: self.{0}.clone()", name.to_lowercase()))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                ));

                current_impl.push_fn(primary_key);


                let mut compare_with_key = codegen::Function::new("compare_with_key");
                compare_with_key.ret("bool");
                compare_with_key.line(&table.primary_key_columns.cols.iter()
                    .map(|name| format!("self.{0} == other.{0}", name.to_lowercase()))
                    .collect::<Vec<String>>()
                    .join("\n&& "),
                );                
                compare_with_key.arg_ref_self();
                compare_with_key.arg("other", &format!("&{}", pdr_report.get_rust_pk_name()));

                current_impl.push_fn(compare_with_key);


                let mut compare_with_other = codegen::Function::new("compare_with_other");
                compare_with_other.ret("bool");
                compare_with_other.arg_ref_self();
                compare_with_other.arg("other", "&Self");
                compare_with_other.line(&table.primary_key_columns.cols.iter()
                    .map(|name| format!("self.{0} == other.{0}", name.to_lowercase()))
                    .collect::<Vec<String>>()
                    .join("\n&& "),
                );    
                current_impl.push_fn(compare_with_other);


                let mut partition_suffix = codegen::Function::new("partition_suffix");
                partition_suffix.ret("String");
                partition_suffix.arg_ref_self();

                if table.primary_key_columns.cols.iter().any(|c| c.to_lowercase() == "settlementdate") {

                    partition_suffix.line(r#"format!("{}_{}", chrono::Datelike::year(self.settlementdate), chrono::Datelike::month(self.settlementdate))"#);
                } else {
                    partition_suffix.line("String::new()");
                }
                current_impl.push_fn(partition_suffix);

                let mut partition_name = codegen::Function::new("partition_name");
                partition_name.ret("String");
                partition_name.arg_ref_self();

                if table.primary_key_columns.cols.iter().any(|c| c.to_lowercase() == "settlementdate") {

                    partition_name.line(
                        &format!(
                            r#"format!("{}_{{}}_{{}}", chrono::Datelike::year(self.settlementdate), chrono::Datelike::month(self.settlementdate))"#,
                            pdr_report.get_partition_base()
                        )
                    );
                } else {
                    partition_name.line(&format!(r#""{}".to_string()"#, pdr_report.get_partition_base()));
                }


                current_impl.push_fn(partition_name);

                current_impl.fmt(&mut fmtr)?;


                let mut pk_struct = codegen::Struct::new(&pdr_report.get_rust_pk_name());
                pk_struct
                    .vis("pub")
                    .derive("Debug")
                    .derive("Clone")
                    .derive("PartialEq")
                    .derive("Eq")
                    .derive("PartialOrd")
                    .derive("Ord");

                for pk_col_name in table.primary_key_columns.cols.iter() {
                    let col = table.columns.columns.iter().find(|col| &col.name == pk_col_name).expect("PK column must exist");
                    
                    // temporary
                    if !col.mandatory {
                        panic!("Non mandatory column in primary key: {:?}", col);
                    }

                    if &col.field_name() == "type" {
                        let field = codegen::Field::new("pub type_", &col.to_rust_type());
                        pk_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPPP") {
                        // parse as DispatchPeriod
                        let field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::DispatchPeriod",
                        );
                        pk_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPP") {
                        // parse as TradingPeriod
                        let field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::TradingPeriod",
                        );
                        pk_struct.push_field(field);
                    } else if col.data_type == mms::DataType::Date {
                        let field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            &col.to_rust_type(),
                        );
                        pk_struct.push_field(field);
                    } else {
                        let field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            &col.to_rust_type(),
                        );
                        pk_struct.push_field(field);
                    };
                }

                pk_struct.fmt(&mut fmtr)?;

                let mut pk_impl = codegen::Impl::new(&pdr_report.get_rust_pk_name());

                let mut compare_with_other_pk = codegen::Function::new("compare_with_other");
                compare_with_other_pk.ret("bool");
                compare_with_other_pk.arg_ref_self();
                compare_with_other_pk.arg("other", "&Self");
                compare_with_other_pk.line(&table.primary_key_columns.cols.iter()
                    .map(|name| format!("self.{0} == other.{0}", name.to_lowercase()))
                    .collect::<Vec<String>>()
                    .join("\n&& "),
                );    
                pk_impl.push_fn(compare_with_other_pk);

                let mut compare_with_row = codegen::Function::new("compare_with_row");
                compare_with_row.ret("bool");
                compare_with_row.arg_ref_self();
                compare_with_row.arg("row", &format!("&{}", pdr_report.get_rust_struct_name()));
                compare_with_row.line(&table.primary_key_columns.cols.iter()
                    .map(|name| format!("self.{0} == row.{0}", name.to_lowercase()))
                    .collect::<Vec<String>>()
                    .join("\n&& "),
                );    
                pk_impl.push_fn(compare_with_row);

                pk_impl.fmt(&mut fmtr)?;

                let mut pk_trait = codegen::Impl::new(&pdr_report.get_rust_pk_name());
                pk_trait.impl_trait("crate::PrimaryKey");
                pk_trait.fmt(&mut fmtr)?;
                
            } else {
                println!("Cannot find:");
                dbg!(mms_report);
            }
        }
        fs::write(
            format!("src/data_model/{}.rs", data_set.to_snake_case()),
            fmt_str,
        )?;
    }

    Ok(())
}
