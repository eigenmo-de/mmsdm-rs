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
    fn as_arrow_type(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } => "arrow2::datatypes::DataType::LargeUtf8".to_string(),
            mms::DataType::Char => "arrow2::datatypes::DataType::LargeUtf8".to_string(),
            mms::DataType::Date => "arrow2::datatypes::DataType::Date32".to_string(),
            mms::DataType::DateTime => "arrow2::datatypes::DataType::Date64".to_string(),
            mms::DataType::Decimal { precision, scale } => format!(
                "arrow2::datatypes::DataType::Decimal({},{})",
                precision, scale
            ),
            mms::DataType::Integer { .. } => "arrow2::datatypes::DataType::Int64".to_string(),
        }
    }
}

impl mms::TableColumn {
    fn clone_or_nothing(&self) -> String {
        if self.comment.contains("YYYYMMDDPPP")
            || self.comment.contains("YYYYMMDDPP")
            || !matches!(self.data_type, mms::DataType::Varchar { .. })
        {
            ""
        } else {
            ".clone()"
        }
        .to_string()
    }
    fn to_rust_type(&self) -> String {
        let formatted_type = if self.comment.contains("YYYYMMDDPPP") {
            "crate::DispatchPeriod".to_string()
        } else if self.comment.contains("YYYYMMDDPP") {
            "crate::TradingPeriod".to_string()
        } else {
            self.data_type.as_rust_type()
        };

        if self.mandatory {
            format!("{}", formatted_type)
        } else {
            format!("Option<{}>", formatted_type)
        }
    }
    fn rust_field_name(&self) -> String {
        if KW.contains(&self.field_name().as_str()) {
            format!("r#{}", self.field_name())
        } else {
            self.field_name()
        }
    }
    fn as_arrow_type(&self) -> String {
        if self.comment.contains("YYYYMMDDPPP") {
            "arrow2::datatypes::DataType::Date64".to_string()
        } else if self.comment.contains("YYYYMMDDPP") {
            "arrow2::datatypes::DataType::Date32".to_string()
        } else {
            self.data_type.as_arrow_type()
        }
    }
    fn as_arrow_field(&self) -> String {
        format!(
            "arrow2::datatypes::Field::new(\"{name}\", {ty}, {nullable})",
            name = self.rust_field_name(),
            ty = self.as_arrow_type(),
            nullable = !self.mandatory,
        )
    }
    fn as_arrow_array_name(&self) -> String {
        format!("{}_array", self.rust_field_name())
    }
    fn as_arrow_array_extractor(&self) -> String {
        let extractor = match (&self.data_type, self.mandatory) {
            (_, _) if self.comment.contains("YYYYMMDDPPP") => format!("row.{}.start().timestamp_millis()", self.rust_field_name()),
            (_, _) if self.comment.contains("YYYYMMDDPP") => format!("row.{}.start().timestamp_millis()", self.rust_field_name()),
            // (_, false) if self.comment.contains("YYYYMMDDPPP") => format!("row.{}.map(|val| val.start().timestamp_millis())", self.rust_field_name()),
            // (_, false) if self.comment.contains("YYYYMMDDPP") => format!("row.{}.map(|val| val.start().timestamp_millis())", self.rust_field_name()),
            (mms::DataType::Varchar { .. }, true) => format!("row.{}", self.rust_field_name()),
            (mms::DataType::Char, true) => format!("row.{}.to_string()", self.rust_field_name()),
            (mms::DataType::Date, true) => format!("i32::try_from((row.{}.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days()).unwrap()", self.rust_field_name()),
            (mms::DataType::DateTime, true) => format!("row.{}.timestamp_millis()", self.rust_field_name()),
            (mms::DataType::Decimal { scale, .. }, true) => format!("{{
                let mut val = row.{};
                val.rescale({scale});
                val.mantissa()
            }}", self.rust_field_name(), scale = scale),
            (mms::DataType::Integer { .. }, true) => format!("row.{}", self.rust_field_name()),
            (mms::DataType::Varchar { .. }, false) => format!("row.{}", self.rust_field_name()),
            (mms::DataType::Char, false) => format!("row.{}.map(|val| val.to_string())", self.rust_field_name()),
            (mms::DataType::Date, false) =>  format!("row.{}.map(|val| i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days()).unwrap())", self.rust_field_name()),
            (mms::DataType::DateTime, false) => format!("row.{}.map(|val| val.timestamp_millis())", self.rust_field_name()),
            (mms::DataType::Decimal { scale, .. }, false) => format!("{{
                row.{}.map(|mut val| {{
                    val.rescale({scale});
                    val.mantissa()
                }})
            }}", self.rust_field_name(), scale = scale),
            (mms::DataType::Integer { .. }, false) => format!("row.{}", self.rust_field_name()),
        };
        format!(
            "{array}.push({extractor});",
            array = self.as_arrow_array_name(),
            extractor = extractor,
        )
    }
    fn as_arrow_array_constructor(&self) -> String {
        match (&self.data_type, self.mandatory) {
            (_, _) if self.comment.contains("YYYYMMDDPPP") => format!(
                "arrow2::array::PrimitiveArray::from_slice({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            (_, _) if self.comment.contains("YYYYMMDDPP") => format!(
                "arrow2::array::PrimitiveArray::from_slice({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            // (_, false) if self.comment.contains("YYYYMMDDPPP") => format!("arrow2::array::PrimitiveArray::from({})", self.as_arrow_array_name()),
            // (_, false) if self.comment.contains("YYYYMMDDPP") => format!("arrow2::array::PrimitiveArray::from({})", self.as_arrow_array_name()),
            (mms::DataType::Varchar { .. }, true) => format!(
                "arrow2::array::Utf8Array::<i64>::from_slice({})",
                self.as_arrow_array_name()
            ),
            (mms::DataType::Char, true) => format!(
                "arrow2::array::Utf8Array::<i64>::from_slice({})",
                self.as_arrow_array_name()
            ),

            (mms::DataType::Varchar { .. }, false) => format!(
                "arrow2::array::Utf8Array::<i64>::from({})",
                self.as_arrow_array_name()
            ),
            (mms::DataType::Char, false) => format!(
                "arrow2::array::Utf8Array::<i64>::from({})",
                self.as_arrow_array_name()
            ),

            (mms::DataType::Date, true) => format!(
                "arrow2::array::PrimitiveArray::from_slice({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            (mms::DataType::DateTime, true) => format!(
                "arrow2::array::PrimitiveArray::from_slice({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            (mms::DataType::Decimal { .. }, true) => format!(
                "arrow2::array::PrimitiveArray::from_slice({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            (mms::DataType::Integer { .. }, true) => format!(
                "arrow2::array::PrimitiveArray::from_slice({})",
                self.as_arrow_array_name()
            ),

            (mms::DataType::Date, false) => format!(
                "arrow2::array::PrimitiveArray::from({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            (mms::DataType::DateTime, false) => format!(
                "arrow2::array::PrimitiveArray::from({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            (mms::DataType::Decimal { .. }, false) => format!(
                "arrow2::array::PrimitiveArray::from({}).to({})",
                self.as_arrow_array_name(),
                self.as_arrow_type()
            ),
            (mms::DataType::Integer { .. }, false) => format!(
                "arrow2::array::PrimitiveArray::from({})",
                self.as_arrow_array_name()
            ),
        }
    }
}

impl mms::TableColumns {
    fn as_arrow_schema(&self) -> String {
        format!(
            "arrow2::datatypes::Schema::new(vec![
    {fields}
])",
            fields = self
                .columns
                .iter()
                .map(|col| col.as_arrow_field())
                .collect::<Vec<_>>()
                .join(",\n    "),
        )
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
            summary = self.summary.get_rust_doc().replace('\t', ""),
            pdr_report = report.get_rust_doc().replace('\t', ""),
            description_opt = self
                .description
                .as_ref()
                .map(|d| d.get_rust_doc().replace('\t', ""))
                .unwrap_or_else(|| "".into()),
            notes_opt = self
                .notes
                .as_ref()
                .map(|n| n.get_rust_doc().replace('\t', ""))
                .unwrap_or_else(|| "".into()),
            primary_key = self.primary_key_columns.get_rust_doc().replace('\t', ""),
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
            format!(
                "{}_{}_v{}",
                self.name.to_snake_case(),
                sub_type.to_snake_case(),
                self.version,
            )
        } else {
            format!("{}_v{}", self.name.to_snake_case(), self.version,)
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
                        let mut field = codegen::Field::new("pub r#type", &col.to_rust_type());
                        field.annotation(vec!["#[serde(rename = \"type\")]"]);
                        field.doc(vec![&col.comment.replace('\t', "")]);
                        current_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPPP") {
                        // parse as DispatchPeriod
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::DispatchPeriod",
                        );
                        field.annotation(vec!["#[serde(with = \"crate::dispatch_period\")]"]);
                        field.doc(vec![&col.comment.replace('\t', "")]);
                        current_struct.push_field(field);
                    } else if col.comment.contains("YYYYMMDDPP") {
                        // parse as TradingPeriod
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            "crate::TradingPeriod",
                        );
                        field.annotation(vec!["#[serde(with = \"crate::trading_period\")]"]);
                        field.doc(vec![&col.comment.replace('\t', "")]);
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
                        field.doc(vec![&col.comment.replace('\t', "")]);
                        current_struct.push_field(field);
                    } else {
                        let mut field = codegen::Field::new(
                            &format!("pub {}", col.field_name()),
                            &col.to_rust_type(),
                        );
                        field.doc(vec![&col.comment.replace('\t', "")]);
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
                    pk_fields = table
                        .primary_key_columns
                        .cols
                        .iter()
                        .map(|name| format!(
                            "    {0}: self.{0}{1}",
                            lowercase_and_escape(name),
                            table
                                .columns
                                .columns
                                .iter()
                                .find(|col| &col.name == name)
                                .unwrap()
                                .clone_or_nothing()
                        ))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                ));

                current_impl.push_fn(primary_key);

                current_impl.associate_type("PrimaryKey", pdr_report.get_rust_pk_name());

                let mut partition_suffix = codegen::Function::new("partition_suffix");
                partition_suffix.ret("Self::Partition");
                partition_suffix.arg_ref_self();

                if table
                    .primary_key_columns
                    .cols
                    .iter()
                    .any(|c| c.to_lowercase() == "settlementdate")
                {
                    current_impl.associate_type("Partition", "(i32, chrono::Month)");
                    partition_suffix.line(r#"(chrono::Datelike::year(&self.settlementdate), num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate)).unwrap())"#);
                } else {
                    current_impl.associate_type("Partition", "()");
                    // partition_suffix.line("()");
                }
                current_impl.push_fn(partition_suffix);

                let mut partition_name = codegen::Function::new("partition_name");
                partition_name.ret("String");
                partition_name.arg_ref_self();

                if table
                    .primary_key_columns
                    .cols
                    .iter()
                    .any(|c| c.to_lowercase() == "settlementdate")
                {
                    partition_name.line(
                        &format!(
                            r#"format!("{}_{{}}_{{}}", chrono::Datelike::year(&self.settlementdate), chrono::Datelike::month(&self.settlementdate))"#,
                            pdr_report.get_partition_base()
                        )
                    );
                } else {
                    partition_name.line(&format!(
                        r#""{}".to_string()"#,
                        pdr_report.get_partition_base()
                    ));
                }

                current_impl.push_fn(partition_name);

                current_impl.fmt(&mut fmtr)?;

                let mut compare_with_row_impl =
                    codegen::Impl::new(pdr_report.get_rust_struct_name());
                compare_with_row_impl.impl_trait("crate::CompareWithRow");
                compare_with_row_impl.associate_type("Row", pdr_report.get_rust_struct_name());
                let mut compare_with_other = codegen::Function::new("compare_with_row");
                compare_with_other.ret("bool");
                compare_with_other.arg_ref_self();
                compare_with_other.arg("row", "&Self::Row");
                compare_with_other.line(
                    &table
                        .primary_key_columns
                        .cols
                        .iter()
                        .map(|name| format!("self.{0} == row.{0}", lowercase_and_escape(name)))
                        .collect::<Vec<String>>()
                        .join("\n&& "),
                );
                compare_with_row_impl.push_fn(compare_with_other);
                compare_with_row_impl.fmt(&mut fmtr)?;

                let mut compare_with_pk_impl =
                    codegen::Impl::new(pdr_report.get_rust_struct_name());
                compare_with_pk_impl.impl_trait("crate::CompareWithPrimaryKey");
                compare_with_pk_impl.associate_type("PrimaryKey", pdr_report.get_rust_pk_name());
                let mut compare_with_key = codegen::Function::new("compare_with_key");
                compare_with_key.ret("bool");
                compare_with_key.line(
                    &table
                        .primary_key_columns
                        .cols
                        .iter()
                        .map(|name| format!("self.{0} == key.{0}", lowercase_and_escape(name)))
                        .collect::<Vec<String>>()
                        .join("\n&& "),
                );
                compare_with_key.arg_ref_self();
                compare_with_key.arg("key", "&Self::PrimaryKey");
                compare_with_pk_impl.push_fn(compare_with_key);
                compare_with_pk_impl.fmt(&mut fmtr)?;

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
                    let col = table
                        .columns
                        .columns
                        .iter()
                        .find(|col| &col.name == pk_col_name)
                        .expect("PK column must exist");

                    // temporary
                    if !col.mandatory {
                        panic!("Non mandatory column in primary key: {:?}", col);
                    }

                    if &col.field_name() == "type" {
                        let field = codegen::Field::new("pub r#type", &col.to_rust_type());
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

                let mut pk_compare_row_impl = codegen::Impl::new(&pdr_report.get_rust_pk_name());
                pk_compare_row_impl.impl_trait("crate::CompareWithRow");
                pk_compare_row_impl.associate_type("Row", pdr_report.get_rust_struct_name());
                let mut compare_with_row = codegen::Function::new("compare_with_row");
                compare_with_row.ret("bool");
                compare_with_row.arg_ref_self();
                compare_with_row.arg("row", "&Self::Row");
                compare_with_row.line(
                    &table
                        .primary_key_columns
                        .cols
                        .iter()
                        .map(|name| format!("self.{0} == row.{0}", lowercase_and_escape(name)))
                        .collect::<Vec<String>>()
                        .join("\n&& "),
                );
                pk_compare_row_impl.push_fn(compare_with_row);
                pk_compare_row_impl.fmt(&mut fmtr)?;

                let mut pk_compare_pk_impl = codegen::Impl::new(&pdr_report.get_rust_pk_name());
                pk_compare_pk_impl.impl_trait("crate::CompareWithPrimaryKey");
                pk_compare_pk_impl.associate_type("PrimaryKey", pdr_report.get_rust_pk_name());
                let mut compare_with_other_pk = codegen::Function::new("compare_with_key");
                compare_with_other_pk.ret("bool");
                compare_with_other_pk.arg_ref_self();
                compare_with_other_pk.arg("key", "&Self::PrimaryKey");
                compare_with_other_pk.line(
                    &table
                        .primary_key_columns
                        .cols
                        .iter()
                        .map(|name| format!("self.{0} == key.{0}", lowercase_and_escape(name)))
                        .collect::<Vec<String>>()
                        .join("\n&& "),
                );
                pk_compare_pk_impl.push_fn(compare_with_other_pk);
                pk_compare_pk_impl.fmt(&mut fmtr)?;

                let mut pk_trait = codegen::Impl::new(&pdr_report.get_rust_pk_name());
                pk_trait.impl_trait("crate::PrimaryKey");
                pk_trait.fmt(&mut fmtr)?;

                let mut arrow_trait = codegen::Impl::new(&pdr_report.get_rust_struct_name());
                arrow_trait.impl_trait("crate::ArrowSchema");
                arrow_trait.r#macro("#[cfg(feature = \"save_as_parquet\")]");

                let mut arrow_schema = codegen::Function::new("arrow_schema");
                arrow_schema.ret("arrow2::datatypes::Schema");
                arrow_schema.line(&table.columns.as_arrow_schema());
                arrow_trait.push_fn(arrow_schema);

                let mut partition_to_batch = codegen::Function::new("partition_to_record_batch");
                partition_to_batch.ret("crate::Result<arrow2::record_batch::RecordBatch>");
                partition_to_batch.arg(
                    "partition",
                    "std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>",
                );

                // partition_to_batch.line("use std::convert::TryFrom;");

                for col in &table.columns.columns {
                    partition_to_batch.line(&format!(
                        "let mut {} = Vec::new();",
                        col.as_arrow_array_name()
                    ));
                }

                partition_to_batch.line("for (_, row) in partition {");

                for col in &table.columns.columns {
                    partition_to_batch.line(&format!("    {}", col.as_arrow_array_extractor()));
                }
                partition_to_batch.line(
                    "}

arrow2::record_batch::RecordBatch::try_new(
    std::sync::Arc::new(Self::arrow_schema()),
    vec![",
                );

                for col in &table.columns.columns {
                    partition_to_batch.line(&format!(
                        "        std::sync::Arc::new({}),",
                        col.as_arrow_array_constructor()
                    ));
                }

                partition_to_batch.line(
                    "    ]
).map_err(Into::into)",
                );

                arrow_trait.push_fn(partition_to_batch);

                arrow_trait.fmt(&mut fmtr)?;
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

const KW: [&'static str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

fn lowercase_and_escape(col_name: &str) -> String {
    if KW.contains(&col_name.to_lowercase().as_str()) {
        format!("r#{}", col_name.to_lowercase())
    } else {
        col_name.to_lowercase()
    }
}

// example for Parquet/Arrow:

// make a fn for rescale!
// how to properly handle nullable? Optoin in the array?

// #[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
// pub struct DispatchUnitScada1 {
//     #[serde(with = "crate::mms_datetime")]
//     pub settlementdate: chrono::NaiveDateTime,
//     /// Dispatchable Unit Identifier
//     pub duid: String,
//     /// Instantaneous MW reading from SCADA at the start of the Dispatch interval
//     pub scadavalue: Option<rust_decimal::Decimal>,
// }

// impl DispatchUnitScada1 {

//     #[cfg(feature = "save_as_parquet")]
//     fn get_schema() -> arrow2::datatypes::Schema {
//         use arrow2::datatypes::*;

//         Schema::new(vec![
//             Field::new("settlementdate", DataType::Date64, false),
//             Field::new("duid", DataType::Utf8, false),
//             Field::new("scadavalue", DataType::Decimal(16,6), true),
//         ])
//     }

//     #[cfg(feature = "save_as_parquet")]
//     fn from_partition(ptn: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>) -> crate::Result<arrow2::record_batch::RecordBatch> {

//         use arrow2::array;
//         use std::convert::TryFrom;

//         let mut settlementdate_array = Vec::new();
//         let mut duid_array = Vec::new();
//         let mut scadavalue_array = Vec::new();

//         for (_, row) in ptn {
//             settlementdate_array.push(u64::try_from(row.settlementdate.timestamp_millis()).unwrap());
//             duid_array.push(row.duid);
//             let mut rescaled = row.scadavalue.unwrap_or_default();
//             rescaled.rescale(6);
//             scadavalue_array.push(rescaled.mantissa());
//         }

//         arrow2::record_batch::RecordBatch::try_new(
//             std::sync::Arc::new(Self::get_schema()),
//             vec![
//                 std::sync::Arc::new(array::PrimitiveArray::from_slice(settlementdate_array).to(arrow2::datatypes::DataType::Date64)),
//                 std::sync::Arc::new(array::Utf8Array::<i64>::from_slice(duid_array)),
//                 std::sync::Arc::new(array::PrimitiveArray::from_slice(scadavalue_array).to(arrow2::datatypes::DataType::Decimal(16,6)))
//             ]
//         ).map_err(Into::into)
//     }
// }
