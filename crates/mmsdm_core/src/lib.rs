//#![deny(clippy::all)]
//#![deny(warnings)]
use serde::{Deserialize, Serialize};
use std::{collections, fmt, io, iter, result, str};

use chrono::TimeZone;

mod error;
pub use error::*;

// this is useful to get the date part of nem settlementdate / lastchanged fields
pub fn to_nem_date(ndt: &chrono::NaiveDateTime) -> chrono::Date<chrono_tz::Tz> {
    to_nem_datetime(ndt).date()
}

// this is useful to get the datetime part of nem settlementdate / lastchanged fields
pub fn to_nem_datetime(ndt: &chrono::NaiveDateTime) -> chrono::DateTime<chrono_tz::Tz> {
    chrono_tz::Australia::Brisbane
        .from_local_datetime(ndt)
        .unwrap()
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum RecordType {
    C,
    I,
    D,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AemoHeader {
    pub record_type: RecordType,
    pub data_source: String,
    pub file_name: String,
    pub from_participant: String,
    pub to_participant: String,
    #[serde(with = "mms_date")]
    pub effective_date: chrono::NaiveDate,
    #[serde(with = "mms_time")]
    pub effective_time: chrono::NaiveTime,
    pub serial_number: i64,
    pub file_name_2: Option<String>,
    pub serial_number_2: Option<i64>,
}

impl AemoHeader {
    pub fn get_effective(&self) -> chrono::NaiveDateTime {
        self.effective_date.and_time(self.effective_time)
    }

    /// This function as currently implemented will generally produce
    /// a name close but not exact to the original name
    pub fn get_filename(&self) -> String {
        format!(
            "{}_{}_{}{}_{:016}.CSV",
            self.to_participant,
            self.file_name,
            self.effective_date.format("%Y%m%d"),
            self.effective_time.format("%H%M%S"),
            self.serial_number,
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AemoFooter {
    pub record_type: RecordType,
    pub end_of_report: String,
    pub line_count_inclusive: usize,
}


#[derive(Debug, Clone)]
pub struct Subtable {
    pub headings: csv::StringRecord,
    pub data: Vec<csv::StringRecord>,
}

impl Subtable {
    pub fn append(&mut self, record: csv::StringRecord) {
        self.data.push(record);
    }
    pub fn new(headings: csv::StringRecord) -> Subtable {
        Subtable {
            headings,
            data: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileKey {
    pub data_set_name: String,
    pub table_name: Option<String>,
    pub version: i32,
}
impl FileKey {
    pub fn table_name(&self) -> &str {
        if let Some(t) = &self.table_name {
            t
        } else {
            ""
        }
    }
    pub fn with_version(&self, version: i32) -> FileKey {
        let original = self.clone();
        FileKey {
            version,
            ..original
        }
    }
}

impl fmt::Display for FileKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(table_name) = &self.table_name {
            write!(f, "{}_{}_{}", self.data_set_name, table_name, self.version)
        } else {
            write!(f, "{}_{}", self.data_set_name, self.version)
        }
    }
}

pub trait Partition: std::hash::Hash + PartialEq + Eq {
    fn get_suffix(&self) -> String;
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct YearMonth {
    pub year: i32,
    pub month: chrono::Month,
}

impl Partition for YearMonth {
    fn get_suffix(&self) -> String {
        format!("_{:02}_{:04}", self.month.number_from_month(), self.year)
    }
}
impl Partition for () {
    fn get_suffix(&self) -> String {
        String::new()
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
pub trait GetTable:
    serde::Serialize + serde::de::DeserializeOwned + Send + Clone + fmt::Debug
{
    type PrimaryKey: PrimaryKey;
    type Partition: Partition;
    fn get_file_key() -> FileKey;
    fn partition_suffix(&self) -> Self::Partition;
    fn partition_name(&self) -> String;
    fn primary_key(&self) -> Self::PrimaryKey;
}

pub trait PrimaryKey: PartialOrd + Ord + PartialEq + Eq {}

pub trait CompareWithRow {
    type Row: GetTable;
    fn compare_with_row(&self, row: &Self::Row) -> bool;
}

pub trait CompareWithPrimaryKey {
    type PrimaryKey: PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool;
}

// handle the INSERT-UPDATE logic with lastchanged if applicibale
// if the rows don't match via CompareWithRow, then the self should always be returned
// only when the rows match and the other 'row' is newer, and this replacement behaviour is defined
// should the other 'row' be returned
pub trait LatestRow: GetTable + CompareWithRow<Row = Self> {
    fn latest_row(&mut self, row: Self) -> Self;
}

#[cfg(feature = "arrow")]
pub trait ArrowSchema: GetTable {
    fn arrow_schema() -> arrow2::datatypes::Schema;
    fn partition_to_chunk(
        partition: impl Iterator<Item=Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>>;
}

pub fn data_partition_to_csv<T, W>(
    partition: collections::BTreeMap<T::PrimaryKey, T>,
    writer: &mut W,
) -> crate::Result<()>
where
    T: GetTable + CompareWithRow<Row = T>,
    W: io::Write,
{
    let mut csv_wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(writer);

    for (_, row) in partition {
        csv_wtr.serialize(row)?;
    }

    csv_wtr.flush()?;

    Ok(())
}

pub fn data_partition_from_csv<T, R>(
    reader: R,
) -> crate::Result<collections::BTreeMap<T::PrimaryKey, T>>
where
    T: GetTable + CompareWithRow<Row = T>,
    R: io::Read,
{
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);
    let mut output = collections::BTreeMap::new();
    for res in reader.deserialize() {
        let row: T = res?;
        output.insert(row.primary_key(), row);
    }
    Ok(output)
}

pub fn required_partitions<T>(data: &[T]) -> collections::HashSet<T::Partition>
where
    T: serde::de::DeserializeOwned + Send + GetTable + CompareWithRow<Row = T>,
{
    data.iter().map(|row| row.partition_suffix()).collect()
}

// in memory
// the input map is all
pub fn merge_with_partitions<T>(
    mut existing: collections::HashMap<
        <T as GetTable>::Partition,
        collections::BTreeMap<T::PrimaryKey, T>,
    >,
    data: &[T],
) -> collections::HashMap<<T as GetTable>::Partition, collections::BTreeMap<T::PrimaryKey, T>>
where
    T: serde::Serialize + Send + GetTable + CompareWithRow<Row = T> + fmt::Debug,
{
    for row in data {
        let partition = row.partition_suffix();
        let pk = row.primary_key();
        if let Some(entry) = existing.get_mut(&partition) {
            entry.entry(pk).or_insert_with(|| row.clone());
        } else {
            existing.insert(partition, iter::once((pk, row.clone())).collect());
        }
    }

    existing
}

// on filesystem
// potentially do one partition at a time
// then we can do each partition in a seperate thread where they are available
// fn append_or_create_csv_at<P, T>(path: P, data: &[T]) -> crate::Result<()>
// where
//     T: serde::Serialize + Send + GetTable + CompareWithRow<Row=T> + fmt::Debug,
//     P: AsRef<path::Path>,
// {

//     let mut files = collections::HashMap::new();
//     for row in data {
//         let  file_name = format!("{}{}.csv", row.partition_name(), row.partition_suffix().get_suffix());

//         let file_path = path.as_ref().join(&file_name);
//         // check if we already have the file handle and data
//         if files.get_mut(&file_name).is_none() {
//             let mut csv_writer = csv::WriterBuilder::new();
//             let mut file_data = collections::BTreeMap::new();
//             if file_path.exists() {
//                 let f = fs::File::open(&file_path)?;
//                 let buf = io::BufReader::new(f);
//                 let mut reader = csv::Reader::from_reader(buf);
//                 for result in reader.deserialize() {
//                     let existing_row: T = result?;
//                     file_data.insert(existing_row.primary_key(), existing_row);
//                 }
//                 csv_writer.has_headers(false);
//             } else {
//                 csv_writer.has_headers(true);
//             }

//             let file_handle = fs::OpenOptions::new().append(true).create(true).open(&file_path)?;
//             let csv_writer = csv_writer.from_writer(file_handle);

//             files.insert(file_name.to_string(), (csv_writer, file_data));
//         }

//         if let Some((csv_writer, file_data)) = files.get_mut(&file_name) {
//             if let Some(_existing_val) = file_data.get_mut(&row.primary_key()) {
//                 log::warn!("Row {:?} was already present in file {:?}", row, &file_path);
//                 // let latest = existing_val.latest_row(row);
//                 // csv_writer.serialize()
//                 // #TODO: compare against lastchanged if relevant
//             } else {
//                 csv_writer.serialize(row)?;
//             }

//         }
//     }

//     Ok(())
// }

// Represents a given dispatch period (5 min period)
// Parsed from YYYYMMDDPPP
#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
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
    pub fn start(&self) -> chrono::NaiveDateTime {
        self.date.and_hms(
            u32::from((self.period - 1) / 12),
            u32::from((self.period - 1) % 12) * 5,
            0,
        )
    }
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

        let period = s[8..11].parse::<u16>()?;

        if !(1..=288).contains(&period) {
            return Err(Error::InvalidDispatchPeriod(s.into()));
        }

        Ok(crate::DispatchPeriod {
            date: chrono::NaiveDate::parse_from_str(&s[0..8], DispatchPeriod::format())?,
            period,
        })
    }
}

impl<'de> serde::Deserialize<'de> for DispatchPeriod {
    fn deserialize<D>(d: D) -> result::Result<crate::DispatchPeriod, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        //dbg!(&s);
        match s.parse() {
            Err(e) => {
                dbg!(&s[0..7]);
                dbg!(&e);
                Err(serde::de::Error::custom(e))
            }
            Ok(o) => Ok(o),
        }
        //s.parse().map_err(Error::custom)
    }
}

impl serde::Serialize for DispatchPeriod {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// #[allow(dead_code)] // Depending on features this may not be used

// Represents a given trading period (30 min period)
// Parsed from YYYYMMDDPP
#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
pub struct TradingPeriod {
    date: chrono::NaiveDate,
    period: u8,
}

impl TradingPeriod {
    pub fn date(&self) -> chrono::NaiveDate {
        self.date
    }
    pub fn period(&self) -> u8 {
        self.period
    }
    fn format() -> &'static str {
        "%Y%m%d"
    }
    pub fn start(&self) -> chrono::NaiveDateTime {
        self.date.and_hms(
            u32::from((self.period - 1) / 2),
            30 * u32::from((self.period - 1) % 2),
            0,
        )
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

        let period = s[8..10].parse::<u8>()?;

        if !(1..=48).contains(&period) {
            return Err(Error::InvalidTradingPeriod(s.into()));
        }

        Ok(crate::TradingPeriod {
            date: chrono::NaiveDate::parse_from_str(&s[0..8], TradingPeriod::format())?,
            period,
        })
    }
}

impl<'de> serde::Deserialize<'de> for TradingPeriod {
    fn deserialize<D>(d: D) -> result::Result<crate::TradingPeriod, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for TradingPeriod {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}


// Following the pattern from: https://serde.rs/custom-date-format.html
// To allow use with serde(with = "")
pub mod mms_datetime {
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
        let s = String::deserialize(d)?;
        chrono::NaiveDateTime::parse_from_str(&s, FORMAT).map_err(Error::custom)
    }
}

pub mod mms_datetime_opt {
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
        let s = String::deserialize(d)?;
        if s.is_empty() {
            Ok(None)
        } else {
            chrono::NaiveDateTime::parse_from_str(&s, FORMAT)
                .map_err(Error::custom)
                .map(Some)
        }
    }
}

pub mod mms_date {
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
        let s = String::deserialize(d)?;
        chrono::NaiveDate::parse_from_str(&s, FORMAT).map_err(Error::custom)
    }
}

pub mod mms_time {
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
        let s = String::deserialize(d)?;
        chrono::NaiveTime::parse_from_str(&s, FORMAT).map_err(Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_period() {
        assert!(matches!("20211101000".parse::<DispatchPeriod>(), Err(_)));
        assert!(matches!("20211101289".parse::<DispatchPeriod>(), Err(_)));
        assert!(matches!("20211501288".parse::<DispatchPeriod>(), Err(_)));
        assert!(matches!("20211132288".parse::<DispatchPeriod>(), Err(_)));

        assert_eq!(
            "20211101001".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(0, 0, 0),
        );

        assert_eq!(
            "20211101002".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(0, 5, 0),
        );

        assert_eq!(
            "20211101287".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(23, 50, 0),
        );

        assert_eq!(
            "20211101288".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(23, 55, 0),
        );
    }

    #[test]
    fn trading_period() {
        assert!(matches!("2021110100".parse::<TradingPeriod>(), Err(_)));
        assert!(matches!("2021110149".parse::<TradingPeriod>(), Err(_)));
        assert!(matches!("2021150148".parse::<TradingPeriod>(), Err(_)));
        assert!(matches!("2021113248".parse::<TradingPeriod>(), Err(_)));

        assert_eq!(
            "2021110101".parse::<TradingPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(0, 0, 0),
        );

        assert_eq!(
            "2021110102".parse::<TradingPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(0, 30, 0),
        );

        assert_eq!(
            "2021110147".parse::<TradingPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(23, 0, 0),
        );

        assert_eq!(
            "2021110148".parse::<TradingPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 11, 1).and_hms(23, 30, 0),
        );
    }
}
