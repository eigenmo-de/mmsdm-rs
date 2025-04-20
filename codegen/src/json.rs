use std::{any, collections::BTreeMap, path::PathBuf, str::FromStr};

use crate::{
    html_tree::{Element, ElementParser},
    mms::{DataType, TableNotes},
};
use anyhow::{anyhow, bail, ensure, Context};
use html5ever::tokenizer::Token::EOFToken;
use log::{error, info};

const PACKAGES_TO_SKIP: &[&str] = &["CONFIGURATION", "HISTORICAL_TABLES", "VOLTAGE_INSTRUCTIONS"];

// fn parse_tables(el: &Element) -> PackageTables {

// }

#[derive(Clone, Debug)]
struct Package {
    comment: String,
    tables: BTreeMap<String, PackageTable>,
}

impl Package {
    fn merge(&mut self, other: Package) -> anyhow::Result<()> {
        for (key, table) in other.tables {
            ensure!(!self.tables.contains_key(&key), "Duplicated key: {key}");

            self.tables.insert(key, table);
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct PackageTable {
    link: String,
    visibility: String,
    comment: String,
}

fn parse_package(parsed: &Element) -> anyhow::Result<(String, Package)> {
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

    info!("Found package: {title}: {description}");

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

#[derive(Clone, Debug)]
struct Table {
    comment: String,
    description: String,
    notes: BTreeMap<String, TableNote>,
    // preserve ordering
    primary_key_columns: Vec<String>,
    // preserve ordering
    index_columns: Vec<String>,
    // preserve ordering
    content: Vec<TableColumn>,
}

#[derive(Clone, Debug)]
struct TableNote {
    comment: String,
    value: String,
}

#[derive(Clone, Debug)]
struct TableColumn {
    name: String,
    data_type: DataType,
    mandatory: bool,
    comment: String,
}

fn parse_tables(parsed: &Element) -> anyhow::Result<BTreeMap<String, Table>> {
    let mut parsed_tables = BTreeMap::new();

    let table_starts = parsed
        .iter_dfs_elements_of_tag("h2")
        .filter(|el| {
            el.iter_dfs_content()
                .next()
                .map(|c| c.starts_with("Table:"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    for table in table_starts {
        // get the next five/six tables: commnet, description, notes, pk, maybe index, content,

        let headings = table
            .iter_dfs_elements_of_tag("h3")
            .map(|el| {
                el.iter_dfs_content()
                    .map(|s| s.as_str())
                    .collect::<String>()
            })
            .take(6)
            .collect::<Vec<_>>();

        let tables = table
            .iter_dfs_elements_of_tag("table")
            .take(6)
            .collect::<Vec<_>>();

        ensure!(
            headings.len() >= 5,
            "Headings of len {} is too short: {:?}",
            headings.len(),
            headings
        );

        // item[0] => comment
        // item[1] => description
        // item[2] => notes
        // item[3] => primary key
        // item[4] => index OR content
        // item[5] => IF item[4] == index, content, ELSE skip

        let name_and_comment = tables[0]
            .iter_dfs_elements_of_tag("td")
            .filter_map(|el| {
                if el.attributes.get("width")? != "75%" {
                    return None;
                };
                el.iter_dfs_content().next().cloned()
            })
            .collect::<Vec<_>>();

        let [name, comment] = name_and_comment.as_slice() else {
            bail!("");
        };

        ensure!(
            headings[1] == "Description",
            "Parsing table {name}: Unexpected heading at [1], should be Description but was {}",
            headings[1]
        );

        let description = tables[1]
            .iter_dfs_content()
            .map(|s| s.as_str())
            .collect::<String>();

        ensure!(
            headings[2] == "Notes",
            "Parsing table {name}: Unexpected heading at [2], should be Notes but was {}",
            headings[2]
        );

        let notes = tables[2]
            .iter_dfs_elements_of_tag("tr")
            .skip(1)
            .map(anyhow::Ok)
            .map(|res| {
                let row = res?;

                let name = row
                    .iter_dfs_elements_of_tag("td")
                    .nth(0)
                    .ok_or_else(|| anyhow!("Missing td at index [0] in table note"))?
                    .iter_dfs_content()
                    .map(|s| s.as_str())
                    .collect::<String>();

                let comment = row
                    .iter_dfs_elements_of_tag("td")
                    .nth(1)
                    .ok_or_else(|| anyhow!("Missing td at index [1] in table note"))?
                    .iter_dfs_content()
                    .map(|s| s.as_str())
                    .collect::<String>();

                let value = row
                    .iter_dfs_elements_of_tag("td")
                    .nth(2)
                    .ok_or_else(|| anyhow!("Missing td at index [2] in table note"))?
                    .iter_dfs_content()
                    .map(|s| s.as_str())
                    .collect::<String>();

                Ok((name, TableNote { comment, value }))
            })
            .collect::<anyhow::Result<BTreeMap<String, TableNote>>>()
            .with_context(|| format!("Error parsing notes for table {name}"))?;

        let (primary_key_columns, index_columns, content) = if headings[3] == "Primary Key Columns"
        {
            let pk = tables[3]
                .iter_dfs_elements_of_tag("tr")
                .skip(1)
                .filter_map(|el| el.iter_dfs_content().next())
                .cloned()
                .collect::<Vec<_>>();

            ensure!(headings[4] == "Content", "");

            (pk, Vec::new(), Vec::new())
        } else {
            let pk = tables[3]
                .iter_dfs_elements_of_tag("tr")
                .skip(1)
                .filter_map(|el| el.iter_dfs_content().next())
                .cloned()
                .collect::<Vec<_>>();

            ensure!(headings[4] == "Index", "");

            let index = tables[4]
                .iter_dfs_elements_of_tag("tr")
                .skip(1)
                .filter_map(|el| el.iter_dfs_content().next())
                .cloned()
                .collect::<Vec<_>>();
            
            ensure!(headings[5] == "content", "");

            (pk, index, Vec::new())
        };

        parsed_tables.insert(
            name.to_string(),
            Table {
                comment: comment.to_string(),
                description,
                notes,
                primary_key_columns,
                index_columns,
                content,
            },
        );
    }

    Ok(parsed_tables)
}

// page types
// a) package:X
// b) package... continued
// c) table
// d) table... continued

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FileNameParts {
    number: u16,
    extra: Option<u16>,
}

impl FromStr for FileNameParts {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(
            s.starts_with("Elec") && s.ends_with(".htm"),
            "Invalid file name expected `Elec<items>.htm` but got `{s}`"
        );

        let Some((_, name)) = s.split_once("Elec") else {
            bail!("");
        };

        let removed_htm = name.replace(".htm", "");

        match removed_htm.split_once('_') {
            Some((number, extra)) => Ok(FileNameParts {
                number: number
                    .parse()
                    .with_context(|| format!("Unexpected first number `{number}` in name `{s}`"))?,
                extra: Some(extra.parse().with_context(|| {
                    format!("Unexpected second number `{extra}` in name `{s}`")
                })?),
            }),
            None => Ok(FileNameParts {
                number: removed_htm
                    .parse()
                    .with_context(|| format!("Unexpected number `{removed_htm}` in name `{s}`"))?,
                extra: None,
            }),
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    let mut files = BTreeMap::<FileNameParts, PathBuf>::new();
    let mut readdir = tokio::fs::read_dir("./cache").await?;

    while let Some(item) = readdir.next_entry().await? {
        let path = item.path();
        files.insert(
            path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow!("Unexpected file name {}", path.display()))?
                .parse()?,
            path,
        );
    }

    let mut packages = BTreeMap::<String, Package>::new();

    for (_, file) in files.iter().take(10) {
        // go through all cached files
        // and try to parse a package from each file.

        info!("attempting to parse package from: {}", file.display());
        let package_file = tokio::fs::read_to_string(file).await?;

        let parsed = ElementParser::parse_from_string(package_file)?;

        // the title

        match parse_package(&parsed) {
            Ok((name, package)) if PACKAGES_TO_SKIP.contains(&name.as_str()) => {
                info!(
                    "Skipping package: {name} with {} tables",
                    package.tables.len()
                );
                continue;
            }
            Ok((name, package)) => match packages.get_mut(&name) {
                Some(existing) => {
                    existing.merge(package)?;
                }
                None => {
                    packages.insert(name, package);
                }
            },
            Err(e) => {
                let t = parse_tables(&parsed)?;
                dbg!(t);
                // bail!("Path: `{}`, Error parsing package: {e:?}", file.display());
            }
        }
    }

    // dbg!(packages);

    Ok(())
}
