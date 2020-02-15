use std::{error, fmt, io, convert, collections, num};
use serde::{de, Deserialize, Serialize};
use serde::de::Error as SerdeDeError; // need to expose trait but don't want to use name

use chrono_tz::Australia::Brisbane;
use chrono::TimeZone;
// one module per file, and re-exported
pub mod dispatch_is;
pub mod yestbid;


#[derive(Debug)]
pub enum Error {
    MissingFooterRecord,
    MissingHeaderRecord,
    MissingFile(FileKey),
    EmptyRow,
    UnexpectedRowType(String),
    TooShortRow(usize),
    IncorrectLineCount { got: usize, expected: usize },
    ThreadBroken,
    ParseInt(num::ParseIntError),
    Csv(csv::Error),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeaderRecord =>  write!(f, "aemo file is missing the first `c` record"),
            Self::MissingFooterRecord => write!(f, "aemo file is missing the final `c` record"),
            Self::MissingFile((name, sub_name, version)) => write!(f, "aemo file was missing {}.{}.v{} section in the file ", name, sub_name, version),
            Self::EmptyRow => write!(f, "aemo file row is empty"),
            Self::UnexpectedRowType(t) => write!(f, "unexpeted row type of {}", t),
            Self::TooShortRow(len) => write!(f, "aemo file data row of length {} is too short", len),
            Self::IncorrectLineCount { got, expected } => write!(f, "aemo file was supposed to be {} lines long but was instead {} lines long", expected, got),
            Self::ThreadBroken => write!(f, "Broken Thread"),
            Self::ParseInt(e) => write!(f, "parse int error: {}", e),
            Self::Csv(e) => write!(f, "csv error: {}", e),
            Self::Json(e) => write!(f, "serde json error: {}", e),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::Csv(error)
    }
}

impl error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AemoHeader {
    record_type: char,
    data_source: String,
    file_name: String,
    participant_name: String,
    privacy_level: String,
    #[serde(deserialize_with = "au_date_deserialize")]
    effective_date: chrono::NaiveDate,
    #[serde(deserialize_with = "au_time_deserialize")]
    effective_time: chrono::NaiveTime,
    serial_number: u64,
    file_name_2: String,
    serial_number_2: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AemoFooter {
    record_type: char,
    end_of_report: String,
    line_count_inclusive: usize,
}


#[derive(Debug, Clone)]
pub struct RawAemoFile {
    pub header: AemoHeader,
    pub data: collections::HashMap<FileKey, Vec<csv::StringRecord>>,
    //footer: AemoFooter, // don't reall
}

type FileKey = (String, String, i32);

// potentially have RawAemoFile<T> where T: forms the key of the hashmap??

impl RawAemoFile {

    pub fn from_bufread(br: impl io::Read) -> Result<Self> {

        let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_reader(br);
        let mut records = reader.records();
        let header: AemoHeader = records.next().ok_or(Error::MissingHeaderRecord)??.deserialize(None)?;

        // placeholder
        let mut footer: Result<AemoFooter> = Err(Error::MissingFooterRecord);
        let mut data: collections::HashMap<FileKey, Vec<csv::StringRecord>> = collections::HashMap::new();

        for record in records {
            let record = record?;
            match record.get(0) {
                Some("C") => {
                    footer = record.deserialize(None).map_err(convert::Into::into);
                },
                Some("D") => {
                    let row_len = record.len();
                    if row_len < 5 {
                        Err(Error::TooShortRow(row_len))?;
                    }
                    let file: String = record[1].into();
                    let sub_file: String = record[2].into();
                    let sub_file_version: i32 = record[3].parse()?;

                    // remove the unwanted fields from the stringrecord
                    let rest_record = record.into_iter().skip(4).fold(csv::StringRecord::new(), |mut acc, x| {
                        acc.push_field(x);
                        acc
                    });

                    if let Some((k, mut v)) = data.remove_entry(&(file.clone(), sub_file.clone(), sub_file_version)) {
                        v.push(rest_record);
                        data.insert(k, v);
                    } else {
                        data.insert((file.clone(), sub_file.clone(), sub_file_version), vec![rest_record]);
                    }

                    // would be more ideal but can't use because rest_record is moved into the first closure
                    // data.entry((sub_file, sub_file_version))
                    //     .and_modify(|v| v.push(rest_record))
                    //     .or_insert(vec![rest_record.clone()]);
                },
                Some("I") => continue, //"i" row, or unexpected row
                Some(t) => Err(Error::UnexpectedRowType(t.into()))?, //unexpected row, as correct files only have "C", "I" and "D"
                None => Err(Error::EmptyRow)?,
            }
        }
        // set footer
        let expected_line_count = footer?.line_count_inclusive;

        let file = Self {
            header,
            data,
        };

        let data_rows = file.data.iter().fold(0, |acc, (_, v)| acc + 1 + v.len());

        if data_rows + 2 == expected_line_count {
            Ok(file)
        } else {
            Err(Error::IncorrectLineCount { got: data_rows + 2, expected: expected_line_count } )
        }
    }
}

fn to_nem_date(ndt: &chrono::NaiveDateTime) -> chrono::Date<chrono_tz::Tz> {
    Brisbane.from_local_datetime(ndt).unwrap().date()
}

trait FileKeyable {
    fn key() -> FileKey;
}

trait GetFromRawAemo {
    type Output: FileKeyable + serde::de::DeserializeOwned;
    fn from_map(data: &mut collections::HashMap<FileKey, Vec<csv::StringRecord>>) -> Result<Vec<Self::Output>> {
        data.remove_entry(&Self::Output::key())
            .ok_or_else(|| Error::MissingFile(Self::Output::key()))?
            .1
            .into_iter()
            .map(|rec| rec.deserialize(None))
            .collect::<std::result::Result<Vec<Self::Output>, csv::Error>>()
            .map_err(convert::Into::into)
    }
}

fn au_datetime_deserialize<'de, D>(d: D) -> std::result::Result<chrono::NaiveDateTime, D::Error>
where
   D: serde::Deserializer<'de>
{
   let s = serde::Deserialize::deserialize(d)?;
   chrono::NaiveDateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S").map_err(de::Error::custom)
}

fn opt_au_datetime_deserialize<'de, D>(d: D) -> std::result::Result<Option<chrono::NaiveDateTime>, D::Error>
where
   D: serde::Deserializer<'de>
{
   let a_str: &'de str = serde::Deserialize::deserialize(d)?;
   if a_str.len() == 0 {
       Ok(None)
   } else {
     chrono::NaiveDateTime::parse_from_str(a_str, "%Y/%m/%d %H:%M:%S")
       .map_err(D::Error::custom)
       .map(|inr| Some(inr))

   }
}

fn au_date_deserialize<'de, D>(d: D) -> std::result::Result<chrono::NaiveDate, D::Error>
where
   D: serde::Deserializer<'de>
{
   let s = serde::Deserialize::deserialize(d)?;
   chrono::NaiveDate::parse_from_str(s, "%Y/%m/%d").map_err(de::Error::custom)

}

fn au_time_deserialize<'de, D>(d: D) -> std::result::Result<chrono::NaiveTime, D::Error>
where
   D: serde::Deserializer<'de>
{
   let s = serde::Deserialize::deserialize(d)?;
   chrono::NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(de::Error::custom) 

}
