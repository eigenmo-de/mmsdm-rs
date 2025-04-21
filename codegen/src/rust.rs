use anyhow::Context;
use codegen::Scope;
use heck::{ToSnakeCase, ToTitleCase, ToUpperCamelCase};
use itertools::Itertools;
use log::info;
use std::{
    collections::{self, HashMap},
    fmt::Debug,
    fs, str,
};

use anyhow::anyhow;

use crate::{
    VERSION,
    json::{DataModel, Table},
};

use crate::KW;
use crate::{mms, pdr};
use serde::Deserialize;

impl mms::DataType {
    fn as_rust_type(&self) -> String {
        match self {
            // mms::DataType::Varchar { .. } | mms::DataType::Char=> "mmsdm_core::MaybeStackStr",
            mms::DataType::Varchar { .. } | mms::DataType::Char => "alloc::string::String",
            mms::DataType::Date | mms::DataType::DateTime => "chrono::NaiveDateTime",
            mms::DataType::Decimal { .. } => "rust_decimal::Decimal",
            mms::DataType::Integer { .. } => "i64",
        }
        .into()
    }
    fn as_arrow_type(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } | mms::DataType::Char => {
                "arrow::datatypes::DataType::Utf8".to_string()
            }
            mms::DataType::Date | mms::DataType::DateTime => {
                "arrow::datatypes::DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None)"
                    .to_string()
            }
            mms::DataType::Decimal { precision, scale } => format!(
                "arrow::datatypes::DataType::Decimal128({},{})",
                precision, scale
            ),
            mms::DataType::Integer { .. } => "arrow::datatypes::DataType::Int64".to_string(),
        }
    }
    fn as_arrow_builder(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } | mms::DataType::Char => {
                "arrow::array::builder::StringBuilder::new()".to_string()
            }
            mms::DataType::Date | mms::DataType::DateTime => {
                "arrow::array::builder::TimestampMillisecondBuilder::new()".to_string()
            }
            mms::DataType::Decimal { .. } => format!(
                "arrow::array::builder::Decimal128Builder::new().with_data_type({})",
                self.as_arrow_type()
            ),

            mms::DataType::Integer { .. } => {
                "arrow::array::builder::Int64Builder::new()".to_string()
            }
        }
    }
    fn as_arrow_builder_ty(&self) -> String {
        match self {
            mms::DataType::Varchar { .. } | mms::DataType::Char => {
                "arrow::array::builder::StringBuilder".to_string()
            }
            mms::DataType::Date | mms::DataType::DateTime => {
                "arrow::array::builder::TimestampMillisecondBuilder".to_string()
            }
            mms::DataType::Decimal { .. } => "arrow::array::builder::Decimal128Builder".to_string(),

            mms::DataType::Integer { .. } => "arrow::array::builder::Int64Builder".to_string(),
        }
    }
}

impl mms::TableColumn {
    fn to_string_or_nothing(&self) -> String {
        if self.data_stored_in_string() {
            "().to_string()"
        } else {
            ""
        }
        .to_string()
    }
    fn to_rust_type(&self) -> String {
        dbg!(
            self,
            self.is_dispatch_period(),
            self.is_trading_period(),
            &self.comment
        );
        if self.is_dispatch_period() {
            "mmsdm_core::DispatchPeriod".to_string()
        } else if self.is_trading_period() {
            "mmsdm_core::TradingPeriod".to_string()
        } else if self.mandatory {
            self.data_type.as_rust_type()
        } else {
            format!("Option<{}>", self.data_type.as_rust_type())
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
        if self.is_dispatch_period() || self.is_trading_period() {
            "arrow::datatypes::DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None)"
                .to_string()
        } else {
            self.data_type.as_arrow_type()
        }
    }
    fn as_arrow_builder_ty(&self) -> String {
        if self.is_dispatch_period() || self.is_trading_period() {
            mms::DataType::DateTime.as_arrow_builder_ty()
        } else {
            self.data_type.as_arrow_builder_ty()
        }
    }
    fn as_arrow_builder(&self) -> String {
        if self.is_dispatch_period() || self.is_trading_period() {
            mms::DataType::DateTime.as_arrow_builder()
        } else {
            self.data_type.as_arrow_builder()
        }
    }
    fn as_arrow_field(&self) -> String {
        format!(
            "arrow::datatypes::Field::new(\"{name}\", {ty}, {nullable})",
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
            (_, _) if self.is_dispatch_period() || self.is_trading_period() => {
                format!(
                    "row.{}.start().and_utc().timestamp_millis()",
                    self.rust_field_name()
                )
            }
            // (_, false) if self.comment.to_uppercase().contains("YYYYMMDDPPP") || self.comment.to_uppercase().contains("YYYYMMDDPP") => {
            //     format!("row.{}.map(|v| v.start().timestamp_millis())", self.rust_field_name())
            // }
            // (mms::DataType::Varchar { .. }, true) if !self.data_stored_in_string() => {
            //     format!("row.{}.start().timestamp_millis()", self.rust_field_name())
            // }
            // (mms::DataType::Varchar { .. }, false) if !self.data_stored_in_string() => {
            //     format!("row.{}.map(|v| v.start().timestamp_millis())", self.rust_field_name())
            // }
            (mms::DataType::Date | mms::DataType::DateTime, true) => {
                format!(
                    "row.{}.and_utc().timestamp_millis()",
                    self.rust_field_name()
                )
            }
            (mms::DataType::Date | mms::DataType::DateTime, false) => {
                format!(
                    "row.{}.map(|val| val.and_utc().timestamp_millis())",
                    self.rust_field_name()
                )
            }

            (mms::DataType::Decimal { scale, .. }, true) => format!(
                "{{
                let mut val = row.{};
                val.rescale({scale});
                val.mantissa()
            }}",
                self.rust_field_name(),
                scale = scale
            ),
            (mms::DataType::Decimal { scale, .. }, false) => format!(
                "{{
                row.{}.map(|mut val| {{
                    val.rescale({scale});
                    val.mantissa()
                }})
            }}",
                self.rust_field_name(),
                scale = scale
            ),

            (mms::DataType::Varchar { .. } | mms::DataType::Char, _) => {
                format!("row.{}()", self.rust_field_name())
            }
            (mms::DataType::Integer { .. }, _) => format!("row.{}", self.rust_field_name()),
        };
        if self.mandatory || self.is_dispatch_period() || self.is_trading_period() {
            format!(
                "builder.{array}.append_value({extractor});",
                array = self.as_arrow_array_name(),
                extractor = extractor,
            )
        } else {
            format!(
                "builder.{array}.append_option({extractor});",
                array = self.as_arrow_array_name(),
                extractor = extractor,
            )
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

impl mms::PkColumns {
    fn get_rust_doc(&self) -> String {
        self.cols
            .iter()
            .map(|c| format!("* {}", c))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Table {
    pub fn get_rust_doc(&self, name: &str, report: &pdr::Report) -> String {
        //use heck::TitleCase;
        format!(
            r#"# Summary

## {name}

{summary}

{pdr_report}

# Description
{description_opt}

# Notes
{notes_opt}

# Primary Key Columns

{primary_key}
"#,
            summary = self.comment.replace('\t', ""),
            pdr_report = report.get_rust_doc().replace('\t', ""),
            description_opt = self.description.replace('\t', ""),
            notes_opt = self
                .notes
                .iter()
                .map(|(name, note)| {
                    format!("* ({name}) {} {}", note.comment, note.value).replace('\t', "")
                })
                .collect::<Vec<_>>()
                .join("\n"),
            primary_key = self
                .primary_key_columns
                .iter()
                .map(|c| format!("* {}", c).replace('\t', ""))
                .collect::<Vec<_>>()
                .join("\n"),
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
    pub fn get_rust_manager_struct_name(&self) -> String {
        if let Some(sub_type) = &self.sub_type {
            format!(
                "{}{}{}",
                self.name.to_upper_camel_case(),
                sub_type.to_upper_camel_case(),
                self.version
            )
        } else {
            format!("{}{}", self.name.to_upper_camel_case(), self.version)
        }
    }
    pub fn get_rust_struct_name(&self) -> String {
        if let Some(sub_type) = &self.sub_type {
            format!(
                "{}{}{}Row",
                self.name.to_upper_camel_case(),
                sub_type.to_upper_camel_case(),
                self.version
            )
        } else {
            format!("{}{}Row", self.name.to_upper_camel_case(), self.version)
        }
    }
    pub fn get_rust_struct_mapping_name(&self) -> String {
        format!("{}Mapping", self.get_rust_manager_struct_name())
    }
    pub fn get_rust_arrow_builder_name(&self) -> String {
        format!("{}Builder", self.get_rust_manager_struct_name())
    }
    pub fn get_rust_pk_name(&self) -> String {
        format!("{}PrimaryKey", self.get_rust_manager_struct_name())
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
}

fn codegen_struct(
    name: &str,
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    let mut manager_struct = codegen::Struct::new(&pdr_report.get_rust_manager_struct_name());
    manager_struct.vis("pub");
    manager_struct.field("extract_row_partition", format!("alloc::boxed::Box<dyn Fn(&{}<'_>) -> mmsdm_core::PartitionValue + Send + Sync + 'static>", pdr_report.get_rust_struct_name()));
    manager_struct.field("row_partition_key", "mmsdm_core::PartitionKey");
    manager_struct.fmt(fmtr)?;

    {
        let mut manager_impl = codegen::Impl::new(&pdr_report.get_rust_manager_struct_name());

        let func = manager_impl.new_fn("new");
        func.vis("pub");
        func.arg("row_partition_key", "mmsdm_core::PartitionKey");
        func.arg("func", "impl Fn(&<Self as mmsdm_core::GetTable>::Row<'_>) -> mmsdm_core::PartitionValue + Send + Sync + 'static");

        func.ret("Self");

        func.line(
            "Self { extract_row_partition: alloc::boxed::Box::new(func), row_partition_key, }",
        );

        manager_impl.fmt(fmtr)?;
    }

    let mut mapping = codegen::Struct::new(&pdr_report.get_rust_struct_mapping_name());
    mapping.vis("pub");
    mapping.tuple_field(format!("[usize; {}]", table.columns().count()));
    mapping.fmt(fmtr)?;

    let mut current_struct = codegen::Struct::new(&pdr_report.get_rust_struct_name());
    current_struct
        .vis("pub")
        .doc(&table.get_rust_doc(name, pdr_report))
        .derive("Debug")
        .derive("PartialEq")
        .derive("Eq")
        .generic("'data");

    for col in table.columns() {
        dbg!(
            col.is_dispatch_period(),
            col.is_trading_period(),
            &col.comment
        );
        if col.is_dispatch_period() {
            // parse as DispatchPeriod
            let mut field = codegen::Field::new(
                &format!("pub {}", col.field_name()),
                "mmsdm_core::DispatchPeriod",
            );
            field.doc(col.comment.replace('\t', ""));
            current_struct.push_field(field);
        } else if col.is_trading_period() {
            // parse as TradingPeriod
            let mut field = codegen::Field::new(
                &format!("pub {}", col.field_name()),
                "mmsdm_core::TradingPeriod",
            );
            field.doc(col.comment.replace('\t', ""));
            current_struct.push_field(field);
        } else if matches!(
            col.data_type,
            mms::DataType::Char | mms::DataType::Varchar { .. }
        ) {
            let mut field = codegen::Field::new(
                &format!("pub {}", col.escaped_field_name()),
                "core::ops::Range<usize>",
            );
            field.doc(col.comment.replace('\t', ""));
            current_struct.push_field(field);
        } else if matches!(col.data_type, mms::DataType::Date | mms::DataType::DateTime) {
            let mut field = codegen::Field::new(
                &format!("pub {}", col.escaped_field_name()),
                &col.to_rust_type(),
            );
            field.doc(col.comment.replace('\t', ""));
            current_struct.push_field(field);
        } else {
            let mut field = codegen::Field::new(
                &format!("pub {}", col.escaped_field_name()),
                &col.to_rust_type(),
            );
            field.doc(col.comment.replace('\t', ""));
            current_struct.push_field(field);
        };
    }

    if table.has_any_string_columns() {
        current_struct.push_field(codegen::Field::new(
            "backing_data",
            "mmsdm_core::CsvRow<'data>",
        ));
    } else {
        current_struct.push_field(codegen::Field::new(
            "backing_data",
            "core::marker::PhantomData<&'data ()>",
        ));
    }

    current_struct.fmt(fmtr)?;

    {
        let mut struct_impl =
            codegen::Impl::new(format!("{}<'data>", pdr_report.get_rust_struct_name()));
        struct_impl.generic("'data");

        for col in table.columns() {
            if col.data_stored_in_string() {
                // need to add a getter function
                let field_name = col.escaped_field_name();
                let func = struct_impl.new_fn(&format!("{field_name}"));

                func.vis("pub");
                func.arg_ref_self();

                if col.mandatory {
                    func.ret("&str");
                    func.line(format!("core::ops::Index::index(self.backing_data.as_slice(), self.{field_name}.clone())"));
                } else {
                    func.ret("Option<&str>");
                    func.line(format!("if self.{field_name}.is_empty() {{"));
                    func.line("None");
                    func.line("} else {");
                    func.line(format!("Some(core::ops::Index::index(self.backing_data.as_slice(), self.{field_name}.clone()))"));
                    func.line("}");
                }
            }
        }

        struct_impl.fmt(fmtr)?;
    }

    Ok(())
}

fn codegen_impl_get_table(
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    let mut current_impl = codegen::Impl::new(&pdr_report.get_rust_manager_struct_name());
    current_impl.impl_trait("mmsdm_core::GetTable");

    current_impl.associate_type(
        "Row<'row>",
        format!("{}<'row>", pdr_report.get_rust_struct_name()),
    );

    {
        let mut from_row = codegen::Function::new("from_row");
        from_row.generic("'data");
        from_row.arg("row", "mmsdm_core::CsvRow<'data>");
        from_row.arg("field_mapping", "&Self::FieldMapping");
        from_row.ret("mmsdm_core::Result<Self::Row<'data>>");
        from_row.line(&format!("Ok({} {{", pdr_report.get_rust_struct_name()));

        for (idx, col) in table.columns().enumerate() {
            let field_name = col.field_name();
            let field_name_escaped = col.escaped_field_name();
            let mandatory_part = match col.mandatory {
                true => "",
                // dispatch and trading period columns are _never_ null, even if they are stated as not mandatory...
                false if col.is_trading_period() || col.is_dispatch_period() => "",
                false => "opt_",
            };
            match &col.data_type {
                mms::DataType::DateTime | mms::DataType::Date
                    if !(col.is_trading_period() || col.is_dispatch_period()) =>
                {
                    from_row.line(format!("{field_name_escaped}: row.get_{mandatory_part}custom_parsed_at_idx(\"{field_name}\", field_mapping.0[{idx}], mmsdm_core::mms_datetime::parse)?,"));
                }

                _ if col.data_stored_in_string() => {
                    from_row.line(format!(
                        "{field_name_escaped}: row.get_{mandatory_part}range(\"{field_name}\", field_mapping.0[{idx}])?,"
                    ));
                }
                mms::DataType::Decimal { .. }
                    if !(col.is_trading_period() || col.is_dispatch_period()) =>
                {
                    from_row.line(format!("{field_name_escaped}: row.get_{mandatory_part}custom_parsed_at_idx(\"{field_name}\", field_mapping.0[{idx}], mmsdm_core::mms_decimal::parse)?,"));
                }
                _ => {
                    from_row.line(format!("{field_name_escaped}: row.get_{mandatory_part}parsed_at_idx(\"{field_name}\", field_mapping.0[{idx}])?,"));
                }
            }
        }

        if table.has_any_string_columns() {
            from_row.line("backing_data: row,");
        } else {
            from_row.line("backing_data: core::marker::PhantomData,");
        };

        from_row.line("})");

        current_impl.push_fn(from_row);
    }

    {
        let mut field_mapping_from_row = codegen::Function::new("field_mapping_from_row");
        field_mapping_from_row.ret("mmsdm_core::Result<Self::FieldMapping>");
        field_mapping_from_row.generic("'a");

        field_mapping_from_row.arg("mut row", "mmsdm_core::CsvRow<'a>");

        // verify it is an I row
        field_mapping_from_row.line("if !row.is_heading() {");
        field_mapping_from_row.line("return Err(mmsdm_core::Error::UnexpectedRowType(alloc::format!(\"Expected an I row but got {row:?}\")));");
        field_mapping_from_row.line("}");

        // verify file key matches
        field_mapping_from_row.line("let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;");
        field_mapping_from_row.line("if !Self::matches_file_key(&row_key, row_key.version) {");
        field_mapping_from_row.line("return Err(mmsdm_core::Error::UnexpectedRowType(alloc::format!(\"Expected a row matching {}.{}.v{} but got {row_key}\", Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION)));");
        field_mapping_from_row.line("}");

        // calculate mapping
        field_mapping_from_row.line("let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;");
        field_mapping_from_row
            .line("for (field_index, field) in Self::COLUMNS.iter().enumerate() {");
        // usize MAX is fine here, this will only happen for nullable fields.
        field_mapping_from_row.line("base_mapping[field_index] = row.iter_fields().position(|f| f == *field).unwrap_or(usize::MAX);");
        field_mapping_from_row.line("}");
        field_mapping_from_row.line(format!(
            "Ok({}(base_mapping))",
            pdr_report.get_rust_struct_mapping_name()
        ));

        current_impl.push_fn(field_mapping_from_row);
    }

    // {
    //     let mut partition_suffix_from_row = codegen::Function::new("partition_suffix_from_row");
    //     partition_suffix_from_row.ret("mmsdm_core::Result<Self::Partition>");
    //     partition_suffix_from_row.generic("'a");

    //     if let ControlFlow::Break(col) = table.partition_column() {
    //         partition_suffix_from_row.arg("row", "mmsdm_core::CsvRow<'a>");
    //         let (idx, _) = table
    //             .columns()
    //             .columns
    //             .iter()
    //             .enumerate()
    //             .find(|(_, search_col)| search_col.field_name() == col.field_name())
    //             .unwrap();
    //         let field_name = col.field_name();
    //         let idx = idx + 4;
    //         let field_name_escaped = col.escaped_field_name();

    //         match &col.data_type {
    //             mms::DataType::DateTime | mms::DataType::Date
    //                 if !(col.is_trading_period() || col.is_dispatch_period()) =>
    //             {
    //                 partition_suffix_from_row.line(format!(r#"let {field_name_escaped}= row.get_custom_parsed_at_idx("{field_name}", {idx}, mmsdm_core::mms_datetime::parse)? - {};"#, table.frequency().duration()));
    //             }
    //             _ if col.is_trading_period() => {
    //                 partition_suffix_from_row.line(format!(r#"let {field_name_escaped}: mmsdm_core::TradingPeriod = row.get_parsed_at_idx("{field_name}",{idx})?;"#));
    //             }
    //             _ if col.is_dispatch_period() => {
    //                 partition_suffix_from_row.line(format!(r#"let {field_name_escaped}: mmsdm_core::DispatchPeriod = row.get_parsed_at_idx("{field_name}",{idx})?;"#));
    //             }
    //             otherwise => {
    //                 // panic!("Invalid partition column: {col:?} of type {:?}", col.data_type);
    //             }
    //         }

    //         partition_suffix_from_row.line(format!(r#"Ok(mmsdm_core::YearMonth {{ year: chrono::NaiveDateTime::from({colname}).year(), month: num_traits::FromPrimitive::from_u32(chrono::NaiveDateTime::from({colname}).month()).unwrap() }})"#, colname = col.field_name()));
    //     } else {
    //         partition_suffix_from_row.arg("_row", "mmsdm_core::CsvRow<'a>");
    //         partition_suffix_from_row.line("Ok(())");
    //     }
    //     current_impl.push_fn(partition_suffix_from_row);
    // }

    let mut file_version = codegen::Function::new("matches_file_key");
    file_version.arg("key", "&mmsdm_core::FileKey<'_>");
    file_version.arg("version", "i32");
    file_version.ret("bool");
    file_version.line("version == key.version && Self::DATA_SET_NAME == key.data_set_name() && Self::TABLE_NAME == key.table_name()");
    current_impl.push_fn(file_version);

    current_impl.associate_type("FieldMapping", pdr_report.get_rust_struct_mapping_name());
    current_impl.associate_const("VERSION", "i32", pdr_report.version.to_string(), "");
    current_impl.associate_const(
        "DATA_SET_NAME",
        "&'static str",
        format!("\"{}\"", pdr_report.name),
        "",
    );
    current_impl.associate_const(
        "TABLE_NAME",
        "&'static str",
        format!("\"{}\"", pdr_report.sub_type.as_deref().unwrap_or_default()),
        "",
    );

    // the +4 here is to offset for the skipped columns!
    let array_contents = table
        .columns()
        .enumerate()
        .map(|(idx, _)| format!("{}", idx + 4))
        .collect::<Vec<_>>()
        .join(",");

    current_impl.associate_const(
        "DEFAULT_FIELD_MAPPING",
        "Self::FieldMapping",
        format!(
            "{}([{array_contents}])",
            pdr_report.get_rust_struct_mapping_name()
        ),
        "",
    );

    let array_contents = table
        .columns()
        .map(|c| format!("\"{}\"", c.name))
        .collect::<Vec<_>>()
        .join(",");
    current_impl.associate_const(
        "COLUMNS",
        "&'static [&'static str]",
        format!("&[{array_contents}]"),
        "",
    );

    let mut primary_key = codegen::Function::new("primary_key");
    primary_key.ret(&pdr_report.get_rust_pk_name());
    primary_key.arg("row", "&Self::Row<'_>");
    primary_key.line(&format!(
        r#"{pk_name} {{
{pk_fields}
}}"#,
        pk_name = pdr_report.get_rust_pk_name(),
        pk_fields = table
            .primary_key_columns()
            .iter()
            .map(|name| format!(
                "    {0}: row.{0}{1}",
                lowercase_and_escape(name),
                table
                    .columns()
                    .find(|col| &col.name == name)
                    .unwrap()
                    .to_string_or_nothing()
            ))
            .collect::<Vec<String>>()
            .join(",\n"),
    ));

    current_impl.push_fn(primary_key);

    current_impl.associate_type("PrimaryKey", pdr_report.get_rust_pk_name());

    let mut partition_value = codegen::Function::new("partition_value");
    partition_value.ret("mmsdm_core::PartitionValue");
    partition_value.arg_ref_self();
    partition_value.arg("row", "&Self::Row<'_>");
    partition_value.line("(self.extract_row_partition)(row)");
    current_impl.push_fn(partition_value);

    let mut partition_name = codegen::Function::new("partition_name");
    partition_name.ret("alloc::string::String");
    partition_name.arg_ref_self();
    partition_name.arg("row", "&Self::Row<'_>");
    partition_name.line(&format!(
        r#"alloc::format!("{}_{{}}", self.partition_value(row))"#,
        pdr_report.get_partition_base()
    ));
    current_impl.push_fn(partition_name);

    let mut partition_key = codegen::Function::new("partition_key");
    partition_key.ret("mmsdm_core::PartitionKey");
    partition_key.arg_ref_self();
    partition_key.line("self.row_partition_key");
    current_impl.push_fn(partition_key);

    {
        let mut to_owned = codegen::Function::new("to_static");
        to_owned.generic("'a");
        to_owned.arg("row", "&Self::Row<'a>");
        to_owned.ret("Self::Row<'static>");

        to_owned.line(&format!("{} {{", pdr_report.get_rust_struct_name()));

        for (_, col) in table.columns().enumerate() {
            let field_name = col.field_name();

            let field_name_escaped = if KW.contains(&field_name.as_str()) {
                format!("r#{field_name}")
            } else {
                field_name.to_string()
            };

            // this .clone() is cheap as all fields are copy!
            to_owned.line(format!(
                "{field_name_escaped}: row.{field_name_escaped}.clone(),"
            ));
        }

        if table.has_any_string_columns() {
            to_owned.line("backing_data: row.backing_data.to_owned(),");
        } else {
            to_owned.line("backing_data: core::marker::PhantomData,");
        };

        to_owned.line("}");

        current_impl.push_fn(to_owned);
    }

    current_impl.fmt(fmtr)?;
    Ok(())
}

fn codegen_pk(
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    let mut pk_struct = codegen::Struct::new(&pdr_report.get_rust_pk_name());
    pk_struct
        .vis("pub")
        .derive("Debug")
        .derive("Clone")
        .derive("PartialEq")
        .derive("Eq")
        .derive("PartialOrd")
        .derive("Ord");

    for pk_col_name in table.primary_key_columns().iter() {
        let col = table
            .columns()
            .find(|col| &col.name == pk_col_name)
            .expect("PK column must exist");

        // temporary
        if !col.mandatory {
            panic!("Non mandatory column in primary key: {:?}", col);
        }

        if col.is_dispatch_period() {
            // parse as DispatchPeriod
            let field = codegen::Field::new(
                &format!("pub {}", col.field_name()),
                "mmsdm_core::DispatchPeriod",
            );
            pk_struct.push_field(field);
        } else if col.is_trading_period() {
            // parse as TradingPeriod
            let field = codegen::Field::new(
                &format!("pub {}", col.field_name()),
                "mmsdm_core::TradingPeriod",
            );
            pk_struct.push_field(field);
        // } else if col.data_type == mms::DataType::Date {
        //     let field = codegen::Field::new(
        //         &format!("pub {}", col.field_name()),
        //         &col.to_rust_type(),
        //     );
        //     pk_struct.push_field(field);
        } else {
            let field = codegen::Field::new(
                &format!("pub {}", col.escaped_field_name()),
                &col.to_rust_type(),
            );
            pk_struct.push_field(field);
        };
    }

    pk_struct.fmt(fmtr)?;

    Ok(())
}

fn codegen_impl_pk(pdr_report: &pdr::Report, fmtr: &mut codegen::Formatter) -> anyhow::Result<()> {
    let mut pk_trait = codegen::Impl::new(pdr_report.get_rust_pk_name());
    pk_trait.impl_trait("mmsdm_core::PrimaryKey");
    pk_trait.fmt(fmtr)?;
    Ok(())
}

fn codegen_impl_compare_with_row_on_struct(
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    let mut compare_with_row_impl =
        codegen::Impl::new(format!("{}<'data>", pdr_report.get_rust_struct_name()));
    compare_with_row_impl.impl_trait("mmsdm_core::CompareWithRow");
    compare_with_row_impl.generic("'data");
    compare_with_row_impl.associate_type(
        "Row<'other>",
        format!("{}<'other>", pdr_report.get_rust_struct_name()),
    );
    let mut compare_with_other = codegen::Function::new("compare_with_row");
    compare_with_other.generic("'other");
    compare_with_other.ret("bool");
    compare_with_other.arg_ref_self();
    compare_with_other.arg("row", "&Self::Row<'other>");
    compare_with_other.line(
        &table
            .primary_key_columns()
            .iter()
            .map(|name| {
                if table
                    .columns()
                    .find(|c| c.name == *name)
                    .unwrap()
                    .data_stored_in_string()
                {
                    format!("self.{0}() == row.{0}()", lowercase_and_escape(name))
                } else {
                    format!("self.{0} == row.{0}", lowercase_and_escape(name))
                }
            })
            .collect::<Vec<String>>()
            .join("\n&& "),
    );
    compare_with_row_impl.push_fn(compare_with_other);
    compare_with_row_impl.fmt(fmtr)?;

    Ok(())
}

fn codegen_impl_compare_with_pk_on_struct(
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    let mut compare_with_pk_impl =
        codegen::Impl::new(format!("{}<'data>", pdr_report.get_rust_struct_name()));
    compare_with_pk_impl.generic("'data");
    compare_with_pk_impl.impl_trait("mmsdm_core::CompareWithPrimaryKey");
    compare_with_pk_impl.associate_type("PrimaryKey", pdr_report.get_rust_pk_name());
    let mut compare_with_key = codegen::Function::new("compare_with_key");
    compare_with_key.ret("bool");
    compare_with_key.line(
        &table
            .primary_key_columns()
            .iter()
            .map(|name| {
                if table
                    .columns()
                    .find(|c| c.name == *name)
                    .unwrap()
                    .data_stored_in_string()
                {
                    format!("self.{0}() == key.{0}", lowercase_and_escape(name))
                } else {
                    format!("self.{0} == key.{0}", lowercase_and_escape(name))
                }
            })
            .collect::<Vec<String>>()
            .join("\n&& "),
    );
    compare_with_key.arg_ref_self();
    compare_with_key.arg("key", "&Self::PrimaryKey");
    compare_with_pk_impl.push_fn(compare_with_key);
    compare_with_pk_impl.fmt(fmtr)?;
    Ok(())
}

fn codegen_impl_compare_with_row_on_pk(
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    let mut pk_compare_row_impl = codegen::Impl::new(pdr_report.get_rust_pk_name());
    pk_compare_row_impl.impl_trait("mmsdm_core::CompareWithRow");
    pk_compare_row_impl.generic("'data");
    pk_compare_row_impl.associate_type(
        "Row<'other>",
        format!("{}<'other>", pdr_report.get_rust_struct_name()),
    );
    let mut compare_with_row = codegen::Function::new("compare_with_row");
    compare_with_row.generic("'other");
    compare_with_row.ret("bool");
    compare_with_row.arg_ref_self();
    compare_with_row.arg("row", "&Self::Row<'other>");
    compare_with_row.line(
        &table
            .primary_key_columns()
            .iter()
            .map(|name| {
                if table
                    .columns()
                    .find(|c| c.name == *name)
                    .unwrap()
                    .data_stored_in_string()
                {
                    format!("self.{0} == row.{0}()", lowercase_and_escape(name))
                } else {
                    format!("self.{0} == row.{0}", lowercase_and_escape(name))
                }
            })
            .collect::<Vec<String>>()
            .join("\n&& "),
    );
    pk_compare_row_impl.push_fn(compare_with_row);
    pk_compare_row_impl.fmt(fmtr)?;
    Ok(())
}

fn codegen_impl_compare_with_pk_on_pk(
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    let mut pk_compare_pk_impl = codegen::Impl::new(pdr_report.get_rust_pk_name());
    pk_compare_pk_impl.impl_trait("mmsdm_core::CompareWithPrimaryKey");
    pk_compare_pk_impl.associate_type("PrimaryKey", pdr_report.get_rust_pk_name());
    let mut compare_with_other_pk = codegen::Function::new("compare_with_key");
    compare_with_other_pk.ret("bool");
    compare_with_other_pk.arg_ref_self();
    compare_with_other_pk.arg("key", "&Self::PrimaryKey");
    compare_with_other_pk.line(
        &table
            .primary_key_columns()
            .iter()
            .map(|name| format!("self.{0} == key.{0}", lowercase_and_escape(name)))
            .collect::<Vec<String>>()
            .join("\n&& "),
    );
    pk_compare_pk_impl.push_fn(compare_with_other_pk);
    pk_compare_pk_impl.fmt(fmtr)?;

    Ok(())
}

fn codegen_impl_arrow_schema(
    pdr_report: &pdr::Report,
    table: &Table,
    fmtr: &mut codegen::Formatter,
) -> anyhow::Result<()> {
    {
        let mut arrow_trait = codegen::Impl::new(&pdr_report.get_rust_manager_struct_name());
        arrow_trait.impl_trait("mmsdm_core::ArrowSchema");
        arrow_trait.r#macro("#[cfg(feature = \"arrow\")]");

        let mut arrow_schema = codegen::Function::new("schema");
        arrow_schema.ret("arrow::datatypes::Schema");

        let arrow_schema_line = format!(
            "arrow::datatypes::Schema::new(alloc::vec::Vec::from([
        {fields}
    ]))",
            fields = table
                .columns()
                .map(|col| col.as_arrow_field())
                .collect::<Vec<_>>()
                .join(",\n    "),
        );

        arrow_schema.line(arrow_schema_line);
        arrow_trait.push_fn(arrow_schema);

        arrow_trait.associate_type("Builder", pdr_report.get_rust_arrow_builder_name());

        let new_builder = arrow_trait.new_fn("new_builder");
        new_builder.ret("Self::Builder");
        new_builder.line(format!("{} {{", pdr_report.get_rust_arrow_builder_name()));
        for col in table.columns() {
            new_builder.line(format!(
                "{}: {},",
                col.as_arrow_array_name(),
                col.as_arrow_builder()
            ));
        }
        new_builder.line("}");

        let append_row = arrow_trait.new_fn("append_builder");
        append_row.arg("builder", "&mut Self::Builder");
        append_row.arg("row", "Self::Row<'_>");
        for col in table.columns() {
            append_row.line(&format!("    {}", col.as_arrow_array_extractor()));
        }

        let finalize = arrow_trait.new_fn("finalize_builder");
        finalize.arg("builder", "&mut Self::Builder");
        finalize.ret("mmsdm_core::Result<arrow::array::RecordBatch>");
        finalize.line(
            "arrow::array::RecordBatch::try_new(
    alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
    alloc::vec::Vec::from([",
        );

        for col in table.columns() {
            finalize.line(&format!(
                "        alloc::sync::Arc::new(builder.{}.finish()) as alloc::sync::Arc<dyn arrow::array::Array>,",
                col.as_arrow_array_name()
            ));
        }

        finalize.line(
            "    ])
    ).map_err(Into::into)",
        );

        arrow_trait.fmt(fmtr)?;
    }

    {
        let mut scope = Scope::new();
        scope.raw("#[cfg(feature = \"arrow\")]");

        scope.fmt(fmtr)?;
        let mut batch_builder = codegen::Struct::new(&pdr_report.get_rust_arrow_builder_name());
        batch_builder.vis("pub");
        for col in table.columns() {
            batch_builder.push_field(codegen::Field::new(
                &col.as_arrow_array_name(),
                col.as_arrow_builder_ty(),
            ));
        }

        batch_builder.fmt(fmtr)?;
    }

    Ok(())
}

fn prepare_data_set_crate(data_set: &str) -> anyhow::Result<()> {
    std::fs::create_dir_all(format!("crates/{}/src", data_set.to_snake_case()))?;

    fs::write(
        format!("crates/{}/Cargo.toml", data_set.to_snake_case()),
        format!(
            r#"[package]
name = "mmsdm_{data_set}"
version = "0.1.0"
edition = "2021"
license = "MIT"
repository = "https://github.com/eigenmo-de/mmsdm-rs"
description = "Parse and Transform MMSDM data"
resolver = "2"

[dependencies.tracing]
workspace = true
features = []

[dependencies.rust_decimal]
workspace = true
features = []

[dependencies.num-traits]
workspace = true
features = []

[dependencies.chrono]
workspace = true
default-features = false

[dependencies.arrow]
workspace = true
optional = true

[dependencies.mmsdm_core]
# version = "0.3.0"
path = "../mmsdm_core"
default-features = false

[features]
arrow = ["dep:arrow", "mmsdm_core/arrow", "mmsdm_core/std"]
default = []"#,
            data_set = data_set.to_snake_case(),
        ),
    )?;
    Ok(())
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)]
pub struct TableMapping {
    report_name: String, //	PDR Loader report name (CSV Record configuration) as shown in Replication Manager	DISPATCH
    report_sub_type: Option<String>, //	PDR Loader report sub type (CSV Record configuration) as shown in Replication Manager	PRICE
    version: u8, //	PDR Loader report version (CSV Record configuration) as shown in Replication Manager	2
    destination_table: String, //	Table name as per the MMS Data Model documentation	DISPATCHPRICE
    transaction_type: String, //	PDR Loader report transaction type - data loading action	INSERT-UPDATE
    row_filter_type: String, //	PDR Loader report transaction type - row filter in data loading action	LASTCHANGED
}

impl TableMapping {
    pub fn read() -> anyhow::Result<HashMap<mms::Report, pdr::Report>> {
        let mut mapping = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(format!("table_config_v{VERSION}.csv"))?;

        let mut map = collections::HashMap::<_, pdr::Report>::new();

        for row in mapping.deserialize::<TableMapping>() {
            let row = row?;

            map.entry(row.mms())
                .and_modify(|e| {
                    let other_pdr = row.pdr();
                    if other_pdr.version > e.version {
                        *e = other_pdr;
                    }
                })
                .or_insert(row.pdr());
        }
        Ok(map)
    }
}

impl TableMapping {
    pub fn mms(&self) -> mms::Report {
        mms::Report {
            sub_type: self.destination_table.clone(),
        }
    }
    pub fn pdr(&self) -> pdr::Report {
        pdr::Report {
            name: self.report_name.clone(),
            sub_type: self.report_sub_type.clone(),
            version: self.version,
            transaction_type: self.transaction_type.clone(),
            row_filter: self.row_filter_type.clone(),
        }
    }
}

pub fn run(local_info: DataModel) -> anyhow::Result<()> {
    let map = TableMapping::read()?;

    for (data_set, package) in local_info.packages.iter() {
        info!("Processing data set {data_set}");

        if data_set == "HISTORICAL" {
            // skip
            continue;
        }
        // if data_set == "MTPASA" {
        //     // dbg!(&tables);
        //     data_set.push_str("_SOLUTION");
        // }
        prepare_data_set_crate(&data_set)
            .with_context(|| format!("prepare data set {data_set:?}"))?;

        let mut fmt_str = String::new();
        fmt_str.push_str("#![no_std]\n#![allow(unused_imports)]\nextern crate alloc;\nuse alloc::string::ToString;\nuse chrono::Datelike as _;\n#[cfg(feature = \"arrow\")]\nextern crate std;");
        let mut fmtr = codegen::Formatter::new(&mut fmt_str);

        for (table_key, _table_header) in package.tables.iter() {
            println!("Processing table {table_key}");

            let table = local_info
                .tables
                .get(table_key)
                .ok_or_else(|| anyhow!("missing table {table_key}"))?;

            // panic!();
            let mms_report = mms::Report {
                sub_type: table_key.clone(),
            };

            if mms_report.should_skip() {
                continue;
            }

            // dbg!(&data_set, &table_key, &table);

            match map.get(&mms_report) {
                Some(pdr_report) => {
                    codegen_struct(table_key, pdr_report, &table, &mut fmtr)
                        .context("codegen_struct")?;
                    // codegen_impl_from_row(pdr_report, &table, &mut fmtr).context("codegen_impl_from_row")?;
                    codegen_impl_get_table(pdr_report, &table, &mut fmtr)
                        .context("codegen_impl_get_table")?;

                    codegen_pk(pdr_report, &table, &mut fmtr).context("codegen_pk")?;
                    codegen_impl_pk(pdr_report, &mut fmtr).context("codegen_impl_pk")?;

                    codegen_impl_compare_with_row_on_struct(pdr_report, &table, &mut fmtr)
                        .context("codegen_impl_compare_with_row_on_struct")?;
                    codegen_impl_compare_with_pk_on_struct(pdr_report, &table, &mut fmtr)
                        .context("codegen_impl_compare_with_pk_on_struct")?;

                    codegen_impl_compare_with_row_on_pk(pdr_report, &table, &mut fmtr)
                        .context("codegen_impl_compare_with_row_on_pk")?;
                    codegen_impl_compare_with_pk_on_pk(pdr_report, &table, &mut fmtr)
                        .context("codegen_impl_compare_with_pk_on_pk")?;

                    // TODO, fix this!
                    codegen_impl_arrow_schema(pdr_report, &table, &mut fmtr)?;
                }
                None => eprintln!("Cannot find PDR mapping for MMS Report: {mms_report:?}"),
            }
        }

        // apply_all_fn.fmt(false, &mut fmtr)?;
        let path = format!("crates/{}/src/lib.rs", data_set.to_snake_case());

        let formatted = match syn::parse_file(&fmt_str).map(|parsed| prettyplease::unparse(&parsed))
        {
            Ok(formatted) => formatted,
            _ => {
                eprintln!("Failed to format file {path}, saving anyway");
                fmt_str
            }
        };

        fs::write(&path, formatted).with_context(|| format!("writing out to {path}"))?;
    }

    Ok(())
}

fn lowercase_and_escape(col_name: &str) -> String {
    if crate::KW.contains(&col_name.to_lowercase().as_str()) {
        format!("r#{}", col_name.to_lowercase())
    } else {
        col_name.to_lowercase()
    }
}
