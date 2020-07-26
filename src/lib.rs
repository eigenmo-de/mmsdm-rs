#![deny(clippy::all)]
#![deny(warnings)]
use serde::{Deserialize, Serialize};
use std::{collections, convert, fmt, io, num, str};

use chrono::TimeZone;
use chrono_tz::Australia::Brisbane;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub mod mmsdm;

#[cfg(feature = "sql_server")]
pub mod sql_server;

#[cfg(feature = "clickhouse")]
pub mod clickhouse;

// this is useful to get the date part of nem settlementdate / lastchanged fields
pub fn to_nem_date(ndt: &chrono::NaiveDateTime) -> chrono::Date<chrono_tz::Tz> {
    to_nem_datetime(ndt).date()
}

// this is useful to get the datetime part of nem settlementdate / lastchanged fields
pub fn to_nem_datetime(ndt: &chrono::NaiveDateTime) -> chrono::DateTime<chrono_tz::Tz> {
    Brisbane.from_local_datetime(ndt).unwrap()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This occurs when we are missing the footer record which lists the number of rows in the file
    #[error("aemo file is missing the final `c` record")]
    MissingFooterRecord,

    /// This occurs when we are missing the header record which lists metadata about the file
    #[error("aemo file is missing the first `c` record")]
    MissingHeaderRecord,

    /// This occurs when the desired file key can't be found in the RawAemoFile
    #[error("aemo file was missing {}.{}.v{} section in the file ", 0.0, 0.1, 0.2)]
    MissingFile(FileKey),

    /// This occurs when an entire row is empty after the first three columns
    #[error("aemo file row is empty")]
    EmptyRow,

    /// This occurs when a given row in the file doesn't match the expected structure for that section
    /// of the file
    #[error("unexpeted row type of {0}")]
    UnexpectedRowType(String),

    /// This occurs when a given row in the file is shorter than expected
    #[error("aemo file data row of length {0} is too short")]
    TooShortRow(usize),

    /// This occurs when the number of rows in the file is different to the number listed in the
    /// footer
    #[error("aemo file was supposed to be {expected} lines long but was instead {got} lines long")]
    IncorrectLineCount { got: usize, expected: usize },

    /// This occurs when we receive a file_key that we are not familiar with
    #[error("Recieved unexpected file of type {0}")]
    UnhandledFileKey(FileKey),

    #[error("ParseInt error: {0}")]
    ParseInt(#[from] num::ParseIntError),

    #[error("ParseDate error: {0}")]
    ParseDate(#[from] chrono::ParseError),

    #[error("Csv error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Tiberius error: {0}")]
    #[cfg(feature = "sql_server")]
    Tiberius(#[from] tiberius::error::Error),

    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    /// This occurs when failing to parse a dispatch period
    #[error("Invalid dispatch period: {0}")]
    InvalidDispatchPeriod(String),

    /// This occurs when failing to parse a trading period
    #[error("Invalid trading period: {0}")]
    InvalidTradingPeriod(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum RecordType {
    C,
    I,
    D,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AemoHeader {
    record_type: RecordType,
    data_source: String,
    file_name: String,
    participant_name: String,
    privacy_level: String,
    #[serde(with = "mms_date")]
    effective_date: chrono::NaiveDate,
    #[serde(with = "mms_time")]
    effective_time: chrono::NaiveTime,
    serial_number: u64,
    file_name_2: Option<String>,
    serial_number_2: u64,
}

impl AemoHeader {
    pub fn get_filename(&self) -> String {
        format!(
            "{}_{}_{}_{}.CSV",
            self.privacy_level,
            self.file_name,
            self.effective_date.format("%Y%m%d"),
            self.serial_number,
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AemoFooter {
    record_type: char,
    end_of_report: String,
    line_count_inclusive: usize,
}

#[derive(Debug, Clone)]
pub struct AemoFile {
    pub header: AemoHeader,
    data: collections::HashMap<FileKey, Vec<csv::StringRecord>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileKey {
    pub data_set_name: String,
    pub table_name: String,
    pub version: i32,
}

impl fmt::Display for FileKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}_{}_{}",
            self.data_set_name, self.table_name, self.version
        )
    }
}

pub struct FileKeys {}

impl std::iter::Iterator for FileKeys {
    type Item = FileKey;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[doc(hidden)]
pub trait RawAemoFile {
    fn get_records(&self, key: &FileKey) -> Result<rayon::slice::Iter<csv::StringRecord>>;
}

#[doc(hidden)]
impl RawAemoFile for AemoFile {
    fn get_records(&self, key: &FileKey) -> Result<rayon::slice::Iter<csv::StringRecord>> {
        self.data
            .get(&key)
            .ok_or_else(|| Error::MissingFile(key.clone()))
            .map(|d| d.par_iter())
    }
}

/// This trait is designed as a convenient way to extract a Vec of the desired Strct representing
/// a row of the table from the `AemoFile` which represents the whole file.
/// Most `AemoFiles` would contain multiple tables
///
/// ```rust,no_run
/// use mmsdm::{DispatchUnitScada1, DispatchLocalPrice1};
///
/// // option A - using UFCS - this is useful where it is not convenient to
/// // typehint a let binding
/// # fn get_unit_scada(aemo: &AemoFile) -> Result<Vec<DispatchUnitScada1>> {
///     let rows = GetTable::<DispatchUnitScada1>::get_table(aemo)?;
/// #   Ok(rows)
/// # }
///
/// // option B - this is useful when you have a let binding that you
/// // can typehint
/// # fn get_local_price(aemo: &AemoFile) -> Result<Vec<DispatchLocalPrice1>> {
///     let rows: Vec<DispatchLocalPrice1> = aemo.get_table()?;
/// #   Ok(rows)
/// # }
/// ```
pub trait GetTable<T>: RawAemoFile
where
    T: serde::de::DeserializeOwned + Send,
{
    #[doc(hidden)]
    fn get_file_key() -> FileKey;
    fn get_table(&self) -> Result<Vec<T>> {
        self.get_records(&Self::get_file_key())?
            .map(|rec| rec.deserialize(None).map_err(|e| e.into()))
            .collect()
    }
}

// Represents a given dispatch period (5 min period)
// Parsed from YYYYMMDDPPP
#[derive(Debug, Clone, Ord, PartialEq, Eq, PartialOrd)]
pub struct DispatchPeriod {
    date: chrono::NaiveDate,
    period: u16,
}

impl DispatchPeriod {
    pub fn date(&self) -> chrono::NaiveDate {
        self.date
    }
    pub fn period(&self) -> u16 {
        self.period
    }
    fn format() -> &'static str {
        "%Y%m%d"
    }
//    pub fn datetime_starting(&self) -> chrono::NaiveDateTime {
//        self.date.and_hms( self.period as u32 / 12, (self.period as u32 % 12) * 5, 0) + chrono::Duration::hours(4)
//    }
//    pub fn datetime_ending(&self) -> chrono::NaiveDateTime {
//        self.datetime_starting() + chrono::Duration::minutes(5)
//    }
}

impl std::fmt::Display for DispatchPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:03}", self.date.format(Self::format()), self.period)
    }
}

impl std::str::FromStr for DispatchPeriod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if !s.is_ascii() || s.len() != 11 {
            return Err(Error::InvalidDispatchPeriod(s.into()));
        };

        Ok(crate::DispatchPeriod {
            date: chrono::NaiveDate::parse_from_str(&s[0..8], DispatchPeriod::format())?,
            period: s[8..11].parse()?,
        })
    }
}

#[allow(dead_code)] // Depending on features this may not be used
mod dispatch_period {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(d: &crate::DispatchPeriod, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&d.to_string())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<crate::DispatchPeriod, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        //dbg!(&s);
        match s.parse() {
            Err(e) => {
                dbg!(&s[0..7]);
                dbg!(&e);
                Err(Error::custom(e))
            }
            Ok(o) => Ok(o)
        }
        //s.parse().map_err(Error::custom)
    }
}

// Represents a given trading period (30 min period)
// Parsed from YYYYMMDDPP
#[derive(Debug, Clone, Ord, PartialEq, Eq, PartialOrd)]
pub struct TradingPeriod {
    date: chrono::NaiveDate,
    period: u8,
}

impl TradingPeriod {
    pub fn date(&self) -> chrono::NaiveDate {
        self.date
    }
    pub fn period(&self) -> u16 {
        self.period
    }
    fn format() -> &'static str {
        "%Y%m%d"
    }
}

impl std::fmt::Display for TradingPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:02}", self.date.format(Self::format()), self.period)
    }
}

impl std::str::FromStr for TradingPeriod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if !s.is_ascii() || s.len() != 10 {
            return Err(Error::InvalidTradingPeriod(s.into()));
        };

        Ok(crate::TradingPeriod {
            date: chrono::NaiveDate::parse_from_str(&s[0..8], TradingPeriod::format())?,
            period: s[8..10].parse()?,
        })
    }
}

#[allow(dead_code)] // Depending on features this may not be used
mod trading_period {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(d: &crate::TradingPeriod, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&d.to_string())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<crate::TradingPeriod, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        s.parse().map_err(Error::custom)
    }
}

impl AemoFile {
    pub fn file_keys(&self) -> std::collections::hash_map::Keys<FileKey, Vec<csv::StringRecord>> {
        self.data.keys()
    }

    pub fn contains_file(&self, key: FileKey) -> bool {
        self.data.contains_key(&key)
    }

    pub fn from_bufread(br: impl io::BufRead) -> Result<Self> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(br);
        let mut records = reader.records();
        let header: AemoHeader = records
            .next()
            .ok_or(Error::MissingHeaderRecord)??
            .deserialize(None)?;

        // placeholder
        let mut footer: Result<AemoFooter> = Err(Error::MissingFooterRecord);
        let mut data: collections::HashMap<FileKey, Vec<csv::StringRecord>> =
            collections::HashMap::new();

        for record in records {
            let record = record?;
            match record.get(0) {
                Some("C") => {
                    footer = record.deserialize(None).map_err(convert::Into::into);
                }
                Some("D") => {
                    let row_len = record.len();
                    if row_len < 5 {
                        return Err(Error::TooShortRow(row_len));
                    }
                    let key = FileKey {
                        data_set_name: record[1].into(),
                        table_name: record[2].into(),
                        version: record[3].parse()?,
                    };

                    // remove the unwanted fields from the stringrecord
                    let rest_record =
                        record
                            .into_iter()
                            .skip(4)
                            .fold(csv::StringRecord::new(), |mut acc, x| {
                                acc.push_field(x);
                                acc
                            });

                    if let Some((k, mut v)) = data.remove_entry(&key) {
                        v.push(rest_record);
                        data.insert(k, v);
                    } else {
                        data.insert(key, vec![rest_record]);
                    }

                    // would be more ideal but can't use because rest_record is moved into the first closure
                    // data.entry((sub_file, sub_file_version))
                    //     .and_modify(|v| v.push(rest_record))
                    //     .or_insert(vec![rest_record.clone()]);
                }
                Some("I") => continue, //"i" row, or unexpected row
                Some(t) => return Err(Error::UnexpectedRowType(t.into())), //unexpected row, as correct files only have "C", "I" and "D"
                None => return Err(Error::EmptyRow),
            }
        }
        // set footer
        let expected_line_count = footer?.line_count_inclusive;

        let file = Self { header, data };

        let data_rows = file.data.iter().fold(0, |acc, (_, v)| acc + 1 + v.len());

        if data_rows + 2 == expected_line_count {
            Ok(file)
        } else {
            Err(Error::IncorrectLineCount {
                got: data_rows + 2,
                expected: expected_line_count,
            })
        }
    }
}

// Following the pattern from: https://serde.rs/custom-date-format.html
// To allow use with serde(with = "")
mod mms_datetime {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    const FORMAT: &str = "%Y/%m/%d %H:%M:%S";

    pub fn serialize<S>(d: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = d.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<chrono::NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Deserialize::deserialize(d)?;
        chrono::NaiveDateTime::parse_from_str(s, FORMAT).map_err(Error::custom)
    }
}

mod mms_datetime_opt {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y/%m/%d %H:%M:%S";

    pub fn serialize<S>(d: &Option<chrono::NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match d {
            Some(date) => date.format(FORMAT).to_string(),
            None => "".to_string(),
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<chrono::NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &'de str = Deserialize::deserialize(d)?;
        if s.is_empty() {
            Ok(None)
        } else {
            chrono::NaiveDateTime::parse_from_str(s, FORMAT)
                .map_err(Error::custom)
                .map(Some)
        }
    }
}

mod mms_date {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y/%m/%d";

    // because the serialize fn always expects a reference
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(d: &chrono::NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = d.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }
    pub fn deserialize<'de, D>(d: D) -> std::result::Result<chrono::NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Deserialize::deserialize(d)?;
        chrono::NaiveDate::parse_from_str(s, FORMAT).map_err(Error::custom)
    }
}

mod mms_time {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%H:%M:%S";

    // because the serialize fn always expects a reference
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(d: &chrono::NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = d.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<chrono::NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Deserialize::deserialize(d)?;
        chrono::NaiveTime::parse_from_str(s, FORMAT).map_err(Error::custom)
    }
}
