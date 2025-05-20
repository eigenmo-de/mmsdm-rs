use anyhow::Context;
use anyhow::anyhow;
use anyhow::bail;
use log::info;
use reqwest::StatusCode;
use reqwest::Url;

use crate::html_tree::ElementParser;
use crate::json::FileNameParts;

use std::collections::BTreeMap;

use std::time::Duration;

pub async fn run() -> anyhow::Result<()> {
    let mut files = Vec::<FileNameParts>::new();
    tokio::fs::create_dir_all("./cache").await?;
    let mut readdir = tokio::fs::read_dir("./cache").await?;

    while let Some(item) = readdir.next_entry().await? {
        let path = item.path();
        files.push(
            path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow!("Unexpected file name {}", path.display()))?
                .parse()?,
        );
    }

    let base = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/Electricity%20Data%20Model%20Report_files";
    let base_url = base.parse::<Url>()?;
    let url = {
        let mut x = base_url.clone();
        x.path_segments_mut().unwrap().push("Elec2.htm");
        x
    };

    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        ),
    );

    let client = reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36 Edg/133.0.0.0")
        .default_headers(headers)
        .build()?;

    let body = client.get(url).send().await?.text().await?;

    let tree = ElementParser::parse_from_string(body).context("parsing elements")?;

    let mut package_with_table_urls = BTreeMap::<usize, (String, Url)>::new();

    let first_table = tree
        .iter_dfs_elements_of_tag("table")
        .next()
        .ok_or_else(|| anyhow!("No table found"))?;

    for a in first_table.iter_dfs_elements_of_tag("a") {
        let content = a
            .children
            .get(0)
            .and_then(|n| n.content())
            .ok_or_else(|| anyhow!("Expected content missing"))?;
        let url_str = a
            .attributes
            .get("href")
            .ok_or_else(|| anyhow!("Missing href attr for {content}"))?
            .replace("#1", "");

        let url_str_number = url_str
            .replace("Elec", "")
            .replace(".htm", "")
            .parse::<usize>()
            .with_context(|| format!("Parsing number from {url_str}"))?;
        package_with_table_urls.insert(url_str_number, {
            let mut x = base_url.clone();
            x.path_segments_mut().unwrap().push(&url_str);
            (content.to_string(), x)
        });
    }

    for (current_idx, (_, _)) in package_with_table_urls.iter() {
        info!("Proposing to download file with index: {current_idx}");

        if files
            .iter()
            .find(|fnp| usize::from(fnp.number) == *current_idx)
            .is_some()
        {
            continue;
        }

        // this finds the baseline URL
        // we then need iterate by 1 to find the data url
        // then we need to iterate with _1, _2, etc, then iterate the top level by 1, continuing until we reach the next url.

        // option 2 is: assume that the _n never goes above ... 3?
        // and assume that after the last item, we never have to go past say, 80.
        // then just grab all the urls.

        let next_idx = package_with_table_urls
            .keys()
            .filter(|x| *x > current_idx)
            .next()
            .copied()
            .unwrap_or_else(|| current_idx + 5);

        for detail_idx in *current_idx..next_idx {
            if detail_idx > 78 {
                break;
            }

            for underscore_idx in 0..=5 {
                // try to get the detail page

                let target_path = if underscore_idx == 0 {
                    format!("Elec{detail_idx}.htm")
                } else {
                    format!("Elec{detail_idx}_{underscore_idx}.htm")
                };

                let target_url = {
                    let mut x = base_url.clone();
                    x.path_segments_mut().unwrap().push(&target_path);
                    x
                };
                tokio::time::sleep(Duration::from_millis(1000)).await;

                let resp = client.get(target_url.clone()).send().await?;

                match resp.status() {
                    StatusCode::OK => (),
                    StatusCode::NOT_FOUND => {
                        continue;
                    }
                    other => {
                        bail!("Unexpected status code for request at {target_url}: {other}");
                    }
                }

                let html = resp.text().await?;
                info!("Got data from {target_path}");

                tokio::fs::write(&format!("./cache/{target_path}"), html).await?;
                // parse the html
            }
        }
    }

    Ok(())
}
