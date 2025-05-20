use std::{collections::BTreeMap, ffi::OsStr, path::PathBuf, str::FromStr};

use crate::{
    html_tree::{Element, ElementParser},
    mms::{DataType, TableColumn},
};
use anyhow::{Context, anyhow, bail, ensure};
use indexmap::IndexMap;
use log::{info, warn};
use serde::{Deserialize, Serialize};

const PACKAGES_TO_SKIP: &[&str] = &[
    "CONFIGURATION",
    "HISTORICAL_TABLES",
    "VOLTAGE_INSTRUCTIONS",
    "FPP",
    "PD7DAY",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub comment: String,
    pub tables: IndexMap<String, PackageTable>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageTable {
    pub link: String,
    pub visibility: String,
    pub comment: String,
}

fn parse_package_tables(parsed: &Element) -> anyhow::Result<IndexMap<String, PackageTable>> {
    let mut tables = IndexMap::new();

    for tr in parsed.iter_dfs_elements_of_tag("tr") {
        let Ok([name, comment, visibility]) = <[String; 3]>::try_from(
            tr.iter_dfs_elements_of_tag("td")
                .map(|el| el.concat_dfs_content())
                .collect::<Vec<_>>(),
        ) else {
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

    ensure!(!tables.is_empty(), "Unable to parse any package tables");

    Ok(tables)
}

fn parse_package(parsed: &Element) -> anyhow::Result<(String, Package)> {
    let form = parsed
        .iter_dfs_elements_of_tag("table")
        .filter(|e| e.attributes.get("class").map(|x| x.as_str()) == Some("Form"))
        .next()
        .ok_or_else(|| anyhow!("Missing Form table"))?;

    let Ok([title, description]) = <[String; 2]>::try_from(
        form.iter_dfs_elements_of_tag("tr")
            .filter_map(|tr| Some(tr.children.get(1)?.element()?.concat_dfs_content()))
            .collect::<Vec<_>>(),
    ) else {
        bail!("no `tr` with package name/comment available")
    };

    info!("Found package: {title}: {description}");

    let tables_grid = parsed
        .iter_dfs_elements_of_tag("table")
        .filter(|e| e.attributes.get("class").map(|x| x.as_str()) == Some("Grid"))
        .next()
        .ok_or_else(|| anyhow!("Missing Grid table"))?;

    let tables = parse_package_tables(tables_grid)?;

    Ok((
        title.clone(),
        Package {
            comment: description.clone(),
            tables,
        },
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub comment: String,
    pub description: String,
    pub notes: IndexMap<String, TableNote>,
    // preserve ordering
    pub primary_key_columns: Vec<String>,
    // preserve ordering
    pub index_columns: (Vec<String>, Vec<String>, Vec<String>),
    // preserve ordering
    // must be accessed via the `columns()` method
    content: Vec<TableColumn>,
}

impl Table {
    pub fn has_any_string_columns(&self) -> bool {
        self.content
            .iter()
            .any(|c| matches!(c.data_type, DataType::Char | DataType::Varchar { .. }))
    }
    pub fn columns(&self) -> impl Iterator<Item = TableColumn> {
        // if intervention exists, it must be mandatory
        // as it must also be in the primary key

        self.content.clone().into_iter().map(|mut c| {
            if c.name.to_lowercase() == "intervention" {
                c.mandatory = true;
            };
            c
        })
    }

    // pub fn frequency(&self) -> TableFrequency {
    //     // special cases
    //     if self.summary.name == "NEGATIVE_RESIDUE" {
    //         return TableFrequency::FiveMinute;
    //     }

    //     // general 5min cases
    //     if ["DISPATCH", "P5MIN"]
    //         .iter()
    //         .any(|n| self.summary.name.starts_with(n))
    //     {
    //         return TableFrequency::FiveMinute;
    //     }

    //     // general 30min cases
    //     if ["PREDISPATCH", "ROOFTOP"]
    //         .iter()
    //         .any(|n| self.summary.name.starts_with(n))
    //     {
    //         return TableFrequency::HalfHourly;
    //     }

    //     // fallback
    //     // this should especially be used for cases where the resolution has changed over time (eg from 30 to 5)
    //     TableFrequency::Unknown
    // }

    // pub fn find_column(&self, name: &str) -> ControlFlow<TableColumn, ()> {
    //     // make sure in pk
    //     if !self
    //         .primary_key_columns()
    //         .cols
    //         .iter()
    //         .any(|c| c.to_lowercase() == name)
    //     {
    //         return ControlFlow::Continue(());
    //     }

    //     if let Some(col) = self
    //         .columns
    //         .columns
    //         .iter()
    //         .find(|c| c.name.to_lowercase() == name)
    //     {
    //         // make sure mandatory
    //         if !col.mandatory {
    //             return ControlFlow::Continue(());
    //         }

    //         return ControlFlow::Break(col.clone());
    //     }

    //     ControlFlow::Continue(())
    // }

    // pub fn partition_column(&self) -> ControlFlow<TableColumn, ()> {
    //     // in preference order
    //     self.find_column("predispatchseqno")?;
    //     self.find_column("effectivedate")?;
    //     self.find_column("tradingdate")?;
    //     self.find_column("interval_datetime")?;
    //     self.find_column("settlementdate")?;
    //     self.find_column("day")?;
    //     self.find_column("offerdate")?;
    //     // these specific tables have `Integer(3 or 4)` periodid which represents the relative interval only
    //     if !["IRFMAMOUNT", "REALLOCATIONINTERVAL"].contains(&self.summary.name.as_str()) {
    //         self.find_column("periodid")?;
    //     }
    //     self.find_column("datetime")?;

    //     // these are more like transaction time so
    //     // they are last preference
    //     self.find_column("run_datetime")?;
    //     self.find_column("offerdatetime")?;
    //     self.find_column("publish_datetime")?;

    //     ControlFlow::Continue(())
    // }

    pub fn primary_key_columns(&self) -> Vec<String> {
        let mut base = self.primary_key_columns.clone();

        // if intervention exists, it must be in the primary key
        if let Some(intervention) = self
            .columns()
            .find(|c| c.name.to_lowercase() == "intervention")
        {
            if !base.contains(&intervention.name) {
                base.push(intervention.name.to_string());
            }
        }

        base
    }
    // pub fn get_summary_name(&self) -> String {
    //     self.summary.get_name()
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNote {
    pub comment: String,
    pub value: String,
}

fn parse_name_and_comment(table: &Element) -> anyhow::Result<(String, String)> {
    let name_and_comment = table
        .iter_dfs_elements_of_tag("td")
        .filter_map(|el| {
            if el.attributes.get("width")? != "75%" {
                return None;
            };
            Some(el.concat_dfs_content())
        })
        .collect::<Vec<_>>();

    let [name, comment] = name_and_comment.as_slice() else {
        bail!("Unable to find name and comment within data: {name_and_comment:?}");
    };

    Ok((name.to_string(), comment.to_string()))
}

fn parse_description(table: &Element) -> String {
    table.concat_dfs_content()
}

fn parse_notes(table: &Element) -> anyhow::Result<IndexMap<String, TableNote>> {
    table
        .iter_dfs_elements_of_tag("tr")
        .skip(1)
        .map(anyhow::Ok)
        .map(|res| {
            let row = res?;

            let name = row
                .iter_dfs_elements_of_tag("td")
                .nth(0)
                .ok_or_else(|| anyhow!("Missing td at index [0] in table note"))?
                .concat_dfs_content();

            let comment = row
                .iter_dfs_elements_of_tag("td")
                .nth(1)
                .ok_or_else(|| anyhow!("Missing td at index [1] in table note"))?
                .concat_dfs_content();

            let value = row
                .iter_dfs_elements_of_tag("td")
                .nth(2)
                .ok_or_else(|| anyhow!("Missing td at index [2] in table note"))?
                .concat_dfs_content();

            Ok((name, TableNote { comment, value }))
        })
        .collect()
}

fn parse_column_names(table: &Element) -> Vec<String> {
    table
        .iter_dfs_elements_of_tag("tr")
        .skip(1)
        .map(|el| el.concat_dfs_content())
        .collect::<Vec<_>>()
}

fn parse_content(table: &Element) -> anyhow::Result<Vec<TableColumn>> {
    let mut items = Vec::new();

    for row in table.iter_dfs_elements_of_tag("tr").skip(1) {
        let mut parts = Vec::with_capacity(4);

        for column in row.iter_dfs_elements_of_tag("td") {
            parts.push(column.concat_dfs_content());
        }

        match <[String; 4]>::try_from(parts) {
            Ok([name, data_type, mandatory, comment]) => items.push(TableColumn {
                name,
                data_type: data_type.parse()?,
                mandatory: mandatory.contains("X"),
                comment,
            }),
            Err(e) => {
                bail!("Unexpected content row, should have four items: {e:?}",);
            }
        }
    }
    Ok(items)
}

fn parse_package_continued(parsed: &Element) -> anyhow::Result<IndexMap<String, PackageTable>> {
    ensure!(
        parsed
            .iter_dfs_elements_of_tag("h3")
            .map(|el| el.concat_dfs_content())
            .next()
            .unwrap_or_default()
            .starts_with("List of tables"),
        "Unable to find 'List of tables' heading in package-continued page"
    );

    let tables_grid = parsed
        .iter_dfs_elements_of_tag("table")
        .filter(|e| e.attributes.get("class").map(|x| x.as_str()) == Some("Grid"))
        .next()
        .ok_or_else(|| anyhow!("Missing Grid table"))?;

    parse_package_tables(tables_grid)
}

fn parse_content_continued(parsed: &Element) -> anyhow::Result<Vec<TableColumn>> {
    ensure!(
        parsed
            .iter_dfs_elements_of_tag("h3")
            .next()
            .map(|e| e.concat_dfs_content())
            .unwrap_or_default()
            .contains("Content"),
        "Missing `Content` header"
    );

    let table_el = parsed
        .iter_dfs_elements_of_tag("table")
        .next()
        .ok_or_else(|| anyhow!("No table element available"))?;

    parse_content(table_el)
}

fn parse_tables(parsed: &Element) -> anyhow::Result<IndexMap<String, Table>> {
    let mut parsed_tables = IndexMap::new();

    let body_el = parsed
        .iter_dfs_elements_of_tag("body")
        .next()
        .ok_or_else(|| anyhow!("Missing body element"))?;

    let table_starts = body_el
        .iter_child_elements()
        .enumerate()
        .filter_map(|(idx, el)| {
            if el.name == "h2" && el.concat_dfs_content().starts_with("Table:") {
                Some(idx)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    for table in table_starts {
        // get the next five/six tables: commnet, description, notes, pk, maybe index, content,

        let headings = body_el
            .iter_child_elements()
            .skip(table)
            .filter(|el| el.name == "h3")
            .map(|el| el.concat_dfs_content())
            .take(8)
            .collect::<Vec<_>>();

        let tables = body_el
            .iter_child_elements()
            .skip(table)
            .filter(|el| el.name == "table")
            .take(8)
            .collect::<Vec<_>>();

        // item[0] => comment
        // item[1] => description
        // item[2] => notes
        // item[3] => primary key
        // item[4] => index OR content
        // item[5] => IF item[4] == index, content, ELSE skip

        let (name, comment) = parse_name_and_comment(tables[0])?;

        let find_heading = |heading, nth| {
            headings
                .iter()
                .enumerate()
                .filter(|(_, h)| h.starts_with(heading))
                .nth(nth)
                .ok_or_else(|| {
                    anyhow!(
                        "No {heading} heading available for table {name} in headings {headings:?}"
                    )
                })
                .map(|(idx, _)| idx)
        };

        let description_index = find_heading("Description", 0).ok();
        let notes_index = find_heading("Notes", 0)?;
        let primary_key_columns_index = find_heading("Primary Key Columns", 0)?;
        let index_columns_index_1st = find_heading("Index Columns", 0).ok();
        let index_columns_index_2nd = find_heading("Index Columns", 1).ok();
        let index_columns_index_3rd = find_heading("Index Columns", 2).ok();
        let content_index = find_heading("Content", 0)?;

        let description = match description_index {
            Some(idx) => parse_description(tables[idx]),
            None => String::new(),
        };
        let notes = parse_notes(tables[notes_index])
            .with_context(|| format!("Error parsing notes for table {name}"))?;
        let primary_key_columns = parse_column_names(tables[primary_key_columns_index]);
        let index_1st = match index_columns_index_1st {
            Some(idx) => parse_column_names(tables[idx]),
            None => Vec::new(),
        };
        let index_2nd = match index_columns_index_2nd {
            Some(idx) => parse_column_names(tables[idx]),
            None => Vec::new(),
        };
        let index_3rd = match index_columns_index_3rd {
            Some(idx) => parse_column_names(tables[idx]),
            None => Vec::new(),
        };
        let content = parse_content(tables[content_index])?;

        // if name == "PREDISPATCH_MNSPBIDTRK" {
        //     println!("{}", tables[content_index]);

        //     println!("{:#?}", content);
        //     panic!("{}", content.len());
        // }

        parsed_tables.insert(
            name,
            Table {
                comment,
                description,
                notes,
                primary_key_columns,
                index_columns: (index_1st, index_2nd, index_3rd),
                content,
            },
        );
    }

    ensure!(!parsed_tables.is_empty(), "Unable to parse any tables");

    Ok(parsed_tables)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileNameParts {
    pub number: u16,
    pub extra: Option<u16>,
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

pub async fn run() -> anyhow::Result<DataModel> {
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

    let mut packages = IndexMap::<String, Package>::new();
    let mut tables = IndexMap::<String, Table>::new();

    for (_, file) in files.iter() {
        // go through all cached files
        // and try to parse a package from each file.

        if file.file_name() == Some(&OsStr::new("Elec63_1.htm")) {
            continue;
        }

        info!("attempting to parse package from: {}", file.display());
        let package_file = tokio::fs::read_to_string(file).await?;

        let parsed = ElementParser::parse_from_string(package_file)?;

        if parsed
            .iter_dfs_elements_of_tag("h2")
            .next()
            .map(|e| e.concat_dfs_content())
            .unwrap_or_default()
            .starts_with("Diagram: Entities:")
        {
            info!("Diagram only page in file {}, skipping", file.display());
            continue;
        }

        // page types
        // a) package:X
        // b) package... continued
        // c) table
        // d) table... continued

        match parse_package(&parsed) {
            Ok((name, package)) if PACKAGES_TO_SKIP.contains(&name.as_str()) => {
                info!(
                    "Skipping package: {name} with {} tables",
                    package.tables.len()
                );
                continue;
            }
            Ok((name, package)) => {
                let len = package.tables.len();
                match packages.get_mut(&name) {
                    Some(existing) => {
                        existing.merge(package)?;
                    }
                    None => {
                        packages.insert(name, package);
                    }
                };
                info!(
                    "Successfully parsed page {} as package with {len} tables headings",
                    file.display()
                );
                continue;
            }
            Err(e) => {
                warn!(
                    "Issue parsing page {} as package, trying to parse as tables. Error: {e:?}",
                    file.display()
                );
            }
        }

        match parse_package_continued(&parsed) {
            Ok(continued) => {
                info!(
                    "Successfully parsed page {} as package-continued with {} tables headings",
                    file.display(),
                    continued.len()
                );

                let Some((_, last_package)) = packages.last_mut() else {
                    bail!("No last package available");
                };

                for (n, pt) in continued {
                    let name = n.clone();
                    if let Some(_) = last_package.tables.insert(n, pt) {
                        bail!("package {}: Duplicate table {name}", last_package.comment);
                    }
                }

                continue;
            }
            Err(e) => {
                warn!(
                    "Issue parsing page {} as package-continued, trying to parse as tables. Error: {e:?}",
                    file.display()
                );
            }
        }

        match parse_tables(&parsed) {
            Ok(new_tables) => {
                info!(
                    "Successfully parsed page {} as tables with {} tables schemas",
                    file.display(),
                    new_tables.len()
                );

                for (key, new) in new_tables {
                    if tables.contains_key(&key) {
                        warn!("Found duplicate table: {key}")
                    } else {
                        tables.insert(key.clone(), new);
                    }
                }

                continue;
            }
            Err(e) => {
                warn!(
                    "Issue parsing page {} as tables, trying to parse as content-continued. Error: {e:?}",
                    file.display()
                );
            }
        }

        match parse_content_continued(&parsed) {
            Ok(continued) => {
                info!(
                    "Successfully parsed page {} as content-continued with {} columns",
                    file.display(),
                    continued.len()
                );

                let Some((name, last_table)) = tables.last_mut() else {
                    bail!("No last package available");
                };

                warn!("Extending table {name}");

                last_table.content.extend(continued);

                continue;
            }
            Err(e) => {
                warn!(
                    "Issue parsing page {} as content-continued, giving up. Error: {e:?}",
                    file.display()
                );
            }
        }

        bail!("Unable to parse page {}", file.display());
    }

    Ok(DataModel { packages, tables })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataModel {
    pub packages: IndexMap<String, Package>,
    pub tables: IndexMap<String, Table>,
}
