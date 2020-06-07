use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections, fs, iter, str, string};
use structopt::StructOpt;

lazy_static::lazy_static! {
    static ref TR: scraper::Selector = scraper::Selector::parse("tr").unwrap();
    static ref TD: scraper::Selector = scraper::Selector::parse("td").unwrap();
    static ref A: scraper::Selector = scraper::Selector::parse("a").unwrap();
    static ref P: scraper::Selector = scraper::Selector::parse("p").unwrap();
    static ref H3: scraper::Selector = scraper::Selector::parse("h3").unwrap();
    static ref SPAN: scraper::Selector = scraper::Selector::parse("span").unwrap();

}

type MMSPackages = collections::HashMap<String, collections::HashMap<String, MMSTablePage>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSTablePage {
    summary: MMSTableSummary,
    description: Option<MMSDescription>,
    notes: Option<MMSTableNotes>,
    primary_key_columns: MMSPkColumns,
    columns: MMSTableColumns,
}

impl MMSTablePage {
    fn get_doc(&self, report: &PDRReport) -> String {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSTableSummary {
    name: String,
    comment: String,
}

impl MMSTableSummary {
    fn get_doc(&self) -> String {
        format!("## {}\n _{}_", self.name, self.comment)
    }
    fn from_html(tab: &scraper::element_ref::ElementRef) -> anyhow::Result<MMSTableSummary> {
        let mut cells = tab.select(&TD);
        let name_el = cells
            .nth(1)
            .ok_or_else(|| anyhow!("Name cell missing from sumary table"))?;
        let name = name_el
            .select(&A)
            .next()
            .ok_or_else(|| anyhow!("No content in summary table name"))?
            .inner_html()
            .replace(" ", "")
            .replace("\n", "");
        let comment_el = cells
            .nth(1)
            .ok_or_else(|| anyhow!("Comment cell missing from summary table"))?;
        let comment = comment_el
            .select(&P)
            .next()
            .ok_or_else(|| anyhow!("No content in summary table comment cell"))?
            .inner_html()
            .replace("\n", "");
        Ok(MMSTableSummary { name, comment })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSPkColumns {
    cols: Vec<String>,
}

impl MMSPkColumns {
    fn get_sql(&self) -> String {
        use heck::SnakeCase;
        let cols = self.cols.iter()
            .map(|c| c.to_snake_case())
            .collect::<Vec<_>>();
        format!("primary key ({})", cols.join(","))
    }
    fn get_doc(&self) -> String {
        self.cols
            .iter()
            .map(|c| format!("* {}", c))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn from_html(tab: &scraper::element_ref::ElementRef) -> anyhow::Result<MMSPkColumns> {
        let cols = tab
            .select(&P)
            .skip(1)
            .map(|er| er.inner_html().replace("\n", ""))
            .collect();
        Ok(MMSPkColumns { cols })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSDescription {
    inner: String,
}

impl MMSDescription {
    fn get_doc(&self) -> String {
        format!("# Description\n {}", self.inner)
    }
    fn from_html(tab: &scraper::element_ref::ElementRef) -> anyhow::Result<MMSDescription> {
        let inner = tab
            .select(&SPAN)
            .map(|er| er.inner_html())
            .collect::<Vec<_>>()
            .join(" ");
        Ok(MMSDescription { inner })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSTableNotes {
    notes: Vec<MMSTableNote>,
}
impl MMSTableNotes {
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
    fn from_html(tab: &scraper::element_ref::ElementRef) -> anyhow::Result<MMSTableNotes> {
        let notes = tab
            .select(&TR)
            .skip(1)
            .map(|el| MMSTableNote::from_html(&el))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(MMSTableNotes { notes })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSTableNote {
    name: String,
    comment: String,
    value: String,
}

impl MMSTableNote {
    fn get_doc(&self) -> String {
        format!("* ({}) {} {}", self.name, self.comment, self.value)
    }
    fn from_html(tab: &scraper::element_ref::ElementRef) -> anyhow::Result<MMSTableNote> {
        let mut cells = tab.select(&P);
        let name = cells.next().unwrap().inner_html().replace("\n", "");
        let comment = cells.next().unwrap().inner_html().replace("\n", "");
        let value = cells.next().unwrap().inner_html().replace("\n", "");
        Ok(MMSTableNote {
            name,
            comment,
            value,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSTableColumns {
    columns: Vec<MMSTableColumn>,
}
impl MMSTableColumns {
    fn get_sql(&self) -> String {
        self.columns.iter()
            .map(|c| format!("{},", c.get_sql()))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn get_columns_sql(&self, prefix: Option<&'static str>) -> String {
        self.columns.iter()
            .map(|c| if let Some(pfx) = prefix {
                    format!("{}.{}", pfx, c.field_name())
                } else {
                    format!("{}", c.field_name())
                }
            )
            .collect::<Vec<_>>()
            .join(",\n")
    }
    fn get_column_schema(&self) -> String {
        self.columns.iter()
            .map(|c| format!("{} {}", c.field_name(), c.data_type.as_sql_type()))
            .collect::<Vec<_>>()
            .join(",\n")
    }
    fn from_html(tab: &scraper::element_ref::ElementRef) -> anyhow::Result<MMSTableColumns> {
        let columns = tab
            .select(&TR)
            .skip(1)
            .map(|el| MMSTableColumn::from_html(&el))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(MMSTableColumns { columns })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSTableColumn {
    name: String,
    data_type: MMSDataType,
    mandatory: bool,
    comment: String,
}
impl MMSTableColumn {
    fn get_sql(&self) -> String {
        format!("{} {}", 
            self.field_name(),
            self.sql_type(),
        )
    }
    fn sql_type(&self) -> String {
        //if self.comment.contains("YYYYMMDDPP") {
        //} else if self.comment.contains("YYYYMMDDPPP") {
        //} else {
        //}
        if self.mandatory {
            format!("{} not null", self.data_type.as_sql_type())
        } else {
            format!("{} null", self.data_type.as_sql_type())
        }
    }
    fn get_comment(&self) -> &str {
        &self.comment
    }
    fn field_name(&self) -> String {
        use heck::SnakeCase;
        format!("{}", self.name.to_snake_case())
    }
    fn to_rust_type(&self) -> String {
        if self.mandatory {
            format!("{}", self.data_type.as_rust_type())
        } else {
            format!("Option<{}>", self.data_type.as_rust_type())
        }
    }
    fn from_html(tab: &scraper::element_ref::ElementRef) -> anyhow::Result<MMSTableColumn> {
        let mut cells = tab.select(&P);
        let name = cells.next().unwrap().inner_html().replace("\n", "");
        let data_type = cells
            .next()
            .unwrap()
            .inner_html()
            .replace("\n", "")
            .replace(" ", "")
            .parse()?;
        let mandatory = cells.next().unwrap().inner_html().starts_with("X");
        let comment = cells.next().unwrap().inner_html().replace("\n", "");
        Ok(MMSTableColumn {
            name,
            data_type,
            mandatory,
            comment,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum MMSDataType {
    Varchar { length: i32 },
    Char,
    Date,
    Decimal { precision: u8, scale: u8 },
    Integer { precision: u8 },
}

lazy_static::lazy_static! {
    static ref VARCHAR: regex::Regex = regex::Regex::new(r"varchar(2)?\((\d+)\)").unwrap();
    static ref DECIMAL: regex::Regex = regex::Regex::new(r"number\((\d+),(\d+)\)").unwrap();
    static ref INTEGER: regex::Regex = regex::Regex::new(r"number\((\d+)\)").unwrap();
}

impl MMSDataType {
    fn as_rust_type(&self) -> String {
        match self {
            MMSDataType::Varchar { .. } => "String",
            MMSDataType::Char => "char",
            MMSDataType::Date => "chrono::NaiveDateTime",
            MMSDataType::Decimal { .. } => "rust_decimal::Decimal",
            MMSDataType::Integer { .. } => "i64",
        }
        .into()
    }
    fn as_sql_type(&self) -> String {
        match self {
            MMSDataType::Varchar { length } => format!("varchar({})", length), 
            MMSDataType::Char => "char(1)".into(),
            MMSDataType::Date => "datetime2".into(),
            MMSDataType::Decimal { precision, scale } => format!("decimal({},{})", precision, scale),
            MMSDataType::Integer { precision } => format!("decimal({},0)", precision),
        }
    }
    fn parse_varchar(s: &str) -> anyhow::Result<MMSDataType> {
        let length = VARCHAR
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Varchar", s))?[2]
            .parse()?;
        Ok(MMSDataType::Varchar { length })
    }
    fn parse_int(s: &str) -> anyhow::Result<MMSDataType> {
        let precision = INTEGER
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Integer", s))?[1]
            .parse()?;
        Ok(MMSDataType::Integer { precision })
    }
    fn parse_decimal(s: &str) -> anyhow::Result<MMSDataType> {
        let caps = DECIMAL
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Varchar", s))?;
        let (precision, scale) = (caps[1].parse()?, caps[2].parse()?);
        Ok(MMSDataType::Decimal { precision, scale })
    }
}

impl str::FromStr for MMSDataType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<MMSDataType> {
        dbg!(s);
        let input = s.replace(" ", "").to_lowercase();
        let set = regex::RegexSet::new(&[
            r"date",
            r"char\(1\)",
            r"varchar\S+",
            r"number\((\d+)\)",
            r"number\((\d+),(\d+)\)",
        ])
        .unwrap();
        match set
            .matches(&input)
            .into_iter()
            .collect::<Vec<_>>()
            .as_slice()
        {
            [0] => Ok(MMSDataType::Date),
            [1] => Ok(MMSDataType::Char),
            [2] => MMSDataType::parse_varchar(&input),
            [3] => MMSDataType::parse_int(&input),
            [4] => MMSDataType::parse_decimal(&input),
            _ => {
                if input == "varchar(1)" {
                    MMSDataType::parse_varchar(&input)
                } else {
                    Err(anyhow!("Unexpected type {}", input))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PDRReport {
    name: String,
    sub_type: String,
    version: u8,
    transaction_type: String,
    row_filter: String,
}

impl PDRReport {
    fn get_doc(&self) -> String {
        use heck::TitleCase;
        format!(
            r#"* Data Set Name: {mms_data_set_name}
* File Name: {mms_file_name}
* Data Version: {version}"#,
            mms_data_set_name = self.name.to_title_case(),
            mms_file_name = self.sub_type.to_title_case(),
            version = self.version,
        )
    }
    fn struct_name(&self) -> String {
        use heck::CamelCase;
        format!(
            "{}{}{}",
            self.name.to_camel_case(),
            self.sub_type.to_camel_case(),
            self.version
        )
    }
    fn file_key_literal(&self) -> String {
        use heck::ShoutySnakeCase;
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

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
struct MMSReport {
    name: String,
    sub_type: String,
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Code generation on the MMS Data Model")]
enum AemoCodegen {
    Json,
    Rust,
    SqlServerTables,
    SqlServerRustPart,
    // Python,
    // ClickhouseTables,
    // ClickhouseRustPart
    // Parquet,
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    match AemoCodegen::from_args() {
        AemoCodegen::SqlServerTables => {
            let rdr = fs::File::open("mmsdm.json")?;
            let mapping = fs::read_to_string("table_mapping.csv").unwrap();
            let mut map = collections::HashMap::new();
            use std::iter::Iterator;
            for row in mapping.split("\n").skip(1) {
                if row.contains(',') {
                    let pieces = row.split(",").collect::<Vec<&str>>();
                    let mms = MMSReport {
                        name: pieces[0].to_string(),
                        sub_type: pieces[1].to_string(),
                    };
                    let pdr = PDRReport {
                        name: pieces[2].to_string(),
                        sub_type: pieces[3].to_string(),
                        version: pieces[4].parse().unwrap(),
                        transaction_type: pieces[5].to_string(),
                        row_filter: pieces[6].to_string(),
                    };
                    map.insert(mms, pdr);
                }
            }
            // abv
            let mut table_str = r#"
create table FileLog (
    id bigint identity(1,1) not null primary key,
    file_name varchar(255) not null,
    data_set varchar(255) not null,
    sub_type varchar(255) not null,
    version tinyint not null
)
go
            "#.to_string();
            let mut proc_str = String::new();
            let local_info: MMSPackages = serde_json::from_reader(rdr).unwrap();
            for (data_set, tables) in local_info.into_iter() {
                for (table_key, table) in tables.into_iter() {
                    let mms_report = MMSReport {
                        name: data_set.clone(),
                        sub_type: table_key,
                    };
                    if let Some(pdr_report) = map.get(&mms_report) {

                        let create_table = format!(r#"
create table {table_name} (
file_log_id bigint not null references FileLog(id),
{columns}
{primary_key}
)
go
                        "#,
                        table_name = pdr_report.struct_name(),
                        columns = table.columns.get_sql(),
                        primary_key = table.primary_key_columns.get_sql(),
                        );

                        table_str.push_str(&create_table);

                        use heck::CamelCase;
                        let insert_proc = format!(r#"
create or alter procedure Insert{target_table}
    @file_name varchar(255),
    @data nvarchar(max)
begin
declare @header table (id bigint not null);
insert into FileLog(file_name, data_set, sub_type, version)
values (@file_name, '{data_set}', '{sub_type}', {version})
output inserted.id into @header;

insert into {target_table}(
file_log_id,
{insert_columns}
)
select 
(select h.id from header h),
{select_columns}
from openjson(@data) with (
{column_schema}
) d
end
go"#,
                            target_table = pdr_report.struct_name(),
                            data_set = pdr_report.name.to_camel_case(),
                            sub_type = pdr_report.sub_type.to_camel_case(),
                            version = pdr_report.version,
                            insert_columns = table.columns.get_columns_sql(None),
                            select_columns = table.columns.get_columns_sql(Some("d")),
                            column_schema = table.columns.get_column_schema(),
                        );
                        proc_str.push_str(&insert_proc);
                    }
                }
            }
            fs::write("sql/mmsdm_tables.sql", table_str)?;
            fs::write("sql/mmsdm_procs.sql", proc_str)?;
            Ok(())
        }
        AemoCodegen::Rust => {
            let rdr = fs::File::open("mmsdm.json")?;
            let mapping = fs::read_to_string("table_mapping.csv").unwrap();
            let mut map = collections::HashMap::new();
            use std::iter::Iterator;
            for row in mapping.split("\n").skip(1) {
                if row.contains(',') {
                    let pieces = row.split(",").collect::<Vec<&str>>();
                    let mms = MMSReport {
                        name: pieces[0].to_string(),
                        sub_type: pieces[1].to_string(),
                    };
                    let pdr = PDRReport {
                        name: pieces[2].to_string(),
                        sub_type: pieces[3].to_string(),
                        version: pieces[4].parse().unwrap(),
                        transaction_type: pieces[5].to_string(),
                        row_filter: pieces[6].to_string(),
                    };
                    map.insert(mms, pdr);
                }
            }
            // abv
            let local_info: MMSPackages = serde_json::from_reader(rdr).unwrap();
            for (data_set, tables) in local_info.into_iter() {
                let mut fmt_str = String::new();
                let mut fmtr = codegen::Formatter::new(&mut fmt_str);
                for (table_key, table) in tables.into_iter() {
                    let mms_report = MMSReport {
                        name: data_set.clone(),
                        sub_type: table_key,
                    };
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
                                let mut field =
                                    codegen::Field::new("pub type_", &col.to_rust_type());
                                field.annotation(vec!["#[serde(rename = \"type\")]"]);
                                current_struct.push_field(field);
                            } else if col.comment.contains("YYYYMMDDPPP") {
                                // parse as DispatchPeriod
                                let mut field = codegen::Field::new(
                                    &format!("pub {}", col.field_name()),
                                    "crate::DispatchPeriod",
                                );
                                field.annotation(vec![
                                    "#[serde(with = \"crate::dispatch_period\")]",
                                ]);
                                current_struct.push_field(field);
                            } else if col.comment.contains("YYYYMMDDPP") {
                                // parse as TradingPeriod
                                let mut field = codegen::Field::new(
                                    &format!("pub {}", col.field_name()),
                                    "crate::TradingPeriod",
                                );
                                field
                                    .annotation(vec!["#[serde(with = \"crate::trading_period\")]"]);
                                current_struct.push_field(field);
                            } else if col.data_type == MMSDataType::Date {
                                let mut field = codegen::Field::new(
                                    &format!("pub {}", col.field_name()),
                                    &col.to_rust_type(),
                                );
                                if col.mandatory {
                                    field.annotation(vec![
                                        "#[serde(with = \"crate::mms_datetime\")]",
                                    ]);
                                } else {
                                    field.annotation(vec![
                                        "#[serde(with = \"crate::mms_datetime_opt\")]",
                                    ]);
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

                        let mut current_impl = codegen::Impl::new("crate::AemoFile");
                        current_impl
                            .impl_trait(format!("crate::GetTable<{}>", pdr_report.struct_name()));

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
                    format!("src/mmsdm/{}.rs", data_set.to_snake_case()),
                    fmt_str,
                )?;
            }

            Ok(())
        }
        AemoCodegen::Json => {
            let url = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/MMS%20Data%20Model%20Report_files/MMS%20Data%20Model%20Report_toc.htm";
            let body = reqwest::get(url).await?.text().await?;
            let doc = scraper::Html::parse_document(&body);

            let mut info: MMSPackages = collections::HashMap::new();
            let mut current_package = None;
            for el in doc.select(&A) {
                let text = el.inner_html();
                if text.starts_with("Package:") {
                    // later should split this
                    let package = text.split(" ").nth(1).map(string::ToString::to_string);
                    dbg!(&package);
                    current_package = package.clone();
                } else if text.starts_with("Table:") {
                    if let Some(current) = current_package.clone() {
                        let link_val = el.value().attr("href").unwrap();
                        dbg!(&link_val);
                        let url = format!("https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/MMS%20Data%20Model%20Report_files/{}", link_val);
                        let body = reqwest::get(&url).await?.text().await?;
                        let doc = scraper::Html::parse_document(&body);
                        let mut headings = doc.select(&H3);

                        use scraper::element_ref;

                        // extract the table name.  This is unkwnown but is always the first
                        let table_name = headings.next().unwrap();
                        let summary = element_ref::ElementRef::wrap(
                            table_name.next_sibling().unwrap().next_sibling().unwrap(),
                        )
                        .unwrap();

                        // now get other info
                        let mut details = collections::HashMap::new();
                        for h3 in headings {
                            let detail_type = h3.select(&A).next().unwrap().inner_html();
                            let detail_table = element_ref::ElementRef::wrap(
                                h3.next_sibling().unwrap().next_sibling().unwrap(),
                            )
                            .unwrap();
                            if detail_type != "Index" {
                                details.insert(detail_type.replace(" ", ""), detail_table);
                            }
                        }

                        let table_info = MMSTablePage {
                            columns: details
                                .get("Content")
                                .ok_or_else(|| anyhow!("Missing required field Content"))
                                .and_then(|t| MMSTableColumns::from_html(t))?,
                            description: swap_nonreq(
                                details.get("Description").map(MMSDescription::from_html),
                            )?,
                            notes: swap_nonreq(details.get("Notes").map(MMSTableNotes::from_html))?,
                            primary_key_columns: details
                                .get("PrimaryKeyColumns")
                                .ok_or_else(|| {
                                    anyhow!("Missing required field Primary Key Columns")
                                })
                                .and_then(MMSPkColumns::from_html)?,
                            summary: MMSTableSummary::from_html(&summary)?,
                        };
                        let key = table_info.summary.name.clone();
                        info.entry(current)
                            .and_modify(|e| {
                                e.insert(key.clone(), table_info.clone());
                            })
                            .or_insert(iter::once((key, table_info)).collect());
                    }
                };
            }
            let asstr = serde_json::to_string(&info).unwrap();
            fs::write("mmsdm.json", asstr).unwrap();
            Ok(())
        }
        _ => todo!(),
    }
}

fn swap_nonreq<T>(or: Option<anyhow::Result<T>>) -> anyhow::Result<Option<T>> {
    match or {
        Some(Ok(o)) => Ok(Some(o)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}
