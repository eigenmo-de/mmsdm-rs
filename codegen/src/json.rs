use std::collections::BTreeMap;

use crate::html_tree::{Element, ElementParser};
use anyhow::{anyhow, bail};
use log::{error, info};

const PACKAGES_TO_SKIP: &[&str] = &["CONFIGURATION", "HISTORICAL_TABLES", "VOLTAGE_INSTRUCTIONS"];

// fn parse_tables(el: &Element) -> PackageTables {

// }

#[derive(Clone, Debug)]
struct Package {
    comment: String,
    tables: BTreeMap<String, PackageTable>,
}

#[derive(Clone, Debug)]
struct PackageTable {
    link: String,
    visibility: String,
    comment: String,
}

pub fn parse_package(parsed: &Element) -> anyhow::Result<(String, Package)> {
    let form = parsed
        .iter_dfs_elements_of_tag("table")
        .filter(|e| e.attributes.get("class").map(|x| x.as_str()) == Some("Form"))
        .next()
        .ok_or_else(|| anyhow!("Missing Form table"))?;

    let [title, description] = form
        .iter_dfs_elements_of_tag("tr")
        .filter_map(|tr| Some(tr.children.get(1)?.element()?.iter_dfs_content().next()?))
        .collect::<Vec<_>>()[..]
    else {
        bail!("no `tr` with package name/comment available")
    };

    dbg!(title, description);

    let tables_grid = parsed
        .iter_dfs_elements_of_tag("table")
        .filter(|e| e.attributes.get("class").map(|x| x.as_str()) == Some("Grid"))
        .next()
        .ok_or_else(|| anyhow!("Missing Grid table"))?;

    let mut tables = BTreeMap::new();

    for tr in tables_grid.iter_dfs_elements_of_tag("tr") {
        let [name, comment, visibility] = tr.iter_dfs_content().collect::<Vec<_>>()[..] else {
            bail!("unable to extract three columns from row: {tr:?}")
        };

        if name == "Name" {
            continue;
        }

        let Some(a_element) = tr.iter_dfs_elements_of_tag("a").next() else {
            bail!("no `a` element in row: {tr:?}")
        };
        let Some(link) = a_element.attributes.get("href").cloned() else {
            bail!("`a` element present but no href attribute in row: {tr:?}")
        };

        tables.insert(
            name.clone(),
            PackageTable {
                link,
                visibility: visibility.clone(),
                comment: comment.clone(),
            },
        );
    }

    Ok((
        title.clone(),
        Package {
            comment: description.clone(),
            tables,
        },
    ))
}

pub async fn run() -> anyhow::Result<()> {
    let mut files = Vec::new();
    let mut readdir = tokio::fs::read_dir("./cache").await?;

    while let Some(item) = readdir.next_entry().await? {
        files.push(item.path());
    }

    let mut packages = BTreeMap::new();

    for file in files.iter() {
        // go through all cached files
        // and try to parse a package from each file.

        info!("attempting to parse package from: {}", file.display());
        let package_file = tokio::fs::read_to_string(file).await?;

        let parsed = ElementParser::parse_from_string(package_file)?;

        // the title

        match parse_package(&parsed) {
            Ok((name, package)) if PACKAGES_TO_SKIP.contains(&name.as_str()) => {
                info!("Skipping package: {name} with {} tables", package.tables.len());
                continue;
            }
            Ok((name, package)) => {

                match packages.get_mut(key)
                packages.insert(name, package);
            }
            Err(e) => {
                error!("Error parsing package: {e:?}");
                continue;
            }
        }
    }

    dbg!(packages);

    Ok(())
}
