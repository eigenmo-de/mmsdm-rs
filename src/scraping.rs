use std::path;

use scraper::{html, Selector};//, node, selector};

use crate::{Result, BackendError};

pub fn get_filename_from_path(path: &path::Path) -> Result<String> {
    let file_stem = path.file_stem().and_then(|s| s.to_str());
    let extension = path.extension().and_then(|e| e.to_str());
    if let (Some(file), Some(ext)) = (file_stem, extension) {
        Ok(format!("{}.{}", file, ext))
    } else {
        Err(BackendError::InvalidFilePath)
    }
}

pub fn get_file_links_from_page(doc: html::Html) -> Vec<String> {
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
