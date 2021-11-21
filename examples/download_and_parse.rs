use log::info;
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use scraper::{html, Selector}; //, node, selector};
use std::{error, fmt, io, path};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    test_dispatch_is::download_and_parse();

    test_dispatch_scada::download_and_parse();
}

// #[cfg(test)]
// mod test_rooftop_forecast {
//     use mmsdm::data_model;
//     use log::info;
//     const LOCATION: &str = "http://nemweb.com.au/Reports/Current/ROOFTOP_PV/FORECAST/";

//     #[tokio::test]
//     fn download_and_parse() {
//         env_logger::init();
//         let all_data: Vec<data_model::RooftopForecast1> = super::url_get_files(LOCATION).unwrap();
//         info!("Downloaded and processed {} files", all_data.len());
//     }
// }

// #[cfg(test)]
// mod test_rooftop_actual {
//     use mmsdm::data_model;
//     use log::info;
//     const LOCATION: &str = "http://nemweb.com.au/Reports/Current/ROOFTOP_PV/ACTUAL/";

//     #[tokio::test]
//     async fn download_and_parse() {
//         env_logger::init();
//         let all_data: Vec<data_model::RooftopActual2> = super::url_get_files(LOCATION).unwrap();
//         info!("Downloaded and processed {} files", all_data.len());
//     }
// }

// #[cfg(test)]
// mod test_yestbid {
//     use log::info;
//     use mmsdm::data_model;
//     use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
//     const LOCATION: &str = "http://nemweb.com.au/Reports/Current/Yesterdays_Bids_Reports/";

//     #[tokio::test]
//     async fn download_and_parse() {
//         env_logger::init();
//         let all_data: Vec<mmsdm::AemoFile> = super::url_get_files(LOCATION).await.unwrap();

//         info!("Downloaded and processed {} files", all_data.len());

//         let bid_day_offers = all_data
//             .par_iter()
//             .map(mmsdm::AemoFile::get_table::<data_model::OfferBiddayoffer2>)
//             .collect::<Vec<_>>();
//         for bdo in bid_day_offers {
//             info!("BDO rows: {}", bdo.unwrap().len())
//         }
//     }
// }

// #[cfg(test)]
// mod test_dispatch_scada {
//     use log::info;
//     use mmsdm::data_model;
//     use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
//     const LOCATION: &str = "http://nemweb.com.au/Reports/Current/Dispatch_SCADA/";

//     #[tokio::test]
//     async fn download_and_parse() {
//         env_logger::init();
//         let all_data: Vec<mmsdm::AemoFile> = super::url_get_files(LOCATION).await.unwrap();

//         info!("Downloaded and processed {} files", all_data.len());

//         let bid_day_offers = all_data
//             .par_iter()
//             .map(mmsdm::AemoFile::get_table::<data_model::DispatchUnitScada1>)
//             .collect::<Vec<_>>();
//         for bdo in bid_day_offers {
//             info!("BDO rows: {}", bdo.unwrap().len())
//         }
//     }
// }

mod test_dispatch_is {
    use log::info;
    use mmsdm::data_model;
    const LOCATION: &str = "https://nemweb.com.au/Reports/Current/DispatchIS_Reports";

    pub fn download_and_parse() {
        let links = super::get_links(LOCATION).unwrap();

        let rows = links
            .iter()
            .take(8)
            .map(|l| {
                let aemo_file = super::download(LOCATION, l).unwrap();
                aemo_file
                    .get_table::<data_model::DispatchConstraint5>()
                    .unwrap()
            })
            .flatten()
            .collect::<Vec<_>>();

        // let all_data: Vec<mmsdm::AemoFile> = super::url_get_files(LOCATION).await.unwrap();

        info!("Downloaded and processed {} rows", rows.len());

        // let bid_day_offers = all_data.par_iter()
        //     // .map(mmsdm::AemoFile::get_table::<data_model::DispatchPrice4>)
        //     .collect::<Vec<_>>();
        // for row in rows {
        //     info!("rows: {:?}", row)
        // }
    }
}

mod test_dispatch_scada {
    use log::info;
    use mmsdm::data_model;
    const LOCATION: &str = "https://nemweb.com.au/Reports/Current/Dispatch_SCADA";

    pub fn download_and_parse() {
        let links = super::get_links(LOCATION).unwrap();

        let rows = links
            .iter()
            .take(8)
            .map(|l| {
                let aemo_file = super::download(LOCATION, l).unwrap();
                aemo_file
                    .get_table::<data_model::DispatchUnitScada1>()
                    .unwrap()
            })
            .flatten()
            .collect::<Vec<_>>();

        info!("Downloaded and processed {} rows", rows.len());

        // for row in rows {
        //     info!("rows: {:?}", row)
        // }
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
    doc.select(&selector)
        .into_iter()
        .map(|el| el.value().attr("href"))
        .flatten() // only interested in a elements that have a link!
        .map(path::Path::new)
        .map(get_filename_from_path)
        .filter_map(|opt| opt.ok())
        .collect::<Vec<String>>()
}

fn download(base_url: &str, link: &str) -> Result<mmsdm::AemoFile> {
    let path = path::Path::new(&link);
    let file_name = get_filename_from_path(path)?;
    let file_url = format!("{}/{}", base_url, file_name);
    info!("Getting: {}", file_url);
    let bytes = reqwest::blocking::get(&file_url)?
        .error_for_status()?
        .bytes()?;
    let cursor = io::Cursor::new(bytes.to_vec());
    let mut zip = zip::ZipArchive::new(cursor)?;
    let inner_file = zip.by_index(0)?;
    let f = mmsdm::AemoFile::from_reader(inner_file)?;
    Ok(f)
}

fn get_links(url: &str) -> Result<Vec<String>> {
    let doc = reqwest::blocking::get(url)?.text()?;
    log::debug!("Doc: {}", doc);
    let parsed = scraper::Html::parse_document(&doc);

    let links = get_file_links_from_page(parsed);
    info!("Got {} links", links.len());

    Ok(links)
}

#[derive(Debug)]
enum Error {
    InvalidFilePath,
    Io(io::Error),
    Zip(zip::result::ZipError),
    Reqwest(reqwest::Error),
    Aemo(mmsdm::Error),
}

impl From<mmsdm::Error> for Error {
    fn from(error: mmsdm::Error) -> Self {
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
