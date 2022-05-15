use std::{io, num};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This occurs when we are missing the footer record which lists the number of rows in the file
    #[error("aemo file is missing the final `c` record")]
    MissingFooterRecord,

    /// This occurs when we are missing the header record which lists metadata about the file
    #[error("aemo file is missing the first `c` record")]
    MissingHeaderRecord,

    /// This occurs when the desired file key can't be found in the RawAemoFile
    #[error("aemo file was missing {}.{:?}.v{} section in the file ", .0.data_set_name, .0.table_name, .0.version)]
    MissingFile(crate::FileKey),

    /// This occurs when the desired file key can't be found in the RawAemoFile
    #[error(
        "aemo file was missing headings for {}.{:?}.v{} section in the file ", .0.data_set_name, .0.table_name, .0.version
    )]
    MissingSubtableHeadings(crate::FileKey),

    /// This occurs when an entire row is empty after the first three columns
    #[error("aemo file row is empty")]
    EmptyRow,

    #[error("Empty AEMO file: {0:?}")]
    EmptyFile(crate::FileKey),

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
    UnhandledFileKey(crate::FileKey),

    #[error("ParseInt error: {0}")]
    ParseInt(#[from] num::ParseIntError),

    #[error("ParseDate error: {0}")]
    ParseDate(#[from] chrono::ParseError),

    #[error("Csv error: {cause} (headings: {headings:?}, data: {data:?})")]
    CsvRowDe {
        cause: csv::Error,
        headings: Option<csv::StringRecord>,
        data: csv::StringRecord,
    },

    #[error("Error creating file log")]
    CreateFileLogError,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Csv error: {0})")]
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

    #[cfg(feature = "save_as_parquet")]
    #[error(transparent)]
    Arrow(#[from] arrow2::error::ArrowError),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    ConvertToInt(#[from] num::TryFromIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
