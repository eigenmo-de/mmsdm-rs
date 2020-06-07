use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections;
use std::{fs, iter, str, string};

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


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MMSTableSummary {
    name: String,
    comment: String,
}

impl MMSTableSummary {
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
            MMSDataType::Varchar { .. }=> "String",
            MMSDataType::Char=> "char",
            MMSDataType::Date=> "chrono::NaiveDateTime",
            MMSDataType::Decimal { .. } => "rust_decimal::Decimal",
            MMSDataType::Integer { .. }=> "i64",
        }.into()
    }
    fn parse_varchar(s: &str) -> anyhow::Result<MMSDataType> {
        let length = VARCHAR
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Varchar", s))?
            [2]
            .parse()?;
        Ok(MMSDataType::Varchar { length })
    }
    fn parse_int(s: &str) -> anyhow::Result<MMSDataType> {
        let precision = INTEGER
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Integer", s))?
            [1]
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
    fn get_struct_doc(&self) -> String {
        use heck::TitleCase;
        format!(
r#"Data Set Name: {mms_data_set_name}
File Name: {mms_file_name}
Data Version: {version}"#,
    mms_data_set_name = self.name.to_title_case(),
    mms_file_name = self.sub_type.to_title_case(),
    version = self.version,
        )
    }
    fn struct_name(&self) -> String {
        use heck::CamelCase;
        format!("{}{}{}", self.name.to_camel_case(), self.sub_type.to_camel_case(), self.version)
        //format!("{}", self.sub_type.to_camel_case())
    }
    fn data_set_name(&self) -> String {
        use heck::SnakeCase;
        format!("{}", self.name.to_snake_case())
    }
    fn file_key_literal(&self) -> String {
        use heck::ShoutySnakeCase;
        format!(r#"
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if let Ok(rdr) = fs::File::open("mmsdm.json") {
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
        //let settlements = local_info.get("SETTLEMENT_DATA").unwrap();
        for (data_set, tables) in local_info.into_iter() {
            let mut fmt_str = String::new();
            let mut fmtr = codegen::Formatter::new(&mut fmt_str);
            let mut pdr_data_set = None;
            for (table_key, table) in tables.into_iter() {
                let mms_report = MMSReport { name: data_set.clone(), sub_type: table_key };
                if let Some(pdr_report) = map.get(&mms_report) {
                    pdr_data_set = Some(pdr_report.data_set_name());
                    let mut current_struct = codegen::Struct::new(&pdr_report.struct_name());
                    current_struct
                        .vis("pub")
                        .doc(&pdr_report.get_struct_doc())
                        .derive("Debug")
                        .derive("Clone")
                        .derive("PartialEq")
                        .derive("Eq")
                        .derive("serde::Deserialize")
                        .derive("serde::Serialize");
                    for col in table.columns.columns {
                        if &col.field_name() == "type" {
                            let mut field = codegen::Field::new("type_", &col.to_rust_type());
                            field.annotation(vec!("#[serde(rename = \"type\")]"));
                            current_struct.push_field(field);
                        } else if col.data_type == MMSDataType::Date {
                            let mut field = codegen::Field::new(&col.field_name(), &col.to_rust_type());
                            if col.mandatory {
                                field.annotation(vec!("#[serde(with = \"crate::mms_datetime\")]"));
                            } else {
                                field.annotation(vec!("#[serde(with = \"crate::mms_datetime_opt\")]"));
                            };
                            current_struct.push_field(field);

                        } else {
                            let mut field = codegen::Field::new(&col.field_name(), &col.to_rust_type());
                            field.doc(vec![&col.get_comment()]);
                            current_struct.push_field(field);
                            //current_struct.field(&col.field_name(), &col.to_rust_type());
                        };
                    }
                    current_struct.fmt(&mut fmtr)?;

                    let mut current_impl = codegen::Impl::new("crate::AemoFile");
                    current_impl.impl_trait(format!("crate::GetTable<{}>", pdr_report.struct_name()));

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
                fs::write(format!("src/mmsdm/{}.rs", data_set.to_snake_case()), fmt_str)?;
//            if let Some(data_set_name) = pdr_data_set {
//                fs::write(format!("src/mmsdm/{}.rs", data_set_name), fmt_str)?;
//            }
        }

        Ok(())
    } else {
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
                    //dbg!(body);
                    let doc = scraper::Html::parse_document(&body);
                    //let selector = scraper::Selector::parse("body").unwrap();
                    //let htmlbody = doc.select(&selector).next().unwrap();
                    //let els = htmlbody.children().filter_map(scraper::element_ref::ElementRef::wrap).collect::<Vec<_>>();
                    let mut headings = doc.select(&H3);

                    use scraper::element_ref;

                    // extract the table name.  This is unkwnown but is always the first
                    let table_name = headings.next().unwrap();
                    //let table_heading = table_name.select(&link_selector).next().unwrap().inner_html();
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
                            .ok_or_else(|| anyhow!("Missing required field Primary Key Columns"))
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
        //dbg!(info);
        Ok(())
    }
}

fn swap_nonreq<T>(or: Option<anyhow::Result<T>>) -> anyhow::Result<Option<T>> {
    match or {
        Some(Ok(o)) => Ok(Some(o)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}
