use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections, str};
use crate::pdr;
use scraper::{element_ref, html};

lazy_static::lazy_static! {
    static ref TR: scraper::Selector = scraper::Selector::parse("tr").unwrap();
    static ref TD: scraper::Selector = scraper::Selector::parse("td").unwrap();
    static ref TABLE: scraper::Selector = scraper::Selector::parse("table").unwrap();
    static ref A: scraper::Selector = scraper::Selector::parse("a").unwrap();
    static ref P: scraper::Selector = scraper::Selector::parse("p").unwrap();
    static ref H3: scraper::Selector = scraper::Selector::parse("h3").unwrap();
    static ref SPAN: scraper::Selector = scraper::Selector::parse("span").unwrap();

}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Report {
    name: String,
    sub_type: String,
}

pub type Packages = collections::HashMap<String, collections::HashMap<String, TablePage>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TablePage {
    summary: TableSummary,
    description: Option<Description>,
    notes: Option<TableNotes>,
    primary_key_columns: PkColumns,
    columns: TableColumns,
}

impl TablePage {
    pub fn get_summary_name(&self) -> String {
        self.summary.get_name()
    }
    fn get_doc(&self, report: &pdr::Report) -> String {
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
    pub fn from_html(mut docs: Vec<html::Html>) -> anyhow::Result<TablePage> {
        let first = docs.remove(0);
        let mut headings = first.select(&H3);
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

        let mut extra_columns = Vec::new();
        for doc in docs.iter() {
            //dbg!(doc.root_element().html());
            //dbg!(doc.root_element().inner_html());
            let h3 = doc.select(&H3).next().unwrap();
            //assert_eq!(h3.inner_html().trim(), "Content");
            let heading = h3.inner_html();
            if heading.trim() != "Content" {
                dbg!(heading);
                break;
            }
            let tab = doc.select(&TABLE).next().unwrap();
            //dbg!(h3.next_sibling().unwrap().value().as_element());
            //let tab_outer = element_ref::ElementRef::wrap(h3.next_sibling().unwrap()).unwrap();
            //dbg!(tab_outer.inner_html());
            //dbg!(h3.next_sibling().unwrap().value().as_element().unwrap().name());
//            let detail_table = element_ref::ElementRef::wrap(
//                h3.next_sibling().unwrap().next_sibling().unwrap(),
//            )
//            .unwrap();

            let col = TableColumns::from_html(&tab)?;
            extra_columns.push(col);
        }

        let mut first_column_set = details
                .get("Content")
                .ok_or_else(|| anyhow!("Missing required field Content"))
                .and_then(|t| TableColumns::from_html(t))?;

        for extra in extra_columns {
            first_column_set.add_columns(extra);
        }

        let table_info = TablePage {
            columns: first_column_set,
            description: crate::swap_nonreq(
                details.get("Description").map(Description::from_html),
            )?,
            notes: crate::swap_nonreq(details.get("Notes").map(TableNotes::from_html))?,
            primary_key_columns: details
                .get("PrimaryKeyColumns")
                .ok_or_else(|| anyhow!("Missing required field Primary Key Columns"))
                .and_then(PkColumns::from_html)?,
            summary: TableSummary::from_html(&summary)?,
        };
        Ok(table_info)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSummary {
    name: String,
    comment: String,
}

impl TableSummary {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_doc(&self) -> String {
        format!("## {}\n _{}_", self.name, self.comment)
    }
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableSummary> {
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
        Ok(TableSummary { name, comment })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkColumns {
    cols: Vec<String>,
}

impl PkColumns {
    fn get_sql(&self) -> String {
        use heck::SnakeCase;
        let cols = self
            .cols
            .iter()
            .map(|c| c.to_snake_case())
            .collect::<Vec<_>>();
        //format!("primary key ({})", cols.join(","))
        format!("unique ([{}])", cols.join("],["))
    }
    fn get_doc(&self) -> String {
        self.cols
            .iter()
            .map(|c| format!("* {}", c))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<PkColumns> {
        let cols = tab
            .select(&P)
            .skip(1)
            .map(|er| er.inner_html().replace("\n", ""))
            .collect();
        Ok(PkColumns { cols })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    inner: String,
}

impl Description {
    fn get_doc(&self) -> String {
        format!("# Description\n {}", self.inner)
    }
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<Description> {
        let inner = tab
            .select(&SPAN)
            .map(|er| er.inner_html())
            .collect::<Vec<_>>()
            .join(" ");
        Ok(Description { inner })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNotes {
    notes: Vec<TableNote>,
}
impl TableNotes {
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
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableNotes> {
        let notes = tab
            .select(&TR)
            .skip(1)
            .map(|el| TableNote::from_html(&el))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(TableNotes { notes })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNote {
    name: String,
    comment: String,
    value: String,
}

impl TableNote {
    fn get_doc(&self) -> String {
        format!("* ({}) {} {}", self.name, self.comment, self.value)
    }
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableNote> {
        let mut cells = tab.select(&P);
        let name = cells.next().unwrap().inner_html().replace("\n", "");
        let comment = cells.next().unwrap().inner_html().replace("\n", "");
        let value = cells.next().unwrap().inner_html().replace("\n", "");
        Ok(TableNote {
            name,
            comment,
            value,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumns {
    columns: Vec<TableColumn>,
}
impl TableColumns {
    fn add_columns(&mut self, mut other: TableColumns) { 
        self.columns.append(&mut other.columns);
    }
    fn get_sql(&self) -> String {
        self.columns
            .iter()
            .map(|c| format!("{},", c.get_sql()))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn get_columns_sql(&self, prefix: Option<&'static str>) -> String {
        self.columns
            .iter()
            .map(|c| {
                if let Some(pfx) = prefix {
                    format!("{}.[{}]", pfx, c.field_name())
                } else {
                    format!("[{}]", c.field_name())
                }
            })
            .collect::<Vec<_>>()
            .join(",\n")
    }
    fn get_column_schema(&self) -> String {
        self.columns
            .iter()
            .map(|c| format!("[{}] {}", c.field_name(), c.data_type.as_sql_type()))
            .collect::<Vec<_>>()
            .join(",\n")
    }
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableColumns> {
        let columns = tab
            .select(&TR)
            .skip(1)
            .map(|el| TableColumn::from_html(&el))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(TableColumns { columns })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    name: String,
    data_type: DataType,
    mandatory: bool,
    comment: String,
}
impl TableColumn {
    fn get_sql(&self) -> String {
        format!("[{}] {}", self.field_name(), self.sql_type(),)
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
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableColumn> {
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
        Ok(TableColumn {
            name,
            data_type,
            mandatory,
            comment,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum DataType {
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

impl DataType {
    fn as_rust_type(&self) -> String {
        match self {
            DataType::Varchar { .. } => "String",
            DataType::Char => "char",
            DataType::Date => "chrono::NaiveDateTime",
            DataType::Decimal { .. } => "rust_decimal::Decimal",
            DataType::Integer { .. } => "i64",
        }
        .into()
    }
    fn as_sql_type(&self) -> String {
        match self {
            DataType::Varchar { length } => format!("varchar({})", length),
            DataType::Char => "char(1)".into(),
            DataType::Date => "datetime2".into(),
            DataType::Decimal { precision, scale } => {
                format!("decimal({},{})", precision, scale)
            }
            DataType::Integer { precision } => format!("decimal({},0)", precision),
        }
    }
    fn parse_varchar(s: &str) -> anyhow::Result<DataType> {
        let length = VARCHAR
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Varchar", s))?[2]
            .parse()?;
        Ok(DataType::Varchar { length })
    }
    fn parse_int(s: &str) -> anyhow::Result<DataType> {
        let precision = INTEGER
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Integer", s))?[1]
            .parse()?;
        Ok(DataType::Integer { precision })
    }
    fn parse_decimal(s: &str) -> anyhow::Result<DataType> {
        let caps = DECIMAL
            .captures(s)
            .ok_or_else(|| anyhow!("Couldnt parse {} as Varchar", s))?;
        let (precision, scale) = (caps[1].parse()?, caps[2].parse()?);
        Ok(DataType::Decimal { precision, scale })
    }
}

impl str::FromStr for DataType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<DataType> {
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
            [0] => Ok(DataType::Date),
            [1] => Ok(DataType::Char),
            [2] => DataType::parse_varchar(&input),
            [3] => DataType::parse_int(&input),
            [4] => DataType::parse_decimal(&input),
            _ => {
                if input == "varchar(1)" {
                    DataType::parse_varchar(&input)
                } else {
                    Err(anyhow!("Unexpected type {}", input))
                }
            }
        }
    }
}
