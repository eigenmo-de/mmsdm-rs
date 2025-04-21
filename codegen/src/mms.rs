use crate::KW;
use anyhow::anyhow;
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Report {
    // pub name: String,
    pub sub_type: String,
}

impl Report {
    pub fn should_skip(&self) -> bool {
        // skip historical dataset - there are no table definitions anyway
        // self.name == "HISTORICAL"
        // || (self.name == "CONFIGURATION")
        // all the below seems to be missing
        self.sub_type == "CONTRACTAGC"
            || self.sub_type == "CONTRACTLOADSHED"
            || self.sub_type == "CONTRACTREACTIVEPOWER"
            || self.sub_type == "CONTRACTRESTARTSERVICES"
            || self.sub_type == "CONTRACTRESTARTUNITS"
            || self.sub_type == "INTERMITTENT_P5_RUN"
            || self.sub_type == "INTERMITTENT_P5_PRED"
            || self.sub_type == "BILLINGAPCCOMPENSATION"
            || self.sub_type == "BILLINGAPCRECOVERY"
            || self.sub_type == "BILLING_RES_TRADER_RECOVERY"
            || self.sub_type == "BILLING_RES_TRADER_PAYMENT"
            || self.sub_type == "BILLINGIRFM"
            || self.sub_type == "BILLING_DIRECTION_RECONCILIATN"
            || self.sub_type == "BILLWHITEHOLE"
            || self.sub_type == "SETRESERVERECOVERY"
            || self.sub_type == "SETLSHEDRECOVERY"
            || self.sub_type == "SETRPOWERRECOVERY"
            || self.sub_type == ""
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
    pub fn is_trading_period(&self) -> bool {
        (matches!(self.data_type, DataType::Varchar { length } if length >= 10)
            || matches!(self.data_type, DataType::Decimal { scale: 0, precision } if precision >= 10)
            || matches!(self.data_type, DataType::Integer { precision } if precision >= 10))
            && !self.is_dispatch_period()
            && (self.comment.to_uppercase().contains("YYYYMMDDPP")
            || self.comment == "Date and Time of trading interval"
            // || self.comment.starts_with("Settlements Trading Interval")
            // || self.comment.starts_with("Settlement Trading Interval")
            || self.comment.starts_with("Market Trading Interval")
            || self.comment.starts_with("Trading Interval")
            || self.comment.contains("Predispatch run identifier")
            || self.comment.contains("predispatchseqno")
            || self
                .comment
                .to_lowercase()
                .contains("predispatch sequence number"))
    }
    pub fn is_dispatch_period(&self) -> bool {
        (matches!(self.data_type, DataType::Varchar { length } if length >= 11)
            || matches!(self.data_type, DataType::Decimal { scale: 0, precision } if precision >= 11)
            || matches!(self.data_type, DataType::Integer { precision } if precision >= 11))
            && (self.comment.to_uppercase().contains("YYYYMMDDPPP")
                || self.comment.contains("Settlement period"))
    }
    pub fn data_stored_in_string(&self) -> bool {
        matches!(self.data_type, DataType::Char | DataType::Varchar { .. })
            && !self.is_dispatch_period()
            && !self.is_trading_period()
    }
    pub fn field_name(&self) -> String {
        self.name.to_snake_case()
    }
    pub fn escaped_field_name(&self) -> String {
        let field_name = self.field_name();
        if KW.contains(&field_name.as_str()) {
            format!("r#{field_name}")
        } else {
            field_name
        }
    }
    // fn from_html(tab: &element_ref::ElementRef) -> anyhow::Result<TableColumn> {
    //     let mut cells = tab.select(&P);
    //     let name = cells.next().unwrap().inner_html().replace('\n', "");
    //     let data_type = cells
    //         .next()
    //         .unwrap()
    //         .inner_html()
    //         .replace(['\n', ' '], "")
    //         .parse()?;
    //     let mandatory = cells.next().unwrap().inner_html().starts_with('X');
    //     let comment = cells.next().unwrap().inner_html().replace('\n', "");
    //     Ok(TableColumn {
    //         name,
    //         data_type,
    //         mandatory,
    //         comment,
    //     })
    // }
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
