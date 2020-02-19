use std::{error, fmt, path, io::{self, Read}, collections};

use futures::future::Future;

use std::convert::TryFrom;
mod scraping;
mod aemo;
use aemo::yestbid;
use aemo::dispatch_is;
use rand::Rng;

use std::{sync, thread};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
// const NEMWEB: &str = "http://www.nemweb.com.au";

fn download(base_url: &str, link: &str) -> Result<zip::ZipArchive<io::Cursor<Vec<u8>>>> {
    let path = path::Path::new(&link);
    let file_name = scraping::get_filename_from_path(path)?;
    // #TODO
    // change this to info later
    println!("link is {}", file_name);

    let file_url = format!("{}/{}", base_url, file_name);
    let bytes = reqwest::get(&file_url)?.bytes().collect::<std::result::Result<Vec<_>, io::Error>>()?;
    let cursor = io::Cursor::new(bytes);
    let zip = zip::ZipArchive::new(cursor)?;
    Ok(zip)
}

fn archive_to_raw(mut archive: zip::ZipArchive<io::Cursor<Vec<u8>>>) -> Result<aemo::RawAemoFile> {
    let inner_file = archive.by_index(0)?;
    println!("inner file name: {}", inner_file.name());
    let raw = aemo::RawAemoFile::from_bufread(inner_file)?;
    Ok(raw)

}

fn main() -> Result<()> {
    // let url = "http://nemweb.com.au/Reports/Current/Daily_Reports/";
    // let url = "http://nemweb.com.au/Reports/Current/Yesterdays_Bids_Reports/";
    let url = "http://nemweb.com.au/Reports/Current/DispatchIS_Reports/";

    let doc = reqwest::get(url)?.text()?;
    let parsed = scraper::Html::parse_document(&doc);
    // here generate n threads, and then put the links in an arc?
    // then the threads just keep grabbing next linnk as they done
    let all_data = scraping::get_file_links_from_page(parsed)
        [0..=5]
        .par_iter()
        .inspect(|link| println!("downloading {}", link))
        .map(|link| download(url, &link).unwrap())
        .map(|archive| archive_to_raw(archive).unwrap())
        .map(|raw| dispatch_is::File::from_raw(raw).unwrap())
        .collect::<Vec<_>>();

    println!("Downloaded and processed {} files", all_data.len());
    Ok(())
}


#[derive(Debug)]
pub enum BackendError {
    InvalidFilePath,
    Custom(String),
    Io(io::Error),
    Zip(zip::result::ZipError),
    Reqwest(reqwest::Error),
    Clickhouse(clickhouse_rs::errors::Error),
    Aemo(aemo::Error),
}

impl From<aemo::Error> for BackendError {
    fn from(error: aemo::Error) -> Self {
        BackendError::Aemo(error)
    }
}


impl From<clickhouse_rs::errors::Error> for BackendError {
    fn from(error: clickhouse_rs::errors::Error) -> Self {
        BackendError::Clickhouse(error)
    }
}

impl From<io::Error> for BackendError {
    fn from(error: io::Error) -> Self {
        BackendError::Io(error)
    }
}


impl From<zip::result::ZipError> for BackendError {
    fn from(error: zip::result::ZipError) -> Self {
        BackendError::Zip(error)
    }
}


impl From<reqwest::Error> for BackendError {
    fn from(error: reqwest::Error) -> Self {
        BackendError::Reqwest(error)
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFilePath => write!(f, "invalid file path"),
            Self::Io(e) => write!(f, "io error: {}", e),
            Self::Zip(e) => write!(f, "zip error: {}", e),
            Self::Reqwest(e) => write!(f, "reqwest error: {}", e),
            Self::Custom(s) => write!(f, "Had custom error: {}", s),
            Self::Clickhouse(s) => write!(f, "clickhouse error: {}", s),
            Self::Aemo(s) => write!(f, "AEMO error: {}", s),
        }
    }

}

impl error::Error for BackendError {}

type Result<T> = std::result::Result<T, BackendError>;





