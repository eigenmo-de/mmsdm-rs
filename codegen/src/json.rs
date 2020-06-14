use std::{collections, fs, iter, string};
use crate::mms;

lazy_static::lazy_static! {
    static ref TR: scraper::Selector = scraper::Selector::parse("tr").unwrap();
    static ref TD: scraper::Selector = scraper::Selector::parse("td").unwrap();
    static ref A: scraper::Selector = scraper::Selector::parse("a").unwrap();
    static ref P: scraper::Selector = scraper::Selector::parse("p").unwrap();
    static ref H3: scraper::Selector = scraper::Selector::parse("h3").unwrap();
    static ref SPAN: scraper::Selector = scraper::Selector::parse("span").unwrap();

}

const BASE_URL: &str = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/MMS%20Data%20Model%20Report_files";
pub async fn run() -> anyhow::Result<()> {
    let url = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/MMS%20Data%20Model%20Report_files/MMS%20Data%20Model%20Report_toc.htm";
    let body = reqwest::get(url).await?.text().await?;
    let doc = scraper::Html::parse_document(&body);

    let mut info: mms::Packages = collections::HashMap::new();
    let mut current_package = None;

    // All the links down the left side of the page
    // which are organized in a nested tree, first layer
    // is the "Package" and inside the package there are
    // multiple "Tables". 
    for el in doc.select(&A) {
        dbg!(&current_package);
        let text = el.inner_html();
        if text.starts_with("Package:") {
            let package = text.split(" ").nth(1).map(string::ToString::to_string);

            // We need to keep the name of the package that we are
            // currently getting tables for
            current_package = package.clone();
        } else if text.starts_with("Table:") {

            // We can only deal with a "Table" link
            // if we are dealing with a "Package".
            if let Some(current) = current_package.clone() {
                // get the link and remove everything after the #
                let link_val = el.value().attr("href").unwrap().split(".htm#").nth(0).unwrap();
                dbg!(&link_val);
                // some tables have tons of columns and have a paginated 
                // column listing, such that for each page we have to ask
                // for the base url, as well as _1, _2, etc, until we 
                // stop recieving a successfull HTTP response.
                let mut docs = Vec::new();
                for i in 0..=100_usize {
                    let url = if i == 0 {
                        format!("{}/{}.htm", BASE_URL, link_val)
                    } else {
                        format!("{}/{}_{}.htm", BASE_URL, link_val, i)
                    };
                    dbg!(&url);
                    let res = reqwest::get(&url).await?;
                    dbg!(res.status());
                    if res.status().as_u16() == 200 {
                        let body = res.text().await?;
                        let doc = scraper::Html::parse_document(&body);
                        docs.push(doc);
                        if i > 0 {
                            dbg!(body);
                        }
                    } else {
                        break;
                    };
                }

                //let body = reqwest::get(&url).await?.text().await?;
                //let doc = scraper::Html::parse_document(&body);
                let table_info = mms::TablePage::from_html(docs)?;
                let key = table_info.get_summary_name();
                info.entry(current)
                    .and_modify(|e| {
                        e.insert(key.clone(), table_info.clone());
                    })
                    .or_insert(iter::once((key, table_info)).collect());
            }

            // tmp
            //break;
        };
    }
    let asstr = serde_json::to_string(&info).unwrap();
    fs::write("mmsdm.json", asstr).unwrap();
    Ok(())
}
