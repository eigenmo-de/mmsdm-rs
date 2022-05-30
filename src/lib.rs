#![deny(clippy::all)]
#![deny(warnings)]
use std::{collections, fs, io, num};

pub mod data_model;

#[cfg(feature = "sql_server")]
pub mod sql_server;

pub use mmsdm_core::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] mmsdm_core::Error),

    #[error("Tiberius error: {0}")]
    #[cfg(feature = "sql_server")]
    Tiberius(#[from] tiberius::error::Error),

    #[error("Csv error: {0})")]
    Csv(#[from] csv::Error),

    #[error("ParseInt error: {0}")]
    ParseInt(#[from] num::ParseIntError),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),


    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    ConvertFromInt(#[from] num::TryFromIntError),
}

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug, Clone)]
pub struct AemoFile {
    pub header: mmsdm_core::AemoHeader,
    pub data: collections::HashMap<mmsdm_core::FileKey, mmsdm_core::Subtable>,
}


#[derive(Debug)]
pub struct AemoDiskFile {
    pub header: mmsdm_core::AemoHeader,
    handle: zip::ZipArchive<fs::File>,
    keys: collections::HashMap<mmsdm_core::FileKey, csv::StringRecord>,
}

pub struct AemoDiskFileIter<'a, T>
where
    T: serde::de::DeserializeOwned + Send + mmsdm_core::GetTable,
{
    file_key: mmsdm_core::FileKey,
    inner: csv::StringRecordsIntoIter<zip::read::ZipFile<'a>>,
    _d: std::marker::PhantomData<T>,
    headings: csv::StringRecord,
    found_records: bool,
}

impl<'a, T> Iterator for AemoDiskFileIter<'a, T>
where
    T: serde::de::DeserializeOwned + Send + mmsdm_core::GetTable,
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
                && &self.file_key.data_set_name == &record[1]
                && self
                    .file_key
                    .table_name
                    .as_ref()
                    .map(|n| n == &record[2])
                    .unwrap_or_else(|| &record[2] == "")
                && &self.file_key.version.to_string() == &record[3]
            {
                self.found_records = true;
                return Some(record.deserialize(Some(&self.headings)).map_err(|cause| {
                    mmsdm_core::Error::CsvRowDe {
                        cause,
                        headings: Some(self.headings.clone()),
                        data: record.clone(),
                    }.into()
                }));
            } else {
                if self.found_records {
                    return None;
                } else {
                    continue;
                }
            }
        }
        None
    }
}


impl AemoDiskFile {
    pub fn get_table<'a, T>(&'a mut self) -> Result<AemoDiskFileIter<'a, T>>
    where
        T: serde::de::DeserializeOwned + Send + mmsdm_core::GetTable + 'static,
    {
        let latest_version = T::get_file_key();
        for version in (1..=latest_version.version).rev() {
            let current_key = latest_version.with_version(version);
            if self.keys.contains_key(&current_key) {
                return self.get_specific_table(current_key);
            } else {
                log::warn!(
                    "For file key {}, version {} was not available",
                    latest_version,
                    version,
                );
            }
        }
        Err(mmsdm_core::Error::MissingFile(latest_version).into())
    }

    pub fn get_specific_table<'a, T>(
        &'a mut self,
        file_key: mmsdm_core::FileKey,
    ) -> Result<AemoDiskFileIter<'a, T>>
    where
        T: serde::de::DeserializeOwned + Send + mmsdm_core::GetTable,
    {
        // assert that the key exists
        let headings = self
            .keys
            .get(&file_key)
            .ok_or(mmsdm_core::Error::MissingFile(file_key.clone()))?
            .to_owned();

        log::info!("Table found and parsed for file key {}", file_key);

        let inner_file = self.handle.by_index(0)?;

        Ok(AemoDiskFileIter {
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

    pub fn file_keys(&self) -> std::collections::hash_map::Keys<mmsdm_core::FileKey, csv::StringRecord> {
        self.keys.keys()
    }

    pub fn contains_file(&self, key: mmsdm_core::FileKey) -> bool {
        self.keys.contains_key(&key)
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
            let first = records.next().ok_or(mmsdm_core::Error::MissingHeaderRecord)??;

            let header: mmsdm_core::AemoHeader = first.deserialize(None).map_err(|cause| mmsdm_core::Error::CsvRowDe {
                cause,
                headings: None,
                data: first,
            })?;

            // placeholder
            let mut footer: Result<mmsdm_core::AemoFooter> = Err(mmsdm_core::Error::MissingFooterRecord.into());
            let mut keys: collections::HashMap<mmsdm_core::FileKey, csv::StringRecord> =
                collections::HashMap::new();

            let mut num_rows = 1;

        for record in records {
                let record = record?;
                match record.get(0) {
                    Some("D") => {
                        num_rows += 1;
                        let row_len = record.len();
                        if row_len < 5 {
                            return Err(mmsdm_core::Error::TooShortRow(row_len).into());
                        }
                    }
                    Some("I") => {
                        num_rows += 1;
                        let row_len = record.len();
                        if row_len < 5 {
                            return Err(mmsdm_core::Error::TooShortRow(row_len).into());
                        }
                        let key = mmsdm_core::FileKey {
                            data_set_name: record[1].into(),
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

                        footer = record.deserialize(None).map_err(|cause| mmsdm_core::Error::CsvRowDe {
                            cause,
                            headings: None,
                            data: record,
                        }.into());
                }
                    Some(t) => return Err(mmsdm_core::Error::UnexpectedRowType(t.into()).into()), //unexpected row, as correct files only have "C", "I" and "D"
                    None => return Err(mmsdm_core::Error::EmptyRow.into()),
                }
            }

            let expected_line_count = footer?.line_count_inclusive;

            if num_rows != expected_line_count {
                return Err(mmsdm_core::Error::IncorrectLineCount {
                    got: num_rows,
                    expected: expected_line_count,
                }.into());
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

impl AemoFile {
    pub fn get_table<T>(&self) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send + mmsdm_core::GetTable,
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
        Err(mmsdm_core::Error::MissingFile(latest_version).into())
    }
    pub fn get_specific_table<T>(&self, file_key: &mmsdm_core::FileKey) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send + mmsdm_core::GetTable,
    {
        let subtable = self
            .data
            .get(&file_key)
            .ok_or(mmsdm_core::Error::MissingFile(file_key.clone()))?;

        subtable
            .data
            .iter()
            .map(|rec| {
                rec.deserialize(Some(&subtable.headings))
                    .map_err(|cause| mmsdm_core::Error::CsvRowDe {
                        cause,
                        headings: Some(subtable.headings.clone()),
                        data: rec.clone(),
                    }.into())
            })
            .collect()
    }

    pub fn file_keys(&self) -> std::collections::hash_map::Keys<mmsdm_core::FileKey, mmsdm_core::Subtable> {
        self.data.keys()
    }

    pub fn contains_file(&self, key: mmsdm_core::FileKey) -> bool {
        self.data.contains_key(&key)
    }

    pub fn from_reader(br: impl io::Read) -> Result<Self> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(br);
        let mut records = reader.records();
        let first = records.next().ok_or(mmsdm_core::Error::MissingHeaderRecord)??;

        let header: mmsdm_core::AemoHeader = first.deserialize(None).map_err(|cause| mmsdm_core::Error::CsvRowDe {
            cause,
            headings: None,
            data: first,
        })?;

        // placeholder
        let mut footer: Result<mmsdm_core::AemoFooter> = Err(mmsdm_core::Error::MissingFooterRecord.into());
        let mut data: collections::HashMap<mmsdm_core::FileKey, mmsdm_core::Subtable> = collections::HashMap::new();

        for record in records {
            let record = record?;
            match record.get(0) {
                Some("D") => {
                    let row_len = record.len();
                    if row_len < 5 {
                        return Err(mmsdm_core::Error::TooShortRow(row_len).into());
                    }
                    let key = mmsdm_core::FileKey {
                        data_set_name: record[1].into(),
                        table_name: match &record[2] {
                            "" => None,
                            otherwise => Some(otherwise.to_string()),
                        },
                        version: record[3].parse()?,
                    };

                    if let Some(v) = data.get_mut(&key) {
                        v.append(record);
                    } else {
                        return Err(mmsdm_core::Error::MissingSubtableHeadings(key).into());
                    }
                }
                Some("I") => {
                    let row_len = record.len();
                    if row_len < 5 {
                        return Err(mmsdm_core::Error::TooShortRow(row_len).into());
                    }
                    let key = mmsdm_core::FileKey {
                        data_set_name: record[1].into(),
                        table_name: match &record[2] {
                            "" => None,
                            otherwise => Some(otherwise.to_string()),
                        },
                        version: record[3].parse()?,
                    };

                    let lowercased = record.iter().map(|v| v.to_lowercase()).collect();

                    data.insert(key, mmsdm_core::Subtable::new(lowercased));
                }
                Some("C") => {
                    footer = record.deserialize(None).map_err(|cause| mmsdm_core::Error::CsvRowDe {
                        cause,
                        headings: None,
                        data: record,
                    }.into());
                }
                Some(t) => return Err(mmsdm_core::Error::UnexpectedRowType(t.into()).into()), //unexpected row, as correct files only have "C", "I" and "D"
                None => return Err(mmsdm_core::Error::EmptyRow.into()),
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
            Err(mmsdm_core::Error::IncorrectLineCount {
                got: data_rows + 2,
                expected: expected_line_count,
            }.into())
        }
    }
}
