use crate::mms;
use crate::VERSION;
use std::{collections, fs, iter, string, thread, time::Duration};

lazy_static::lazy_static! {
    static ref TR: scraper::Selector = scraper::Selector::parse("tr").unwrap();
    static ref TD: scraper::Selector = scraper::Selector::parse("td").unwrap();
    static ref A: scraper::Selector = scraper::Selector::parse("a").unwrap();
    static ref P: scraper::Selector = scraper::Selector::parse("p").unwrap();
    static ref H3: scraper::Selector = scraper::Selector::parse("h3").unwrap();
    static ref SPAN: scraper::Selector = scraper::Selector::parse("span").unwrap();

}

const BASE_URL: &str = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/MMS%20Data%20Model%20Report_files";

lazy_static::lazy_static! {
    static ref LINK_MATCH: regex::Regex = regex::Regex::new(r"MMS_[0-9]{3}_[0-9]").unwrap();
}

const PACKAGES_TO_SKIP: &[&str] = &["CONFIGURATION", "HISTORICAL_TABLES", "VOLTAGE_INSTRUCTIONS"];

pub async fn run() -> anyhow::Result<()> {
    let url = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/MMS%20Data%20Model%20Report_files/MMS%20Data%20Model%20Report_toc.htm";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        ),
    );
    let client = reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:87.0) Gecko/20100101 Firefox/87.0")
        .default_headers(headers)
        .build()?;
    let body = client.get(url).send().await?.text().await?;
    dbg!(&body);
    let doc = scraper::Html::parse_document(&body);

    let mut info: mms::Packages = collections::BTreeMap::new();
    let mut current_package = None;

    // All the links down the left side of the page
    // which are organized in a nested tree, first layer
    // is the "Package" and inside the package there are
    // multiple "Tables".
    for el in doc.select(&A) {
        dbg!(&current_package);
        let text = el.inner_html();
        if text.starts_with("Package:") {
            dbg!(&text);
            let package = text.split(' ').nth(1).map(string::ToString::to_string);

            // We need to keep the name of the package that we are
            // currently getting tables for
            current_package = package.clone();
        } else if text.starts_with("Table:") {
            // slow down so we don't get rate limited
            thread::sleep(Duration::from_millis(300));

            // We can only deal with a "Table" link
            // if we are dealing with a "Package".
            if let Some(current) = current_package.clone() {
                // get the link and remove everything after the #
                let link_val = el
                    .value()
                    .attr("href")
                    .unwrap()
                    .split(".htm#")
                    .next()
                    .unwrap();

                dbg!(&link_val);

                // some tables have tons of columns and have a paginated
                // column listing, such that for each page we have to ask
                // for the base url, as well as _1, _2, etc, until we
                // stop recieving a successfull HTTP response.
                let mut docs = Vec::new();
                let inner_url = format!("{}/{}.htm", BASE_URL, link_val);

                let res = client.get(&inner_url).send().await?;

                // skip tables from packages we don't want to deal with
                if res.status().as_u16() != 200 {
                    panic!("Link: {}, {}", inner_url, res.status());
                }

                if !PACKAGES_TO_SKIP.contains(&current.as_str()) {
                    let body = res.text().await?;
                    let inner_doc = scraper::Html::parse_document(&body);
                    let mut doc_pages_to_get = Vec::new();

                    for pl in inner_doc.select(&A) {
                        if let Some(href) = pl.value().attr("href") {
                            let page_links = href.split(".htm#").next().unwrap();
                            if LINK_MATCH.captures(page_links).is_some() {
                                doc_pages_to_get.push(page_links.to_string());
                            }
                        }
                    }

                    docs.push(inner_doc);

                    for l in doc_pages_to_get {
                        let get_url = format!("{}/{}", BASE_URL, l);
                        let res = client.get(&get_url).send().await?;

                        if res.status().as_u16() != 200 {
                            panic!("Link a: {}, {}", get_url, res.status());
                        }

                        let body = res.text().await?;
                        let inner_doc = scraper::Html::parse_document(&body);

                        docs.push(inner_doc);
                    }

                    let table_info = mms::TablePage::from_html(docs)?;
                    let key = table_info.get_summary_name();

                    info.entry(current)
                        .and_modify(|e| {
                            e.insert(key.clone(), table_info.clone());
                        })
                        .or_insert_with(|| iter::once((key, table_info)).collect());
                }
            }
        };
    }
    let asstr = serde_json::to_string(&info).unwrap();
    fs::write(format!("mmsdm_v{VERSION}.json"), asstr).unwrap();
    Ok(())
}
