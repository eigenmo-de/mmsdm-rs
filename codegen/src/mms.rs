use anyhow::anyhow;
use scraper::{element_ref, html};
use serde::{Deserialize, Serialize};
use std::{collections, str, ops::ControlFlow};

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
    pub name: String,
    pub sub_type: String,
}

impl Report {
    pub fn should_skip(&self) -> bool {
        // skip historical dataset - there are no table definitions anyway
        (self.name == "HISTORICAL")
        || (self.name == "CONFIGURATION")
        // all the below seems to be missing
        || (self.name == "ANCILLARY_SERVICES" && self.sub_type == "CONTRACTAGC")
        || (self.name == "ANCILLARY_SERVICES" && self.sub_type == "CONTRACTLOADSHED")
        || (self.name == "ANCILLARY_SERVICES" && self.sub_type == "CONTRACTREACTIVEPOWER")
        || (self.name == "ANCILLARY_SERVICES" && self.sub_type == "CONTRACTRESTARTSERVICES")
        || (self.name == "ANCILLARY_SERVICES" && self.sub_type == "CONTRACTRESTARTUNITS")
        || (self.name == "DEMAND_FORECASTS" && self.sub_type == "INTERMITTENT_P5_RUN")
        || (self.name == "DEMAND_FORECASTS" && self.sub_type == "INTERMITTENT_P5_PRED")
        || (self.name == "BILLING_RUN" && self.sub_type == "BILLINGAPCCOMPENSATION")
        || (self.name == "BILLING_RUN" && self.sub_type == "BILLINGAPCRECOVERY")
        || (self.name == "BILLING_RUN" && self.sub_type == "BILLING_RES_TRADER_RECOVERY")
        || (self.name == "BILLING_RUN" && self.sub_type == "BILLING_RES_TRADER_PAYMENT")
        || (self.name == "BILLING_RUN" && self.sub_type == "BILLINGIRFM")
        || (self.name == "BILLING_RUN" && self.sub_type == "BILLING_DIRECTION_RECONCILIATN")
        || (self.name == "BILLING_RUN" && self.sub_type == "BILLWHITEHOLE")
        || (self.name == "SETTLEMENT_DATA" && self.sub_type == "SETRESERVERECOVERY")
        || (self.name == "SETTLEMENT_DATA" && self.sub_type == "SETLSHEDRECOVERY")
        || (self.name == "SETTLEMENT_DATA" && self.sub_type == "SETRPOWERRECOVERY")
        || (self.name == "VOLTAGE_INSTRUCTIONS")
    }
}

pub type Packages = collections::BTreeMap<String, collections::BTreeMap<String, TablePage>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TablePage {
    pub summary: TableSummary,
    pub description: Option<Description>,
    pub notes: Option<TableNotes>,
    primary_key_columns: PkColumns,
    columns: TableColumns,
}

impl TablePage {
    pub fn columns(&self) -> TableColumns {
        let mut base = self.columns.clone();

        // if intervention exists, it must be mandatory
        // as it must also be in the primary key
        if let Some(intervention) =base.columns.iter_mut().find(|c| c.name.to_lowercase() == "intervention") {
            intervention.mandatory = true;
        }
        base
    }

    pub fn find_column(&self, name: &str) -> ControlFlow<TableColumn, ()> {        
        // make sure in pk
        if !self.primary_key_columns().cols.iter().any(|c| c.to_lowercase() == name) {
            return ControlFlow::Continue(());
        }
        
        if let Some(col) = self.columns.columns.iter().find(|c| c.name.to_lowercase() == name) {
            // make sure mandatory
            if !col.mandatory {
                return ControlFlow::Continue(());
            }

            return ControlFlow::Break(col.clone());
        }

        ControlFlow::Continue(())
    }

    pub fn partition_column(&self) -> ControlFlow<TableColumn, ()> {

            // in preference order
            self.find_column("predispatchseqno")?;            
            self.find_column("effectivedate")?;
            self.find_column("tradingdate")?;
            self.find_column("interval_datetime")?;
            self.find_column("settlementdate")?;
            self.find_column("day")?;
            self.find_column("offerdate")?;
            
            // these are more like transaction time so
            // they are last preference
            self.find_column("run_datetime")?;
            self.find_column("offerdatetime")?;
            self.find_column("publish_datetime")?;
            
            ControlFlow::Continue(())
    }

    pub fn primary_key_columns(&self) -> PkColumns {
        let mut base = self.primary_key_columns.clone();

        // if intervention exists, it must be in the primary key
        if let Some(intervention) = self.columns.columns.iter().find(|c| c.name.to_lowercase() == "intervention") {
            if !base.cols.contains(&intervention.name) {
                base.cols.push(intervention.name.to_string());
            }
        }

        base
    }
    pub fn get_summary_name(&self) -> String {
        self.summary.get_name()
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
            let detail_table =
                element_ref::ElementRef::wrap(h3.next_sibling().unwrap().next_sibling().unwrap())
                    .unwrap();
            if detail_type != "Index" {
                details.insert(detail_type.replace(' ', ""), detail_table);
            }
        }

        let mut extra_columns = Vec::new();
        for doc in docs.iter() {
            let h3 = doc.select(&H3).next().unwrap();
            let heading = h3.inner_html();
            if heading.trim() != "Content" {
                dbg!(heading);
                break;
            }
            let tab = doc.select(&TABLE).next().unwrap();

            let col = TableColumns::from_html(&tab)?;
            extra_columns.push(col);
        }

        let mut first_column_set = details
            .get("Content")
            .ok_or_else(|| anyhow!("Missing required field Content"))
            .and_then(TableColumns::from_html)?;

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
    pub name: String,
    pub comment: String,
}

impl TableSummary {
    pub fn get_name(&self) -> String {
        self.name.clone()
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
            .replace([' ','\n'], "");
        let comment_el = cells
            .nth(1)
            .ok_or_else(|| anyhow!("Comment cell missing from summary table"))?;
        let comment = comment_el
            .select(&P)
            .next()
            .ok_or_else(|| anyhow!("No content in summary table comment cell"))?
            .inner_html()
            .replace('\n', "");
        Ok(TableSummary { name, comment })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkColumns {
    pub cols: Vec<String>,
}

impl PkColumns {
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<PkColumns> {
        let cols = tab
            .select(&P)
            .skip(1)
            .map(|er| er.inner_html().replace('\n', ""))
            .collect();
        Ok(PkColumns { cols })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub inner: String,
}

impl Description {
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
    pub notes: Vec<TableNote>,
}

impl TableNotes {
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
    pub name: String,
    pub comment: String,
    pub value: String,
}

impl TableNote {
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableNote> {
        let mut cells = tab.select(&P);
        let name = cells.next().unwrap().inner_html().replace('\n', "");
        let comment = cells.next().unwrap().inner_html().replace('\n', "");
        let value = cells.next().unwrap().inner_html().replace('\n', "");
        Ok(TableNote {
            name,
            comment,
            value,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumns {
    pub columns: Vec<TableColumn>,
}

impl TableColumns {
    fn add_columns(&mut self, mut other: TableColumns) {
        self.columns.append(&mut other.columns);
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
    pub name: String,
    pub data_type: DataType,
    pub mandatory: bool,
    pub comment: String,
}

impl TableColumn {
    pub fn field_name(&self) -> String {
        use heck::SnakeCase;
        self.name.to_snake_case()
    }
    fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableColumn> {
        let mut cells = tab.select(&P);
        let name = cells.next().unwrap().inner_html().replace('\n', "");
        let data_type = cells
            .next()
            .unwrap()
            .inner_html()
            .replace(['\n', ' '], "")
            .parse()?;
        let mandatory = cells.next().unwrap().inner_html().starts_with('X');
        let comment = cells.next().unwrap().inner_html().replace('\n', "");
        Ok(TableColumn {
            name,
            data_type,
            mandatory,
            comment,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataType {
    Varchar { length: i32 },
    Char,
    Date,
    DateTime,
    Decimal { precision: u8, scale: u8 },
    Integer { precision: u8 },
}

lazy_static::lazy_static! {
    static ref VARCHAR: regex::Regex = regex::Regex::new(r"varchar(2)?\((\d+)\)").unwrap();
    static ref DECIMAL: regex::Regex = regex::Regex::new(r"number\((\d+),(\d+)\)").unwrap();
    static ref INTEGER: regex::Regex = regex::Regex::new(r"number\((\d+)\)").unwrap();
    static ref TIMESTAMP: regex::Regex = regex::Regex::new(r"timestamp\((\d+)\)").unwrap();

}

impl DataType {
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
            .ok_or_else(|| anyhow!("Couldnt parse {} as Decimal", s))?;
        let (precision, scale) = (caps[1].parse()?, caps[2].parse()?);
        Ok(DataType::Decimal { precision, scale })
    }
}

impl str::FromStr for DataType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<DataType> {
        let input = s.replace(' ', "").to_lowercase();
        let set = regex::RegexSet::new([
            r"date",
            r"char\(1\)",
            r"varchar\S+",
            r"timestamp\((\d+)\)",
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
            [3] => Ok(DataType::DateTime),
            [4] => DataType::parse_int(&input),
            [5] => DataType::parse_decimal(&input),
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
