// #![deny(clippy::all)] #![deny(warnings)]
use serde::{Deserialize, Serialize};
use std::{collections, convert, io, num}; 

use chrono::TimeZone;
use chrono_tz::Australia::Brisbane;
use log::info;

use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

pub mod mmsdm;
//pub mod daily;
//pub mod dispatch_is;
//pub mod dispatch_scada;
//pub mod predispatch_is;
//pub mod predispatch_sensitivities;
//pub mod rooftop_actual;
//pub mod rooftop_forecast;
//pub mod yestbid;
//pub mod confidential_settlements;

// this is useful to get the date part of nem settlementdate / lastchanged fields
pub fn to_nem_date(ndt: &chrono::NaiveDateTime) -> chrono::Date<chrono_tz::Tz> {
    Brisbane.from_local_datetime(ndt).unwrap().date()
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
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] num::ParseIntError),
    #[error("Csv error: {0}")]
    Csv(#[from] csv::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum RecordType {
    C,
    I,
    D
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
    //effective_date: String,
    #[serde(with = "mms_time")]
    effective_time: chrono::NaiveTime,
    //effective_time: String,
    serial_number: u64,
    file_name_2: Option<String>,
    serial_number_2: u64,
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
    pub data: collections::HashMap<FileKey, Vec<csv::StringRecord>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileKey {
    data_set_name: String,
    table_name: String,
    version: i32,
}

pub struct FileKeys {
}

impl std::iter::Iterator for FileKeys {
    type Item = FileKey;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

//type CsvResult<T> = std::result::Result<T, csv::Error>;

pub trait RawAemoFile {
    fn get_records(&self, key: &FileKey) -> Result<rayon::slice::Iter<csv::StringRecord>>;
}

impl RawAemoFile for AemoFile {
    fn get_records(&self, key: &FileKey) -> Result<rayon::slice::Iter<csv::StringRecord>> {
        self.data.get(&key).ok_or_else(||Error::MissingFile(key.clone())).map(|d| d.par_iter())
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
    T: serde::de::DeserializeOwned + Send
{
    fn get_file_key() -> FileKey;
    fn get_table(&self) -> Result<Vec<T>> {
        self.get_records(&Self::get_file_key())?
            .map(|rec| rec.deserialize(None).map_err(|e| e.into()))
            .collect()
    }
}




impl AemoFile {
    pub fn file_keys(&self) -> std::collections::hash_map::Keys<FileKey, Vec<csv::StringRecord>> {
        self.data.keys()
    }

    pub fn contains_file(&self, key: FileKey) -> bool {
        self.data.contains_key(&key)
    }

    pub fn from_bufread(br: impl io::Read) -> Result<Self> {
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

                    if let Some((k, mut v)) =
                        data.remove_entry(&key)
                    {
                        v.push(rest_record);
                        data.insert(k, v);
                    } else {
                        data.insert(
                            key,
                            vec![rest_record],
                        );
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

pub trait FileKeyable {
    fn key() -> FileKey;
}

pub trait GetFromRawAemo {
    type Output: FileKeyable + serde::de::DeserializeOwned;
    fn from_map(
        data: &mut collections::HashMap<FileKey, Vec<csv::StringRecord>>,
    ) -> Result<Vec<Self::Output>> {
        let key = &Self::Output::key();
        info!("Extracting file {:?}", key);
        data.remove_entry(key)
            .ok_or_else(|| Error::MissingFile(Self::Output::key()))?
            .1
            .into_iter()
            .map(|rec| rec.deserialize(None))
            .collect::<std::result::Result<Vec<Self::Output>, csv::Error>>()
            .map_err(convert::Into::into)
    }
}

// Following the pattern from: https://serde.rs/custom-date-format.html
// To allow use with serde(with = "")
mod mms_datetime {
    use serde::{Serializer, Deserializer, Deserialize, de::Error};

    const FORMAT: &str = "%Y/%m/%d %H:%M:%S";

    pub fn serialize<S>(
        d: &chrono::NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where S: Serializer,
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
    use serde::{Serializer, Deserializer, Deserialize, de::Error};

    const FORMAT: &str = "%Y/%m/%d %H:%M:%S";

    pub fn serialize<S>(
        d: &Option<chrono::NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        let s = match d {
            Some(date) => date.format(FORMAT).to_string(),
            None => "".to_string(),
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        d: D,
    ) -> Result<Option<chrono::NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &'de str = Deserialize::deserialize(d)?;
        if s.len() == 0 {
            Ok(None)
        } else {
            chrono::NaiveDateTime::parse_from_str(s, FORMAT)
                .map_err(Error::custom)
                .map(Some)
        }
    }
}

mod mms_date {
    use serde::{Serializer, Deserializer, Deserialize, de::Error};

    const FORMAT: &str = "%Y/%m/%d";

    pub fn serialize<S>(
        d: &chrono::NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where S: Serializer,
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
    use serde::{Serializer, Deserializer, Deserialize, de::Error};

    const FORMAT: &str = "%H:%M:%S";

    pub fn serialize<S>(
        d: &chrono::NaiveTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where S: Serializer,
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



