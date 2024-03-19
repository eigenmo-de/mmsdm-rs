#![deny(clippy::all)]
//#![deny(warnings)]
#![no_std]
extern crate alloc;
use core::{
    cmp::Ordering,
    fmt,
    hash::Hash,
    num::{NonZeroU16, NonZeroU8},
    ops::{Add, Div, Index, Range, Sub},
    str::{self, FromStr},
};

use alloc::{
    borrow::Cow,
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime};

mod error;
use csv_core::{ReadRecordResult, Reader, ReaderBuilder};
pub use error::*;

#[cfg(feature = "std")]
mod io;
#[cfg(feature = "std")]
pub use io::*;

#[cfg(feature = "arrow")]
mod arrow;
#[cfg(feature = "arrow")]
pub use arrow::*;

#[derive(Debug, Clone, Copy)]
pub enum RecordType {
    C,
    I,
    D,
}

impl FromStr for RecordType {
    type Err = Error;
    fn from_str(s: &str) -> core::prelude::v1::Result<Self, Self::Err> {
        match s.chars().next() {
            Some('C') => Ok(RecordType::C),
            Some('I') => Ok(RecordType::I),
            Some('D') => Ok(RecordType::D),
            _ => Err(Error::UnexpectedRowType(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AemoHeader {
    pub record_type: RecordType,
    pub data_source: String,
    pub file_name: String,
    pub from_participant: String,
    pub to_participant: String,
    pub effective_date: chrono::NaiveDate,
    pub effective_time: chrono::NaiveTime,
    pub serial_number: i64,
    pub file_name_2: Option<String>,
    pub serial_number_2: Option<i64>,
}

impl AemoHeader {
    fn from_row<'data>(row: CsvRow<'data>) -> Result<AemoHeader> {
        Ok(AemoHeader {
            record_type: row.get_parsed_at_idx("record_type", 0)?,
            data_source: row.get_parsed_at_idx("data_source", 1)?,
            file_name: row.get_parsed_at_idx("file_name", 2)?,
            from_participant: row.get_parsed_at_idx("from_participant", 3)?,
            to_participant: row.get_parsed_at_idx("to_participant", 4)?,
            effective_date: row.get_custom_parsed_at_idx("effective_date", 5, mms_date::parse)?,
            effective_time: row.get_custom_parsed_at_idx("effective_time", 6, mms_time::parse)?,
            serial_number: row.get_parsed_at_idx("serial_number", 7)?,
            file_name_2: row.get_opt_parsed_at_idx("file_name_2", 8)?,
            serial_number_2: row.get_opt_parsed_at_idx("serial_number_2", 9)?,
        })
    }
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

#[derive(Debug, Clone)]
pub struct AemoFooter {
    pub record_type: RecordType,
    pub end_of_report: String,
    pub line_count_inclusive: usize,
}

impl AemoFooter {
    fn from_row<'data>(row: CsvRow<'data>) -> Result<AemoFooter> {
        Ok(AemoFooter {
            record_type: row.get_parsed_at_idx("record_type", 0)?,
            end_of_report: row.get_parsed_at_idx("end_of_report", 1)?,
            line_count_inclusive: row.get_parsed_at_idx("line_count_inclusive", 2)?,
        })
    }
}

#[derive(Debug)]
pub struct FileKey<'a> {
    pub data_set_name: Range<usize>,
    pub table_name: Range<usize>,
    pub version: i32,
    backing_data: CsvRow<'a>,
}

impl<'a> Ord for FileKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.data_set_name().cmp(other.data_set_name()) {
            Ordering::Equal => (),
            ord => return ord,
        }

        match self.table_name().cmp(other.table_name()) {
            Ordering::Equal => (),
            ord => return ord,
        }

        match self.version.cmp(&other.version) {
            Ordering::Equal => (),
            ord => return ord,
        }

        Ordering::Equal
    }
}

impl<'a> PartialOrd for FileKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl<'a> PartialEq for FileKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data_set_name() == other.data_set_name()
            && self.table_name() == other.table_name()
            && self.version == other.version
    }
}

impl<'a> Eq for FileKey<'a> {}

impl<'a> Hash for FileKey<'a> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data_set_name().hash(state);
        self.table_name().hash(state);
        self.version.hash(state);
    }
}

impl<'a> FileKey<'a> {
    pub fn backing_data(&self) -> &CsvRow<'a> {
        &self.backing_data
    }
    pub fn from_row<'data>(row: CsvRow<'data>) -> Result<FileKey<'data>> {
        // start at 1 here so we can skip the 0th field which is the C/I/D flag
        Ok(FileKey {
            data_set_name: row.get_range("data_set_name", 1)?,
            table_name: row.get_range("table_name", 2)?,
            version: row.get_parsed_at_idx("version", 3)?,
            backing_data: row,
        })
    }
    pub fn to_owned(&self) -> FileKey<'static> {
        FileKey {
            data_set_name: self.data_set_name.clone(),
            table_name: self.table_name.clone(),
            version: self.version,
            backing_data: self.backing_data.to_owned(),
        }
    }

    pub fn data_set_name(&self) -> &str {
        self.backing_data
            .as_slice()
            .index(self.data_set_name.clone())
    }
    pub fn table_name(&self) -> &str {
        // if self.table_name.is_empty() {
        //     ""
        // } else {
        self.backing_data.as_slice().index(self.table_name.clone())
        // }
    }
    pub fn with_version(self, version: i32) -> FileKey<'a> {
        FileKey { version, ..self }
    }
}

impl<'a> fmt::Display for FileKey<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.table_name.is_empty() {
            write!(
                f,
                "{}_{}_{}",
                self.data_set_name(),
                self.table_name(),
                self.version
            )
        } else {
            write!(f, "{}_{}", self.data_set_name(), self.version)
        }
    }
}

pub trait Partition: core::hash::Hash + PartialEq + Eq + PartialOrd + Ord + Copy {
    fn get_suffix(&self) -> String;
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct YearMonth {
    pub year: i32,
    pub month: chrono::Month,
}

impl PartialOrd for YearMonth {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.cmp(&other).into()
    }
}

impl Ord for YearMonth {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.year.cmp(&other.year) {
            core::cmp::Ordering::Equal => self
                .month
                .number_from_month()
                .cmp(&other.month.number_from_month()),
            ord => ord,
        }
    }
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

// / This trait is designed as a convenient way to extract a Vec of the desired Strct representing
// / a row of the table from the `AemoFile` which represents the whole file.
// / Most `AemoFiles` would contain multiple tables
// /
// / ```rust,no_run
// / use mmsdm::{DispatchUnitScada1, DispatchLocalPrice1};
// /
// / // option A - using UFCS - this is useful where it is not convenient to
// / // typehint a let binding
// / # fn get_unit_scada(aemo: &AemoFile) -> Result<Vec<DispatchUnitScada1>> {
// /     let rows = GetTable::<DispatchUnitScada1>::get_table(aemo)?;
// / #   Ok(rows)
// / # }
// /
// / // option B - this is useful when you have a let binding that you
// / // can typehint
// / # fn get_local_price(aemo: &AemoFile) -> Result<Vec<DispatchLocalPrice1>> {
// /     let rows: Vec<DispatchLocalPrice1> = aemo.get_table()?;
// / #   Ok(rows)
// / # }
// / ```

pub trait GetTable: 'static {
    const DATA_SET_NAME: &'static str;
    const TABLE_NAME: &'static str;
    const VERSION: i32;
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping;
    const COLUMNS: &'static [&'static str];
    type Row<'a>;
    type PrimaryKey: PrimaryKey;
    type Partition: Partition;

    type FieldMapping;
    // the row must be an I row with a matching file key
    fn field_mapping_from_row<'a>(row: CsvRow<'a>) -> Result<Self::FieldMapping>;

    fn partition_suffix_from_row<'a>(row: CsvRow<'a>) -> Result<Self::Partition>;
    fn from_row<'a>(row: CsvRow<'a>, field_mapping: &Self::FieldMapping) -> Result<Self::Row<'a>>;

    fn matches_file_key(key: &FileKey<'_>, version: i32) -> bool;

    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition;
    fn partition_name(row: &Self::Row<'_>) -> String;
    fn primary_key(row: &Self::Row<'_>) -> Self::PrimaryKey;
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static>;
}

pub enum RowValidation {
    Header(AemoHeader),
    Footer(AemoFooter),
    Headings(FileKey<'static>),
}



pub fn handle_row<'input, T>(
    mut csv: CsvRow<'input>,
    version: i32,
    field_mapping: &T::FieldMapping,
) -> Result<Option<<T as GetTable>::Row<'input>>>
where
    T: GetTable,
{
    match csv.record_type {
        Some(RecordType::C) | Some(RecordType::I) => return Ok(None),
        Some(RecordType::D) => {
            if T::matches_file_key(&FileKey::from_row(csv.borrow())?, version) {
                Ok(Some(T::from_row(csv, field_mapping)?))
            } else {
                Ok(None)
            }
        }
        None => {
            Err(Error::UnexpectedRowType(csv.join_fields()))
        }
    }
}

pub struct CsvRow<'a> {
    data: Cow<'a, str>,
    indexes: Cow<'a, [usize]>,
    record_type: Option<RecordType>,
}

impl<'a> Clone for CsvRow<'a> {
    fn clone(&self) -> CsvRow<'static> {
        self.to_owned()
    }
}

impl<'a> PartialEq for CsvRow<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data.as_ref() == other.data.as_ref()
            && self.indexes == other.indexes
    }
}

impl<'a> Eq for CsvRow<'a> {}

impl<'a> fmt::Debug for CsvRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CsvRow")
            .field("data", &self.data)
            .field("indexes", &self.indexes)
            // .field("offset", &self.offset)
            .field("count_fields", &self.count_fields())
            .field("fields", &self.iter_fields().collect::<Vec<_>>())
            .finish()
    }
}

pub struct CsvReader {
    inner: Reader,
}

impl CsvReader {
    pub fn new() -> CsvReader {
        CsvReader {
            inner: ReaderBuilder::new()
                .delimiter(b',')
                .terminator(csv_core::Terminator::CRLF)
                .quote(b'"')
                .build(),
        }
    }

    // TODO: verify that column orders are correct!
    pub fn validate_row(&mut self, row: &str, out: &mut Vec<u8>, indexes_backing: &mut Vec<usize>) -> Result<Option<RowValidation>> {
        let csv = self.read_row(row, out, indexes_backing)?;

        match csv.record_type {
            Some(RecordType::C) => {
                // happy to allocate here as `C` rows are rare!
                if let Ok(header) = AemoHeader::from_row(csv.to_owned()) {
                    return Ok(Some(RowValidation::Header(header)));
                }

                if let Ok(footer) = AemoFooter::from_row(csv.to_owned()) {
                    return Ok(Some(RowValidation::Footer(footer)));
                }

                Err(Error::UnexpectedRowType(csv.data.to_string()))
            }
            Some(RecordType::I) => Ok(Some(RowValidation::Headings(FileKey::from_row(
                csv.to_owned(),
            )?))),
            Some(RecordType::D) => Ok(None),
            None => Err(Error::UnexpectedRowType(csv.join_fields()))
        }
    }

    pub fn read_row<'a>(&mut self, data: &'a str, out: &'a mut Vec<u8>, indexes: &'a mut Vec<usize>) -> Result<CsvRow<'a>> {
        if !data.is_ascii() {
            return Err(Error::UnexpectedRowType(format!(
                "Row contained non ascii characters: {data}"
            )));
        }
        if !data.ends_with('\n') {
            return Err(Error::UnexpectedRowType(format!(
                "Row didn't finish with a newline, unable to parse. It's reccomended to use `BufRead::read_line` or similar which doesn't strip newlines. Data was: {data}"
            )));
        }
        if data.is_empty() {
            return Err(Error::EmptyRow);
        }

        if out.len() != out.capacity() {
            // initialize all elements up to capacity
            out.iter_mut().for_each(|v| {
                *v = 0;
            });
        }

        if indexes.len() != indexes.capacity() {
            // initialize all elements up to capacity
            indexes.iter_mut().for_each(|v| {
                *v = 0;
            });  
        }

        let (status, _bytes_read, bytes_written, num_positions) = self.inner.read_record(data.as_bytes(), out.as_mut_slice(), indexes.as_mut_slice());

        match status {
            // good parse
            ReadRecordResult::Record => Ok(CsvRow {
                // we could consider using from_utf8_unchecked here as it should be safe due to
                // the input data being a valid str. howerver, the performance improvement
                // of ~5% isn't worth it.
                data: Cow::Borrowed(str::from_utf8(&out[0..bytes_written]).expect("valid UTF8")),
                indexes: Cow::Borrowed(&indexes[0..num_positions]),
                record_type: data[0..1].parse().ok(),
            }),
            // should never happen due to data is empty check above
            ReadRecordResult::End => Err(Error::EmptyRow),
            ReadRecordResult::InputEmpty => Err(Error::ParseRow(format!("InputEmpty for input {data}, out len {}, ends len {}", out.len(), indexes.len()))),
            ReadRecordResult::OutputFull => Err(Error::ParseRow(format!("OutputFull for input {data}, out len {}, ends len {}", out.len(), indexes.len()))),
            ReadRecordResult::OutputEndsFull => Err(Error::ParseRow(format!("OutputEndsFull for input {data}, out len {}, ends len {}", out.len(), indexes.len()))),
        }
    }
}


impl<'a> CsvRow<'a> {
    pub fn join_fields(&self) -> String {
        self.iter_fields().collect::<Vec<_>>().join(", ")
    }
    pub fn record_type(&self) -> Option<RecordType> {
        self.record_type
    }
    pub fn is_metadata(&self) -> bool {
        matches!(self.record_type, Some(RecordType::C))
    }
    pub fn is_heading(&self) -> bool {
        matches!(self.record_type, Some(RecordType::I))
    }
    pub fn is_data(&self) -> bool {
        matches!(self.record_type, Some(RecordType::D))
    }
    pub fn is_mmsdm_row(&self) -> bool {
        self.record_type.is_some()
    }
    pub fn borrow<'b>(&'b mut self) -> CsvRow<'b>
    where
        'a: 'b,
    {
        CsvRow {
            data: Cow::Borrowed(&self.data.as_ref()),
            indexes: Cow::Borrowed(&self.indexes.as_ref()), // IndexOptions::MutSlice(self.indexes.as_mut_slice()),
            // offset: self.offset,
            record_type: self.record_type,
        }
    }
    fn get_end_of_field(&self, field_no: usize) -> Option<usize> {
        self.indexes.get(field_no).copied()
    }

    fn get_start_of_field(&self, field_no: usize) -> Option<usize> {
        match field_no.checked_sub(1) {
            None => Some(0),
            Some(field) => self.get_end_of_field(field)
        }
    }

    pub fn iter_fields<'b>(&'b self) -> impl Iterator<Item = &'b str> + 'b {
        (0..self.count_fields()).flat_map(|idx| self.get(idx))
    }
    pub fn to_owned(&'a self) -> CsvRow<'static> {
        CsvRow {
            data: match &self.data {
                Cow::Borrowed(b) => Cow::Owned(b.to_string()),
                Cow::Owned(o) => Cow::Owned(o.to_string()),
            },
            indexes: match &self.indexes {
                Cow::Borrowed(b) => Cow::Owned(b.to_vec()),
                Cow::Owned(o) => Cow::Owned(o.to_vec()),
            },
            // offset: self.offset,
            record_type: self.record_type,
        }
    }
    

    pub fn range(&self, i: usize) -> Option<Range<usize>> {
        if i >= self.count_fields() {
            return None;
        }

        // let lower = i.checked_sub(1).and_then(|idx| self.get_end_of_field(idx).map(|lower| lower + 1)).unwrap_or(0);
        // let upper = self.get_end_of_field(i);

        Some(Range {
            start: self.get_start_of_field(i)?,
            end: self.get_end_of_field(i)?,
        })
    }

    pub fn get_range(&self, name: &'static str, idx: usize) -> Result<Range<usize>> {
        self.range(idx).ok_or_else(|| Error::CannotFindFieldInRow {
            name,
            row: self.to_owned(),
            idx,
        })
    }

    pub fn get_opt_range(&self, name: &'static str, idx: usize) -> Result<Range<usize>> {
        if self.range(idx).map(|r| r.is_empty()).unwrap_or(true) {
            return Ok(Range { start: 0, end: 0 });
        }

        self.get_range(name, idx)
    }

    pub fn as_slice(&self) -> &str {
        &self.data
    }

    pub fn count_fields(&self) -> usize {
        self.indexes.len() + 1
    }
    fn get(&self, idx: usize) -> Option<&str> {
        let range = self.range(idx)?;
        Some(self.as_slice().index(range))
    }

    pub fn get_at_idx(&'a self, name: &'static str, idx: usize) -> Result<&'a str> {
        self.get(idx).ok_or_else(|| Error::CannotFindFieldInRow {
            name,
            row: self.to_owned(),
            idx,
        })
    }

    pub fn get_parsed_at_idx<E, T>(&'a self, name: &'static str, idx: usize) -> Result<T>
    where
        T: FromStr<Err = E>,
        Error: From<E>,
    {
        let data = self.get_at_idx(name, idx)?;
        match data.parse() {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::CannotParseField {
                source: Box::new(Error::from(e)),
                data: data.to_string(),
                name,
                raw_row: self.to_owned(),
            }),
        }
    }

    pub fn get_custom_parsed_at_idx<E, T, F>(
        &'a self,
        name: &'static str,
        idx: usize,
        parser: F,
    ) -> Result<T>
    where
        F: Fn(&str) -> core::result::Result<T, E>,
        Error: From<E>,
    {
        let data = self.get_at_idx(name, idx)?;
        match parser(data) {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::CannotParseField {
                source: Box::new(Error::from(e)),
                data: data.to_string(),
                name,
                raw_row: self.to_owned(),
            }),
        }
    }

    pub fn get_opt_at_idx(&'a self, name: &'static str, idx: usize) -> Result<Option<&'a str>> {
        if self.get(idx).map(|s| s.is_empty()).unwrap_or(true) {
            return Ok(None);
        }

        self.get_at_idx(name, idx).map(Some)
    }

    pub fn get_opt_parsed_at_idx<E, T>(
        &'a self,
        name: &'static str,
        idx: usize,
    ) -> Result<Option<T>>
    where
        T: FromStr<Err = E>,
        Error: From<E>,
    {
        if self.get(idx).map(|s| s.is_empty()).unwrap_or(true) {
            return Ok(None);
        }

        self.get_parsed_at_idx(name, idx).map(Some)
    }

    pub fn get_opt_custom_parsed_at_idx<E, T, F>(
        &'a self,
        name: &'static str,
        idx: usize,
        parser: F,
    ) -> Result<Option<T>>
    where
        F: Fn(&str) -> core::result::Result<T, E>,
        Error: From<E>,
    {
        if self.get(idx).map(|s| s.is_empty()).unwrap_or(true) {
            return Ok(None);
        }

        self.get_custom_parsed_at_idx(name, idx, parser).map(Some)
    }
}

pub trait PrimaryKey: PartialOrd + Ord + PartialEq + Eq {}

pub trait CompareWithRow {
    type Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool;
}

pub trait CompareWithPrimaryKey {
    type PrimaryKey: PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool;
}

// // handle the INSERT-UPDATE logic with lastchanged if applicibale
// // if the rows don't match via CompareWithRow, then the self should always be returned
// // only when the rows match and the other 'row' is newer, and this replacement behaviour is defined
// // should the other 'row' be returned
// pub trait LatestRow<'data>: GetTable<'data> + CompareWithRow<'data, Row = Self<'data>> {
//     fn latest_row(&mut self, row: Self) -> Self;
// }

// pub fn required_partitions<'data, T>(data: &[T]) -> BTreeSet<T::Partition>
// where
//     T: Send + GetTable<'data> + CompareWithRow<'data, Row = T>,
// {
//     data.iter().map(|row| row.partition_suffix()).collect()
// }

// in memory
// the input map is all
// pub fn merge_with_partitions<'data, T>(
//     mut existing: BTreeMap<<T as GetTable<'data>>::Partition, collections::BTreeMap<T::PrimaryKey, T>>,
//     data: &[T],
// ) -> BTreeMap<<T as GetTable<'static>>::Partition, collections::BTreeMap<T::PrimaryKey, T>>
// where
//     T: Send + GetTable<'data> + CompareWithRow<'data, Row = T> + fmt::Debug,
// {
//     for row in data {
//         let partition = row.partition_suffix();
//         let pk = row.primary_key();
//         if let Some(entry) = existing.get_mut(&partition) {
//             entry.entry(pk).or_insert_with(|| row.to_owned());
//         } else {
//             existing.insert(partition, iter::once((pk, row.clone())).to_owned());
//         }
//     }

//     existing
// }

// gets period, 1 based
pub fn naive_time_to_five_min_period(nt: chrono::NaiveTime) -> NonZeroU16 {
    NonZeroU16::new(
        u16::try_from(
            nt.sub(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .num_minutes()
                .div(5)
                .add(1),
        )
        .unwrap(),
    )
    .unwrap()
}

pub fn naive_time_to_thirty_min_period(nt: chrono::NaiveTime) -> NonZeroU8 {
    NonZeroU8::new(
        u8::try_from(
            nt.sub(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .num_minutes()
                .div(30)
                .add(1),
        )
        .unwrap(),
    )
    .unwrap()
}

// Represents a given dispatch period (5 min period)
// Parsed from YYYYMMDDPPP
#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
pub struct DispatchPeriod {
    date: chrono::NaiveDate,
    period: u16,
}

impl From<DispatchPeriod> for NaiveDateTime {
    fn from(value: DispatchPeriod) -> Self {
        value.date.and_time(value.time())
    }
}

impl DispatchPeriod {
    pub fn time(&self) -> chrono::NaiveTime {
        NaiveTime::MIN + Duration::minutes(5 * i64::from(self.period - 1))
    }
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
        self.date
            .and_hms_opt(
                u32::from((self.period - 1) / 12),
                u32::from((self.period - 1) % 12) * 5,
                0,
            )
            .unwrap()
    }
    pub fn from_end(end_time: NaiveDateTime) -> DispatchPeriod {
        DispatchPeriod::from_start(end_time - Duration::minutes(5))
    }
    pub fn from_start(start_time: NaiveDateTime) -> DispatchPeriod {
        DispatchPeriod {
            date: start_time.date(),
            period: naive_time_to_five_min_period(start_time.time()).get(),
        }
    }

    //    pub fn datetime_ending(&self) -> chrono::NaiveDateTime {
    //        self.datetime_starting() + chrono::Duration::minutes(5)
    //    }
}

impl fmt::Display for DispatchPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:03}", self.date.format(Self::format()), self.period)
    }
}

impl str::FromStr for DispatchPeriod {
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
            date: mms_period_datepart::parse(&s[0..8])?,
            period,
        })
    }
}

// Represents a given trading period (30 min period)
// Parsed from YYYYMMDDPP
#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
pub struct TradingPeriod {
    date: chrono::NaiveDate,
    period: u8,
}

impl From<TradingPeriod> for NaiveDateTime {
    fn from(value: TradingPeriod) -> Self {
        value.date.and_time(value.time())
    }
}

impl TradingPeriod {
    pub fn time(&self) -> chrono::NaiveTime {
        NaiveTime::MIN + Duration::minutes(30 * i64::from(self.period - 1))
    }
    pub fn year(&self) -> i32 {
        self.date.year()
    }
    pub fn month(&self) -> u32 {
        self.date.month()
    }
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
        self.date
            .and_hms_opt(
                u32::from((self.period - 1) / 2),
                30 * u32::from((self.period - 1) % 2),
                0,
            )
            .unwrap()
    }

    pub fn from_end(end_time: NaiveDateTime) -> TradingPeriod {
        TradingPeriod::from_start(end_time - Duration::minutes(30))
    }
    pub fn from_start(start_time: NaiveDateTime) -> TradingPeriod {
        TradingPeriod {
            date: start_time.date(),
            period: naive_time_to_thirty_min_period(start_time.time()).get(),
        }
    }
}

impl fmt::Display for TradingPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:02}", self.date.format(Self::format()), self.period)
    }
}

impl str::FromStr for TradingPeriod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if !s.is_ascii() {
            return Err(Error::InvalidTradingPeriod(s.into()));
        };

        match s.len() {
            10 => {
                let period = s[8..10].parse::<u8>()?;

                if !(1..=48).contains(&period) {
                    return Err(Error::InvalidTradingPeriod(s.into()));
                }

                Ok(crate::TradingPeriod {
                    date: mms_period_datepart::parse(&s[0..8])?,
                    period,
                })
            }
            19 | 21 => mms_datetime::parse(s).map(TradingPeriod::from_end),
            _ => Err(Error::InvalidTradingPeriod(s.into())),
        }
    }
}

pub mod mms_datetime {
    use crate::{DispatchPeriod, Error, Result, TradingPeriod};
    use alloc::string::ToString;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    pub fn parse(input: &str) -> Result<NaiveDateTime> {
        let dt = if input.starts_with('"') {
            if input.len() != 21 {
                return Err(Error::ParseDateInternal {
                    message: alloc::format!(
                        "Incorrect length, expected 21 but got {}",
                        input.len()
                    ),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                });
            }

            let year = input[1..=4]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[1..=4].to_string()))?;
            let month = input[6..=7]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[6..=7].to_string()))?;
            let day = input[9..=10]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[9..=10].to_string()))?;
            let hour = input[12..=13]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[11..=13].to_string()))?;
            let minute = input[15..=16]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[15..=16].to_string()))?;
            let second = input[18..=19]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[18..=19].to_string()))?;

            let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
                Error::ParseDateInternal {
                    message: alloc::format!("Invalid values for ymd: {year}-{month}-{day}"),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                }
            })?;

            let time = NaiveTime::from_hms_opt(hour, minute, second).ok_or_else(|| {
                Error::ParseDateInternal {
                    message: alloc::format!("Invalid values for hms: {hour}:{minute}:{second}"),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                }
            })?;

            date.and_time(time)
        } else {
            match input.len() {
                19 => (),
                10 => return Ok(NaiveDateTime::from(input.parse::<TradingPeriod>()?)),
                11 => return Ok(NaiveDateTime::from(input.parse::<DispatchPeriod>()?)),
                _ => {
                    return Err(Error::ParseDateInternal {
                        message: alloc::format!(
                            "Incorrect length, expected `19` but got {}",
                            input.len()
                        ),
                        input: input.to_string(),
                        format: "%Y/%m/%d %H:%M:%S",
                    });
                }
            }

            let year = input[0..=3]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[0..=3].to_string()))?;
            let month = input[5..=6]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[5..=6].to_string()))?;
            let day = input[8..=9]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[8..=9].to_string()))?;
            let hour = input[11..=12]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[10..=12].to_string()))?;
            let minute = input[14..=15]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[14..=15].to_string()))?;
            let second = input[17..=18]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[17..=18].to_string()))?;

            let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
                Error::ParseDateInternal {
                    message: alloc::format!("Invalid values for ymd: {year}-{month}-{day}"),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                }
            })?;

            let time = NaiveTime::from_hms_opt(hour, minute, second).ok_or_else(|| {
                Error::ParseDateInternal {
                    message: alloc::format!("Invalid values for hms: {hour}:{minute}:{second}"),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                }
            })?;

            date.and_time(time)
        };

        Ok(dt)
    }
}

pub mod mms_decimal {
    use rust_decimal::Decimal;

    use crate::Result;

    pub fn parse(input: &str) -> Result<Decimal> {
        match input.parse() {
            Ok(d) => Ok(d),
            Err(e) => {
                if input.contains("E-") || input.contains("e-") {
                    Ok(Decimal::ZERO)
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

pub mod mms_decimal_opt {
    use rust_decimal::Decimal;

    use crate::{mms_decimal, Result};

    pub fn parse(input: &str) -> Result<Option<Decimal>> {
        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(mms_decimal::parse(&input)?))
        }
    }
}

pub mod mms_datetime_opt {
    use crate::mms_datetime;
    use crate::Result;
    use chrono::NaiveDateTime;

    pub fn parse(input: &str) -> Result<Option<NaiveDateTime>> {
        if input.trim().is_empty() {
            return Ok(None);
        }

        mms_datetime::parse(input).map(Some)
    }
}

pub mod mms_date {
    use crate::{Error, Result};
    use alloc::string::ToString;
    use chrono::NaiveDate;

    pub fn parse(input: &str) -> Result<NaiveDate> {
        if input.starts_with('"') {
            if input.len() != 12 {
                return Err(Error::ParseDateInternal {
                    message: alloc::format!(
                        "Incorrect length, expected 12 but got {}",
                        input.len()
                    ),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                });
            }

            let year = input[1..=4]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[1..=4].to_string()))?;
            let month = input[6..=7]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[6..=7].to_string()))?;
            let day = input[9..=10]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[9..=10].to_string()))?;

            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| Error::ParseDateInternal {
                message: alloc::format!("Invalid values for ymd: {year}-{month}-{day}"),
                input: input.to_string(),
                format: "%Y/%m/%d %H:%M:%S",
            })
        } else {
            if input.len() != 10 {
                return Err(Error::ParseDateInternal {
                    message: alloc::format!(
                        "Incorrect length, expected 10 but got {}",
                        input.len()
                    ),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                });
            }

            let year = input[0..=3]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[0..=3].to_string()))?;
            let month = input[5..=6]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[5..=6].to_string()))?;
            let day = input[8..=9]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[8..=9].to_string()))?;

            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| Error::ParseDateInternal {
                message: alloc::format!("Invalid values for ymd: {year}-{month}-{day}"),
                input: input.to_string(),
                format: "%Y/%m/%d %H:%M:%S",
            })
        }
    }
}

pub mod mms_period_datepart {
    use crate::{Error, Result};
    use alloc::string::ToString;
    use chrono::NaiveDate;

    pub fn parse(input: &str) -> Result<NaiveDate> {
        if input.starts_with('"') {
            if input.len() != 10 {
                return Err(Error::ParseDateInternal {
                    message: alloc::format!(
                        "Incorrect length, expected 12 but got {}",
                        input.len()
                    ),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                });
            }

            let year = input[1..=4]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[1..=4].to_string()))?;
            let month = input[5..=6]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[5..=6].to_string()))?;
            let day = input[7..=8]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[7..=8].to_string()))?;

            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| Error::ParseDateInternal {
                message: alloc::format!("Invalid values for ymd: {year}-{month}-{day}"),
                input: input.to_string(),
                format: "%Y/%m/%d %H:%M:%S",
            })
        } else {
            if input.len() != 8 {
                return Err(Error::ParseDateInternal {
                    message: alloc::format!(
                        "Incorrect length, expected 10 but got {}",
                        input.len()
                    ),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                });
            }

            let year = input[0..=3]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[0..=3].to_string()))?;
            let month = input[4..=5]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[4..=5].to_string()))?;
            let day = input[6..=7]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[6..=7].to_string()))?;

            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| Error::ParseDateInternal {
                message: alloc::format!("Invalid values for ymd: {year}-{month}-{day}"),
                input: input.to_string(),
                format: "%Y/%m/%d %H:%M:%S",
            })
        }
    }
}

pub mod mms_time {
    use crate::{Error, Result};
    use alloc::string::ToString;
    use chrono::NaiveTime;

    pub fn parse(input: &str) -> Result<NaiveTime> {
        if input.starts_with('"') {
            if input.len() != 10 {
                return Err(Error::ParseDateInternal {
                    message: alloc::format!(
                        "Incorrect length, expected 10 but got {}",
                        input.len()
                    ),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                });
            }

            let hour = input[1..=2]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[1..=2].to_string()))?;
            let minute = input[4..=5]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[4..=5].to_string()))?;
            let second = input[7..=8]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[7..=8].to_string()))?;

            NaiveTime::from_hms_opt(hour, minute, second).ok_or_else(|| Error::ParseDateInternal {
                message: alloc::format!("Invalid values for hms: {hour}:{minute}:{second}"),
                input: input.to_string(),
                format: "%Y/%m/%d %H:%M:%S",
            })
        } else {
            if input.len() != 8 {
                return Err(Error::ParseDateInternal {
                    message: alloc::format!("Incorrect length, expected 8 but got {}", input.len()),
                    input: input.to_string(),
                    format: "%Y/%m/%d %H:%M:%S",
                });
            }

            let hour = input[0..=1]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[0..=1].to_string()))?;
            let minute = input[3..=4]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[3..=4].to_string()))?;
            let second = input[6..=7]
                .parse()
                .map_err(|e| Error::ParseIntDetailed(e, input[6..=7].to_string()))?;

            NaiveTime::from_hms_opt(hour, minute, second).ok_or_else(|| Error::ParseDateInternal {
                message: alloc::format!("Invalid values for hms: {hour}:{minute}:{second}"),
                input: input.to_string(),
                format: "%Y/%m/%d %H:%M:%S",
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;

    use std::dbg;

    #[test]
    fn test_csv_parse_header() {
        let data = "C,NEMP.WORLD,NEXT_DAY_DISPATCH,AEMO,PUBLIC,2023/01/06,04:10:01,0000000378281515,NEXT_DAY_DISPATCH,0000000378281511\n";
        let mut indexes = Vec::from([0; 10_000]);
        let mut output = Vec::from([0; 10_000]);
        let mut reader = CsvReader::new();
        let row =reader.read_row(&data, &mut output, &mut indexes).unwrap();
        
        dbg!(&row, row.iter_fields().collect::<Vec<_>>());

        AemoHeader::from_row(row).unwrap();
    }

    #[test]
    fn csv_parse_headings() {
        let data = "I,DISPATCH,UNIT_SOLUTION,3,SETTLEMENTDATE,RUNNO,DUID,TRADETYPE,DISPATCHINTERVAL,INTERVENTION,CONNECTIONPOINTID,DISPATCHMODE,AGCSTATUS,INITIALMW,TOTALCLEARED,RAMPDOWNRATE,RAMPUPRATE,LOWER5MIN,LOWER60SEC,LOWER6SEC,RAISE5MIN,RAISE60SEC,RAISE6SEC,DOWNEPF,UPEPF,MARGINAL5MINVALUE,MARGINAL60SECVALUE,MARGINAL6SECVALUE,MARGINALVALUE,VIOLATION5MINDEGREE,VIOLATION60SECDEGREE,VIOLATION6SECDEGREE,VIOLATIONDEGREE,LASTCHANGED,LOWERREG,RAISEREG,AVAILABILITY,RAISE6SECFLAGS,RAISE60SECFLAGS,RAISE5MINFLAGS,RAISEREGFLAGS,LOWER6SECFLAGS,LOWER60SECFLAGS,LOWER5MINFLAGS,LOWERREGFLAGS,RAISEREGAVAILABILITY,RAISEREGENABLEMENTMAX,RAISEREGENABLEMENTMIN,LOWERREGAVAILABILITY,LOWERREGENABLEMENTMAX,LOWERREGENABLEMENTMIN,RAISE6SECACTUALAVAILABILITY,RAISE60SECACTUALAVAILABILITY,RAISE5MINACTUALAVAILABILITY,RAISEREGACTUALAVAILABILITY,LOWER6SECACTUALAVAILABILITY,LOWER60SECACTUALAVAILABILITY,LOWER5MINACTUALAVAILABILITY,LOWERREGACTUALAVAILABILITY,SEMIDISPATCHCAP,DISPATCHMODETIME\n";
        let mut indexes = Vec::from([0; 10_000]);
        let mut output = Vec::from([0; 10_000]);
        let mut reader = CsvReader::new();
        let row =reader.read_row(&data, &mut output, &mut indexes).unwrap();

        assert_eq!(row.iter_fields().count(), 61);

        FileKey::from_row(row).unwrap();
    }

    #[test]
    fn naive_time_to_dispatch_period() {
        assert_eq!(
            naive_time_to_five_min_period(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()).get(),
            1
        );
        assert_eq!(
            naive_time_to_five_min_period(chrono::NaiveTime::from_hms_opt(1, 0, 0).unwrap()).get(),
            13
        );
        assert_eq!(
            naive_time_to_five_min_period(chrono::NaiveTime::from_hms_opt(6, 3, 0).unwrap()).get(),
            73
        );
        assert_eq!(
            naive_time_to_five_min_period(chrono::NaiveTime::from_hms_opt(12, 47, 0).unwrap())
                .get(),
            154
        );
        assert_eq!(
            naive_time_to_five_min_period(chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap())
                .get(),
            175
        );
        assert_eq!(
            naive_time_to_five_min_period(chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap())
                .get(),
            288
        );
    }

    #[test]
    fn dispatch_period() {
        assert!(matches!("20211101000".parse::<DispatchPeriod>(), Err(_)));
        assert!(matches!("20211101289".parse::<DispatchPeriod>(), Err(_)));
        assert!(matches!("20211501288".parse::<DispatchPeriod>(), Err(_)));
        assert!(matches!("20211132288".parse::<DispatchPeriod>(), Err(_)));

        assert_eq!(
            "20211101001".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );

        assert_eq!(
            "20211101002".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(0, 5, 0)
                .unwrap(),
        );

        assert_eq!(
            "20211101287".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(23, 50, 0)
                .unwrap(),
        );

        assert_eq!(
            "20211101288".parse::<DispatchPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(23, 55, 0)
                .unwrap(),
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
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );

        assert_eq!(
            "2021110102".parse::<TradingPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(0, 30, 0)
                .unwrap(),
        );

        assert_eq!(
            "2021110147".parse::<TradingPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(23, 0, 0)
                .unwrap(),
        );

        assert_eq!(
            "2021110148".parse::<TradingPeriod>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 11, 1)
                .unwrap()
                .and_hms_opt(23, 30, 0)
                .unwrap(),
        );
    }
}
