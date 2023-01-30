use log::{info, warn};
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use scraper::{html, Selector}; //, node, selector};
use std::{env, fs, path, thread, time};

const LOCATIONS: &[&str] = &[
    "https://nemweb.com.au/Reports/Current/Next_Day_Dispatch",
    "https://nemweb.com.au/Reports/Current/Next_Day_Actual_Gen",
    // "https://nemweb.com.au/Reports/Current/Next_Day_Offer_FCAS",
    // "https://nemweb.com.au/Reports/Current/Next_Day_Offer_Energy",
    "https://nemweb.com.au/Reports/Current/PredispatchIS_Reports",
    "https://nemweb.com.au/Reports/Current/DispatchIS_Reports",
    "https://nemweb.com.au/Reports/Current/Dispatch_SCADA",
    "http://nemweb.com.au/Reports/Current/ROOFTOP_PV/ACTUAL/",
    "http://nemweb.com.au/Reports/Current/ROOFTOP_PV/FORECAST/",
];

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // we don't download in parallel as this will result in rate limiting from nemweb server

    let num_files = match env::var("MMSDM_NUM_FILES") {
        Ok(num) => num.parse()?,
        _ => 50,
    };

    let storage_location = match env::var("MMSDM_STORAGE_LOCATION") {
        Ok(num) => num,
        _ => "./data".to_string(),
    };

    for location in LOCATIONS {
        let links = get_links(location).unwrap();
        for link in links.into_iter().take(num_files) {
            download(location, &link, &storage_location)?;
        }
    }

    Ok(())
}

// helper functions for testing purposes
// these should also be useful as examples for consumers of the library
fn get_filename_from_path(path: &path::Path) -> anyhow::Result<String> {
    let file_stem = path.file_stem().and_then(|s| s.to_str());
    let extension = path.extension().and_then(|e| e.to_str());
    if let (Some(file), Some(ext)) = (file_stem, extension) {
        Ok(format!("{file}.{ext}"))
    } else {
        Err(anyhow::anyhow!("Invalid file path"))
    }
}

fn get_file_links_from_page(doc: html::Html) -> Vec<String> {
    let selector = Selector::parse("a").unwrap();
    doc.select(&selector)
        .into_iter()
        .filter_map(|el| el.value().attr("href")) // only interested in a elements that have a link!
        .map(path::Path::new)
        .map(get_filename_from_path)
        .filter_map(|opt| opt.ok())
        .collect::<Vec<String>>()
}

fn download(base_url: &str, link: &str, local_path: &str) -> anyhow::Result<()> {
    let path = path::Path::new(&link);
    let file_name = get_filename_from_path(path)?;
    let file_url = format!("{base_url}/{file_name}");

    let mut dest = path::PathBuf::new();
    dest.push(local_path);
    dest.push(link);

    if !dest.as_path().exists() {
        info!("Getting: {}", file_url);
        for _ in 1..=4 {
            match download_file(&file_url) {
                Ok(bytes) => {
                    info!("Saving to {:?}", dest);
                    fs::write(dest, bytes)?;
                    return Ok(());
                }
                Err(e) => match e.status() {
                    Some(code) if code.as_u16() == 403 => {
                        warn!("Rate limited by nemweb, retrying in 5 seconds");
                        thread::sleep(time::Duration::from_secs(5));
                        continue;
                    }
                    _ => {
                        return Err(anyhow::anyhow!(e));
                    }
                },
            }
        }
    } else {
        info!("File already exists, not downloading: {:?} ", dest);
    }
    Ok(())
}

fn download_file(file_url: &str) -> Result<bytes::Bytes, reqwest::Error> {
    reqwest::blocking::get(file_url)?
        .error_for_status()?
        .bytes()
}

fn get_links(url: &str) -> anyhow::Result<Vec<String>> {
    let doc = reqwest::blocking::get(url)?.text()?;
    log::debug!("Doc: {}", doc);
    let parsed = scraper::Html::parse_document(&doc);

    let links = get_file_links_from_page(parsed);
    info!("Got {} links", links.len());

    Ok(links)
}
