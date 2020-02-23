#![cfg(test)]
use std::path;
use scraper::{html, Selector};//, node, selector};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use std::{error, fmt, io::{self, Read}};
use aemo_rs;


#[cfg(test)]
mod test_rooftop_forecast {
    use aemo_rs::rooftop_forecast;
    const LOCATION: &str = "http://nemweb.com.au/Reports/Current/ROOFTOP_PV/FORECAST/";
    #[test]
    fn download_and_parse() {
        let all_data: Vec<rooftop_forecast::File> = super::url_get_files(LOCATION).unwrap();
        println!("Downloaded and processed {} files", all_data.len());
    }

}

#[cfg(test)]
mod test_rooftop_actual {
    use aemo_rs::rooftop_actual;
    const LOCATION: &str = "http://nemweb.com.au/Reports/Current/ROOFTOP_PV/ACTUAL/";
    #[test]
    fn download_and_parse() {
        let all_data: Vec<rooftop_actual::File> = super::url_get_files(LOCATION).unwrap();
        println!("Downloaded and processed {} files", all_data.len());
    }

}

#[cfg(test)]
mod test_yestbid {
    use aemo_rs::yestbid;
    const LOCATION: &str = "http://nemweb.com.au/Reports/Current/Yesterdays_Bids_Reports/";

    #[test]
    fn download_and_parse() {
        let all_data: Vec<yestbid::File> = super::url_get_files(LOCATION).unwrap();
        println!("Downloaded and processed {} files", all_data.len());
    }

}

#[cfg(test)]
mod test_dispatch_is {
    use aemo_rs::dispatch_is;
    const LOCATION: &str = "http://nemweb.com.au/Reports/Current/DispatchIS_Reports/";

    #[test]
    fn download_and_parse() {
        let all_data: Vec<dispatch_is::File> = super::url_get_files(LOCATION).unwrap();
        println!("Downloaded and processed {} files", all_data.len());
    }

}

#[cfg(test)]
mod test_dispatch_scada {
    use aemo_rs::dispatch_scada;
    const LOCATION: &str = "http://nemweb.com.au/Reports/Current/Dispatch_SCADA/";

    #[test]
    fn download_and_parse() {
        let all_data: Vec<dispatch_scada::File> = super::url_get_files(LOCATION).unwrap();
        println!("Downloaded and processed {} files", all_data.len());
    }

}


#[cfg(test)]
mod test_daily {
    use aemo_rs::daily;
    const LOCATION: &str = "http://nemweb.com.au/Reports/Current/Daily_Reports/";

    #[test]
    fn download_and_parse() {
        let all_data: Vec<daily::File> = super::url_get_files(LOCATION).unwrap();
        println!("Downloaded and processed {} files", all_data.len());
    }

}




// helper functions for testing purposes
// these should also be useful as examples for consumers of the library
fn get_filename_from_path(path: &path::Path) -> Result<String> {
    let file_stem = path.file_stem().and_then(|s| s.to_str());
    let extension = path.extension().and_then(|e| e.to_str());
    if let (Some(file), Some(ext)) = (file_stem, extension) {
        Ok(format!("{}.{}", file, ext))
    } else {
        Err(Error::InvalidFilePath)
    }
}

fn get_file_links_from_page(doc: html::Html) -> Vec<String> {
    let selector = Selector::parse("a").unwrap();
    doc
        .select(&selector)
        .into_iter()
        .map(|el| el.value().attr("href"))
        .filter_map(|opt| opt) // only interested in a elements that have a link!
        .map(path::Path::new)
        .map(get_filename_from_path)
        .filter_map(|opt| opt.ok())
        .collect::<Vec<String>>()
}

fn download(base_url: &str, link: &str) -> Result<zip::ZipArchive<io::Cursor<Vec<u8>>>> {
    let path = path::Path::new(&link);
    let file_name = get_filename_from_path(path)?;
    let file_url = format!("{}/{}", base_url, file_name);
    let bytes = reqwest::get(&file_url)?.bytes().collect::<std::result::Result<Vec<_>, io::Error>>()?;
    let cursor = io::Cursor::new(bytes);
    let zip = zip::ZipArchive::new(cursor)?;
    Ok(zip)
}

fn archive_to_raw(mut archive: zip::ZipArchive<io::Cursor<Vec<u8>>>) -> Result<aemo_rs::RawAemoFile> {
    let inner_file = archive.by_index(0)?;
    println!("inner file name: {}", inner_file.name());
    let raw = aemo_rs::RawAemoFile::from_bufread(inner_file)?;
    Ok(raw)

}

fn url_get_files<T: aemo_rs::AemoFile>(url: &str) -> Result<Vec<T>> {
    let doc = reqwest::get(url)?.text()?;
    let parsed = scraper::Html::parse_document(&doc);
    get_file_links_from_page(parsed)
        [0..=5]
        .par_iter()
        .inspect(|link| println!("downloading {}", link))
        .map(|link| {
            download(url, &link)
                .and_then(archive_to_raw)
                .and_then(|raw| T::from_raw(raw).map_err(Error::Aemo))
        })
        .collect::<Result<Vec<_>>>()
}

#[derive(Debug)]
enum Error {
    InvalidFilePath,
    Io(io::Error),
    Zip(zip::result::ZipError),
    Reqwest(reqwest::Error),
    Aemo(aemo_rs::Error),
}

impl From<aemo_rs::Error> for Error {
    fn from(error: aemo_rs::Error) -> Self {
        Error::Aemo(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}


impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Error::Zip(error)
    }
}


impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFilePath => write!(f, "invalid file path"),
            Self::Io(e) => write!(f, "io error: {}", e),
            Self::Zip(e) => write!(f, "zip error: {}", e),
            Self::Reqwest(e) => write!(f, "reqwest error: {}", e),
            Self::Aemo(s) => write!(f, "AEMO error: {}", s),
        }
    }

}

impl error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

