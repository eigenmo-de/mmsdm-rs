

use crate::html_tree::ElementParser;



const PACKAGES_TO_SKIP: &[&str] = &["CONFIGURATION", "HISTORICAL_TABLES", "VOLTAGE_INSTRUCTIONS"];


fn parse_tables(el: &Element) -> PackageTables {

}

pub async fn run() -> anyhow::Result<()> {
    let mut files = Vec::new();
    let mut readdir =  tokio::fs::read_dir("./cache").await?;

    while let Some(item) = readdir.next_entry().await? {
        
        files.push(item.path());
    }



    let mut current_package = String::new();




    let package_file = tokio::fs::read_to_string("./cache/Elec5.htm").await?;

    let parsed = ElementParser::parse_from_string(package_file)?;


    for el in parsed.iter_dfs().take(25) {
        match el {
            crate::html_tree::ElementOrContent::Content(c) => {
                println!("{c}");
            }
            crate::html_tree::ElementOrContent::Element(element) => {
                println!("{}", element.name);
            }
        }
        
    }

    // find all the package pages (including extras...)

    // find all the table pages (including extras)
    
    // concat the table pages into reasonable groups

    // println!("{}", parsed);





    // for (package_name, mut url) in data
    //     .into_iter()
    //     .filter(|(k, _)| !PACKAGES_TO_SKIP.contains(&k.as_str()))
    // {
    //     let urls = [url.clone(), {
    //         // secondary url that is _sometimes_ needed
    //         url.set_path(&url.path().replace(".htm", "_1.htm"));
    //         url
    //     }];

    //     for url in urls {
    //         info!("Getting data for pacakge {package_name} at {url}");

    //         let req = client.get(url.clone()).send().await?;

    //         if req.status().as_u16() == 404 {
    //             info!("Didn't find _1 url for {package_name} - not all packages have a _1 url so this is normal.");
    //             // the only case we can continue on error
    //             continue;
    //         }

    //         let data = req
    //             .error_for_status()
    //             .context(package_name.to_string())?
    //             .text()
    //             .await?;

    //         let parsed = TablePageParserCell::parse_from_string(data)
    //             .with_context(|| format!("Parsing data for {package_name}"))?;

    //         match package_with_table_urls.get_mut(&package_name) {
    //             Some(existing) => {
    //                 existing.extend(parsed);
    //             }
    //             None => {
    //                 package_with_table_urls.insert(package_name.clone(), parsed);
    //             }
    //         }
    //         // avoid getting rate limited...
    //         tokio::time::sleep(Duration::from_millis(500)).await;
    //     }
    // }

    // All the links down the left side of the page
    // which are organized in a nested tree, first layer
    // is the "Package" and inside the package there are
    // multiple "Tables".

    // for el in doc.select(&A) {
    //     dbg!(&current_package);
    //     let text = el.inner_html();
    //     dbg!(&text);
    //     if text.starts_with("Package:") {
    //         dbg!(&text);
    //         let package = text.split(' ').nth(1).map(string::ToString::to_string);

    //         // We need to keep the name of the package that we are
    //         // currently getting tables for
    //         current_package = package.clone();
    //     } else if text.starts_with("Table:") {
    //         // slow down so we don't get rate limited
    //         thread::sleep(Duration::from_millis(300));

    //         // We can only deal with a "Table" link
    //         // if we are dealing with a "Package".
    //         if let Some(current) = current_package.clone() {
    //             // get the link and remove everything after the #
    //             let link_val = el
    //                 .value()
    //                 .attr("href")
    //                 .unwrap()
    //                 .split(".htm#")
    //                 .next()
    //                 .unwrap();

    //             dbg!(&link_val);

    //             // some tables have tons of columns and have a paginated
    //             // column listing, such that for each page we have to ask
    //             // for the base url, as well as _1, _2, etc, until we
    //             // stop recieving a successfull HTTP response.
    //             let mut docs = Vec::new();
    //             let inner_url = format!("{}/{}.htm", BASE_URL, link_val);

    //             let res = client.get(&inner_url).send().await?;

    //             // skip tables from packages we don't want to deal with
    //             if res.status().as_u16() != 200 {
    //                 panic!("Link: {}, {}", inner_url, res.status());
    //             }

    //             if !PACKAGES_TO_SKIP.contains(&current.as_str()) {
    //                 let body = res.text().await?;
    //                 let inner_doc = scraper::Html::parse_document(&body);
    //                 let mut doc_pages_to_get = Vec::new();

    //                 for pl in inner_doc.select(&A) {
    //                     if let Some(href) = pl.value().attr("href") {
    //                         let page_links = href.split(".htm#").next().unwrap();
    //                         if LINK_MATCH.captures(page_links).is_some() {
    //                             doc_pages_to_get.push(page_links.to_string());
    //                         }
    //                     }
    //                 }

    //                 docs.push(inner_doc);

    //                 for l in doc_pages_to_get {
    //                     let get_url = format!("{}/{}", BASE_URL, l);
    //                     let res = client.get(&get_url).send().await?;

    //                     if res.status().as_u16() != 200 {
    //                         panic!("Link a: {}, {}", get_url, res.status());
    //                     }

    //                     let body = res.text().await?;
    //                     let inner_doc = scraper::Html::parse_document(&body);

    //                     docs.push(inner_doc);
    //                 }

    //                 let table_info = mms::TablePage::from_html(docs)?;
    //                 let key = table_info.get_summary_name();

    //                 info.entry(current)
    //                     .and_modify(|e| {
    //                         e.insert(key.clone(), table_info.clone());
    //                     })
    //                     .or_insert_with(|| iter::once((key, table_info)).collect());
    //             }
    //         }
    //     };
    // }
    // let asstr = serde_json::to_string(&info).unwrap();
    // fs::write(format!("mmsdm_v{VERSION}.json"), asstr).unwrap();
    Ok(())
}
