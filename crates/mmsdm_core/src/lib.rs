#![deny(clippy::all)]
#![deny(warnings)]
use serde::{Deserialize, Serialize};
use std::{collections, fmt, fs, io, iter, result, str};

use chrono::TimeZone;

mod error;
pub use error::*;

#[cfg(feature = "sql_server")]
pub mod sql_server;

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

#[cfg(feature = "sql_server")]
pub trait SaveToSqlServer: GetTable {
    const SQL_EXEC: &'static str;
}

#[cfg(feature = "arrow")]
pub trait ArrowSchema: GetTable {
    fn arrow_schema() -> arrow2::datatypes::Schema;
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
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

#[derive(Debug, Clone)]
pub struct MemoryFile {
    pub header: AemoHeader,
    pub data: collections::HashMap<FileKey, Subtable>,
}

#[derive(Debug)]
pub struct DiskFile {
    pub header: AemoHeader,
    handle: zip::ZipArchive<fs::File>,
    keys: collections::HashMap<FileKey, csv::StringRecord>,
}

pub struct FileIter<'a, T>
where
    T: serde::de::DeserializeOwned + Send + GetTable,
{
    file_key: FileKey,
    inner: csv::StringRecordsIntoIter<zip::read::ZipFile<'a>>,
    _d: std::marker::PhantomData<T>,
    headings: csv::StringRecord,
    found_records: bool,
}

impl<'a, T> Iterator for FileIter<'a, T>
where
    T: serde::de::DeserializeOwned + Send + GetTable,
{
    type Item = Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // we loop here as we may need to scan through multiple records of the underlying iterator
        // to return one here.

        // optimization: once we start not finding matches after finding at leaast one, we cna always return none.
        while let Some(val) = self.inner.next() {
            // record can be safely unwrapped here as we have already checked when creating the AemoDiskFile
            // we also don't need to check the row length as it has been checked already
            let record = val.unwrap();
            if &record[0] == "D"
                && self.file_key.data_set_name == record[1]
                && self
                    .file_key
                    .table_name
                    .as_ref()
                    .map(|n| n == &record[2])
                    .unwrap_or_else(|| record[2].is_empty())
                && self.file_key.version.to_string() == record[3]
            {
                self.found_records = true;
                return Some(record.deserialize(Some(&self.headings)).map_err(|cause| {
                    Error::CsvRowDe {
                        cause,
                        headings: Some(self.headings.clone()),
                        data: record.clone(),
                    }
                }));
            } else if self.found_records {
                return None;
            } else {
                continue;
            }
        }
        None
    }
}

pub type AemoDiskFile = DiskFile;
pub type AemoFile = MemoryFile;

pub enum MmsFile<'a> {
    Disk(&'a mut DiskFile),
    Memory(&'a mut MemoryFile),
}

impl<'a> MmsFile<'a> {
    pub fn file_keys(&self) -> collections::HashSet<FileKey> {
        match self {
            MmsFile::Disk(d) => d.keys.keys().cloned().collect(),
            MmsFile::Memory(m) => m.data.keys().cloned().collect(),
        }
    }
    pub fn get_table<T>(&mut self) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send + GetTable + 'static,
    {
        match self {
            MmsFile::Disk(d) => d.get_table()?.collect(),
            MmsFile::Memory(m) => m.get_table(),
        }
    }
    pub fn header(&self) -> &AemoHeader {
        match self {
            MmsFile::Disk(d) => &d.header,
            MmsFile::Memory(m) => &m.header,
        }
    }
}

impl<'a> From<&'a mut DiskFile> for MmsFile<'a> {
    fn from(d: &'a mut DiskFile) -> Self {
        MmsFile::Disk(d)
    }
}
impl<'a> From<&'a mut MemoryFile> for MmsFile<'a> {
    fn from(m: &'a mut MemoryFile) -> Self {
        MmsFile::Memory(m)
    }
}

impl DiskFile {
    pub fn contains_file(&self, key: &FileKey) -> bool {
        self.keys.contains_key(key)
    }

    pub fn get_table<T>(&'_ mut self) -> Result<FileIter<'_, T>>
    where
        T: serde::de::DeserializeOwned + Send + GetTable + 'static,
    {
        let latest_version = T::get_file_key();
        for version in (1..=latest_version.version).rev() {
            let current_key = latest_version.with_version(version);
            if self.contains_file(&current_key) {
                return self.get_specific_table(current_key);
            } else {
                log::warn!(
                    "For file key {}, version {} was not available",
                    latest_version,
                    version,
                );
            }
        }
        Err(Error::MissingFile(latest_version))
    }
    pub fn get_specific_table<T>(&'_ mut self, file_key: FileKey) -> Result<FileIter<'_, T>>
    where
        T: serde::de::DeserializeOwned + Send + GetTable,
    {
        // assert that the key exists
        let headings = self
            .keys
            .get(&file_key)
            .ok_or_else(|| Error::MissingFile(file_key.clone()))?
            .to_owned();

        log::info!("Table found and parsed for file key {}", file_key);

        let inner_file = self.handle.by_index(0)?;

        Ok(FileIter {
            headings,
            file_key,

            inner: csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(true)
                .from_reader(inner_file)
                .into_records(),
            _d: std::marker::PhantomData,
            found_records: false,
        })
    }

    /// This assumes that the file is in the standard ZIP format as downloaded
    /// from nemweb.com.au or FTP server.
    /// This simply validates the file and stores which subfiles are present
    /// but doesn't actually parse the data.
    /// This is more memory efficient than using AemoFile and reading into memory
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self> {
        // baked in assumption that data is a ZIP and contains only one file.
        let f = fs::File::open(path.as_ref())?;
        let mut zip = zip::ZipArchive::new(f)?;
        let (header, keys) = {
            let inner_file = zip.by_index(0)?;
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(true)
                .from_reader(inner_file);
            let mut records = reader.records();
            let first = records.next().ok_or(Error::MissingHeaderRecord)??;

            let header: AemoHeader = first.deserialize(None).map_err(|cause| Error::CsvRowDe {
                cause,
                headings: None,
                data: first,
            })?;

            // placeholder
            let mut footer: Result<AemoFooter> = Err(Error::MissingFooterRecord);
            let mut keys: collections::HashMap<FileKey, csv::StringRecord> =
                collections::HashMap::new();

            let mut num_rows = 1;

            for record in records {
                let record = record?;
                match record.get(0) {
                    Some("D") => {
                        num_rows += 1;
                        let row_len = record.len();
                        if row_len < 5 {
                            return Err(Error::TooShortRow(row_len));
                        }
                    }
                    Some("I") => {
                        num_rows += 1;
                        let row_len = record.len();
                        if row_len < 5 {
                            return Err(Error::TooShortRow(row_len));
                        }
                        let key = FileKey {
                            data_set_name: record[1].to_string(),
                            table_name: match &record[2] {
                                "" => None,
                                otherwise => Some(otherwise.to_string()),
                            },
                            version: record[3].parse()?,
                        };
                        let lowercased = record.iter().map(|v| v.to_lowercase()).collect();

                        keys.insert(key, lowercased);
                    }
                    Some("C") => {
                        num_rows += 1;

                        footer = record.deserialize(None).map_err(|cause| Error::CsvRowDe {
                            cause,
                            headings: None,
                            data: record,
                        });
                    }
                    Some(t) => return Err(Error::UnexpectedRowType(t.into())), //unexpected row, as correct files only have "C", "I" and "D"
                    None => return Err(Error::EmptyRow),
                }
            }

            let expected_line_count = footer?.line_count_inclusive;

            if num_rows != expected_line_count {
                return Err(Error::IncorrectLineCount {
                    got: num_rows,
                    expected: expected_line_count,
                });
            }
            (header, keys)
        };

        Ok(AemoDiskFile {
            header,
            handle: zip,
            keys,
        })
    }
}

impl MemoryFile {
    pub fn get_table<T>(&self) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send + GetTable,
    {
        let latest_version = T::get_file_key();
        for version in (1..=latest_version.version).rev() {
            let current_key = latest_version.with_version(version);
            match self.get_specific_table(&current_key) {
                Ok(parsed) => {
                    log::info!("Table found and parsed for file key {}", current_key);
                    return Ok(parsed);
                }
                Err(e) => {
                    log::warn!(
                        "For file key {}, version {} was not available: {}",
                        latest_version,
                        version,
                        e,
                    );
                }
            }
        }
        Err(Error::MissingFile(latest_version))
    }
    pub fn get_specific_table<T>(&self, file_key: &FileKey) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send + GetTable,
    {
        let subtable = self
            .data
            .get(file_key)
            .ok_or_else(|| Error::MissingFile(file_key.clone()))?;

        subtable
            .data
            .iter()
            .map(|rec| {
                rec.deserialize(Some(&subtable.headings))
                    .map_err(|cause| Error::CsvRowDe {
                        cause,
                        headings: Some(subtable.headings.clone()),
                        data: rec.clone(),
                    })
            })
            .collect()
    }

    pub fn file_keys(&self) -> std::collections::hash_map::Keys<FileKey, Subtable> {
        self.data.keys()
    }

    pub fn contains_file(&self, key: FileKey) -> bool {
        self.data.contains_key(&key)
    }

    pub fn from_reader(br: impl io::Read) -> Result<Self> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(br);
        let mut records = reader.records();
        let first = records.next().ok_or(Error::MissingHeaderRecord)??;

        let header: AemoHeader = first.deserialize(None).map_err(|cause| Error::CsvRowDe {
            cause,
            headings: None,
            data: first,
        })?;

        // placeholder
        let mut footer: Result<AemoFooter> = Err(Error::MissingFooterRecord);
        let mut data: collections::HashMap<FileKey, Subtable> = collections::HashMap::new();

        for record in records {
            let record = record?;
            match record.get(0) {
                Some("D") => {
                    let row_len = record.len();
                    if row_len < 5 {
                        return Err(Error::TooShortRow(row_len));
                    }
                    let key = FileKey {
                        data_set_name: record[1].to_string(),
                        table_name: match &record[2] {
                            "" => None,
                            otherwise => Some(otherwise.to_string()),
                        },
                        version: record[3].parse()?,
                    };

                    if let Some(v) = data.get_mut(&key) {
                        v.append(record);
                    } else {
                        return Err(Error::MissingSubtableHeadings(key));
                    }
                }
                Some("I") => {
                    let row_len = record.len();
                    if row_len < 5 {
                        return Err(Error::TooShortRow(row_len));
                    }
                    let key = FileKey {
                        data_set_name: record[1].to_string(),
                        table_name: match &record[2] {
                            "" => None,
                            otherwise => Some(otherwise.to_string()),
                        },
                        version: record[3].parse()?,
                    };

                    let lowercased = record.iter().map(|v| v.to_lowercase()).collect();

                    data.insert(key, Subtable::new(lowercased));
                }
                Some("C") => {
                    footer = record.deserialize(None).map_err(|cause| Error::CsvRowDe {
                        cause,
                        headings: None,
                        data: record,
                    });
                }
                Some(t) => return Err(Error::UnexpectedRowType(t.into())), //unexpected row, as correct files only have "C", "I" and "D"
                None => return Err(Error::EmptyRow),
            }
        }
        // set footer
        let expected_line_count = footer?.line_count_inclusive;

        let file = Self { header, data };

        let data_rows = file
            .data
            .iter()
            .fold(0, |acc, (_, v)| acc + 1 + v.data.len());

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
