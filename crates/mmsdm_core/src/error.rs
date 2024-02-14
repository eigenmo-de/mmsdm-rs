#[cfg(feature = "std")]
extern crate std;
use alloc::{boxed::Box, string::String};
use core::{
    convert::Infallible,
    fmt,
    num::{self, ParseIntError},
};

#[cfg(feature = "std")]
use std::io;

use crate::CsvRow;

#[derive(Debug)]
pub enum Error {
    /// This occurs when we are missing the footer record which lists the number of rows in the file
    MissingFooterRecord,

    /// This occurs when we are missing the header record which lists metadata about the file
    MissingHeaderRecord,

    /// This occurs when the desired file key can't be found in the RawAemoFile
    MissingFile {
        data_set_name: &'static str,
        table_name: &'static str,
        version: Option<i32>,
    },

    /// This occurs when the desired file key can't be found in the RawAemoFile
    MissingSubtableHeadings(crate::FileKey<'static>),

    /// This occurs when an entire row is empty after the first three columns
    EmptyRow,

    EmptyFile(crate::FileKey<'static>),

    /// This occurs when a given row in the file doesn't match the expected structure for that section
    /// of the file
    UnexpectedRowType(String),

    /// This occurs when a given row in the file is shorter than expected
    TooShortRow(usize),

    /// This occurs when the number of rows in the file is different to the number listed in the
    /// footer
    IncorrectLineCount {
        got: usize,
        expected: usize,
    },

    /// This occurs when we receive a file_key that we are not familiar with
    UnhandledFileKey(crate::FileKey<'static>),

    ParseInt(num::ParseIntError),
    ParseIntDetailed(num::ParseIntError, String),
    Decimal(rust_decimal::Error),

    ParseDate {
        source: chrono::ParseError,
        input: String,
        format: &'static str,
    },

    ParseDateInternal {
        input: String,
        format: &'static str,
        message: String,
    },

    CreateFileLogError,

    /// This occurs when failing to parse a dispatch period
    InvalidDispatchPeriod(String),

    /// This occurs when failing to parse a trading period
    InvalidTradingPeriod(String),

    ConvertFromInt(num::TryFromIntError),

    CannotFindFieldInRow {
        name: &'static str,
        row: CsvRow<'static>,
        idx: usize,
    },

    CannotParseField {
        source: Box<Error>,
        data: String,
        name: &'static str,
        raw_row: CsvRow<'static>,
    },

    #[cfg(feature = "std")]
    Io(io::Error),
    #[cfg(feature = "arrow")]
    Arrow(arrow::error::ArrowError),
    #[cfg(feature = "std")]
    Zip(zip::result::ZipError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Decimal(e) => write!(f, "Decimal parsing error: {e}"),
            Error::MissingFooterRecord => f.write_str("aemo file is missing the final `c` record"),
            Error::MissingHeaderRecord => f.write_str("aemo file is missing the first `c` record"),
            Error::MissingFile { data_set_name, table_name, version: None } => write!(f, "aemo file was missing any {data_set_name}.{table_name} section in the file"),
            Error::MissingFile { data_set_name, table_name, version: Some(v) } => write!(f, "aemo file was missing {data_set_name}.{table_name}.v{v} section in the file"),
            Error::MissingSubtableHeadings(k) => write!(f, "aemo file was missing headings for {}.{:?}.v{} section in the file", k.data_set_name(), k.table_name(), k.version),
            Error::EmptyRow => f.write_str("aemo file row is empty"),
            Error::EmptyFile(k) => write!(f, "Empty AEMO file: {k:?}"),
            Error::UnexpectedRowType(s) => write!(f, "unexpeted row type of {s}"),
            Error::TooShortRow(x) => write!(f, "aemo file data row of length {x} is too short"),
            Error::IncorrectLineCount { got, expected } => write!(f, "aemo file was supposed to be {expected} lines long but was instead {got} lines long"),
            Error::UnhandledFileKey(x) => write!(f, "Recieved unexpected file of type {x}"),
            Error::ParseInt(x) => write!(f, "ParseInt error: {x}"),
            Error::ParseIntDetailed(x, input) => write!(f, "ParseInt error: {x}, from input: {input}"),
            Error::ParseDate { source, input, format }=> write!(f, "Failed to parse {input} with format {format} due to error: {source}"),
            Error::ParseDateInternal { message, input, format }=> write!(f, "Failed to parse {input} with format {format} due to error: {message}"),
            Error::CreateFileLogError => f.write_str("Error creating file log"),
            Error::InvalidDispatchPeriod(x) => write!(f, "Invalid dispatch period: {x}"),
            Error::InvalidTradingPeriod(x) => write!(f, "Invalid trading period: {x}"),
            Error::ConvertFromInt(x) => write!(f, "TryFromIntError: {x}"),
            Error::CannotFindFieldInRow { name, row, idx } => write!(f, "Unable to find field {name} at index {idx} in row {row:?}"),
            Error::CannotParseField { name, data, source, raw_row } => write!(f, "Unable to parse field {name} from {data} due to {source} in row {raw_row:?}"),
            #[cfg(feature = "std")]    
            Error::Io(e) => write!(f, "Io error: {e:?}"),
            #[cfg(feature = "arrow")]    
            Error::Arrow(e) => write!(f, "Arrow error: {e:?}"),
            #[cfg(feature = "std")]    
            Error::Zip(e) => write!(f, "Zip error: {e:?}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "arrow")]
impl From<arrow::error::ArrowError> for Error {
    fn from(value: arrow::error::ArrowError) -> Self {
        Error::Arrow(value)
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

#[cfg(feature = "std")]
impl From<zip::result::ZipError> for Error {
    fn from(value: zip::result::ZipError) -> Self {
        Error::Zip(value)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}

impl From<rust_decimal::Error> for Error {
    fn from(value: rust_decimal::Error) -> Self {
        Error::Decimal(value)
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(value: num::TryFromIntError) -> Self {
        Error::ConvertFromInt(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
